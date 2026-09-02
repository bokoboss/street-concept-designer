# R3A — Desktop Shell / Read-only 2D Integration

Status: PLANNED / NEXT R3 EXECUTION PACKET

## Objective

Prove the production desktop/frontend/2D-renderer boundary using accepted R1/R2 semantics **before** road editing begins.

R3A must produce a real application shell that can:
- launch as a Tauri application in hosted Windows qualification;
- run its frontend independently in browser/mock mode for visual/E2E QA;
- display one accepted Scenario as 2D engineering geometry derived from the Rust kernel;
- switch Select vs Hand/Pan safely;
- select semantic Road/Component/Junction/Corner objects from the 2D viewport;
- synchronize that selection with a read-only hierarchy/inspector;
- preserve the accepted engineering/application boundaries.

R3A is a platform/render integration proof, not yet a road editor.

## Entry

Requires:
- R2 completed/accepted;
- post-R2 baseline/control-plane PR accepted;
- `docs/development/R3_STACK_RESEARCH_GATE.md` = GO WITH CONDITIONS;
- exact accepted `main` base recorded at branch creation.

Work mode:
**STRICT**

Reasons:
- adopts the production desktop/frontend stack;
- adds public Rust/frontend transport interfaces;
- establishes renderer/selection boundaries;
- adds multiple dependencies/toolchains;
- creates the shell later editor packets will build on.

Mandatory:
- dependency research conditions;
- scrutiny before implementation;
- full relevant validation;
- independent review before merge.

## Product architecture

Required dependency direction:

```text
kernel
  ↑
project-core
  ↑
project-session
  ↑
editor-bridge
  ↑
Tauri shell
  ↕
React/Pixi frontend
```

`project-io` remains a sibling application service and is not required for the R3A read-only scene path.

No frontend package may become a dependency of Rust kernel/project crates.

## Preferred repository shape

Conceptually:

```text
crates/
  editor-bridge/
    Cargo.toml
    src/

apps/
  desktop/
    package.json
    package-lock.json
    src/
    tests/
    src-tauri/
      Cargo.toml
      src/
      capabilities/
      tauri.conf.json
```

Exact names may vary only for a repository-grounded reason.

Do not move accepted R1/R2 packages merely for aesthetics.

## Candidate dependencies

Research source:
`docs/development/R3_STACK_RESEARCH_GATE.md`

Preferred direct candidates at packet start:

Rust/application:
- `tauri 2.11.5` family, exact Cargo resolution recorded;
- Tauri build/runtime dependencies required by the standard shell;
- `serde` only above project-core/kernel for transport DTOs.

Frontend:
- `@tauri-apps/api 2.11.1`;
- `@tauri-apps/cli 2.11.4`;
- `react 19.2.8`;
- `react-dom 19.2.8`;
- `vite 8.2.2`;
- `@vitejs/plugin-react 6.1.1`;
- `typescript 7.0.2`, compatibility-gated;
- `pixi.js 8.20.1`;
- `@types/react 19.2.18`;
- `@types/react-dom 19.2.5`;
- `@playwright/test 1.62.1`.

Use Node 24 LTS where the hosted/frontend environment permits.

Use npm + committed `package-lock.json`.

No Tailwind/UI framework is required in R3A.

No `@pixi/react`.

No Three.js.

No MapLibre.

No Tauri shell/fs/http/dialog plugin unless a material R3A requirement proves it necessary. R3A should need none.

## Dependency gate

Before adoption, executor must:
1. verify the exact current versions above against the date of execution;
2. inspect package/Cargo lock resolution;
3. record direct/transitive licenses in the dependency register;
4. stop on incompatible licensing;
5. confirm TypeScript 7 works cleanly with the selected React/Vite/Pixi/Playwright versions.

If TypeScript 7 compatibility requires hacks/patches:
- do not add shims blindly;
- compare TypeScript 6.0.x as a bounded fallback;
- document the evidence and decision.

## Tauri security policy

R3A shell must:
- load bundled/local frontend only;
- define explicit capabilities for only the custom commands used;
- grant no remote-origin capability;
- expose no shell/process/network/filesystem permission;
- avoid plugins not required by R3A;
- return structured application errors;
- never panic across user-triggerable IPC if a normal error is possible.

Do not weaken Tauri ACL/capabilities for convenience.

## Editor bridge

Create a pure/testable Rust application bridge above ProjectSession.

Preferred concept:

```rust
EditorBackend {
    session: ProjectSession,
}
```

Exact ownership may differ.

The bridge must not depend on Tauri types.

