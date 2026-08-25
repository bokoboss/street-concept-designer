# R1 — Geometry & Semantic Kernel Spike

## Mission

Prove the minimum semantic/geometry architecture required for a map-first street/intersection concept editor before building production UI.

R1 is an architecture-risk spike, not a polished product phase.

## Authoritative context

Read:
- `ENGINEERING_CONSTITUTION.md`
- `PROJECT_PROFILE.md`
- `docs/product/GOLDEN_WORKFLOWS.md`
- `docs/architecture/SEMANTIC_MODEL.md`
- `docs/architecture/TECHNICAL_ARCHITECTURE.md`
- `docs/ux/UX_ARCHITECTURE.md`
- `docs/development/ENGINEERING_WORKFLOW.md`

## Questions R1 must answer

1. Can one canonical station-based road model robustly drive both 2D and 3D output?
2. Can lane width/lifecycle profiles represent widening, taper, add/drop, and turn pockets without special-case polygons?
3. Can geometry crossing be separated cleanly from explicit junction topology?
4. Can simple T, four-leg, and skewed junction surfaces/connectivity be generated deterministically?
5. Can large/georeferenced-style coordinates be handled safely using a local rendering origin?
6. Which kernel implementation strategy should be adopted for production: Rust/native+WASM or TypeScript?

## Scope

### A. Geometry primitives / alignment

Implement and test conceptual alignment primitives sufficient for the spike:
- straight line;
- circular arc;
- smooth cubic conceptual curve.

Required conceptual API:
- total length;
- `pointAt(s)`;
- `tangentAt(s)`;
- `normalAt(s)`;
- projection of an XY point to alignment/station;
- adaptive/stable sampling;
- explicit validity/error handling.

Clothoid/spiral is not required, but the abstraction must not prevent adding it later.

### B. Station-based cross section

Represent ordered semantic components and station-varying widths.

Minimum component set for the spike:
- traffic lane;
- median;
- shoulder/edge strip.

Width profiles may use piecewise-linear interpolation for R1.

Prove:
- constant lane width;
- widening/narrowing;
- zero-to-full-width lane appearance;
- full-to-zero lane disappearance;
- right-turn pocket using the general lane lifecycle.

No dedicated turn-pocket polygon data model is allowed.

### C. Derived 2D road geometry

Generate deterministic left/right/component boundaries from alignment + lateral offsets.

At minimum produce:
- road/component boundary polylines;
- pavement/surface polygons or equivalent render primitives;
- stable ids that preserve semantic component identity.

### D. Candidate junction vs connected junction

For two or more road alignments:
- detect geometric conflict/crossing as a candidate;
- keep the network disconnected until an explicit connect/create operation;
- represent participating roads/approaches;
- generate minimal junction surface geometry;
- store explicit lane-to-lane connections/movements for connected fixtures.

Required fixtures:
- 90-degree T junction;
- skewed T junction;
- 90-degree four-leg junction;
- skewed four-leg junction;
- unequal approach widths;
- divided-to-undivided conceptual case if feasible within the spike.

### E. Shared 2D/3D derivation proof

Create minimal diagnostic views only.

2D:
- visualize sampled alignment, component boundaries, surfaces, lane identities, and junction connection paths.

3D:
- derive mesh strips/surfaces from the same semantic model and geometry outputs;
- basic flat elevation is sufficient;
- no presentation assets/material polish.

A semantic edit to a canonical fixture must update both views without maintaining duplicate engineering models.

### F. Coordinate/local-origin proof

Use at least one fixture with large world/project XY values. Demonstrate that:
- canonical coordinates preserve engineering values;
- renderer-facing coordinates subtract a local origin;
- derived 2D/3D output stays finite and stable.

### G. Kernel-language decision

Do not choose a language from preference alone.

The spike must document evidence for Rust/native+WASM vs TypeScript using:
- numerical/geometry robustness;
- deterministic behavior;
- property/fuzz testing ecosystem;
- WASM/browser integration risk;
- native desktop integration risk;
- developer iteration complexity;
- performance measurements on representative fixtures;
- maintainability of a shared 2D/3D kernel.

A small compatibility/proof harness is acceptable before selecting the primary R1 implementation path. Avoid implementing the full kernel twice.

## Canonical fixtures

At minimum:
1. straight two-lane road;
2. divided four-lane road;
3. circular curve;
4. smooth/S-style conceptual curve;
5. road widening;
6. lane add/drop;
7. right-turn pocket: 0 width → taper → full storage → termination;
8. median widening/opening geometry probe;
9. 90-degree T junction;
10. skewed T junction;
11. 90-degree four-leg junction;
12. skewed four-leg junction;
13. large-coordinate road;
14. nearly parallel crossing/adversarial case;
15. coincident/near-coincident endpoint case.

