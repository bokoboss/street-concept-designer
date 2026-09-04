# Product Roadmap

Baseline: post-R2 editor plan (2026-09-03).

## Roadmap policy

This roadmap defines **product outcomes and gates**, not permission to implement every listed feature now.

- R0 is accepted foundation/governance work.
- R1 is ACCEPTED/COMPLETED.
- R2 is ACCEPTED/COMPLETED after R2A → R2B → R2C independent acceptance.
- R3 is control-plane packetized after post-R2 scrutiny; R4+ remain provisional until preceding evidence is accepted.
- Each stage requires its own bounded execution contract before coding.
- A later stage may be split/reordered after evidence.
- Do not preserve an earlier implementation choice merely because this roadmap mentioned it as a candidate.

---

# R0 — Clean-Slate Product & Development Foundation

Status: ACCEPTED.

Outcomes:
- clean repository independent of legacy `road-concept-builder`;
- product vision and Golden Workflows;
- UX architecture;
- semantic/technical boundaries;
- asset/standards/map/command policies;
- Engineering Development Workflow installed;
- CI integrity gate;
- research/competitor baseline;
- R1 execution packetization.

Gate:
repository contains enough authoritative context that coding agents do not invent product direction.

---

# R1 — Geometry / Semantic Architecture Proof

Status: ACCEPTED / COMPLETED.

Purpose:
remove the largest technical unknowns before building a production editor.

## R1A — Alignment & Lane Lifecycle

Prove:
- native/WASM kernel feasibility hypothesis;
- reference alignment/stationing;
- line/arc/smooth conceptual curve;
- ordered cross-section components;
- station-based width profiles;
- widening/narrowing/lane add/drop;
- right-turn pocket through general lane lifecycle;
- determinism/tolerances/property/fuzz/benchmark discipline.

## R1B — Junction Geometry & Topology

Blocked until R1A acceptance.

Prove:
- candidate junction detection;
- geometry vs topology separation;
- T/X/skew cases;
- approach cut stations;
- independent corner geometry;
- robust pavement surface derivation;
- lane-to-lane connectivity;
- adversarial topology/geometry fixtures.

## R1C — Shared 2D / 3D Derivation Proof

Blocked until R1A + R1B acceptance.

Prove:
- one canonical semantic model;
- derived 2D primitives and 3D mesh buffers;
- semantic-id synchronization;
- local render-origin strategy;
- deterministic rebuild;
- no renderer-owned engineering truth.

Exit decision:
- choose/confirm kernel language/toolchain/dependencies;
- document architecture decision;
- list remedial work before productionization.

R1 is not an app release.

---

# R2 — Production Project Core

Status: ACCEPTED / COMPLETED.

See:
- `specs/R2_PRODUCTION_PROJECT_CORE.md`
- `specs/execution/R2A_PROJECT_SCENARIO_CORE.md`
- `specs/execution/R2B_PERSISTENCE_MIGRATION.md`
- `specs/execution/R2C_COMMAND_HISTORY.md`

Product outcome:
a production-quality semantic project can be created, mutated deterministically, saved, reopened, and validated without relying on a polished editor.

Packet outcomes:

## R2A — Project / Scenario Domain Core
- production package boundary preserving std-only engineering kernel;
- Project/Scenario root model;
- stable ProjectId / ScenarioId;
- scenario lineage/duplication/isolation;
- project-scoped semantic identity;
- explicit traffic-side/coordinate context;
- deterministic project validation.

## R2B — Versioned Persistence & Migration
- separate persistence/schema DTO boundary;
- canonical versioned JSON semantic document for R2 evidence;
- save/load round trip;
- stable semantic ids;
- authored junction persistence with derived-state reconstruction;
- deterministic migration harness;
- future/malformed schema rejection.

## R2C — Command / Transaction / Undo / Redo
- typed semantic mutation path;
- preview vs commit;
- atomic transaction;
- stale revision handling;
- scenario-lock enforcement;
- undo/redo core;
- persistence + R1 shared-rebuild integration;
- R2 integrated exit gate.

Initial desktop/application shell is not required for R2 unless a separately reviewed boundary proof demonstrates necessity.

Key UAT:
- UAT-08 undo/redo integrity at domain level;
- UAT-11 save/reopen/schema integrity.

Non-goals:
- polished workspace;
- online basemap;
- large asset library;
- standards rule catalog.

Gate:
canonical project state survives save/reopen/undo/rebuild deterministically.

---

# R3 — 2D Road Authoring Alpha

Status: R3A ACCEPTED/COMPLETED; R3B IS THE NEXT SEPARATELY GATED PACKET AND HAS NOT STARTED.

