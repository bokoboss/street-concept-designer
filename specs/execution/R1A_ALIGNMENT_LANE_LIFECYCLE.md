# R1A Execution Contract — Alignment & Lane Lifecycle

## Objective
Prove the first bounded slice of Issue #2: kernel feasibility, reference alignment/stationing, ordered cross-section components, and station-based lane width/lifecycle behavior.

## Scope
- verify workflow/project baseline and toolchain;
- perform a minimal Rust native+WASM feasibility check;
- implement line, circular-arc, and smooth conceptual-curve alignment primitives if feasible;
- implement length, pointAt(s), tangent, normal, point projection, and stable sampling;
- implement ordered semantic cross-section components;
- implement piecewise-linear station-based width profiles;
- prove constant width, widening/narrowing, lane add/drop, and a right-turn pocket using the same general lane lifecycle;
- add canonical/adversarial fixtures, deterministic tests, property-based coverage where useful, and preliminary benchmarks;
- document dependency/WASM/native risks;
- use `docs/architecture/GEOMETRY_PRECISION_TOLERANCE_POLICY.md` as the required numerical-policy baseline rather than inventing local epsilon constants.

## Out of scope
Junction topology/surfaces, product UI, maps, production 2D/3D rendering, scenarios, assets, Thai numeric standards, AI runtime, persistence/export, terrain, simulation, swept path, and roundabouts.

## Authoritative baseline
- Repository: `bokoboss/street-concept-designer`
- Accepted main SHA: `f41de61db74ac4837cb18f09cf7c3e74686d9450`
- Issue: #2
- Parent spec: `specs/R1_GEOMETRY_SPIKE.md`
- Project facts: `PROJECT_PROFILE.md`
- Constitution: `ENGINEERING_CONSTITUTION.md`
- Numerical policy: `docs/architecture/GEOMETRY_PRECISION_TOLERANCE_POLICY.md`

## Execution routing
Start with the lowest-cost coding model expected to reliably execute this already-bounded packet, using high reasoning effort. Escalate only after a concrete failing fixture is reproduced and diagnosed as a fundamental numerical, geometry, WASM/toolchain, or cross-cutting architecture problem.

## Invariants
- metres are canonical domain units;
- screen/renderer coordinates are never engineering truth;
- accepted coordinates/results are finite;
- stations are monotonic and bounded;
- component widths are non-negative;
- lane identity remains coherent across width-profile sections;
- right-turn pocket uses the general lane lifecycle/profile mechanism, not a dedicated overlay polygon;
- equal semantic input produces deterministic equivalent output;
- no dependency on `road-concept-builder`;
- tolerances must be named, centralized/documented, and tested; do not fix failing geometry by scattered epsilon inflation;
- abstractions must allow a future spiral/clothoid without implementing it now.

## Execution strategy
Use one writer on a dedicated branch/worktree such as `codex/r1a-alignment-lanes`. Read-only reviewers may inspect numerical invariants, dependency/WASM compatibility, or tests. Do not run multiple writers on the kernel surface.

### A0 — Baseline and workflow
Run the installed Engineering Development Workflow validation, confirm accepted main SHA/Issue #2, and record toolchain versions.

### A1 — Kernel feasibility
Create the minimum native+WASM proof needed to test the candidate Rust kernel path. Stop with evidence if a required dependency/toolchain creates a material target, licensing, or maintenance problem.

### A2 — Alignment
Implement and qualify line, arc, smooth conceptual curve, stationing/query/projection/sampling APIs.

### A3 — Cross section / profiles
Implement ordered components plus piecewise-linear station-based width profiles.

### A4 — Lane lifecycle
Prove widening, narrowing, lane add/drop, and right-turn pocket through the same lifecycle model.

### A5 — Qualification
Run all gates, scrutinize the diff, fix findings, and prepare an Evidence Package.

## Success gates
| Gate | Criterion | Required evidence |
|---|---|---|
| A-G0 | Workflow install validates | upstream installer validation output |
| A-G1 | Native+WASM feasibility established or explicitly blocked | build/test evidence |
| A-G2 | Alignment APIs pass canonical fixtures | unit/golden tests |
| A-G3 | Projection/station/tangent/normal/sampling stable on adversarial cases | tests |
| A-G4 | Width profiles preserve ordered, non-negative semantic components | tests/property tests |
| A-G5 | Lane add/drop and right-turn pocket use general lifecycle | model + tests + diff |
| A-G6 | Equal input is deterministic | regression tests |
| A-G7 | Large-coordinate fixture remains finite in canonical data | test |
| A-G8 | Property/fuzz-style invariant strategy covers applicable risks | evidence |
| A-G9 | Preliminary benchmarks recorded; no obvious architecture blocker | benchmark notes |
| A-G10 | Numerical tolerances are named/tested and no scattered magic-epsilon workaround is introduced | policy/diff scrutiny |
| A-G11 | No junction/UI/map/asset scope creep | diff scrutiny |

## Stop conditions
Stop rather than broaden scope if native+WASM feasibility fails materially, the alignment model requires a semantic redesign, determinism needs an accepted architecture change, lane lifecycle cannot represent required cases cleanly, or passing a gate would require junction/product-UI work.

## Definition of done
R1A is done only when A-G0 through A-G11 pass, or a mandatory gate is explicitly BLOCKED with evidence and an architecture decision request. Do not start R1B automatically.

## Final report
Include changed modules, exact commands/results, native/WASM status, fixture inventory, tests/property checks/benchmarks, tolerance policy/assumptions, commit/PR/CI identifiers, limitations, and recommendation to proceed/remediate/escalate.