## Required invariants

At least:
- all coordinates finite;
- primitive/alignment lengths valid;
- station values monotonic and bounded;
- tangent/normal finite where defined;
- component widths non-negative;
- lane identity/lifecycle valid across sections;
- derived boundaries have stable ordering where defined;
- no unhandled NaN/Inf;
- no degenerate triangles in accepted 3D surface output;
- connected junction references valid existing approaches/lanes;
- candidate crossing does not mutate network topology;
- deterministic output for equal input.

## Adversarial tests

Include representative cases such as:
- 1 mm gap/overlap-style geometry;
- almost tangent/almost parallel alignments;
- very short primitive;
- abrupt width-profile change;
- lane starting at zero width;
- large coordinates plus small local geometry;
- near-coincident road endpoints.

## Test strategy

Use the strongest suitable mechanisms supported by the chosen kernel approach:
- unit/example tests;
- golden/canonical fixture snapshots using stable semantic/geometric values;
- property-based tests;
- fuzzing where practical;
- deterministic repeat tests;
- benchmark/regression measurements.

Visual snapshots alone are not sufficient qualification evidence.

## Spike performance observations

Record measured results; these are investigation targets, not permanent product guarantees.

Observe at least:
- `pointAt/tangentAt/normalAt` throughput/latency;
- regeneration of a representative 500 m road;
- variable-width lane/pocket regeneration;
- simple junction generation;
- complex four-leg fixture;
- renderer handoff payload size/time.

Prefer interaction-scale results that plausibly support immediate editing; if a robust full regeneration is too slow, document preview-during-drag vs full-on-commit strategy.

## Explicit non-goals

Do not implement:
- polished product workspace;
- online maps/satellite providers;
- Tauri packaging unless needed for a kernel compatibility proof;
- final asset library;
- Thailand-standard dimension rules;
- AI/LLM integration;
- terrain or vertical alignment;
- traffic simulation;
- swept-path analysis;
- CAD/DXF/DWG/OpenDRIVE export;
- production persistence/migrations;
- roundabout engine.

## Forbidden shortcuts

- road/SVG polygon as canonical source of truth;
- turn pocket as arbitrary polygon overlay;
- independent 2D and 3D geometry calculations;
- junction defined only by polygon Boolean union;
- geometric crossing automatically creating connectivity;
- screen pixels/renderer scale stored as engineering truth;
- silent tolerance hacks without named policy/tests;
- large UI build before kernel qualification.

## Success gates

| Gate | Criterion | Required evidence |
|---|---|---|
| R1-G1 | Alignment API passes canonical + adversarial tests | tests + fixture evidence |
| R1-G2 | Variable-width semantic lane lifecycle proves widening/add/drop/right-turn pocket | model + tests |
| R1-G3 | Candidate crossing remains topologically disconnected until explicit create/connect | tests |
| R1-G4 | T/four-leg/skewed connected fixtures produce valid surfaces and lane connections | tests + diagnostic output |
| R1-G5 | Same canonical fixtures drive both minimal 2D and 3D outputs | shared-model evidence |
| R1-G6 | Large-coordinate/local-origin fixture is finite/stable | test + diagnostic evidence |
| R1-G7 | Determinism/invariants/property/fuzz strategy passes agreed suite | logs/results |
| R1-G8 | Benchmarks are recorded and no obvious interaction-blocking architecture flaw is found | benchmark report |
| R1-G9 | Rust/WASM vs TypeScript decision is justified by evidence | ADR/recommendation |
| R1-G10 | No prohibited production/UI scope was pulled into the spike | diff/review |

## Stop conditions

Stop and report instead of expanding scope if:
- the proposed semantic model cannot represent required longitudinal behavior cleanly;
- junction geometry requires a fundamentally different topology model;
- chosen geometry dependencies cannot support the required build target safely;
- WASM/native constraints invalidate the candidate architecture;
- deterministic geometry cannot be achieved without a material architecture change;
- passing a gate would require implementing substantial out-of-scope product UI.

## Deliverables

- spike implementation/harness;
- canonical fixtures;
- automated tests;
- benchmark notes/results;
- diagnostic 2D and 3D proof;
- architecture decision/recommendation;
- evidence package;
- explicit limitations and next-stage recommendation.

## Definition of done

R1 is complete only when every mandatory gate is PASS or explicitly BLOCKED with an architecture decision. Agent confidence or visually plausible output is not sufficient.
