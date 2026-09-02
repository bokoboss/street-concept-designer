# Post-R2 Editor Readiness

Date: 2026-09-02

## Status

**READY FOR R3 CONTROL-PLANE PACKETIZATION.**

R2 Production Project Core is completed and merged.

R3 implementation is not authorized by this document alone. Each R3 packet requires:
- exact accepted `main` base;
- Issue/execution contract;
- dependency/research conditions where applicable;
- scrutiny;
- bounded execution;
- CI/evidence;
- independent review appropriate to risk.

## Accepted R1/R2 production foundation

The following are evidence-backed:

### Engineering kernel
- Rust std-only engineering kernel;
- Windows/MSVC and Linux qualification;
- WASM compile path;
- f64 metric engineering coordinates;
- reference-alignment/station API;
- line/circular-arc/smooth conceptual curve primitives;
- component-based cross sections;
- exact semantic width/lifecycle breakpoints;
- explicit geometry-vs-topology separation;
- first-class Junctions;
- authored corner/connectivity intent;
- shared renderer-neutral 2D/3D derivation;
- semantic renderer identities;
- local render-origin conversion before f32.

### Project core
- ProjectId / ScenarioId;
- Existing / Alternative scenarios;
- authored Scenario order;
- lineage-preserving Scenario duplication;
- deep-copy Scenario isolation;
- ProjectSemanticRef = ScenarioId scope + scenario-local semantic ref;
- TrafficSide / CoordinateContext;
- controlled/validated RoadNetwork ownership.

### Persistence
- isolated `project-io`;
- explicit schema DTO boundary;
- strict compact JSON schema v1;
- exact finite f64 round trip;
- deterministic output;
- duplicate-key-safe parsing;
- explicit migration harness;
- authored Junction reconstruction;
- clean R1C rebuild after load.

### Session / commands
- isolated `project-session`;
- typed Scenario-scoped commands;
- one preview/commit evaluation path;
- atomic multi-command transactions;
- monotonic session revision;
- stale-proposal rejection;
- Scenario lock enforcement;
- snapshot-based undo/redo;
- redo invalidation;
- persistence and R1C rebuild at committed/undone/redone state.

## Accepted runtime/package direction

Current Rust package direction:

```text
kernel
  ↑
project-core
  ↑
project-session

project-io -> project-core + kernel
```

R3 adds application layers **above** these packages.

The frontend/desktop layer must never become a second engineering model.

## R3 stack research result

See:
`docs/development/R3_STACK_RESEARCH_GATE.md`

Decision:
**GO WITH CONDITIONS**

Preferred R3 stack:
- Tauri 2.11.x current supported line;
- React 19.2.x;
- Vite 8.2.x;
- TypeScript 7.0.x if actual ecosystem qualification is clean;
- PixiJS 8.20.x using WebGL;
- npm + package-lock;
- Playwright for browser-mode UX/E2E.

No Three.js in R3A.

No MapLibre in R3.

## Application boundary

Preferred:

```text
React workspace
  ↓ typed requests
Tauri command adapter
  ↓
editor/application bridge
  ↓
ProjectSession + kernel

R1C derived 2D snapshot
  ↓ transport DTO
PixiJS
```

The Tauri command file should remain a thin adapter.

Application/backend behavior should be testable outside Tauri.

## Frontend transport is not project persistence

R3 transport DTOs are:
- app/runtime interfaces;
- renderer/session responses;
- allowed to evolve with the application.

They are not:
- canonical project-file schema;
- renderer source-of-truth;
- permission to edit project JSON.

Do not reuse R2B JSON as a convenient frontend mutation API.

## Selection state

Application-level selection is ephemeral UI/session state.

Selection stores project-scoped semantic references.

Pixi display-object ids, DOM ids, screen pixels, and hit-test indexes are not semantic identity.

## R3 newly discovered semantic gap — compound alignment

Current R1/R2 Road owns exactly one Alignment primitive.

Product/UX intent requires practical road alignment authoring through multiple line/arc/smooth segments.

Creating several independent Roads or a renderer-only chained path would violate canonical semantics.

Therefore a compound/reference-alignment extension is required before R3 multi-segment road authoring can be accepted.

This requires its own strict semantic/persistence gate.

Likely affected surfaces:
- kernel Alignment abstraction;
- stable segment identity;
- station continuity;
- projection/sampling;
- R1C derivation;
- semantic selection;
- R2B schema migration;
- persistence fidelity;
- later editing commands.

