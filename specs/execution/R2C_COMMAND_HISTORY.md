# R2C — Command / Transaction / Undo / Redo

## Objective

Prove the production semantic mutation path on the accepted R2A Project and R2B persistence model.

R2C is the integrated R2 exit gate.

## Entry

Requires R2A and R2B accepted/merged.

Record exact accepted `main` base before first change.

## Core architecture

Committed engineering edits converge on one transaction engine:

```text
intent
 → typed command(s)
 → candidate Project copy
 → command/domain validation
 → Project validation
 → commit atomically
 → revision/history
 → disposable R1 derived rebuild
```

Preview follows the same candidate path but does not commit.

## Session vs canonical project

Introduce the smallest session/controller boundary required.

Concept:

```text
ProjectSession
  project
  revision
  undo history
  redo history
```

Session revision/history are not required canonical persisted Project fields.

Do not couple ProjectSession to React/Tauri/renderer objects.

## Typed commands

Implement a bounded representative command set sufficient to exercise:

### Project/scenario
- AddScenario;
- DuplicateScenario;
- RenameScenario;
- SetScenarioLock.

### Road
- AddRoad with a validated semantic Road payload or equivalent;
- ReplaceRoad with a validated semantic Road payload or equivalent.

### Junction/topology
- CreateJunctionFromCandidate;
- SetCornerRadius;
- ReplaceLaneConnections / set manual connectivity.

Exact type names may differ.

Do not implement the complete AI command catalog.

## Target scoping

Every scenario mutation command must target ScenarioId explicitly.

Road/Junction/Component targets are interpreted within that Scenario.

Do not allow a raw RoadId to accidentally modify a same-id road in another Scenario.

## Scenario lock

Committed mutation of a locked Scenario must fail deterministically.

Read-only preview/explanation may inspect locked scenarios.

Scenario-management command to unlock may be separately allowed according to explicit target semantics.

## Transaction

One Transaction may contain one or more typed commands.

Required metadata conceptually:
- TransactionId / caller supplied stable id;
- command version(s);
- expected session revision;
- human-readable summary;
- source/provenance category optional but bounded.

Do not use random ids inside deterministic kernel/domain code.

## Atomicity

Apply transaction to isolated candidate state.

If any command fails:
- canonical Project unchanged;
- revision unchanged;
- undo/redo unchanged;
- no partial R1 invalidation leaks;
- no history entry.

Only after complete validation does candidate replace canonical Project.

## Preview

`preview(transaction)`:
- validates expected revision;
- applies to candidate copy;
- returns candidate Project / validation result / summary;
- canonical Project unchanged;
- revision unchanged;
- history unchanged.

Preview candidate may be used to build R1C snapshots for later UI.

Preview must not create a shadow model with separate mutation rules.

## Stale proposal/revision

Use a monotonic session revision or equivalent explicit precondition.

A transaction/proposal created for revision N must reject at revision N+1 unless deliberately reconstructed/rebased by a later layer.

Undo/redo should also move session revision forward rather than resurrecting an old revision number, so stale proposals remain stale.

## Undo implementation

Preferred bounded R2C implementation:
snapshot-based semantic history.

For each committed human transaction retain enough before/after Project state to restore deterministic semantics.

This is permitted because:
- Project is still bounded;
- correctness is more important than premature optimization;
- product semantics do not depend on history storage mechanism.

Benchmark it.

If snapshots are already an obvious blocker for normal R2 fixture sizes, report evidence and escalate rather than hiding the problem.

## Undo

Required:
- one transaction → one history item;
- restores equivalent pre-transaction Project;
- ids restored exactly;
- R1 derived state rebuilds equivalently;
- redo becomes available;
- revision increments.

## Redo

Required:
- restores equivalent post-transaction Project;
- does not re-run an ambiguous external/AI decision;
- revision increments;
- repeated undo/redo deterministic.

New committed transaction after undo clears redo.

## Human-readable history

History summary should represent semantic intent, not low-level field changes.

Examples:
- Duplicate Existing as Alternative A
- Add Road R01
- Create Junction J01
- Set corner radius to 12 m

No renderer operations in history.

## Persistence integration

Undo history does not need to persist.

But at each canonical state:
- current Project can be saved via R2B;
- reopened project equals the saved canonical state;
- clean R1C snapshot rebuild is equivalent.

Required integrated test sequence:

1. create representative Project;
2. commit scenario duplication;
3. commit road/junction semantic edit;
4. preview another edit then cancel;
5. save/load committed Project;
6. undo;
7. save/load undone Project;
8. redo;
9. save/load redone Project;
10. clean R1C rebuild after each state.

## Invalid/failed command tests

At minimum:
- duplicate ScenarioId;
- target Scenario missing;
- target Road missing;
- locked Scenario mutation;
- invalid Road payload;
- stale expected revision;
- invalid corner id/radius;
- invalid manual lane connection;
- multi-command transaction where command 1 succeeds on candidate but command 2 fails;
- ensure no canonical partial mutation/history entry.

## R1 invalidation/regeneration

Commands that replace Road or edit junction authored state must use accepted R1 APIs.

Do not manually repair derived surfaces/meshes in the command layer.

After commit, accepted R1B/R1C regeneration remains the owner of derived geometry.

## Tests

Preserve all R1 and R2A/R2B tests.

Required:
- preview immutability;
- single-command commit;
- atomic multi-command;
- failure atomicity;
- stale revision;
- scenario lock;
- undo;
- redo;
- redo clear after divergent commit;
- repeated cycles;
- scenario-local target scoping;
- save/load at command states;
- R1C rebuild equivalence;
- deterministic history summary/order.

## Benchmarks

Exploratory:
- preview representative transaction;
- commit;
- undo;
- redo;
- snapshot history memory/clone cost for representative multi-scenario project;
- R1C rebuild after commit.

No SLA.

## Success gates

C-G0 accepted R2A/R2B base/workflow.
C-G1 one typed path mutates canonical Project.
C-G2 preview uses same validation path and does not mutate canonical state.
C-G3 transaction atomicity.
C-G4 stale revision/preconditions explicit.
C-G5 scenario lock enforced.
C-G6 undo returns equivalent pre-state; redo equivalent post-state.
C-G7 failed commands never enter history/partially mutate.
C-G8 save/load + clean R1C rebuild works for committed/undone/redone states.
C-G9 snapshot-history benchmark shows no immediate architecture blocker or is explicitly escalated.
C-G10 all R1/R2 regressions green and no UI/R3 scope creep.

## R2 exit decision

If all C gates pass, R2 overall may be accepted.

Final recommendation:
- `PROCEED_TO_R3`
- `REMEDIATE_R2`
- `ARCHITECTURE_ESCALATION`

## Evidence

Create:
`docs/development/R2C_COMMAND_HISTORY_EVIDENCE.md`

Do not start R3 automatically.
