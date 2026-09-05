import { useState } from "react";
import ReactDOM from "react-dom/client";
import { invoke } from "@tauri-apps/api/core";
import { decodeScene, toBytes, type ScenePacket } from "../types";
import "./styles.css";

type Size = "S" | "M" | "L";
type CandidateName = "native-tauri" | "wasm-bindgen";

interface Candidate {
  init(): Promise<void>;
  revision(): Promise<number>;
  scene(size: Size): Promise<Uint8Array>;
  preview(sample: number): Promise<Uint8Array>;
  commit(sample: number): Promise<number>;
  reset(): Promise<number>;
  stale(): Promise<void>;
}

class NativeCandidate implements Candidate {
  async init() {
    await invoke("bridge_reset");
  }
  async revision() { return invoke<number>("bridge_revision"); }
  async scene(size: Size) { return toBytes(await invoke<unknown>("bridge_scene", { size })); }
  async preview(sample: number) { return toBytes(await invoke<unknown>("bridge_preview_scene", { sample })); }
  async commit(sample: number) { return invoke<number>("bridge_commit", { sample }); }
  async reset() { return invoke<number>("bridge_reset"); }
  async stale() { await invoke("bridge_stale_probe"); }
}

class WasmCandidate implements Candidate {
  private constructor(private readonly session: { revision(): bigint; scene(size: string): Uint8Array; preview_scene(sample: number): Uint8Array; commit(sample: number): bigint; reset(): bigint; stale_probe(): void }) {}

  static async create() {
    const module = await import("./wasm/bridge_wasm.js");
    await module.default();
    return new WasmCandidate(new module.WasmBridge());
  }

  async init() { this.session.reset(); }
  async revision() { return Number(this.session.revision()); }
  async scene(size: Size) { return this.session.scene(size); }
  async preview(sample: number) { return this.session.preview_scene(sample); }
  async commit(sample: number) { return Number(this.session.commit(sample)); }
  async reset() { return Number(this.session.reset()); }
  async stale() { this.session.stale_probe(); }
}

function percentile(values: number[], percentileValue: number): number {
  const sorted = [...values].sort((a, b) => a - b);
  return sorted[Math.min(sorted.length - 1, Math.ceil(sorted.length * percentileValue) - 1)] ?? 0;
}

function summary(values: number[]) {
  return {
    count: values.length,
    p50_ms: percentile(values, 0.5),
    p95_ms: percentile(values, 0.95),
    p99_ms: percentile(values, 0.99),
    max_ms: values.length ? Math.max(...values) : 0,
  };
}

async function timedScene(candidate: Candidate, size: Size) {
  const started = performance.now();
  const raw = await candidate.scene(size);
  const bridgeFinished = performance.now();
  const decodeStarted = performance.now();
  const packet = decodeScene(raw);
  const finished = performance.now();
  return {
    total_ms: finished - started,
    bridge_ms: bridgeFinished - started,
    decode_ms: finished - decodeStarted,
    bytes: raw.byteLength,
    primitive_count: packet.primitiveCount,
    semantic_id_count: packet.semanticIdCount,
    coordinate_count: packet.coordinateCount,
  };
}

async function timedPreview(candidate: Candidate, sample: number) {
  const started = performance.now();
  const raw = await candidate.preview(sample);
  const bridgeFinished = performance.now();
  const decodeStarted = performance.now();
  const packet = decodeScene(raw);
  const finished = performance.now();
  return { total_ms: finished - started, bridge_ms: bridgeFinished - started, decode_ms: finished - decodeStarted, packet };
}

function heartbeat() {
  const intervals: number[] = [];
  let previous: number | undefined;
  let frame: number;
  const tick = (time: number) => {
    if (previous !== undefined) intervals.push(time - previous);
    previous = time;
    frame = requestAnimationFrame(tick);
  };
  frame = requestAnimationFrame(tick);
  return () => {
    cancelAnimationFrame(frame);
    return {
      frames: intervals.length,
      over_33_3_ms: intervals.filter((interval) => interval > 33.3).length,
      over_100_ms: intervals.filter((interval) => interval > 100).length,
      max_interval_ms: Math.max(0, ...intervals),
    };
  };
}

function wait(ms: number) {
  return new Promise<void>((resolve) => window.setTimeout(resolve, ms));
}