Packet sequence:
1. R3A — Composite Alignment Productionization
2. R3B — Desktop Runtime / Binding / 2D Renderer Proof
3. R3C — Road Authoring Commands & Generic Configurations
4. R3D — Workspace / Selection / Road Draw & Alignment Edit
5. R3E — Cross Section / Lane Lifecycle / Turn Pocket
6. R3F — Local Reference Image / Calibration / Persistence
7. R3G — Integrated R3 UAT / Qualification

Post-R2 scrutiny found that the accepted R1 Road owns only one line/arc/smooth alignment primitive. R3 Road Draw must not fake one user road as multiple Road objects, so R3A is the mandatory first packet.

See:
- `specs/R3_2D_ROAD_AUTHORING_ALPHA.md`
- `docs/development/POST_R2_EDITOR_READINESS.md`
- `docs/development/R3_APPLICATION_STACK_RESEARCH.md`
- `specs/execution/R3A_COMPOSITE_ALIGNMENT.md`

Product outcome:
a traffic engineer can start a local project/reference image and build/edit semantic roads in 2D without CAD.

Expected capabilities:
- desktop/editor shell;
- primary workspace structure;
- 2D renderer adapter;
- Select / Hand / navigation separation;
- object hierarchy/properties;
- user image/site-plan reference layer;
- known-distance scale calibration;
- road alignment draw/edit;
- cross-section editor;
- reusable road configurations/presets as generators;
- lane width/lifecycle editing;
- right/left-turn pocket UI using general semantic command;
- snapping basics;
- meaningful history;
- engineering-only plan rendering.

3D:
may remain diagnostic or minimal at this stage except what R1 already proved.

Key UAT:
- simplified UAT-01 through turn-pocket creation;
- UAT-03 road widening;
- UAT-06 calibration;
- first-time-user low-fidelity usability benchmark.

Gate:
road authoring is faster/easier than manual graphic drafting while retaining exact dimensions.

---

# R4 — Junction / Access / Network Alpha

Product outcome:
real project-access and intersection concepts become first-class editable semantic designs.

Expected capabilities:
- candidate junction UI;
- Create / Grade Separated / Ignore;
- project access/driveway workflow;
- T/four-leg/skewed intersections;
- per-corner radius/geometry editing;
- lane movement review;
- lane-to-lane connectivity;
- traffic/channelizing island starter;
- median and median opening;
- approach stop/crossing attachment model;
- selection/snapping refinements for network editing;
- node cleanup primitives as needed.

Key UAT:
- UAT-01 through project access/junction;
- UAT-02 skewed intersection;
- relevant adversarial geometry fixtures.

Gate:
junction edits remain semantic, deterministic, and manually overridable without polygon-paint workflows.

---

# R5 — Traffic Features / Markings / Validation Alpha

Product outcome:
core traffic-engineering concept drawings become complete enough for real internal project use.

Expected capabilities:
- procedural lane/edge lines;
- stop lines;
- crosswalks;
- arrows;
- hatch/chevron/gore;
- marking attachment/regeneration;
- median-opening treatment;
- U-turn treatment built on general primitives;
- bus bay/slip-lane candidate features where general model supports them;
- Issues panel;
- internal invalidity vs advisory validation separation;
- StandardProfile plumbing without pretending full compliance;
- generated-content override/provenance model.

Thailand:
only page-verified source values may be encoded as authoritative defaults.

Key UAT:
- UAT-04 median/U-turn;
- UAT-10 standards provenance when profile work is ready;
- GW-01 plan becomes visually/semantically complete.

Gate:
a typical access/intersection concept can be prepared without external drawing software.

---

# R6 — Scenarios / Live 3D / Starter Assets Alpha

Product outcome:
the same engineering model supports alternative comparison and immediate client-readable 3D.

Expected capabilities:
- Existing / Alternative A / B;
- scenario duplicate/rename/lock;
- overlay compare;
- split 2D/3D;
- synchronized semantic selection;
- engineering 3D materials;
- pavement/curb/median/sidewalk meshes generated from accepted derived engineering geometry rather than renderer-owned road calculations;
- procedural road markings in 3D;
- starter asset metadata/runtime;
- starter vehicles/trees/lighting/safety assets;
- one semantic asset with independently appropriate 2D plan and 3D representations;
- internal declarative asset-definition/representation contracts sufficient to drive properties, placement, and rendering without requiring a public plugin API;
- pure/deterministic representation builders where practical;
- explicit geometry-variant versus per-instance state for repeated assets;
- deterministic bounded seeds/variant pools for persisted presentation variation where useful;
- geometry-variant caching plus instancing/batching where benchmark evidence supports it;
- semantic selection identity preserved through instancing/batching, with renderer proxies only as a disposable implementation technique if needed;
- point/along-edge/area placement;
- deterministic asset distributions;
- runtime-optimized versus deterministic baked/export representation paths where appropriate;
- simple building massing;
- saved views starter.

