import init, { WasmBridge } from "./benchmark/wasm/bridge_wasm.js";
import { decodeScene, type ScenePacket } from "./types";

type Size = "S" | "M" | "L";

let sessionPromise: Promise<WasmBridge> | undefined;

async function session(): Promise<WasmBridge> {
  sessionPromise ??= init().then(() => new WasmBridge());
  return sessionPromise;
}

export async function bridgeInfo() {
  const current = await session();
  return { bridge_owner: "wasm-bindgen" as const, revision: Number(current.revision()) };
}

export async function bridgeRevision(): Promise<number> {
  return Number((await session()).revision());
}

export async function bridgeScene(size: Size): Promise<ScenePacket> {
  return decodeScene((await session()).scene(size));
}

export async function bridgePreviewScene(sample: number): Promise<ScenePacket> {
  return decodeScene((await session()).preview_scene(sample));
}

export async function bridgeCommit(sample: number): Promise<number> {
  return Number((await session()).commit(sample));
}

export async function bridgeReset(): Promise<number> {
  return Number((await session()).reset());
}

export async function bridgeStaleProbe(): Promise<void> {
  (await session()).stale_probe();
}