async function dragRun(candidate: Candidate, run: number) {
  await candidate.init();
  const stopHeartbeat = heartbeat();
  const durations: number[] = [];
  const pending: Promise<void>[] = [];
  let inFlight = 0;
  let maximumInFlight = 0;
  let completed = 0;
  let superseded = 0;
  let outOfOrder = 0;
  let latestIssued = -1;
  let lastCompleted = -1;
  let failures = 0;

  for (let sample = 0; sample < 120; sample += 1) {
    latestIssued = sample;
    inFlight += 1;
    maximumInFlight = Math.max(maximumInFlight, inFlight);
    const started = performance.now();
    const request = candidate.preview(sample)
      .then((raw) => {
        const bridgeFinished = performance.now();
        decodeScene(raw);
        durations.push(performance.now() - started);
        if (sample < lastCompleted) outOfOrder += 1;
        if (sample < latestIssued) superseded += 1;
        lastCompleted = Math.max(lastCompleted, sample);
        completed += 1;
        void bridgeFinished;
      })
      .catch(() => { failures += 1; })
      .finally(() => { inFlight -= 1; });
    pending.push(request);
    await wait(1000 / 60);
  }
  await Promise.all(pending);
  const raf = stopHeartbeat();
  return {
    run,
    requested: 120,
    completed,
    failed: failures,
    superseded,
    out_of_order: outOfOrder,
    maximum_in_flight: maximumInFlight,
    duration_summary: summary(durations),
    duration_values: durations,
    raf,
  };
}

function assertEqualPacket(label: string, nativeRaw: unknown, wasmRaw: unknown) {
  const nativeBytes = toBytes(nativeRaw);
  const wasmBytes = toBytes(wasmRaw);
  if (nativeBytes.byteLength !== wasmBytes.byteLength) throw new Error(`${label} byte length mismatch`);
  for (let index = 0; index < nativeBytes.byteLength; index += 1) {
    if (nativeBytes[index] !== wasmBytes[index]) throw new Error(`${label} byte mismatch at ${index}`);
  }
  const nativePacket = decodeScene(nativeBytes);
  const wasmPacket = decodeScene(wasmBytes);
  if (JSON.stringify(nativePacket) !== JSON.stringify(wasmPacket)) throw new Error(`${label} semantic mismatch`);
}

async function assertCandidateParity(native: Candidate, wasm: Candidate) {
  await native.init();
  await wasm.init();
  for (const size of ["S", "M", "L"] as const) {
    assertEqualPacket(`scene-${size}`, await native.scene(size), await wasm.scene(size));
  }
  assertEqualPacket("preview", await native.preview(60), await wasm.preview(60));
  const [nativeCommit, wasmCommit] = await Promise.all([native.commit(60), wasm.commit(60)]);
  if (nativeCommit !== wasmCommit) throw new Error("commit revision mismatch");
  const [nativeReset, wasmReset] = await Promise.all([native.reset(), wasm.reset()]);
  if (nativeReset !== wasmReset) throw new Error("reset revision mismatch");
  for (const [name, candidate] of [["native", native], ["wasm", wasm]] as const) {
    try {
      await candidate.stale();
    } catch {
      continue;
    }
    throw new Error(`${name} stale probe unexpectedly succeeded`);
  }
}