R6 adoption gate must separately qualify the actual Three.js renderer path, including WebGL/WebGPU compatibility on representative Windows/office hardware. Pascal Editor prior art is informative but does not lock WebGPU, its scene architecture, or any dependency.

Key UAT:
- UAT-05 complete-street concept;
- UAT-07 2D/3D sync;
- GW-06 basic presentation.

Gate:
3D adds understanding without creating a second editing model.

---

# R7 — Map / Standards / Export Beta

Product outcome:
the editor is practical for real-world project context, professional export, and traceable engineering guidance.

Expected capabilities:
- MapLibre or R1/R2-confirmed map renderer;
- provider abstraction;
- permitted map/aerial provider integration;
- OSM-derived context where licensed/appropriate;
- organizational WMS/XYZ support as prioritized;
- provider rights/capability enforcement;
- reference attribution/export rules;
- high-resolution plan export;
- transparent/neutral export;
- vector plan export if robust;
- reference-included export only when allowed;
- standards profile version pinning;
- verified Thailand DOH/DRR starter guidance/assets;
- source/page traceability UI.

Possible additions if evidence supports:
- GeoTIFF;
- simple OSM building massing;
- expanded sign/signal/lighting pack.

Gate:
a project can start from lawful real-world context and produce report-ready outputs with traceable provenance.

---

# R8 — Product Beta / UX Hardening

Product outcome:
the application is reliable enough for sustained real project use and field UAT.

Expected work:
- full Golden Workflow UAT;
- autosave/crash recovery;
- project recovery;
- performance optimization;
- renderer/geometry profiling;
- large-project behavior;
- keyboard workflow;
- accessibility;
- focus/presentation mode;
- error/recovery UX;
- selection/snapping polish;
- visual regression;
- Windows packaging;
- per-user/no-admin installer qualification;
- Portable ZIP qualification on clean standard-user office PCs;
- offline installer and Portable Offline/WebView2 strategy qualification;
- code-signing/SmartScreen release path;
- dependency/license notice generation;
- asset LOD/instancing tuning;
- deterministic variant-cache and GPU resource lifecycle/disposal qualification;
- batched/instanced selection and outline stress testing;
- semantic-change-set/incremental renderer regeneration with clean-full-rebuild equivalence regression;
- WebGL/WebGPU renderer compatibility/fallback evidence on representative office PCs where 3D adoption requires it;
- schema migration tests;
- security/threat review for untrusted project/assets.

Optional feature promotion only if core quality is strong:
- roundabout;
- advanced U-turn;
- bus bay/slip lane;
- more comparison modes.

Gate:
no severe workflow friction or architecture regressions in real user projects.

---

# R9 — Release Candidate

Product outcome:
qualified Windows-first standalone release candidate.

Required qualification:
- clean-machine per-user install without elevation;
- clean-machine Portable launch without installation;
- offline installer qualification;
- Portable Offline runtime strategy qualified or explicitly deferred with evidence;
- signed/validated package strategy;
- open/save/migrate representative projects;
- no external editor dependency;
- offline core editing;
- deterministic kernel regression;
- full automated validation;
- Golden UAT acceptance;
- performance baseline;
- accessibility review;
- asset provenance/license completeness;
- dependency licenses/notices;
- SBOM direction implemented as required;
- map-provider terms/attribution verified;
- standards profile provenance verified;
- crash/recovery tests;
- security review;
- release notes and known limitations.

Release claim:
“rapid engineering concept design” — not statutory compliance, detailed civil design, or traffic simulation unless separately qualified.

---

# Post-R9 / Optional Modules

These are deliberately outside initial release dependency.

Potential:
- public plugin/API ecosystem only after internal definition/registry contracts are stable and product value justifies third-party extensibility;
- roundabout advanced module;
- swept-path/design-vehicle analysis;
- traffic count overlays;
- queue/movement visualization;
- video-analysis/TMC integration;
- HCM/calculation integration;
- traffic animation;
- OpenDRIVE;
- DXF/GeoJSON/KML;
- terrain/vertical alignment;
- AI natural-language copilot;
- AI alternative generation;
- cloud/collaboration.

Each optional module requires a separate product justification and architecture gate.

---

# Sequencing constraints

The following order is intentional:

1. semantic/numerical proof before UI polish;
2. project persistence/commands before trusting real work;
3. road authoring before junction breadth;
4. junction semantics before special cases;
5. engineering markings/validation before large presentation library;
6. shared-model 3D before rich 3D;
7. provider licensing before map-dependent workflows;
8. standards provenance before authoritative defaults;
9. real UAT before release feature expansion.

## Anti-scope-creep rule

A stage may add a dependency/feature only if it directly helps its declared product outcome or closes a known architecture risk. “We might need it later” is not sufficient justification.
