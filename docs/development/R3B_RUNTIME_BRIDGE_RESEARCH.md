# R3B Desktop Runtime / Bridge / 2D Renderer Research Gate

Date: 2026-09-04

## Decision supported

Decide whether Street Concept Designer is ready to execute R3B and define the bounded experiment that will:

1. adopt the minimum desktop/frontend/2D-renderer stack;
2. compare WASM-owned `ProjectSession` with native Tauri-owned `ProjectSession`;
3. select exactly one semantic-session owner from evidence;
4. prove that accepted R1/R2/R3A engineering state can be presented in a real Windows desktop 2D renderer without creating a second engineering model.

This research does not select the bridge in advance and does not authorize R3C road-authoring commands.

## Verified project state

Accepted current baseline before this control-plane packet:
`fa7386c82e2766ca05e1bfb6f1d0196b65839db5`

Accepted foundations now include:

- R1A alignment/stationing/lane lifecycle;
- R1B topology/junction semantics;
- R1C one shared deterministic 2D/3D derived-scene boundary with local render origin and semantic renderer identity;
- R2A Project/Scenario domain;
- R2B strict versioned persistence;
- R2C typed preview/commit/revision/undo/redo `ProjectSession`;
- R3A one Road-owned ordered composite alignment, cumulative stationing, strict schema-v2 migration, and preserved R1/R2 semantics.

R3A is no longer a blocker.

Protected project requirements remain:

- canonical semantic engineering state is Rust-owned;
- renderer objects/caches are disposable;
- one semantic source drives 2D and 3D;
- Windows-first desktop;
- packaged runtime must not require localhost, Node, Rust, Python, or a service;
- no dual native/WASM semantic sessions;
- rich 3D, maps, reference-image persistence, and R3C authoring commands are not R3B scope.

## Refreshed external evidence

Observed from official/upstream sources on 2026-09-04. These are observations, not accepted dependency pins.

### Tauri 2

Official Tauri ecosystem release page currently reports:

- `tauri` crate 2.11.5;
- `tauri-cli` 2.11.4;
- `@tauri-apps/api` 2.11.1;
- `@tauri-apps/cli` 2.11.4.

Sources:
- https://tauri.app/release/
- https://github.com/tauri-apps/tauri

Tauri supports a static web frontend and recommends Vite for SPA frameworks. Its production frontend model does not require SSR or a localhost application server.

Source:
- https://tauri.app/start/frontend/

Tauri IPC uses asynchronous message passing. Ordinary serializable command responses use JSON serialization; Tauri documents `tauri::ipc::Response` for optimized ArrayBuffer responses and Channels for streamed data.

Sources:
- https://tauri.app/concept/inter-process-communication/
- https://v2.tauri.app/develop/calling-rust/

License family:
MIT OR Apache-2.0 where applicable.

### React

React documentation remains on the 19.2 line. npm currently reports React 19.2.8 as latest stable.

Sources:
- https://react.dev/versions
- https://www.npmjs.com/package/react

License:
MIT.

### Vite

Vite currently supports the 8.2 line; npm reports 8.2.2 at this research date.

Sources:
- https://vite.dev/releases
- https://www.npmjs.com/package/vite

Vite 8 requires Node 20.19+ or 22.12+. Node 24 is an active LTS line in September 2026 and is the preferred R3 build/CI major unless actual package compatibility evidence says otherwise.

Sources:
- https://v8.vite.dev/guide/
- https://nodejs.org/en/about/previous-releases

License:
MIT.

### TypeScript

npm reports TypeScript 7.0.2 as latest stable at this research date.

Source:
- https://www.npmjs.com/package/typescript

Exact TypeScript pin must be resolved together with React/Vite during the adoption branch. Do not adopt a canary/nightly compiler.

License:
Apache-2.0.

### PixiJS

npm reports PixiJS 8.20.0 as latest stable at this research date. PixiJS 8 supports WebGL and WebGPU; its current Application documentation lists WebGL as the default renderer preference.

Sources:
- https://www.npmjs.com/package/pixi.js
- https://pixijs.com/8.x/guides/components/application
- https://pixijs.com/8.x/guides/components/events

License:
MIT.

R3B should explicitly use WebGL. WebGPU remains outside R3B so GPU/backend variability does not contaminate the bridge decision.

### wasm-bindgen

Current upstream manifest reports `wasm-bindgen` 0.2.127. Numeric slices are represented to JavaScript as TypedArray views, which is suitable for a bounded bulk-data comparator.

