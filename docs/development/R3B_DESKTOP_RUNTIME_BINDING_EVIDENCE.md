# R3B Desktop Runtime Binding Evidence

Status: implementation evidence captured on 2026-09-05; local proof is complete, hosted G10/G12 qualification is externally blocked by GitHub billing enforcement, and G14 is PASS WITH CONDITIONS.

## Baseline and scope

- Repository: `https://github.com/bokoboss/street-concept-designer`
- Execution branch: `codex/r3b-desktop-runtime-binding-2d`
- Accepted base: `784a44663039898424c6ef39f0b98b003ffa734a`
- Implementation head under review: `4014fda` (`Close R3B review findings`)
- Scope: R3B desktop runtime, native/WASM bridge comparison, and 2D derived-scene proof only.
- R3C remains blocked and was not implemented.

The root Rust workspace remains unchanged as an application-independent kernel. `apps/desktop` is an isolated Cargo workspace with `bridge-common`, `bridge-wasm`, and `src-tauri`.

## Production owner decision

Selected owner: **WASM-owned `ProjectSession` via `wasm-bindgen`**.

The native Tauri candidate and WASM candidate use the same `bridge-common::BridgeSession`, the same current-schema fixture, and the same operation set. The comparator now asserts exact packet-byte/decoded-semantic parity for S/M/L scenes and preview, equal commit/reset revisions, and rejection of both stale probes before recording timings. The measured semantic result is equivalent. Both paths meet the responsiveness floor, while the WASM path is materially faster on the critical drag-preview p95. The production Tauri build has no `bridge-common` dependency or native session state; the native candidate is behind the `native-benchmark` Cargo feature and is reachable only by the comparator build.

Selection order result:

1. Semantic correctness and single-owner integrity: tie; both use the same Rust `ProjectSession` implementation.
2. UI responsiveness: WASM wins; drag-preview p95 is 0.3 ms versus 2.8 ms.
3. Scene transfer: WASM wins at every measured S/M/L p95.
4. Startup: WASM initialization p95 is 0.1 ms versus native 1.5 ms.
5–6. WASM adds a browser artifact, but that lower-priority cost does not outweigh the measured responsiveness advantage.

Decision register update: `docs/product/DECISION_REGISTER.md`.

## Comparator method

- Host: Windows 10, WebView2 `152.0.4191.62`.
- Same Tauri WebView2 page, same fixture, same operation sequence.
- Warm-up: 3 runs per candidate.
- Recorded: 5 runs per candidate.
- Latest result generated at `2026-09-05T13:45:59.747Z`; the native owner assertion runs once outside timed initialization so both candidates measure reset-only initialization.
- Scene sizes: S/M/L.
- Drag probe: 120 previews per run at 60 Hz, five runs, concurrent `requestAnimationFrame` heartbeat.
- Payload packet: 48-byte header, local f64 derivation, f32 conversion only at the wire/GPU boundary, scoped semantic IDs.
- Fixture: current project-file schema v2, EPSG:32647 context, two-segment composite alignment, two traffic lanes, one shoulder, existing and alternative scenarios.

### Operation latency evidence

| Operation | Native p50/p95/p99/max (ms) | WASM p50/p95/p99/max (ms) |
|---|---:|---:|
| Initialization/reset | 1.1 / 1.5 / 1.5 / 1.5 | 0.1 / 0.1 / 0.1 / 0.1 |
| Revision read | 1.1 / 1.3 / 1.3 / 1.3 | 0.0 / 0.0 / 0.0 / 0.0 |
| Preview total | 1.7 / 2.1 / 2.1 / 2.1 | 0.1 / 0.2 / 0.2 / 0.2 |
| Preview bridge | 1.6 / 2.1 / 2.1 / 2.1 | 0.1 / 0.2 / 0.2 / 0.2 |
| Preview decode | 0.0 / 0.1 / 0.1 / 0.1 | 0.0 / 0.0 / 0.0 / 0.0 |
| Commit + reset | 2.6 / 2.7 / 2.7 / 2.7 | 0.1 / 0.1 / 0.1 / 0.1 |
| Controlled stale error | 1.1 / 1.3 / 1.3 / 1.3 | 0.0 / 0.1 / 0.1 / 0.1 |

