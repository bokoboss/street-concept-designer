# R3 — 2D Road Authoring Alpha

Status: R3A ACCEPTED / COMPLETED; R3B CONTROL-PLANE PLANNED; R3B IMPLEMENTATION NOT STARTED

## Product outcome

A traffic engineer can create or open a local project, work in a production desktop 2D editor, draw/edit one semantic road with an engineering-meaningful reference alignment, change exact cross-section/lane dimensions, create longitudinal widening/turn-pocket features parametrically, undo/redo meaningful edits, and work over a calibrated local reference image without CAD/Illustrator.

R3 is the first product stage that must expose a real editor workflow. It is not a visual mockup milestone.

## Why R3 begins with a kernel prerequisite

Post-R2 scrutiny found that the accepted R1 `Alignment` is a single primitive:
- line;
- circular arc;
- smooth conceptual curve.

A production road authoring workflow needs one Road identity and one cumulative station domain across an ordered sequence of alignment segments.

Do **not** fake this by splitting one user road into multiple Road objects.

Therefore R3A productionizes composite alignment before desktop UI implementation.

## R3 packet sequence

No packet starts the next automatically.

1. **R3A — Composite Alignment Productionization**
2. **R3B — Desktop Runtime / Binding / 2D Renderer Proof**
3. **R3C — Road Authoring Commands & Generic Configurations**
4. **R3D — Workspace / Selection / Road Draw & Alignment Edit**
5. **R3E — Cross Section / Lane Lifecycle / Turn Pocket**
6. **R3F — Local Reference Image / Calibration / Persistence**
7. **R3G — Integrated R3 UAT / Qualification**

Later packet specifications are intentionally not fully locked before preceding evidence exists.

## Architecture boundaries

### Canonical ownership

```text
Project / Scenario / Road semantics
        ↓
ProjectSession transaction boundary
        ↓
kernel + derived scene
        ↓
application bridge
        ↓
2D renderer adapter
        ↓
PixiJS presentation / hit-test cache
```

The renderer may never become engineering truth.

### Desktop/application direction

Subject to R3B adoption evidence:

```text
Tauri 2 desktop shell
  ├─ OS/file services
  └─ static React + TypeScript frontend
          ├─ application state
          ├─ semantic selection/tool state
          └─ PixiJS 2D renderer
```

Rust-to-UI semantic-session ownership remains evidence-gated until R3B completes the WASM-versus-native-IPC comparator.

See:
`docs/development/R3_APPLICATION_STACK_RESEARCH.md`.

## R3A — Composite Alignment Productionization

Status: **ACCEPTED / COMPLETED** via PR #32, squash merge `127aa01556eeb73c1daca160b3e2fa68e211dca8`.

Outcome:
one Road may own a stable ordered sequence of accepted alignment primitives with one continuous station domain and deterministic point/tangent/normal/projection/sampling behavior.

Required:
- stable segment identity;
- deterministic segment order;
- cumulative station mapping;
- continuity validation;
- R1C shared derivation compatibility;
- junction detection/regeneration compatibility;
- persistence schema migration;
- native/MSVC/Linux/WASM qualification.

No UI/frontend dependency.

## R3B — Desktop Runtime / Binding / 2D Renderer Proof

Status: **NEXT / SEPARATELY GATED**.

Authoritative planning inputs:
- `docs/development/R3B_RUNTIME_BRIDGE_RESEARCH.md`
- `specs/execution/R3B_DESKTOP_RUNTIME_BINDING_2D_RENDERER.md`

Outcome:
a minimal production desktop window renders a representative accepted Project/R1C 2D scene with semantic selection mapping.

Candidate dependencies:
- Tauri 2;
- React + TypeScript;
- Vite;
- PixiJS 8;
- only the minimum binding/package dependencies selected by the bridge experiment.

Must:
- measure WASM-owned session vs native-Tauri-IPC session;
- choose exactly one canonical session owner;
- demonstrate Windows desktop build/smoke;
- render local-origin scene;
- preserve semantic id selection;
- prove renderer cache is disposable;
- record exact dependency/license/transitive evidence.

No polished workspace and no map/3D dependency adoption.

## R3C — Road Authoring Commands & Generic Configurations

Outcome:
R3 editor intent can be represented as typed deterministic transactions without React constructing canonical Road internals.

Initial command surface should cover only R3 needs, including equivalent semantics for:
- CreateRoad;
- EditAlignment / replace alignment definition;
- ApplyRoadConfiguration;
- SetComponentWidth / SetWidthProfile;
- AddLaneTransition;
- AddTurnPocket;
- remove/reverse equivalent where needed for undo through normal history.

Generic road configurations should initially prioritize:
- RC-01 Two-Lane Undivided;
- RC-03 Four-Lane Divided;
- RC-04 Six-Lane Divided;
- RC-07 Project Access / Driveway if needed by the R3 workflow.

