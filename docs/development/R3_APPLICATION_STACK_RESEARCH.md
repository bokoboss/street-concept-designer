# R3 Application Stack and Bridge Research Gate

Date: 2026-09-03

## Decision

Determine the production direction for the first desktop/editor stage without prematurely locking an unproven Rust-to-UI transport.

The decision covers:
- desktop shell;
- frontend framework/build tool;
- 2D renderer;
- Rust project-session / derived-scene access from the UI.

It does **not** authorize R3 implementation by itself.

## Verified project facts

- R1 accepted Rust as the engineering kernel and qualified native/MSVC, Linux, and WASM compile paths.
- R1C accepted one shared derived engineering snapshot for 2D/3D plus semantic selection identity and local render origin.
- R2 accepted Project/Scenario domain state, strict versioned persistence/migration, typed preview/commit, monotonic session revision, and snapshot undo/redo.
- The product is Windows-first, desktop, offline-core, per-user-installable, and must later support no-install Portable distribution.
- Tauri 2 + React/TypeScript, PixiJS, Three.js, and MapLibre were candidates, not adopted dependencies.
- R3 is a 2D Road Authoring Alpha. Rich 3D and online map-provider integration are not prerequisites for its first packets.

## Current external evidence

Primary/official sources checked on 2026-09-03:

- Tauri current ecosystem release page reported `tauri 2.11.5`; Tauri 2 remains frontend-agnostic and can host static HTML/JS/WASM in its WebView:
  - https://tauri.app/release/
  - https://tauri.app/start/frontend/
- Tauri commands use message-passing IPC. Normal command values are serializable through a JSON-RPC-like transport; Tauri documents optimized ArrayBuffer responses for large binary data and ordered Channels for streaming:
  - https://tauri.app/concept/inter-process-communication/
  - https://v2.tauri.app/develop/calling-rust/
  - https://tauri.app/develop/calling-frontend/
- React official documentation reports React 19.2 as the current major/minor documentation line:
  - https://react.dev/versions
- Vite official release documentation shows Vite 8 as current and regular patches on the 8.2 line:
  - https://vite.dev/releases
- PixiJS upstream release history reports v8.19.0 as the latest v8 release observed during this gate:
  - https://github.com/pixijs/pixijs/releases
- `wasm-bindgen` supports high-level Rust/JavaScript bindings, generated TypeScript surfaces, and numeric TypedArray exchange. Its documentation also notes that serialization strategy performance depends on payload shape and runtime:
  - https://wasm-bindgen.github.io/wasm-bindgen/
  - https://wasm-bindgen.github.io/wasm-bindgen/reference/types/number-slices.html
  - https://wasm-bindgen.github.io/wasm-bindgen/reference/arbitrary-data-with-serde.html

Observed versions are research observations, **not pins**. Exact package versions/licenses/transitive graphs must be recorded by the adoption PR under the dependency register.

## Options considered

### A. Tauri native ProjectSession + Tauri IPC

Pattern:

```text
React/PixiJS
    ⇅ Tauri IPC
native Rust app state
    ↓
ProjectSession / project-io / kernel
```

Advantages:
- one native Rust owner for canonical ProjectSession;
- straightforward native file/system integration;
- no WASM binding layer for domain state.

Risks:
- command/preview calls cross asynchronous message-passing IPC;
- normal structured values are serialized;
- frequent direct-manipulation preview and large derived scenes may make transport cost material;
- renderer-hot-path performance must be measured rather than assumed.

### B. ProjectSession + project-io inside WASM in the WebView

Pattern:

```text
React/PixiJS
    ⇅ wasm-bindgen
WASM ProjectSession / project-io / kernel
    ⇅ Tauri only for OS/file services
native Tauri shell
```

Advantages:
- one authoritative editing session remains close to the renderer;
- no Tauri IPC round trip for every semantic preview;
- Rust kernel/project crates already compile for WASM;
- numeric scene buffers can use TypedArray-oriented bindings.

Risks:
- binding API and error translation are new production interfaces;
- heavy Rust/WASM work initially shares the WebView/UI thread unless later moved to a worker;
- file/system services still cross the Tauri boundary;
- exact payload/serialization strategy needs evidence.

### C. Dual native + WASM semantic sessions

Rejected.

Keeping one native canonical session and a second WASM semantic session for interactive preview creates synchronization and stale-state failure modes and weakens the single-source-of-truth rule. A renderer may cache derived data, but the product must not own two competing semantic sessions.

## Stack recommendation

For R3 application work, proceed with the following **candidate-to-adoption path**:

- Desktop shell: **Tauri 2**
- UI: **React + TypeScript**
- Frontend build: **Vite**
- 2D renderer: **PixiJS 8**
- 3D: **do not adopt in the first R3 runtime packet**
- MapLibre/provider integration: **do not adopt in the first R3 runtime packet**
- React application model: client-only/static frontend; no SSR or React Server Components are required for this desktop editor.

This set fits the locked desktop/offline architecture while limiting initial dependency scope.

## Bridge recommendation

Use a **WASM-first hypothesis**, not an architecture lock.

The R3 runtime/binding packet must implement a bounded comparator using the same representative project/scene and editor operations:

1. WASM direct binding path;
2. native Tauri IPC path using appropriate optimized binary response where applicable.

Measure at minimum:
- session initialization/load;
- no-op/read query;
- transaction preview round trip;
- transaction commit round trip;
- representative R1C-derived 2D snapshot transfer;
- repeated drag-preview cadence;
- error translation;
- clean rebuild equivalence.

The packet must then select exactly one semantic-session owner for production R3. The losing path is removed from production code unless retained only as isolated benchmark evidence.

## Security / deployment conditions

- No localhost application server may be required in packaged runtime.
- Tauri capability/permission configuration must expose only required native commands/plugins.
- Project/session semantics must not be duplicated in JavaScript.
- File contents from untrusted projects/references remain data, not executable web content.
- Exact Tauri/plugin/frontend dependency versions and notices must be recorded at adoption.
- Windows/MSVC desktop build/smoke is mandatory before adopting the application stack.

## Verdict

**GO WITH CONDITIONS**

Conditions:
1. Do not adopt Three.js or MapLibre in the first runtime packet.
2. Do not lock native IPC versus WASM by preference alone; run the bridge comparator.
3. Maintain exactly one authoritative ProjectSession.
4. Dependency pins/licenses/transitives require explicit adoption evidence.
5. R3 Road Draw implementation remains blocked until the composite-alignment gap identified in the post-R2 scrutiny is resolved.

## Implication for execution

The first R3 implementation packet is **not** the desktop shell. R3 must first productionize a multi-segment alignment because the accepted R1 Road currently owns only one line/arc/smooth primitive.

After that acceptance, execute the runtime/binding proof and adopt the desktop/2D stack.