async function benchmarkCandidate(name: CandidateName, candidate: Candidate) {
  for (let warmup = 0; warmup < 3; warmup += 1) {
    await candidate.init();
    await candidate.preview(warmup);
  }

  const initTimes: number[] = [];
  const revisionTimes: number[] = [];
  const commitResetTimes: number[] = [];
  const staleTimes: number[] = [];
  const previewTimes: number[] = [];
  const previewBridgeTimes: number[] = [];
  const previewDecodeTimes: number[] = [];
  const scenes: Record<Size, { samples: ReturnType<typeof summary>; bytes: number; primitive_count: number; semantic_id_count: number; coordinate_count: number; encode_bridge_decode: { bridge_ms: number[]; decode_ms: number[] } }> = {} as never;

  for (let run = 0; run < 5; run += 1) {
    let started = performance.now();
    await candidate.init();
    initTimes.push(performance.now() - started);

    started = performance.now();
    await candidate.revision();
    revisionTimes.push(performance.now() - started);

    for (const size of ["S", "M", "L"] as const) {
      const sample = await timedScene(candidate, size);
      const current = scenes[size] ??= { samples: summary([]), bytes: sample.bytes, primitive_count: sample.primitive_count, semantic_id_count: sample.semantic_id_count, coordinate_count: sample.coordinate_count, encode_bridge_decode: { bridge_ms: [], decode_ms: [] } };
      const values = (current as unknown as { _values?: number[] })._values ??= [];
      values.push(sample.total_ms);
      current.encode_bridge_decode.bridge_ms.push(sample.bridge_ms);
      current.encode_bridge_decode.decode_ms.push(sample.decode_ms);
    }

    const preview = await timedPreview(candidate, run * 10);
    previewTimes.push(preview.total_ms);
    previewBridgeTimes.push(preview.bridge_ms);
    previewDecodeTimes.push(preview.decode_ms);

    started = performance.now();
    await candidate.commit(run * 10);
    await candidate.reset();
    commitResetTimes.push(performance.now() - started);

    started = performance.now();
    let staleRejected = false;
    try {
      await candidate.stale();
    } catch {
      staleRejected = true;
    }
    if (!staleRejected) throw new Error(`${name} stale probe unexpectedly succeeded`);
    staleTimes.push(performance.now() - started);
  }

  const dragRuns = [];
  for (let run = 0; run < 5; run += 1) dragRuns.push(await dragRun(candidate, run));
  const normalizedScenes = Object.fromEntries(Object.entries(scenes).map(([size, value]) => {
    const values = (value as unknown as { _values?: number[] })._values ?? [];
    delete (value as unknown as { _values?: number[] })._values;
    return [size, { ...value, samples: summary(values), bridge_summary: summary(value.encode_bridge_decode.bridge_ms), decode_summary: summary(value.encode_bridge_decode.decode_ms) }];
  }));
  const aggregateDuration = summary(dragRuns.flatMap((run) => run.duration_values));
  return {
    candidate: name,
    warmup_runs: 3,
    recorded_runs: 5,
    initialization: summary(initTimes),
    revision_read: summary(revisionTimes),
    preview_total: summary(previewTimes),
    preview_bridge: summary(previewBridgeTimes),
    preview_decode: summary(previewDecodeTimes),
    commit_and_reset: summary(commitResetTimes),
    stale_error: summary(staleTimes),
    scenes: normalizedScenes,
    drag_preview: {
      requested_samples: 120,
      cadence_hz: 60,
      runs: dragRuns.map((run) => {
        const { duration_values: _durationValues, ...publicRun } = run;
        return publicRun;
      }),
      aggregate_duration: aggregateDuration,
      total_completed: dragRuns.reduce((sum, run) => sum + run.completed, 0),
      total_superseded: dragRuns.reduce((sum, run) => sum + run.superseded, 0),
      total_out_of_order: dragRuns.reduce((sum, run) => sum + run.out_of_order, 0),
      maximum_in_flight: Math.max(...dragRuns.map((run) => run.maximum_in_flight)),
      raf_over_33_3_ms: dragRuns.reduce((sum, run) => sum + run.raf.over_33_3_ms, 0),
      raf_over_100_ms: dragRuns.reduce((sum, run) => sum + run.raf.over_100_ms, 0),
    },
  };
}

async function runComparator() {
  const info = await invoke<{ bridge_owner: string }>("bridge_info");
  if (info.bridge_owner !== "native-tauri") throw new Error(`unexpected native owner: ${info.bridge_owner}`);
  const nativeCandidate = new NativeCandidate();
  const wasmCandidate = await WasmCandidate.create();
  await assertCandidateParity(nativeCandidate, wasmCandidate);
  const native = await benchmarkCandidate("native-tauri", nativeCandidate);
  const wasm = await benchmarkCandidate("wasm-bindgen", wasmCandidate);
  return { generated_at: new Date().toISOString(), method: "R3B five-run Windows WebView2 comparator", candidates: { native, wasm } };
}

function BenchmarkApp() {
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<unknown>(null);
  const [error, setError] = useState<string | null>(null);
  const run = async () => {
    setRunning(true); setError(null);
    try {
      const output = await runComparator();
      setResult(output);
      document.title = "R3B benchmark complete";
    } catch (caught) {
      setError(caught instanceof Error ? caught.message : String(caught));
    } finally { setRunning(false); }
  };
  return <main className="benchmark-shell">
    <h1>R3B bridge comparator</h1>
    <p>Warm-up: 3 runs. Recorded: 5 runs per candidate. Drag: 120 requests at 60 Hz with concurrent rAF heartbeat.</p>
    <button type="button" onClick={() => void run()} disabled={running}>{running ? "Running…" : "Run comparator"}</button>
    {error && <pre className="error">{error}</pre>}
    {result !== null && <pre id="benchmark-result">{JSON.stringify(result, null, 2)}</pre>}
  </main>;
}

ReactDOM.createRoot(document.getElementById("root")!).render(<BenchmarkApp />);
