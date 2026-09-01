# R2 — Production Project Core

## Status

PLANNED / PACKETIZED AFTER ACCEPTED R1.

R1 technical proof is complete. R2 productionizes the accepted semantic/kernel foundation into a durable project-domain core before any polished editor shell is built.

## Product outcome

A non-trivial Street Concept Designer project can be:

1. created in memory;
2. organized into Existing / Alternative scenarios;
3. mutated only through deterministic semantic transactions;
4. previewed without mutating canonical state;
5. undone/redone coherently;
6. serialized as a versioned semantic project document;
7. reopened and migrated deterministically;
8. validated and rebuilt into the accepted R1 shared 2D/3D derivation;
9. compared for semantic equivalence without renderer caches.

R2 is a **project/domain core**, not a UI milestone.

---

# Why R2 is split

Persistence and undo/redo are both high-risk but have different failure modes. Implementing them together with a desktop shell would create unnecessary coupling.

R2 is therefore split into three gated packets:

## R2A — Project / Scenario Domain Core

Prove:
- Project and Scenario roots;
- stable ProjectId / ScenarioId;
- TrafficSide and minimal coordinate-context metadata;
- scenario role / lock semantics;
- validated RoadNetwork ownership per scenario;
- scenario duplication;
- project-scoped semantic identity;
- deterministic validation/equality.

No persistence library and no command history yet.

## R2B — Versioned Persistence & Migration

Blocked until R2A acceptance.

Prove:
- versioned canonical project document;
- persistence DTO separated from runtime/internal derived state;
- deterministic save/load;
- stable ids survive round trip;
- renderer caches/derived R1C DTOs are absent from canonical storage;
- authored junction intent survives round trip while derived junction geometry is rebuilt;
- schema migration harness;
- unsupported future version failure;
- canonical JSON document encoding for R2 evidence while final package/container remains deferred.

## R2C — Command / Transaction / Undo / Redo

Blocked until R2A + R2B acceptance.

Prove:
- one typed semantic mutation path;
- preview vs commit separation;
- atomic multi-command transaction;
- stale revision/precondition rejection;
- locked-scenario enforcement;
- failed transaction leaves canonical state untouched;
- meaningful undo/redo;
- redo invalidation after new commit;
- persistence after committed/undone/redone states;
- R1 shared derivation rebuild equivalence after command/history operations.

R2C is the integrated R2 exit gate.

---

# Accepted R1 evidence

R2 must preserve the accepted R1 decisions and regressions:

- Rust engineering kernel;
- canonical f64 engineering coordinates in metres;
- reference alignment + stationing;
- ordered cross-section components;
- exact semantic width/lifecycle breakpoints;
- lane add/drop/taper/turn pocket through general lifecycle;
- geometry crossing != topology;
- explicit first-class junctions;
- independent authored corner state;
- explicit lane-to-lane connectivity with manual-override preservation;
- one owned f64 shared derivation feeding diagnostic 2D/3D;
- structured semantic renderer identity;
- local render origin before f32 conversion;
- deterministic disposable derived output.

R2 must not weaken R1 to make persistence or undo easier.

---

# Project / scenario identity policy

## Project identity

ProjectId is stable persistent identity independent from:
- file name;
- project display name;
- renderer handles;
- application session.

## Scenario identity

ScenarioId is stable persistent identity within one Project.

## Scenario duplication invariant

Duplicating a Scenario:

- creates a **new ScenarioId**;
- preserves the copied scenario's internal semantic RoadId / JunctionId / ComponentId / CornerId / LaneConnectionId values where the corresponding semantic objects continue to represent the same design lineage;
- deep-copies scenario engineering state so later mutations do not cross-contaminate source and duplicate.

This supports Existing ↔ Alternative semantic comparison.

Therefore an object reference that must be globally unambiguous at project/application scope must include ScenarioId plus the scenario-local semantic identity.

Conceptually:

```text
ProjectSemanticRef
  scenario_id
  semantic_ref
```

