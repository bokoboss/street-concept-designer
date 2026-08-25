# R1C Execution Contract — Shared 2D/3D Derivation Proof

## Entry condition
R1C may start only after R1A and R1B are accepted. It is a proof stage, not the production editor build.

## Objective
Prove that one accepted semantic/kernel snapshot can drive both minimal diagnostic 2D and 3D representations without duplicating engineering calculations or creating renderer-owned truth.

## Scope
- define a minimal renderer-facing derived snapshot/DTO contract;
- generate 2D diagnostic primitives for roads/components/junctions;
- generate minimal 3D mesh primitives from the same accepted semantic/geometry snapshot;
- preserve stable semantic ids into both representations for synchronized selection/highlighting proof;
- prove local rendering-origin handling for large project/georeferenced-style coordinates;
- verify that deleting renderer caches and rebuilding yields deterministic equivalent output;
- measure generation/update cost on canonical R1 fixtures;
- produce an evidence-based recommendation for production kernel integration and renderer boundaries.

## Out of scope
- polished React/Tauri application shell;
- final PixiJS/Three.js visual styling;
- basemap/provider integration;
- asset library;
- scenarios/project persistence;
- standards rules;
- AI runtime;
- production export;
- terrain, buildings, lighting, animation.

## Architecture invariants
- 2D and 3D consume the same canonical semantic/kernel result;
- no renderer computes independent lane/junction engineering geometry;
- stable semantic ids survive derivation;
- engineering coordinates remain f64/metres upstream; GPU/render-local conversion is derived;
- mesh/vector caches are disposable;
- selection/highlighting maps through semantic ids rather than coordinate matching;
- equal semantic input produces deterministic equivalent derived output.

## Required proof fixtures
Use accepted R1A/R1B canonical fixtures including at least:
- straight/curved road;
- variable-width lane/taper/turn pocket;
- T junction;
- skewed/four-leg junction;
- large-coordinate project-local-origin case.

## Success gates
| Gate | Criterion |
|---|---|
| C-G0 | R1A/R1B accepted baseline and workflow validation confirmed |
| C-G1 | one derived snapshot contract feeds both diagnostic 2D and 3D |
| C-G2 | 2D and 3D preserve matching stable semantic ids |
| C-G3 | no duplicate renderer-side engineering geometry algorithms are introduced |
| C-G4 | large-coordinate fixture remains stable using local render origin |
| C-G5 | deleting/rebuilding derived caches gives deterministic equivalent output |
| C-G6 | synchronized selection/highlight proof works by semantic id |
| C-G7 | canonical fixtures produce finite/non-degenerate diagnostic geometry |
| C-G8 | generation/update benchmark observations recorded |
| C-G9 | production kernel/renderer integration recommendation documented with evidence |
| C-G10 | no product-shell/map/asset/presentation scope creep |

## Stop conditions
Stop if a renderer requires its own engineering geometry model, if stable semantic identity cannot be preserved across derivation, or if coordinate precision requires a material redesign of canonical state.

## Definition of done
R1C is complete only when the shared-model proof and architecture recommendation are evidence-backed. Production UI work must begin in a later explicitly specified stage.
