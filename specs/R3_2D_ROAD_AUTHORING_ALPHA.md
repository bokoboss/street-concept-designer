# R3 — 2D Road Authoring Alpha

Status: **CONTROL-PLANE PACKETIZED / R3A NEXT / IMPLEMENTATION NOT YET AUTHORIZED**

## Product outcome

A traffic engineer can use a real desktop-style application workspace to inspect and then author semantic roads in 2D without CAD/Illustrator, while all engineering state continues to flow through the accepted Rust kernel, ProjectSession, persistence, and command architecture.

R3 is the first user-facing editor stage.

A plausible visual shell is not enough. R3 must prove that visible interactions remain semantic, deterministic, exact, undoable, and persistence-safe.

## Entry baseline

Requires:
- R1 completed;
- R2 Production Project Core completed;
- post-R2 editor readiness accepted;
- R3 stack research gate accepted;
- exact accepted `main` base recorded for each execution packet.

## Packet sequence

R3 executes through separately accepted packets.

### R3A — Desktop Shell / Read-only 2D Integration

Purpose:
prove the production application/renderer boundary before editing.

Outcome:
- Tauri desktop shell;
- React/Vite/TypeScript frontend;
- thin Rust application bridge;
- PixiJS WebGL plan rendering from R1C 2D derivation;
- project/scenario workspace skeleton;
- Select vs Hand/Pan;
- semantic selection;
- read-only hierarchy/inspector;
- browser/mock visual/E2E evidence;
- hosted Windows/MSVC Tauri build.

No engineering edit command is exposed from the UI yet.

### R3B — Compound Alignment Productionization

Purpose:
close the semantic gap between the current one-primitive Road alignment and practical multi-segment road authoring.

Expected proof:
- canonical multi-segment alignment;
- stable alignment-segment identity;
- continuous global stationing;
- deterministic sampling/projection;
- explicit joint/continuity policy;
- R1C rendering;
- project-scoped selection identity;
- versioned persistence/migration;
- full R1/R2 regression preservation.

This is a STRICT semantic/schema packet and requires a dedicated execution contract after R3A acceptance.

### R3C — Road Draw / Alignment Edit

Purpose:
turn the accepted compound alignment into a usable 2D road-authoring interaction.

Expected proof:
- road draw mode;
- selection/edit handles;
- exact numeric alignment properties;
- basic snap candidates;
- preview → commit;
- one meaningful undo item;
- stale-preview protection;
- no renderer-owned engineering truth.

### R3D — Cross Section / Lifecycle / Presets / Turn Pocket

Purpose:
make roads useful for real traffic-engineering concepts.

Expected proof:
- contextual cross-section editor;
- exact component widths;
- ordered components;
- generic road configuration generators;
- width/lifecycle edits;
- lane widening/add/drop;
- turn-pocket workflow through general lane lifecycle;
- plan preview/commit/undo.

### R3E — Reference Image Calibration / Integrated R3 UAT

Purpose:
let a project begin from a user site-plan/image and qualify the end-to-end R3 Alpha workflow.

Expected proof:
- persisted reference-image domain;
- source/file relationship;
- lock/display state;
- known-distance A-B calibration;
- explicit recalibration;
- engineering geometry unaffected by display-only changes;
- integrated R3 Golden UAT subset;
- UX Review Gate;
- first-time usability evidence.

R3E is the R3 exit gate.

No packet starts the next automatically.

## Shared architecture after R2

```text
                    Canonical persisted Project
                              |
                       ProjectSession
                    transaction / undo
                              |
                  Rust application/editor bridge
                              |
                    Tauri command boundary
                              |
             React application state / UI intent
                              |
               PixiJS renderer / hit-test state
```

Renderer/application state is downstream and disposable.

## Frontend state categories

### May be frontend/session state
- active tool;
- Select vs Hand;
- viewport transform;
- hover;
- selected ProjectSemanticRef;
- open/closed inspector sections;
- transient drag pointer state;
- pending form text before semantic preview.