Sources:
- https://github.com/wasm-bindgen/wasm-bindgen
- https://wasm-bindgen.github.io/wasm-bindgen/reference/types/number-slices.html

License:
MIT OR Apache-2.0.

## Stack decision

### Proceed with the candidate family

R3B may adopt, subject to exact lockfile qualification:

- desktop shell: Tauri 2;
- UI: React + TypeScript;
- build: Vite;
- package manager: npm + checked-in `package-lock.json`;
- build/CI Node major: Node 24 LTS;
- 2D renderer: PixiJS 8, forced to WebGL for R3B;
- WASM candidate binding: wasm-bindgen only as required by the comparator.

Do not globally install Tauri CLI or frontend packages. Use project-local npm dependencies and Cargo dependencies.

### Keep application dependencies above the engineering workspace

R3B introduces platform-specific and frontend dependencies. They must not leak into the accepted engineering-kernel workspace or make the inherited root WASM gates compile Tauri.

Preferred structure:

```text
apps/desktop/
  package.json
  package-lock.json
  src/
  src-tauri/          # standalone/excluded Cargo package
  bridge-wasm/        # comparator candidate, standalone/excluded Cargo package
```

The exact names may vary, but the invariant is mandatory:

- root engineering crates continue their existing native/WASM qualification;
- Tauri and wasm-bindgen application adapters live above them;
- the Tauri app crate and comparator-only WASM crate are not ordinary members of the root engineering Cargo workspace;
- if Cargo workspace discovery requires it, use an explicit root-workspace `exclude` rather than weakening inherited root gates.

After bridge selection, the losing semantic-owner implementation must not remain reachable from the production application.

## Comparator design

### Common semantic fixture

Both candidates must operate on exactly the same fixed current-schema project fixture.

The fixture must contain accepted R3A composite-alignment semantics and be reproducible/reviewable. It may be generated once from accepted constructors and checked in as a current-v2 R3B fixture.

Both paths must prove the same resulting:

- Project/Scenario ids;
- session revision;
- preview semantics;
- commit semantics;
- derived 2D primitive content;
- semantic renderer ids;
- local render origin.

### Common operation set

Measure at minimum:

1. initialize/load `ProjectSession`;
2. revision/read query;
3. one existing R2C `preview` transaction that causes real engineering derivation;
4. the matching `commit` transaction and reset;
5. derive the accepted R1C 2D scene;
6. transfer the representative 2D scene to JavaScript;
7. stale/error translation;
8. repeated drag-like preview cadence.

The benchmark adapter may construct a fixed existing R2C transaction in Rust. It must actually call the accepted `ProjectSession` preview/commit path and must not invent a parallel JavaScript mutation model.

R3B does not define the future R3C editor-command catalog.

### Scene-transfer sizes

Use:

- S: the real derived scene from the canonical R3B fixture;
- M: deterministic benchmark-only replication sufficient to exercise a materially larger transfer;
- L: a larger deterministic benchmark-only replication.

For M/L, replication may occur at the derived/render-wire layer with unique renderer ids and known translations. It is benchmark data only and must never be fed back into canonical project state.

Record primitive counts and byte sizes rather than relying on labels alone.

### Candidate A — native Tauri owner

```text
React/PixiJS
   ⇅ Tauri invoke/IPC
native ProjectSession
   ↓
kernel / R1C derived scene
```

Requirements:

- exactly one native `ProjectSession`;
- small control/error messages may use normal serializable commands;
- bulk scene transfer must exercise Tauri's appropriate optimized byte/ArrayBuffer response mechanism when JSON serialization would dominate;
- no JavaScript copy of Project/Scenario/Road engineering state.

### Candidate B — WASM owner

```text
React/PixiJS
   ⇅ wasm-bindgen
WASM ProjectSession
   ↓
kernel / R1C derived scene
Tauri shell only for native OS services
```

Requirements:

- exactly one WASM `ProjectSession`;
- bulk numeric/byte data should use TypedArray/Uint8Array-oriented transfer rather than stringify/parse as the only path;
- no second native semantic session;
- R3B does not add a Web Worker architecture. Main-thread blocking must be measured as part of the decision.

## Performance / responsiveness method

The comparator must not decide from a single stopwatch measurement.

For each candidate:

- warm up before recording;
- run at least 5 benchmark runs;
- record p50, p95, p99 and maximum for critical operations;
- record representative payload bytes and primitive counts;
- run a 120-sample drag-like preview sequence at a 60 Hz requested input cadence;
- record completed previews, superseded/coalesced previews if implemented, maximum in-flight count, and out-of-order responses;
- run a concurrent `requestAnimationFrame` heartbeat and record frame intervals over 33.3 ms and over 100 ms;
- identify whether delay is Rust derivation, bridge transport/serialization, JavaScript decode, or renderer update where practical.

A benchmark request id may be used to discard stale visual responses. It is presentation/request sequencing only and must not replace `ProjectSession` revision semantics.

### R3B responsiveness floor

This is an adoption floor for the development proof, not a published product SLA.

On the primary Windows execution machine:

- representative drag-preview bridge+session p95 should remain at or below 33.3 ms;
- no bridge architecture should introduce repeatable >100 ms UI-thread stalls during the representative drag loop;
- the minimal rendered scene must remain interactive enough for pointer selection while the benchmark is active.

Final ordinary-office-PC performance requirements remain R8 work.

## Bridge selection rule

Use ordered evidence, not preference:

1. semantic correctness and single-owner integrity;
2. UI responsiveness / main-thread health during repeated preview;
3. representative preview and scene-transfer p95;
4. startup/load behavior;
5. implementation and maintenance complexity;
6. dependency/build/package complexity.

A performance difference is material when it is repeatable and approximately 20% or greater in a critical p95 metric across the repeated runs without a countervailing regression in a higher-priority criterion.

If performance is effectively equivalent, choose the path with the smaller production interface/dependency/build complexity and document why.

If evidence conflicts materially and the ordered criteria do not produce a defensible winner, stop with `ARCHITECTURE_ESCALATION`; do not keep both semantic owners.

## Minimal PixiJS proof after bridge selection

After selecting and cleaning up the bridge, the production R3B app must:

- initialize PixiJS with WebGL preference explicitly;
- consume the accepted derived 2D scene, not recompute engineering geometry;
- apply the snapshot local render origin before GPU float conversion;
- render a representative Road/CrossSection-derived scene;
- map pointer selection back to the semantic/scoped renderer id;
- hold selection in application-level semantic-id state;
- visibly highlight the selected representation;
- expose a minimal diagnostic readout of selected semantic id/session revision/bridge owner;
- dispose/rebuild renderer caches and reproduce equivalent semantic mapping/geometry.

This is a proof window, not the R3D production workspace.

## Security / runtime conditions

R3B packaged runtime must:

- serve bundled static assets; no localhost plugin/server;
- use no SSR/RSC/server runtime;
- make no network request for core startup;
- add no shell/http/fs/dialog plugin unless specifically required by R3B (none is expected);
- keep Tauri capability/permission configuration minimal;
- use an explicit restrictive CSP with no wildcard remote origin; document any WASM-specific CSP allowance if the selected path requires it;
- treat fixture/project bytes as data, never executable HTML/JS;
- not expose devtools/development server as a release runtime dependency.

A Vite development server is allowed only during development.

## Dependency qualification conditions

The implementation PR must record:

- exact direct npm/Cargo versions;
- exact lockfiles;
- frontend and application Cargo transitive dependency/license inventory;
- Tauri/React/Vite/PixiJS/TypeScript/wasm-bindgen licenses for the actually selected graph;
- any notices/dual-license choices needed downstream;
- Node/npm versions used for qualification.

The root accepted engineering dependency graph must remain independently auditable.

## Options rejected for R3B

- dual native + WASM semantic sessions;
- Electron;
- Next.js/SSR/RSC;
- localhost runtime plugin/server;
- Three.js;
- MapLibre;
- WebGPU proof;
- Web Worker architecture;
- R3C editor command design;
- file-open/save/reference-image UI;
- final installer/Portable qualification.

## Research verdict

**GO WITH CONDITIONS**

R3B is ready for a bounded implementation/selection experiment after its control-plane contract is merged and an exact execution base is pinned.

Conditions carried into execution:

1. compare both bridges against the same semantic fixture and operation set;
2. measure UI-thread responsiveness, not latency alone;
3. select exactly one semantic-session owner and remove/isolate the loser from production;
4. PixiJS R3B proof uses WebGL;
5. app-specific Cargo packages stay outside the root engineering workspace gates;
6. static/no-localhost packaged runtime;
7. exact dependency/version/license evidence before acceptance;
8. Windows/MSVC build and real desktop smoke;
9. inherited R1/R2/R3A gates remain green;
10. independent review before merge.