Exact dimensions are generic/user-editable unless source-backed. Do not label them Thai standards.

All commands:
- are ScenarioId-scoped;
- use accepted default numerical policy;
- preview through the shared transaction path;
- commit atomically;
- preserve readable history;
- reject locked targets.

## R3D — Workspace / Selection / Road Draw & Alignment Edit

Outcome:
a traffic engineer can create/edit a Road from the 2D viewport.

Required UX:
- viewport-dominant workspace;
- explicit Select vs Hand;
- active scenario clearly visible;
- hierarchy and contextual inspector;
- semantic selection from renderer hit-test;
- direct manipulation and exact numeric editing of the same property;
- Road Draw mode using R3A composite alignment;
- deterministic snap candidates;
- snap never silently creates topology;
- Esc/cancel restores prior canonical state;
- preview does not flood history.

Do not add R4 Junction authoring.

## R3E — Cross Section / Lane Lifecycle / Turn Pocket

Outcome:
the selected road can be configured and dimensioned without manual polygon drafting.

Required:
- compact cross-section editor;
- generic road configurations generate editable semantic components;
- numeric component widths;
- station-based width/lifecycle editing;
- widening/narrowing;
- parameterized right/left turn pocket using general lane lifecycle;
- preview/apply/cancel;
- exact undo/redo equivalence;
- no hard-coded unverified Thai numeric defaults.

## R3F — Local Reference Image / Calibration / Persistence

Outcome:
a local JPG/PNG/site-plan can act as a calibrated, visibly subordinate reference.

Required:
- Q0 unscaled and Q1 user-calibrated states;
- A-B known-distance calibration;
- optional rotation/north only if bounded correctly;
- lock/visibility/opacity/display controls;
- reference quality/provenance metadata;
- persisted reference metadata with schema migration;
- clear separation between image pixels and engineering coordinates;
- recalibration never silently transforms existing engineering geometry;
- save/reopen retains calibration and quality state;
- engineering-only operation remains possible when reference is hidden/missing.

The final physical .scd package/container remains out of scope; local file embedding/link policy must be bounded to R3 needs without pretending the final package is chosen.

## R3G — Integrated exit qualification

Relevant human/UAT coverage:

### UAT-01 subset
- create/open project;
- draw primary road;
- apply a generic divided-road configuration;
- add a parameterized turn pocket;
- inspect exact values;
- undo/redo.

R4 access/junction creation is excluded from R3 exit.

### UAT-03
- widen/narrow component/lane over a station range;
- verify exact lifecycle;
- undo/redo;
- save/reopen.

### UAT-06
- import image;
- calibrate A-B known distance;
- verify metre scale;
- lock/hide/display-adjust reference;
- prove reference display changes do not alter engineering geometry.

### UAT-08 subset
- road creation;
- road edit;
- configuration application;
- widening/turn-pocket;
- full undo/redo sequence.

### UAT-11
- save/reopen current R3 project;
- preserve semantic ids;
- rebuild PixiJS scene from canonical state with no renderer cache requirement.

### First-time-user UX
A traffic engineer who has not been trained on internal implementation details should be able to:
1. create a blank or image-reference project;
2. draw a road;
3. apply/edit a cross section;
4. add a turn pocket;
5. undo/redo;
6. save/reopen.

The benchmark must record observed friction rather than invent a time SLA before testing.

## Protected invariants

R3 must not:
- weaken semantic-model authority;
- create a separate JavaScript engineering model;
- let PixiJS object geometry become canonical state;
- create topology from snapping;
- bypass ProjectSession for meaningful mutation;
- introduce arbitrary custom TolerancePolicy as user/project state;
- silently upgrade old project semantics during migration;
- hard-code unverified Thai standards;
- require Node/Rust/Python on end-user runtime;
- require a localhost runtime server;
- introduce cloud/auth/collaboration;
- begin R4.

## Scope exclusions

Not R3:
- production Junction/access editor;
- traffic islands, crossings, markings;
- live production 3D editor;
- Three.js adoption unless a separate gate explicitly changes sequencing;
- MapLibre/provider integration;
- standards validation profiles;
- AI runtime;
- asset library;
- high-resolution export implementation;
- autosave/crash recovery;
- final installer/Portable qualification;
- final .scd package/container.

## Acceptance rule

R3 is accepted only when:
- R3A-R3F are separately accepted;
- R3G relevant automated and human UX/UAT gates pass;
- Windows desktop runtime evidence exists;
- actual UI workflow is reviewed against `docs/ux/UX_REVIEW_GATE.md`;
- no unresolved architecture/schema/engineering blocker remains.

Final R3 recommendation exactly:
- `PROCEED_TO_R4`
- `REMEDIATE_R3`
- `ARCHITECTURE_ESCALATION`

Do not start R4 automatically.