It may expose:
- workspace/project summary;
- Scenario summaries;
- derived 2D scene response;
- read-only semantic inspection data needed by R3A.

R3A bridge must not expose engineering mutation commands yet.

## Initial Project/session

R3A does not need project file dialogs.

The real Tauri shell may start with:
- a deterministic valid blank Project with one Existing Scenario;
- or an explicitly labelled development/demo fixture that cannot be confused with persisted user work.

Browser/Playwright qualification may use a static mock transport fixture.

Do not add hidden demo engineering data as production canonical truth.

Do not add recent-project/localStorage persistence.

## Transport DTO boundary

Create explicit transport DTOs.

They must be separate from:
- R2B project-file DTO;
- internal kernel structs;
- Pixi display objects.

At minimum transport:

### Workspace summary
- ProjectId;
- project name;
- TrafficSide;
- current session revision if useful for later command preconditions;
- ordered Scenario summaries:
  - ScenarioId;
  - name;
  - role;
  - locked.

### 2D scene
For one explicitly requested ScenarioId:
- ScenarioId;
- render origin;
- deterministic list of 2D primitives;
- each primitive:
  - ProjectSemanticRef-compatible identity;
  - PrimitiveRole;
  - polygon/polyline kind;
  - render-local f64 points;
- deterministic scene bounds/extents where needed.

Use accepted R1C `derive_diagnostic_2d`.

Do not recalculate road/component geometry in the bridge.

## Project-semantic identity transport

Transport identity must preserve Scenario scope.

Conceptually:

```text
scenarioId
semanticRef {
  kind
  roadId/componentId/junctionId/cornerId/connectionId...
}
```

No string concatenation whose parsing is ambiguous.

No coordinate-based identity.

LaneConnection may exist semantically even when R1C has no geometric movement guide; do not fabricate geometry to make it selectable.

## Tauri commands

Keep thin and coarse.

R3A should need approximately:
- `get_workspace_summary`;
- `get_scene_2d(scenario_id)`.

Optional:
- semantic detail/inspector query only if the frontend cannot derive read-only display fields from the scene/workspace response without exposing excessive data.

Do not expose:
- generic `execute_json`;
- project JSON mutation;
- arbitrary file paths;
- shell execution;
- raw `&mut Project` semantics.

The Tauri command adapter should primarily:
1. validate/deserialize request;
2. lock managed backend state;
3. invoke bridge method;
4. return transport DTO/error.

Do not hold a mutex across async await.

Synchronous commands are preferred unless evidence requires async.

## R1C local-origin use

R1C Diagnostic2D already emits **render-local f64 coordinates** and an explicit project-space render origin.

Transport those local coordinates.

Do not:
- send ~1e9 project coordinates to Pixi and let GPU conversion lose precision;
- re-subtract origin independently in several frontend modules.

The frontend viewport operates in render-local scene coordinates.

Project coordinates remain Rust canonical state.

## PixiJS renderer

Use PixiJS WebGL explicitly.

Responsibilities:
- create stage/canvas;
- render provided polygons/polylines;
- visual role styling;
- pan/zoom transforms;
- hit testing;
- semantic selection metadata;
- selection/hover styling;
- resize.

Prohibited:
- cross-section evaluation;
- width interpolation;
- lateral offset;
- Junction geometry;
- topology;
- standards rules.

Do not use WebGPU in R3A.

Do not add `@pixi/react`; manage a Pixi application through an isolated React component/hook with correct mount/unmount cleanup.

## Workspace UX

Implement the minimum R3 workspace shell from existing UX docs.

Required visible structure:

- top bar:
  - project name;
  - active Scenario selector/indicator;
  - Undo/Redo placeholders or disabled state if no edit history is exposed yet;
  - 2D active view;
- left panel:
  - hierarchy/layers starter focused on current Scenario engineering objects;
- center:
  - dominant 2D Pixi viewport;
- right:
  - read-only contextual Properties;
- bottom/compact tool area:
  - Select;
  - Hand.

Do not add:
- Junction/Marking/Measure/Snap tools merely as dead buttons;
- AI panel;
- map panel;
- asset library;
- export;
- 3D toggle;
unless clearly labelled future-disabled placeholders materially improve UX, and avoid placeholder clutter by default.

## Visual character

Use existing UX direction:
- professional engineering editor;
- calm;
- precise;
- compact;
- premium;
- less intimidating than CAD;
- not SaaS dashboard;
- not game editor.

Viewport must dominate.

Avoid:
- excessive cards;
- oversized dashboard spacing;
- decorative gradients that reduce plan contrast;
- consumer-app rounded-card overload.