### Scene transfer evidence

All sizes produce identical packet counts and byte sizes for both candidates.

| Size | Bytes | Primitives | Scoped IDs | Coordinates | Total p50/p95/p99/max native (ms) | Total p50/p95/p99/max WASM (ms) |
|---|---:|---:|---:|---:|---:|---:|
| S | 4,622 | 49 | 4 | 418 | 1.3 / 1.6 / 1.6 / 1.6 | 0.1 / 0.2 / 0.2 / 0.2 |
| M | 39,041 | 392 | 32 | 3,344 | 2.2 / 2.3 / 2.3 / 2.3 | 0.4 / 0.5 / 0.5 / 0.5 |
| L | 317,039 | 3,136 | 256 | 26,752 | 8.0 / 9.6 / 9.6 / 9.6 | 2.6 / 3.7 / 3.7 / 3.7 |

Bridge/decode breakdown p95 (ms):

| Size | Native bridge | Native decode | WASM bridge | WASM decode |
|---|---:|---:|---:|---:|
| S | 1.5 | 0.1 | 0.2 | 0.0 |
| M | 2.3 | 0.2 | 0.3 | 0.2 |
| L | 8.4 | 1.4 | 2.0 | 1.7 |

### Drag and heartbeat evidence

| Candidate | Aggregate p50/p95/p99/max (ms) | Requested | Completed | Failed | Superseded | Out-of-order | Max in-flight | rAF >33.3 ms | rAF >100 ms | Max rAF interval |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Native Tauri | 2.1 / 2.8 / 3.7 / 3.9 | 600 | 600 | 0 | 0 | 0 | 1 | 0 | 0 | 17.3 ms |
| WASM | 0.2 / 0.5 / 0.6 / 0.8 | 600 | 600 | 0 | 0 | 0 | 1 | 0 | 0 | 17.0 ms |

Both paths are below the 33.3 ms p95 floor and show no repeatable heartbeat interval over 100 ms. WASM is approximately 82% lower on aggregate drag-preview p95.

## Runtime and renderer proof

- Production `src/wasmBridge.ts` owns one WASM session wrapper and exposes only derived scene packets, revision, and controlled session operations.
- Production Tauri `src-tauri` has no semantic session state by default.
- `src-tauri` native commands and `bridge-common` dependency are `native-benchmark`-feature-only.
- PixiJS 8 is initialized with `preference: "webgl"` and checked with the public `RendererType.WEBGL` enum rather than a minified constructor name.
- Renderer consumes decoded derived primitives only.
- Pointer selection uses the preserved scoped semantic ID; selected geometry is highlighted and displayed.
- Local render origin is carried in the packet and applied before f32 vertex conversion.
- Renderer children are destroyed and rebuilt; signatures derived from the newly-created Pixi children (scoped labels and local geometry bounds) are compared before/after rebuild.
- The production Vite output contains only `index.html` plus the selected WASM runtime. Comparator output is isolated to `benchmark-dist`.
- Local production WebView2 smoke verified `WebGL forced`, `wasm-bindgen`, the 49-primitive scene, a selected scoped lane id (`scenario-preview/road/R3B-road/component/lane-right`), and `Cache rebuild = equivalent` after the destroy/rebuild action.

## Security and packaging

- Production frontend is static and bundled through Tauri `frontendDist`; no localhost runtime server is used.
- `devUrl` exists only for development tooling.
- CSP keeps `default-src`, `connect-src`, image, style, object, base URI, and frame ancestor restrictions bounded to the proof. `'wasm-unsafe-eval'` is present because the selected WASM owner must compile its module in WebView2.
- Pixi's `pixi.js/unsafe-eval` static polyfill is imported so Pixi WebGL works without enabling the broader `'unsafe-eval'` CSP source.
- Capabilities contain only `core:default` for the main window.
- Exact dependency/license evidence is recorded in `docs/development/DEPENDENCY_LICENSE_REGISTER.md`, `apps/desktop/CARGO_LICENSE_REPORT.md`, and the checked-in npm/Cargo lockfiles.

## Gate status at evidence capture

