# R2A — Project / Scenario Domain Core

## Objective

Introduce the minimum production-quality Project/Scenario semantic root around the accepted R1 RoadNetwork without adding persistence, UI, or command history yet.

R2A establishes durable identity/scoping and scenario semantics that R2B/R2C can depend on.

## Entry

Requires:
- R1 completed/accepted;
- post-R1 accepted `main` exact SHA recorded at execution start;
- Engineering Development Workflow validation;
- all R1A/R1B/R1C regression workflows green.

## Scope

Implement only:
- ProjectId;
- ScenarioId;
- Project;
- Scenario;
- ScenarioRole/Kind sufficient for Existing vs Alternative;
- TrafficSide (LHT/RHT);
- minimal coordinate-context metadata;
- validated scenario ownership of accepted R1 RoadNetwork;
- scenario lock flag as canonical intent;
- add/remove/rename/duplicate scenario domain operations as appropriate;
- project/scenario validation;
- project-scoped semantic reference strategy;
- deterministic equality/clone behavior;
- R2A fixtures/benchmarks/evidence.

## Identity invariants

### Project

ProjectId:
- stable;
- not derived from file path/name;
- no whitespace/empty invalid id;
- renderer-independent.

### Scenario

ScenarioId:
- stable within Project;
- unique within Project;
- display name is not identity.

### Scenario-local R1 ids

RoadId/JunctionId/ComponentId/etc remain scenario-local semantic ids.

Scenario duplication:
- new ScenarioId;
- preserve internal semantic ids/content;
- no shared mutable engineering state between source and duplicate.

Project/application-level reference must include ScenarioId plus semantic object identity.

A raw RoadId is insufficient to distinguish:
- Existing/R01
- Alternative-A/R01

## Scenario semantics

Minimum roles:
- Existing
- Alternative

Do not encode Alternative A/B in enum variants; those are display names.

Scenario lock:
- stored semantic protection intent;
- enforcement by committed mutations is fully exercised in R2C;
- R2A domain APIs that mutate a locked Scenario should reject when reasonably applicable rather than normalize around the lock.

Active selected Scenario is session/UI state, not required in canonical Project.

## Project fields

Minimum durable R2A Project semantics:

```text
Project
  id
  name
  traffic_side
  coordinate_context
  scenarios[]
```

Do not add empty fake implementations for standards/references/assets/presentation merely because future schema docs list them.

They should enter canonical code when their roadmap stages implement real semantics.

## Coordinate context

Keep minimal and explicit.

Allowed:
- optional CRS/reference identifier/string metadata;
- optional descriptive coordinate-context metadata if useful.

Do not:
- implement transformations;
- infer CRS from numeric magnitude;
- store R1C render origin as canonical coordinate context;
- introduce map/provider logic.

## RoadNetwork ownership

Each Scenario owns its own RoadNetwork value.

Do not use shared mutable network references between Scenarios.

Read APIs may return borrowed immutable references.

Avoid adding unrestricted mutation escape hatches that would later bypass R2C command enforcement without need.

Tests/builders may construct validated RoadNetwork values before inserting them into a Scenario.

## Validation

Add deterministic Project validation at minimum for:
- valid/nonempty ProjectId;
- valid/nonempty ScenarioIds;
- unique ScenarioIds;
- finite/valid coordinate metadata where numeric;
- at least one Scenario;
- no invalid Scenario role/state combination;
- scenario RoadNetwork referential/coherence checks available from accepted R1 APIs or bounded additive validation.

Do not reject a project based on later standards advisories.

If R1 RoadNetwork lacks a required read-only coherence validator, add the smallest additive validator needed without changing accepted R1 semantics.

## Scenario duplication

Provide a deterministic duplication operation.

Inputs conceptually:
- source ScenarioId;
- new ScenarioId;
- new display name;
- role normally Alternative;
- lock state explicitly chosen/defaulted.

Required result:
- source unchanged;
- duplicate gets new ScenarioId;
- internal Road/Junction/etc ids preserved;
- engineering state initially equivalent;
- subsequent mutation of duplicate does not alter source;
- R1C snapshots from both initially equivalent modulo Scenario/project scoping metadata not present in the R1C snapshot itself.

## Project-scoped semantic refs

Prove an application-level identity wrapper or equivalent concept.

At minimum distinguish the same R1 `SemanticRef` in two scenarios.

Concept:

```rust
ProjectSemanticRef {
    scenario_id,
    semantic_ref
}
```

Do not modify R1C `SemanticRef` merely to make it project-scoped; R1C snapshot refs are correctly scoped within one scenario/network snapshot.

## Determinism

Equal Project input produces equal validation/duplication result.

Do not auto-generate ids from randomness in the core. IDs should be caller-supplied until an application-level id service is deliberately introduced.

Avoid wall-clock timestamps in equality-critical canonical state unless explicitly caller-supplied.

## Canonical fixtures

At minimum:
1. minimal project + Existing;
2. Existing with two roads + junction;
3. duplicate Existing → Alternative A;
4. same RoadId in Existing and Alternative, distinct ProjectSemanticRef;
5. scenario lock;
6. invalid duplicate ScenarioId;
7. source/duplicate isolation;
8. R1C snapshot derivable from each scenario.

## Adversarial

- empty/whitespace ids;
- duplicate scenario ids;
- missing source scenario duplication;
- lock mutation attempt if R2A exposes mutation;
- large-coordinate R1 project;
- stale/invalid R1 junction state according to accepted R1 policy;
- repeated duplication determinism.

## Tests / CI

Preserve all R1 workflows.

Add R2A qualification:
- fmt;
- strict clippy;
- all tests;
- Windows/MSVC;
- Linux/native+WASM build;
- R2A benchmark observations;
- workflow integrity.

## Benchmarks

Exploratory:
- clone/duplicate representative scenario;
- validate project with several scenarios;
- project-scoped semantic reference lookup if implemented.

No SLA.

## Success gates

A-G0 exact base/workflow.
A-G1 Project/Scenario root is renderer-independent.
A-G2 Scenario ids unique and stable.
A-G3 Scenario duplication preserves internal semantic ids but isolates mutation.
A-G4 ProjectSemanticRef distinguishes same local semantic id across scenarios.
A-G5 TrafficSide/coordinate context explicit and not render-origin state.
A-G6 project validation deterministic.
A-G7 accepted R1 snapshots derive from scenario networks unchanged.
A-G8 adversarial identity/isolation tests pass.
A-G9 benchmarks show no obvious blocker.
A-G10 no persistence/command/UI/R3 scope creep.

## Stop conditions

Return `ARCHITECTURE_ESCALATION` if:
- scenario duplication cannot preserve internal ids without breaking R1 invariants;
- project-global identity requires material rewrite of accepted R1 ids;
- wrapping RoadNetwork requires material R1 redesign;
- project validation cannot distinguish semantic state from disposable derived state.

## Evidence

Create:
`docs/development/R2A_PROJECT_SCENARIO_EVIDENCE.md`

Final recommendation:
- `PROCEED_TO_R2B`
- `REMEDIATE_R2A`
- `ARCHITECTURE_ESCALATION`

Do not start R2B automatically.