Exact type design is an R2A implementation detail.

RoadId alone is not assumed globally unique across all scenarios.

---

# Canonical vs derived state

Canonical R2 project content may include:
- Project metadata/context;
- scenario metadata;
- scenario RoadNetwork authored/semantic state;
- future standards/reference/presentation records only when their stages implement them.

Canonical project content must not include as authoritative truth:
- R1C DerivedEngineeringSnapshot;
- 2D polygons/polylines;
- 3D mesh buffers;
- renderer local coordinates;
- GPU handles;
- hit-test indexes;
- UI selection/hover;
- junction derived pavement/corner sample caches when they can be regenerated from authored semantic state.

A load/rebuild path must regenerate equivalent accepted R1 derived geometry.

---

# Scenario model policy

Initial semantic roles:

- Existing
- Alternative

Scenario display name is editable and is not identity.

Scenario lock is canonical project intent used to protect Existing or other scenarios from committed mutations.

Active/current scenario selection is session/UI state and is **not** required as canonical engineering truth.

Storage optimization is hidden from semantic APIs. R2 may initially deep-copy scenarios; snapshot/delta/structural sharing may evolve later without changing product semantics.

---

# Coordinate / traffic context

Canonical engineering units remain metres.

R2A should introduce only the minimum durable context required now:
- TrafficSide: LHT / RHT;
- optional coordinate/CRS reference metadata sufficient to avoid ambiguous coordinate interpretation.

Do not confuse canonical/project coordinate context with R1C render-local origin.

No map provider, georeferencing transformation engine, calibration workflow, or basemap implementation belongs in R2.

---

# Persistence policy for R2B

R2B should separate:

```text
Runtime/domain state
        ↕ validated conversion
Versioned persistence DTO
        ↕ encoding
Canonical project document bytes/text
```

Do not make internal Rust memory layout the file schema.

## R2 canonical document encoding

JSON is the preferred R2 evidence encoding because it is inspectable, deterministic enough under an explicit canonical ordering policy, and compatible with later packaging.

This does **not** lock the final physical project container.

A future `.scd` package may contain the canonical JSON document plus permitted references/assets/caches. That packaging decision remains separately gated.

## Candidate serialization dependencies

`serde` + `serde_json` are preferred candidates for R2B, subject to exact-version/license/security review at execution start.

Do not add them in R2A.

## Persistence must exclude derived truth

For Junctions, persistence must capture authored semantic intent sufficient to reconstruct/revalidate:
- JunctionId;
- participating road relationship;
- explicit candidate/topology relationship needed for reconstruction;
- authored corner-radius state;
- connectivity mode;
- manual connection intent;
- required semantic options/ids.

Do not persist derived approaches, pavement surface, sampled corner arcs, or R1C render geometry as canonical truth.

On load:
1. restore roads;
2. restore authored junction semantics through a controlled restoration API;
3. recompute/revalidate disposable junction geometry/connectivity;
4. build R1C snapshot only on demand.

---

# Migration policy

Schema version is explicit.

R2B migration harness must prove:
- current version loads directly;
- unsupported future version fails clearly;
- at least one **explicitly labelled pre-release R2 migration fixture** migrates deterministically without inventing engineering assumptions.

The pre-release fixture is test evidence, not a claim that a released historical format existed.

Migration must never:
- change pinned engineering meaning silently;
- apply new standards;
- regenerate ids unnecessarily;
- reinterpret LHT/RHT;
- rewrite valid numeric engineering values.

---

# Command / transaction policy for R2C

R2C does not need every future command.

It needs enough representative typed commands to prove the architecture across project/scenario, road, and junction state.

Minimum families should include:
- AddScenario / DuplicateScenario / RenameScenario / SetScenarioLock;
- AddRoad or equivalent validated road insertion;
- ReplaceRoad or equivalent semantic road edit payload;
- CreateJunctionFromCandidate;
- SetCornerRadius;
- ReplaceLaneConnections or equivalent manual connectivity edit.