| Gate | Status | Evidence |
|---|---|---|
| G0 | PASS | Exact branch/base, tracking, clean tracked worktree, and 0/0 state verified before mutation. |
| G1 | PASS | Exact package/toolchain/license graph and lockfiles. |
| G2 | PASS | Isolated desktop Cargo workspace and fixed current-schema fixture. |
| G3 | PASS | Same fixture/operations, shared Rust session implementation, parity tests, and controlled stale errors. |
| G4 | PASS | Real Windows WebView2 five-run comparator above; hosted MSVC rerun remains required for final CI qualification. |
| G5 | PASS | WASM is the sole default production owner; native Tauri is comparator-feature-only. |
| G6 | PASS | Production WebView2 smoke rendered the derived scene with WebGL forced and no controlled error. |
| G7 | PASS | Local-origin packet, scoped semantic-ID decode, and runtime lane selection proof. |
| G8 | PASS | Destroy/rebuild runtime action returned `equivalent`; scene-signature check is deterministic. |
| G9 | PASS | Static frontend, capability, CSP, and feature isolation. |
| G10 | BLOCKED — external | Local GNU build/launch passed; implementation-head hosted run [33970822285](https://github.com/bokoboss/street-concept-designer/actions/runs/33970822285) did not start its Windows/MSVC job because GitHub reported an account billing/spending-limit failure. |
| G11 | PASS | Inherited root formatting, clippy, tests, benches, and wasm32 build passed locally. |
| G12 | BLOCKED — external | Reproducible clean Linux/Windows CI workflow is committed; implementation-head hosted run [33970822285](https://github.com/bokoboss/street-concept-designer/actions/runs/33970822285) was rejected before `npm ci` or any build step for the same billing/spending-limit failure. |
| G13 | PASS | This evidence record, dependency register, and decision-register update. |
| G14 | PASS WITH CONDITIONS | Fresh-context review and independent PR review findings were addressed in `4014fda`; no unresolved architecture, scope, comparator, renderer, security, or dependency finding remains. Condition: hosted implementation-head G10/G12 must run green before acceptance. |

### Hosted qualification status

- Implementation-head R3B workflow [33970822285](https://github.com/bokoboss/street-concept-designer/actions/runs/33970822285) was triggered for `4014fda`; both Linux and Windows jobs were not started because GitHub reported: “recent account payments have failed or your spending limit needs to be increased.”
- A supported rerun of the earlier R3B workflow produced the same pre-start failure. The inherited R1A workflow [33968727299](https://github.com/bokoboss/street-concept-designer/actions/runs/33968727299) was also rejected before job startup for the same external reason.
- No hosted job-step, MSVC artifact, or CI benchmark result exists to claim. This is an account/control-plane blocker, not an implementation or architecture result.

## Local verification

Passed locally:

- root Rust fmt, clippy, workspace tests/benches, and wasm32 release build;
- desktop TypeScript check and Vite production/benchmark builds;
- desktop Cargo fmt and clippy with and without `native-benchmark`;
- bridge-common tests;
- WASM artifact generation and Node smoke import;
- optimized Windows GNU Tauri build and launch smoke test;
- real Windows WebView2 comparator shown above;
- production WebView2 selection and renderer-cache rebuild smoke.

The local environment has no `link.exe`/`cl.exe`; therefore the local GNU result is not substituted for G10. The committed workflow is the authoritative MSVC qualification path, but its final-head run is currently blocked before startup by GitHub billing enforcement.
The local GNU feature-gated Tauri test link also hit MinGW's `export ordinal too large` limitation; the bridge-common tests, both clippy variants, optimized runtime builds, and the hosted MSVC test/build path remain the relevant evidence.

## Known limitations

- This is a bounded R3B proof fixture, not survey-accurate engineering design or standards compliance.
- Windows packaging, signing, updater, fixed WebView2 runtime, and portable distribution remain out of scope.
- Final G10/G12 closure depends on hosted MSVC CI becoming runnable and green. G14 is reviewed with the stated condition; no R3C work has started.

## Current recommendation

`REMEDIATE_R3B` — unblock the GitHub Actions account billing/spending limit, rerun the final-head workflow, and complete G10/G12 before acceptance. No architecture escalation is indicated, and R3C remains blocked.