Do not hide this gap in frontend code.

## R3 newly discovered semantic gap — reference image

R3 product scope includes:
- user image/site-plan import;
- known-distance calibration;
- lock/display controls;
- persistence.

R2 intentionally contains no ReferenceLayer domain/schema placeholder.

Therefore reference-image state requires a real later R3 packet covering:
- reference identity;
- source/file relationship policy;
- calibration transform;
- lock;
- display properties/presentation-vs-engineering distinction;
- project persistence/migration;
- explicit recalibration semantics.

Do not store reference calibration only in React state/localStorage.

## Local Windows environment constraint

Current local Windows machine lacks normal MSVC linker/compiler tools used by Tauri.

Do not install Visual Studio Build Tools or change system configuration automatically.

Until a qualified local MSVC environment is available:
- frontend/browser visual work uses Vite + Playwright/mock bridge;
- pure Rust application bridge is tested through Cargo;
- actual Tauri Windows build is qualified in hosted MSVC CI;
- native desktop runtime/UAT requires a built artifact on a suitable Windows machine before claims of final desktop usability.

This is an environment limitation, not permission to use unsupported Windows GNU as the production Tauri path.

## R3 packetization

R3 should execute in the following bounded sequence:

### R3A — Desktop Shell / Read-only 2D Integration

Prove:
- Tauri/React/Vite production shell boundary;
- thin Rust/frontend bridge;
- PixiJS WebGL rendering of accepted R1C 2D primitives;
- semantic hit-test selection;
- Select vs Hand/Pan;
- read-only hierarchy/inspector synchronization;
- responsive low-fidelity workspace;
- browser/mock E2E + hosted Tauri build.

No road editing yet.

### R3B — Compound Alignment Productionization

Prove:
- multi-segment canonical road alignment;
- stable segment identity;
- deterministic stationing/projection/sampling;
- clear joint continuity policy;
- R1C derivation;
- renderer selection identity;
- project persistence schema migration;
- R1/R2 regression compatibility.

No broad UI work.

### R3C — Road Draw / Alignment Edit

Prove:
- user draws/edits compound road alignment in 2D;
- direct manipulation and exact numeric edit converge through typed backend transactions;
- basic snapping;
- preview/commit/undo;
- selection/hierarchy/properties;
- no renderer-owned geometry.

### R3D — Cross Section / Lifecycle / Presets / Turn Pocket

Prove:
- contextual cross-section editor;
- exact width editing;
- lifecycle profile editing;
- generic road configurations as generators;
- lane widening/add/drop;
- right/left-turn-pocket workflow using general lifecycle;
- meaningful preview/undo.

### R3E — Reference Image Calibration & R3 Alpha UAT

Prove:
- reference-image domain/persistence;
- import/display/lock;
- A-B known-distance calibration;
- optional rotation/north where bounded;
- reference state never engineering truth;
- recalibration does not silently distort engineering geometry;
- integrated R3 workflow/UAT and UX Review Gate.

R3E is the R3 exit gate.

## R3 scope exclusions

Do not pull forward:
- production 3D/Three.js;
- MapLibre/online provider;
- road markings;
- standards profiles/Thai numeric rules;
- assets;
- AI runtime;
- export;
- autosave/recovery;
- installer/portable qualification;
- R4 junction-editing UX.

The existing R1 diagnostic 3D proof may remain available only as backend evidence.

## R3 acceptance philosophy

R3 is the first stage where a plausible-looking screenshot is especially dangerous.

Acceptance must prove:
- the UI is backed by accepted semantic commands;
- direct manipulation and numeric values agree;
- selection resolves to semantic ids;
- preview does not mutate canonical state;
- undo/redo are meaningful;
- display transforms do not alter engineering coordinates;
- browser/mock evidence is distinguished from native Tauri evidence.

## Human UX authority

Automated E2E proves behavior, not whether the editor is genuinely easier than CAD/manual graphics.

The human owner remains the authority for:
- first-time usability;
- layout density;
- property wording;
- direct manipulation feel;
- whether R3 workflow is meaningfully useful for real traffic-engineering concept work.

## Readiness decision

- **R3A: READY after the post-R2 control-plane PR is accepted/merged and exact base is recorded.**
- **R3B: architecture required before road-authoring acceptance.**
- R3C/R3D/R3E remain blocked by preceding accepted packets.
