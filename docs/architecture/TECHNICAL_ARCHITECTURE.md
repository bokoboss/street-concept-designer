# Technical Architecture — Pre-Spike Baseline

This document defines boundaries, not a final technology lock.

## Layers

1. Canonical semantic project model
2. Geometry/topology kernel
3. Command/transaction layer
4. Validation/rules layer
5. 2D renderer adapter
6. 3D renderer adapter
7. map/reference adapter
8. asset system
9. desktop shell/UI
10. persistence/export

Dependencies should point inward toward semantics/kernel rather than renderer-specific state.

Related architecture policies:
- `docs/architecture/COMMAND_TRANSACTION_MODEL.md`
- `docs/architecture/MAP_BASEMAP_POLICY.md`
- `docs/architecture/WINDOWS_DISTRIBUTION_POLICY.md`
- `docs/assets/ASSET_SYSTEM.md`
- `docs/assets/ASSET_PRODUCTION_PIPELINE.md`

## Candidate product stack

Desktop/UI candidate:
- Tauri 2
- React + TypeScript

2D candidate:
- PixiJS for scalable interactive plan rendering
- HTML/SVG overlay where semantic labels/handles/accessibility benefit

3D candidate:
- Three.js

Map candidate:
- MapLibre GL JS with provider abstraction governed by `MAP_BASEMAP_POLICY.md`.
- MapLibre is a renderer candidate, not a license to use any specific basemap/provider.

Geometry-kernel candidates to test:
- Rust compiled native/WASM, or
- TypeScript if it meets robustness/performance/maintainability gates.

Do not lock the kernel language until R1 evidence exists.

## Coordinate policy

- canonical engineering coordinates use floating-point metres;
- preserve project/georeferenced coordinate context separately;
- use a local rendering origin before GPU float32 transfer;
- renderer scale is view state, never engineering truth.

## Derived outputs

2D primitives, 3D meshes, SVG exports, GLB exports, thumbnails, and renderer caches must be reproducible from canonical semantic state.

## Command layer

Every meaningful edit is a typed semantic command/transaction with validation, preview where appropriate, apply, undo/redo semantics, and audit/provenance metadata as needed. Manual UI, imports, automation, and AI must converge on this same command path. See `COMMAND_TRANSACTION_MODEL.md`.

## Map/reference layer

Map imagery/data is reference context only. Renderer and provider are separate abstractions. Provider capabilities/terms must govern digitization, caching, offline use, export, and attribution without altering canonical engineering geometry. See `MAP_BASEMAP_POLICY.md`.

## Asset layer

Engineering markings and many roadside assets should be procedural/semantic; complex presentation props may use normalized 2D/3D representations with explicit provenance. The user's normal workflow must not depend on manually authoring SVG or 3D models.


## Desktop distribution boundary

The product is a desktop application whose UI may use web technologies internally. The release architecture must support:
- per-user Windows installation without Administrator rights;
- no-install Portable distribution;
- offline core editing;
- no end-user Node/Rust/Python/local-server prerequisite;
- WebView2 runtime/user-data handling appropriate to each distribution profile.

Deployment/storage decisions must not alter canonical project semantics. See `WINDOWS_DISTRIBUTION_POLICY.md`.