### Must remain backend/canonical
- Roads;
- Alignment;
- component/lane profiles;
- Junction topology;
- Project/Scenario identity;
- TrafficSide;
- persisted ReferenceLayer semantics when R3E adds them.

### Must never become canonical engineering truth
- Pixi DisplayObject ids;
- DOM nodes;
- CSS pixels;
- canvas transforms;
- GPU buffers;
- browser localStorage copies of the engineering project.

## Mutation invariant

Once R3 editing begins:

```text
UI intent
 -> typed application request
 -> Rust command/transaction construction
 -> ProjectSession preview/commit
 -> accepted kernel/project validation
 -> derived R1C scene
 -> frontend renderer
```

The UI must not edit canonical JSON or recreate engineering geometry algorithms.

## Selection invariant

Selection uses ProjectSemanticRef-compatible identity.

2D hit test:
```text
Pixi object hit
 -> transport semantic ref
 -> shared app selection
 -> hierarchy/properties/highlight
```

Do not infer object identity by reverse-searching coordinates when the rendered primitive already carries semantic identity.

## Rendering invariant

PixiJS may perform:
- world-to-screen transform;
- pan/zoom;
- styling;
- hit testing;
- selection/hover overlay;
- labels/handles later.

PixiJS may not compute:
- road widths;
- component offsets;
- lane lifecycle;
- Junction pavement;
- topology;
- engineering station values.

## R3 stack policy

Authoritative research:
`docs/development/R3_STACK_RESEARCH_GATE.md`

R3A candidates:
- Tauri 2.11.x;
- React 19.2.x;
- Vite 8.2.x;
- TypeScript 7.0.x, compatibility-gated;
- PixiJS 8.20.x WebGL;
- npm + package-lock;
- Playwright.

No Three.js in R3A.

No MapLibre in R3.

## Windows qualification policy for R3

Current local workstation lacks the supported Windows/MSVC C++ linker/toolchain required by Tauri.

R3 must not solve this through silent system installation or unsupported production GNU assumptions.

Until explicitly changed:
- local/frontend UX proof may run in Vite/browser mode;
- browser tests use a mock transport bridge;
- Rust bridge logic is directly tested;
- hosted Windows/MSVC builds the Tauri shell;
- native desktop UAT requires a built Windows artifact and must be reported separately from browser-only evidence.

## Compound-alignment blocker

Current canonical `Alignment` is exactly one primitive.

R3C multi-segment road authoring is blocked until R3B resolves this.

R3A is intentionally read-only so it can prove the renderer/application stack without pretending this semantic gap is already solved.

## Reference-image blocker

R2 Project has no ReferenceLayer domain.

Reference image/calibration must become explicit domain/persistence state in R3E.

Do not implement it only as frontend/localStorage state.

## R3 non-goals

Do not implement in R3 unless a packet explicitly changes this scope:
- production 3D/Three.js;
- MapLibre/online basemap/provider;
- markings;
- standards guidance;
- Thai numeric defaults;
- asset library;
- AI Copilot runtime;
- engineering export;
- autosave/recovery;
- Windows installer/portable release qualification;
- R4 Junction editing workflow.

## R3 acceptance fixtures

Build on accepted R1/R2 fixtures.

By R3 exit, at minimum qualify:
- straight + curved/multi-segment road;
- generic divided-road configuration;
- variable-width/lifecycle case;
- turn-pocket case;
- Existing + Alternative isolation;
- large coordinates;
- calibrated user reference image;
- commit/undo/redo/save/reopen/rebuild.

## UAT

R3 exit should execute the relevant bounded portions of:
- UAT-01 through basic road + turn-pocket creation;
- UAT-03 road widening;
- UAT-06 reference-image calibration;
- UAT-08 undo/redo where R3 interactions exist.

Do not claim UAT steps for junction/access/markings/export/3D that are intentionally later.

## R3 exit decision

After R3E independent acceptance:

- `PROCEED_TO_R4`
- `REMEDIATE_R3`
- `ARCHITECTURE_ESCALATION`

R4 must not start automatically.
