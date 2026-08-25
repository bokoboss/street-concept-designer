# R1B Execution Contract — Junction Geometry & Topology

## Entry condition
R1B may start only after R1A is accepted with evidence. Do not begin automatically from an R1A worker.

## Objective
Prove that road geometry crossing can remain separate from explicit network topology, and that simple T, four-leg, and skewed junctions can produce deterministic pavement/corner geometry plus explicit lane-to-lane connectivity.

## Scope
- consume accepted R1A alignment/cross-section/lane lifecycle model without redesigning it casually;
- detect geometric crossing/conflict as a non-topological candidate;
- represent explicit junction creation separately from candidate detection;
- model approaches and per-corner geometry;
- generate deterministic simple junction pavement/surface geometry;
- prove T, four-leg, and skewed fixtures, including unequal approach widths where practical;
- represent lane-to-lane connections and movement categories explicitly;
- prove that changing lane geometry invalidates/regenerates relevant candidate/derived connection geometry deterministically;
- add canonical/adversarial tests, invariants, property/fuzz-style coverage where appropriate, and benchmark observations.

## Out of scope
- polished editor UI;
- online maps;
- production renderer styling;
- roundabout engine;
- U-turn special module;
- traffic simulation;
- signal timing;
- Thai numeric design rules;
- swept path;
- terrain/vertical alignment;
- final persistence/export design.

## Architecture invariants
- geometric crossing does not silently create topology;
- junction is a first-class semantic object;
- polygon union alone is not the junction model;
- corner geometry is independently addressable per corner;
- lane connections reference stable semantic lanes;
- movement paths/visual guides are derived from connectivity, not substitutes for it;
- renderer geometry is not source-of-truth;
- all output remains deterministic for equal semantic input.

## Canonical fixtures
At minimum:
- 90-degree T junction;
- skewed T junction;
- 90-degree four-leg junction;
- skewed four-leg junction;
- unequal approach widths;
- divided-to-undivided candidate if R1A model supports it cleanly;
- endpoint meeting vs true crossing;
- ignored/grade-separated candidate proving no topology mutation.

Adversarial cases should include near-tangent/acute geometry, near-coincident endpoints, short approach segments, and large project coordinates.

## Success gates
| Gate | Criterion |
|---|---|
| B-G0 | R1A accepted baseline and workflow validation confirmed |
| B-G1 | crossing/candidate state exists without network connectivity |
| B-G2 | explicit create-junction operation produces first-class topology |
| B-G3 | T/four-leg/skewed canonical fixtures generate deterministic valid junction geometry |
| B-G4 | per-corner geometry is independently addressable and testable |
| B-G5 | lane-to-lane connectivity/movements reference stable lanes explicitly |
| B-G6 | changing relevant lane/approach geometry regenerates/invalidates derived junction data deterministically |
| B-G7 | adversarial fixtures do not produce NaN, silent topology, invalid/self-intersecting accepted surfaces, or orphan connections |
| B-G8 | property/fuzz/invariant strategy covers applicable topology/geometry risks |
| B-G9 | benchmark observations show no obvious architectural blocker |
| B-G10 | no roundabout/UI/map/standards/simulation scope creep |

## Stop conditions
Stop and report if R1B requires a material rewrite of accepted R1A primitives, if robust junction generation cannot be achieved without changing the semantic model, or if geometry/topology separation proves impractical. Do not hide the problem behind tolerance escalation or special-case polygons.

## Definition of done
All mandatory gates pass, or a blocker is documented with the smallest reproducible fixture and an explicit architecture decision request. Do not start R1C automatically.
