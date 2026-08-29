# Product Roadmap

Baseline: pre-implementation Generation 2 plan.

## Roadmap policy

This roadmap defines **product outcomes and gates**, not permission to implement every listed feature now.

- R0 is accepted foundation/governance work.
- R1 is the current technical spike and is already packetized R1A → R1B → R1C.
- R2+ remain provisional until preceding evidence is accepted.
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

Provisional until R1 accepted.

Product outcome:
a production-quality semantic project can be created, mutated deterministically, saved, reopened, and validated without relying on a polished editor.

Expected capabilities:
- production kernel/package structure based on R1 evidence;
- stable semantic ids;
- project/scenario root model;
- versioned project schema;
- save/load round trip;
- migration harness;
- command/transaction framework;
- undo/redo core;
- internal geometry validation;
- canonical fixture library promoted from R1;
- CI for native/WASM/build/test as selected;
- initial desktop/application scaffold only as required.

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
- pavement/curb/median/sidewalk meshes;
- procedural road markings in 3D;
- starter asset metadata/runtime;
- starter vehicles/trees/lighting/safety assets;
- point/along-edge/area placement;
- deterministic asset distributions;
- simple building massing;
- saved views starter.

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
- plugin/API ecosystem;
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