R3A visual styling remains provisional; exact palette/font/iconography is still evidence-gated.

No remote webfont dependency. Use system/local font stack unless a later licensed asset decision is made.

## Select mode

Select is the default edit/inspection mode.

Clicking a selectable Pixi primitive must resolve to its carried project-semantic reference.

Required:
- Road alignment click -> Road selection;
- component surface click -> RoadComponent selection;
- Junction surface -> Junction selection;
- Corner curve -> Corner selection.

Selection then updates:
- viewport highlight;
- hierarchy focus/selected row where represented;
- right inspector identity/type.

If primitives overlap, use deterministic R3A priority sufficient for the fixtures:
1. corner/feature;
2. component surface;
3. Junction surface;
4. Road alignment/road.

Document the priority.

Do not build the full future ambiguity-cycle chooser in R3A unless needed.

## Hand / Pan mode

Hand mode:
- canvas drag pans;
- wheel/trackpad zoom supported in bounded form;
- engineering object click does not select/move geometry;
- pointer cursor/state clearly differs from Select.

Select mode may allow wheel zoom, but drag must not move engineering geometry because editing is out of R3A.

No pan/zoom operation enters ProjectSession history.

Viewport transform is UI state only.

## View transform

Implement deterministic:
- fit scene;
- pan;
- zoom around cursor or viewport center according to one documented behavior;
- resize.

Keep screen-space/pixel values out of kernel/domain state.

Large-coordinate fixture must render correctly because the scene is already localized by R1C.

## Hierarchy

R3A hierarchy is read-only.

At minimum:
- Scenario;
- Roads;
- Road components;
- Junctions;
- Corners where practical.

Selecting a hierarchy item should update the shared application selection and viewport highlight.

If a semantic object has no render primitive in R3A, hierarchy selection may still update inspector without inventing geometry.

Do not equate visual hierarchy with future persisted Layer model.

## Inspector

Read-only.

Show bounded useful semantic identity:
- object type;
- stable id;
- owning Road/Scenario where applicable;
- role/kind.

Do not expose raw JSON.

Do not add numeric mutation fields in R3A.

## Shared frontend selection

Use one application selection store/context.

Do not duplicate independent selection state in:
- Pixi;
- hierarchy;
- inspector.

Pixi consumes selected semantic ref and displays highlight.

Hierarchy/inspector consume the same selected ref.

Do not add a large state-management dependency in R3A unless React's normal state/context becomes demonstrably insufficient.

## Mock bridge for browser E2E

The frontend must be runnable without Tauri using a mock transport adapter.

Mock data:
- deterministic;
- test-only/development-only;
- derived-shaped, not treated as canonical Project truth;
- includes straight/curved road component surfaces, Junction and Corner primitives;
- includes a large-coordinate case represented with small local coordinates + explicit render origin.

Production app selects the Tauri transport, not the mock.

Do not let mock behavior define separate engineering rules.

## Browser/E2E qualification

Use Playwright.

At minimum prove:
1. workspace loads with viewport dominant;
2. hierarchy and inspector visible at supported desktop size;
3. Select is default;
4. click Road primitive selects Road;
5. click component selects component and inspector updates;
6. hierarchy click highlights same semantic object in viewport;
7. Hand mode prevents selection and pans;
8. switching back Select restores selection behavior;
9. zoom changes view, not semantic coordinates/identity;
10. fit scene works;
11. resize does not break layout/canvas;
12. large-coordinate localized fixture renders/selects;
13. no console errors/unhandled promise rejections;
14. Pixi app/listeners clean up correctly across mount/unmount/navigation if applicable.

Capture screenshots at a small set of deterministic desktop viewport sizes for UX review evidence.

## Tauri-native qualification

Because local workstation lacks supported MSVC build tools, do not silently install them.

Required hosted Windows/MSVC:
- frontend npm ci/build/typecheck;
- Cargo check/test for Rust workspace;
- Tauri application build, preferably `tauri build --no-bundle` or equivalent executable qualification for R3A;
- confirm compiled desktop executable artifact exists.

Installer/MSI/NSIS qualification is NOT R3A.

If practical, upload a Windows executable artifact for later manual runtime UAT, but do not call it a release package.

Linux/macOS Tauri package qualification is not required for Windows-first R3A.

## Local/browser qualification

If Node is already available locally:
- `npm ci`;
- typecheck;
- Vite production build;
- Playwright Chromium;
- optionally headed screenshot inspection.

If Node is not available:
- do not install globally;
- hosted frontend CI is mandatory;
- report local visual limitation.