Exact enum/class structure is implementation-defined.

## ProjectSession boundary

A session-level command engine may own:
- current Project;
- monotonic session revision;
- undo stack;
- redo stack.

Session revision is not automatically canonical persisted project content.

## Preview

Preview applies a transaction to an isolated candidate copy and returns:
- candidate Project;
- validation result;
- human-readable summary/change information sufficient for later UI.

Preview must not:
- increment canonical revision;
- enter history;
- alter the current project.

## Commit

A transaction:
1. checks expected revision/preconditions;
2. applies all constituent commands to a candidate copy;
3. validates the resulting project;
4. commits atomically only if all commands succeed;
5. records one human-level history item;
6. clears redo after a new non-redo commit.

No partial state is accepted.

## Undo / redo

R2C may use snapshot-based history as the initial bounded implementation if evidence shows it is correct and acceptable for R2-scale fixtures.

Snapshot history is an implementation mechanism, **not** a product/file-schema commitment.

Required:
- undo restores equivalent pre-transaction canonical state;
- redo restores equivalent post-transaction state;
- semantic ids remain stable;
- derived R1 snapshots rebuild equivalently;
- failed transactions do not enter history;
- preview never enters history.

History persistence across application restart is deferred unless separately justified. Canonical project state must not depend on persisted undo history.

---

# Validation layers

R2 must distinguish:

1. document/container/schema validity;
2. canonical semantic referential integrity;
3. R1 internal geometry/topology coherence;
4. command validity;
5. later engineering standards advisories.

A standards advisory is not file corruption.

---

# R2 acceptance fixtures

Maintain all R1 tests and add a durable R2 fixture library.

At minimum include a non-trivial project with:
- Existing scenario;
- Alternative A duplicated from Existing;
- at least two roads;
- one junction;
- variable-width component / turn pocket;
- authored corner override;
- manual lane connectivity override where valid.

R2 integrated acceptance must prove:
- scenario isolation;
- stable object lineage across duplicate;
- save/load;
- migration;
- command preview/cancel;
- atomic commit;
- undo/redo;
- cache-free shared derivation rebuild;
- deterministic equivalent state.

---

# Performance / scale

R2 benchmarks are architecture observations, not release SLAs.

Measure representative:
- scenario duplication;
- project validation;
- serialize/deserialize;
- migration;
- command preview/commit;
- undo/redo using the chosen history mechanism;
- clean R1C snapshot rebuild after load/undo/redo.

Do not optimize by weakening semantics.

If snapshot history is clearly unsuitable even for normal R2 fixtures, report evidence and escalate before inventing a complex history architecture.

---

# R2 non-goals

Do not implement in R2:
- polished desktop/editor shell;
- React UI;
- PixiJS production renderer;
- Three.js production renderer;
- MapLibre/provider integration;
- user reference-image calibration UI;
- markings/assets library;
- Thai numeric standards;
- AI/LLM integration;
- production export;
- scenario overlay UI;
- autosave/crash recovery;
- Windows installer/portable packaging;
- collaboration/cloud;
- roundabout/U-turn special engine;
- swept path/simulation.

A minimal application scaffold is allowed only if strictly required to prove a production boundary. It must not become an excuse to start R3.

---

# R2 exit gate

R2 is complete only when:

1. R2A accepted;
2. R2B accepted;
3. R2C accepted;
4. all R1 regressions remain green;
5. one non-trivial canonical project survives:
   - create;
   - duplicate scenario;
   - semantic mutation transaction;
   - preview/cancel;
   - commit;
   - save;
   - load;
   - undo/redo in-session;
   - clean R1C rebuild;
6. no renderer cache or UI state is required to reconstruct engineering state;
7. production-boundary recommendation for R3 is documented.

Final R2 disposition:
- `PROCEED_TO_R3`
- `REMEDIATE_R2`
- `ARCHITECTURE_ESCALATION`

R2C acceptance is the integrated R2 exit decision.
