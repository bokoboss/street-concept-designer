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
- MapLibre GL JS with provider abstraction

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

Every meaningful edit is a typed command with validation, preview where appropriate, apply, inverse/undo, and serialization/audit metadata as needed. AI uses the same command API as manual UI.