Pure Rust bridge tests must remain runnable with Cargo.

## Tests

### Rust
At minimum:
- EditorBackend rejects/propagates invalid ScenarioId;
- workspace Scenario order equals Project authored order;
- scene derives from selected Scenario only;
- same local RoadId in two Scenarios transports distinct project-scoped refs;
- Diagnostic2D output is passed through without engineering recomputation;
- render origin/local coordinates preserved;
- transport ordering deterministic;
- no session mutation from read-only queries.

### TypeScript/browser
- transport type fixtures;
- semantic-ref equality/keying helper;
- renderer selection mapping;
- Select/Hand behavior;
- hierarchy/inspector sync;
- viewport pan/zoom/fit;
- resize/cleanup.

## CI

Add a dedicated R3A workflow.

Required jobs:

### Frontend/browser Linux
- Node version recorded;
- `npm ci`;
- TypeScript typecheck;
- Vite production build;
- Playwright Chromium tests;
- deterministic screenshots/artifacts where useful.

### Rust Linux
- fmt;
- strict Clippy workspace;
- workspace check/tests;
- editor-bridge tests;
- existing kernel/project-session/project-io WASM gates where currently required.

### Windows/MSVC
- Rust 1.98.0 unless separately changed;
- Node frontend install/typecheck/build;
- workspace Cargo checks/tests;
- Tauri shell compile/build;
- editor-bridge tests.

Existing R1/R2 workflows remain green.

Engineering Workflow Integrity remains green.

## Performance observations

R3A is not an optimization stage, but record:
- transport JSON payload size for representative 2D scene;
- Rust derive + DTO conversion time;
- browser scene construction time;
- fit/pan/selection response on representative fixture.

Do not define a permanent FPS SLA in R3A.

Flag if the representative read-only scene produces an obvious interaction blocker.

## Dependency/license evidence

Update:
`docs/development/DEPENDENCY_LICENSE_REGISTER.md`

Record:
- direct frontend packages;
- relevant transitive packages/license inventory using lockfile tooling or a deterministic report;
- Tauri Rust dependency family/license;
- Playwright browser-test dependency.

Do not copy a dependency list from research if lockfile resolution differs.

## Evidence package

Create:
`docs/development/R3A_DESKTOP_2D_INTEGRATION_EVIDENCE.md`

Include:
- exact base/branch/final HEAD/PR;
- work mode STRICT;
- research-gate result;
- exact dependencies/versions/licenses;
- Node/npm versions;
- package architecture;
- Tauri security/capabilities;
- bridge/transport DTO design;
- render-origin handling;
- Pixi WebGL design;
- workspace UX;
- Select/Hand behavior;
- semantic selection sync;
- mock-vs-production transport separation;
- Rust tests;
- Playwright tests/screenshots;
- payload/performance observations;
- hosted Windows Tauri build;
- local environment limitations;
- R1/R2 regression/workflow results;
- scope audit;
- known limitations.

## R3A gates

- A-G0 exact accepted base/workflow/research conditions.
- A-G1 dependency direction preserves R1/R2 core.
- A-G2 Tauri shell is thin and security capability is minimal.
- A-G3 transport DTO is distinct from project persistence/runtime/private layout.
- A-G4 Pixi renders only accepted R1C 2D primitives; no engineering duplication.
- A-G5 render-local origin preserves large-coordinate behavior.
- A-G6 semantic selection synchronizes viewport/hierarchy/inspector.
- A-G7 Select vs Hand/Pan behavior is explicit and safe.
- A-G8 browser/mock Playwright UX evidence passes with no console errors.
- A-G9 hosted Windows/MSVC Tauri build passes; dependencies/licenses recorded.
- A-G10 all R1/R2 regressions + workflow integrity green; no editing/R3B+ scope creep.

## Stop / escalation

Return `ARCHITECTURE_ESCALATION` if:
- Tauri requires engineering state to be duplicated into frontend truth;
- accepted R1C 2D primitives cannot support semantic selection without renderer-side engineering recomputation;
- local-origin policy cannot survive transport/Pixi integration;
- TypeScript/frontend stack requires material unsupported patching;
- Pixi WebGL is unusable in qualified WebView2/Chromium evidence;
- the Tauri bridge requires project JSON mutation;
- security requires broad remote/shell/fs permissions merely for R3A.

Return `REMEDIATE_R3A` for bounded implementation/UX/test defects.

## Final recommendation

Exactly one:
- `PROCEED_TO_R3B`
- `REMEDIATE_R3A`
- `ARCHITECTURE_ESCALATION`

Do not start R3B automatically.
