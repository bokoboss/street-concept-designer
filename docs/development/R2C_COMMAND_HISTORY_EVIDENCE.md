# R2C Command / Transaction / Undo / Redo Evidence

## Execution identity and scope

- Exact accepted execution base: `d7fee6d5ddb03cba7cb38b10dac785c872a21346` (`Refresh accepted baseline after R2B`).
- Execution branch: `codex/r2c-command-history`.
- Final implementation HEAD before evidence closeout: `192fdc34d44da0a94e1060c8ca168311455fa6b2` (implementation commit `bf987d4d220cc66a7e1b3da6533cbddf1e2170e7` plus the first documentation-only evidence commit).
- PR: #27, open and unmerged at independent review.
- Independent review record: PR #27 review `5085930417`, decision **PASS WITH CONDITIONS**; the only condition was this evidence-only closeout plus green final-head CI.
- The final evidence-only commit necessarily follows the implementation SHA; PR #27 and Issue #26 record the final branch tip to avoid a self-referential commit hash in this file.
- Work mode: `STRICT`; mode confidence: `HIGH`.
- Workspace write boundary: `D:\R&D\street-concept-designer`.
- Local filesystem writes remained confined to `D:\R&D\street-concept-designer`; no external local filesystem, global, or system configuration writes occurred.
- The control plane clarified that GitHub operations on this target repository were authorized project workflow, not prohibited external filesystem writes. The existing R2C branch was then pushed unchanged and PR #27 was opened. No merge or R3 branch was made.
- Stage boundary: R2C only. No UI, Tauri, React, maps, assets, standards, AI runtime, persisted command log, event sourcing, autosave, crash recovery, or final package work was introduced.

The accepted base and branch were verified before implementation. The branch started clean and exactly at the accepted base after origin synchronization.

## Workflow and toolchain baseline

Engineering Development Workflow validation used workflow version `1.5.0` and upstream validation commit `9e2494616393f0c397f065db274a0ce572206599`. The validation result was `VALIDATION PASS (8 managed files, 2 project-owned files)`.

Local toolchain evidence:

- `rustc 1.98.0 (88d9e12ae 2026-08-18)`; host `x86_64-pc-windows-msvc`.
- `cargo 1.98.0 (797e8a9bc 2026-08-05)`.
- Active toolchain: `1.98.0-x86_64-pc-windows-msvc`, selected by `rust-toolchain.toml`.
- Installed target: `wasm32-unknown-unknown`.
- The local MSVC host lacks `link.exe` and `cl.exe`; native MSVC test/build execution therefore could not be completed locally. The GNU fallback used for local evidence was process-scoped and did not modify global PATH.

## Package architecture and dependency boundary

R2C adds the smallest application-facing semantic session package:

```text
street-concept-designer-kernel
              ↑
          project-core
              ↑
        project-session
```

`crates/project-session` contains `ProjectSession`, typed commands and transactions, private snapshot history, R2C tests, and the exploratory benchmark. Its normal dependencies are only:

```text
street-concept-designer-kernel
street-concept-designer-project-core
```

`street-concept-designer-project-io` is a dev-dependency used by integration tests and the benchmark. It is absent from the runtime dependency tree. No new third-party dependency was added.

## ProjectSession and canonical-state boundary

`ProjectSession` owns:

```text
Project
u64 revision
private undo snapshots
private redo snapshots
```

Construction calls `Project::validate_default()` and rejects the zero-scenario builder state. The initial revision is zero. `project()` returns only `&Project`; there is no mutable project, scenario, or network escape hatch. `Project` remains the only persisted engineering truth; revision and history are session state.

`HistoryItem` exposes only `TransactionId` and summary metadata. The before/after `Project` snapshots are private implementation state and are not exposed by public history accessors or the `ProjectSession` debug surface.

## Transaction and command model

`TransactionId` is a caller-supplied `String` wrapper. It rejects empty and whitespace-containing values and uses no randomness, timestamp, hidden counter, or global state. `Transaction` owns a caller-supplied id, expected revision, trimmed meaningful summary, and a non-empty ordered `Vec<Command>`. It contains no UI, renderer, JSON, or raw language-model payload.

The bounded typed command inventory is:

- `AddScenario`.
- `DuplicateScenario` with explicit source id, new id, name, role, and lock state.
- `RenameScenario`.
- `SetScenarioLock`.
- `AddRoad` with explicit `ScenarioId`.
- `ReplaceRoad` with explicit `ScenarioId`.
- `CreateJunctionFromCandidate` with explicit `ScenarioId`, current candidate, junction id, and `JunctionOptions`.
- `SetCornerRadius` with explicit scenario, junction, corner, and radius.
- `ReplaceLaneConnections` with explicit scenario, junction, and complete authored manual connections.

Every engineering command carries an explicit `ScenarioId`; equal local road, junction, lane, and component identities in separate scenarios are not resolved by name or renderer coordinates.

All command geometry and validation uses accepted project-core/kernel APIs. The command layer does not reconstruct topology, surfaces, corner arcs, lane connectivity, or renderer geometry. Every command path uses `TolerancePolicy::default()` internally; no numerical policy is part of transaction/session API or persistence.

## Shared evaluation, preview, and commit

Preview and commit both call the single private `evaluate_transaction` path. It checks the expected revision, clones the canonical project, applies commands sequentially to the candidate, validates the final candidate with `Project::validate_default()`, and creates transaction metadata. A semantic no-op is rejected as `NoOpTransaction`.

Preview returns the isolated candidate and leaves the canonical project, revision, undo stack, and redo stack unchanged. The equivalence test compares the preview candidate with the state after committing the same transaction at the unchanged revision; they are equal.

Commit calculates the checked next revision before adopting state, then adopts the fully validated candidate, creates exactly one history entry, advances once, and clears redo. A failed command or stale revision leaves project, revision, undo, and redo state unchanged. The required partial-success fixture (`AddRoad R03` followed by duplicate `AddRoad R03`) passes atomically.

## Revision, stale proposals, and history

Revision transitions are monotonic checked `u64` increments:

```text
commit: N -> N+1
undo:   N -> N+1
redo:   N -> N+1
preview/failure: N -> N
```

The private overflow helper rejects `u64::MAX` before mutation. Every transaction, undo, and redo operation checks its expected current revision and returns `StaleRevision` on mismatch. Proposals remain stale after a commit, undo, or redo; no automatic rebase exists.

History is snapshot-based and transaction-level. One multi-command transaction creates one metadata item and one undo step. Undo restores the exact private `before` snapshot and moves the entry to redo. Redo restores the stored `after` snapshot without re-executing command or external logic and moves the entry back to undo. A divergent commit after undo clears redo. Repeated commit/undo/redo cycles preserve exact project identity and monotonic revision.

## Lock semantics

Locked scenarios remain readable and may be duplicated because duplication does not mutate the source. `SetScenarioLock` is the explicit lock-control operation and permits both locked-to-unlocked and unlocked-to-locked transitions.

The following mutations reject locked targets in both preview and commit: rename, add road, replace road, create junction, corner-radius edit, and manual lane-connectivity replacement. Ordinary engineering editing succeeds after explicit unlock. Lock rejection is returned through the renderer-independent project/session error model.

## Road and junction integration

`AddRoad` clones the target scenario network, calls `RoadNetwork::add_road`, then replaces the complete network through project-core. `ReplaceRoad` collects affected junction ids before calling `RoadNetwork::replace_road`, then calls `RoadNetwork::regenerate_junction` for each affected junction. `Regenerated`, `ManualConnectivityIncompatible`, and `Stale` are accepted; actual kernel errors fail the transaction. Stale authored intent is retained by the accepted kernel behavior.

`CreateJunctionFromCandidate` calls `RoadNetwork::create_junction`, allowing the kernel to re-detect current geometry and reject stale or ignored candidates. Corner radius and manual connectivity use `Junction::set_corner_radius` and `Junction::replace_lane_connections`; authored manual invalidity fails atomically without automatic fallback.

## Persistence and R1C rebuild evidence

Integration tests use `project-io` only around the canonical `session.project()` value. The sequence covers scenario duplication, semantic edit, non-mutating preview, committed save/load, undo save/load, redo save/load, and clean R1C rebuild at every state. Encode/decode equality and clean `DerivedEngineeringSnapshot -> diagnostic 2D -> diagnostic 3D` output equality pass for committed, undone, and redone projects.

The persistence negative test confirms project JSON contains none of the transaction id, summary, revision, undo, redo, or history state. No R2B schema or persistence behavior was changed.

Inherited R2B limitation remains documented and unchanged: a stale junction whose candidate no longer exists may be saveable but can fail reopen with `CandidateStale`. The persistence acceptance fixture deliberately remains reopenable and does not conceal this limitation.

## Tests and gates

Final local commands and results:

- `cargo fmt --all -- --check`: pass.
- strict workspace Clippy with `--locked --offline --workspace --all-targets --all-features -- -D warnings`: pass.
- workspace check with all targets: pass.
- workspace release build: pass.
- explicit R2C integration suite: **24 passed, 0 failed**.
- full workspace test run: **114 tests passed, 0 failed**: 89 carried R1/R2A/R2B tests, 24 R2C integration tests, and 1 R2C revision-overflow unit test. Existing benchmark targets also completed successfully in the all-targets run.
- project-session strict Clippy target: pass.
- project-session WASM release build: pass.

The full workspace run preserved the following regression counts: R1A 24, R1B 20, R1C 12, R2A 17, R2B 16, plus R2C 25.

## Benchmark and snapshot storage proxy

The exploratory benchmark uses a representative 3-scenario project and 500 iterations under optimized Rust 1.98.0 GNU fallback execution on Windows x86_64:

| Operation | Total | Per operation |
| --- | ---: | ---: |
| Project clone | 8.375 ms | 16.749 µs |
| Preview | 22.648 ms | 45.297 µs |
| Commit | 43.432 ms | 86.864 µs |
| Undo | 56.936 ms | 113.872 µs |
| Redo | 57.556 ms | 115.112 µs |
| Clean R1C rebuild after commit | 169.128 ms | 338.257 µs |

Canonical project JSON was 4,851 bytes. The transparent snapshot storage proxy is `4,851 * 2 = 9,702 bytes` per history entry. This is a JSON-size proxy, not an exact Rust heap measurement. It shows no immediate blocker at the normal R2 fixture scale; no structural-sharing or history-compression architecture was introduced.

## CI and WASM qualification

`.github/workflows/r2c-command-history.yml` adds:

- Linux fmt, strict workspace Clippy, workspace check/tests, explicit project-session tests, R2C benchmark, kernel WASM, project-session WASM, and project-io WASM release builds.
- Windows/MSVC Rust 1.98.0 fmt, strict Clippy, workspace check/tests, explicit project-session tests, and release workspace build.

Hosted qualification on PR #27 passed:

- R2C Command and History Qualification run `33594300115`: **PASS**.
  - Linux job `100134512595`: **PASS**.
  - Windows/MSVC Rust 1.98 job `100134512837`: **PASS**.
- R2B regression run `33594299986`: **PASS**.
- R2A regression run `33594300118`: **PASS**.
- R1A regression run `33594300013`: **PASS**.
- R1B regression run `33594300020`: **PASS**.
- R1C regression run `33594300056`: **PASS**.
- Engineering Workflow Integrity run `33594300096`: **PASS**.

Hosted logs independently confirm:
- R2C integration tests: **24/24 PASS**.
- R2C revision-overflow unit test: **1/1 PASS**.
- Kernel WASM release build: **PASS**.
- Project-session WASM release build: **PASS**.
- Project-io WASM release build: **PASS**.

Hosted optimized Linux benchmark (500 iterations):
- Project clone: **17.292 µs/op**.
- Preview: **59.135 µs/op**.
- Commit: **96.126 µs/op**.
- Undo: **113.711 µs/op**.
- Redo: **132.921 µs/op**.
- Clean R1C rebuild after commit: **337.749 µs/op**.

Local MSVC native execution remains unavailable on the workstation because `link.exe`/`cl.exe` are absent; hosted Windows/MSVC provides the required platform qualification.

## R2C acceptance gate matrix

| Gate | Result | Evidence |
| --- | --- | --- |
| C-G0 accepted R2A/R2B base/workflow | Pass | Exact base and Workflow 1.5.0 validation recorded above. |
| C-G1 one typed path mutates canonical session project | Pass | Private evaluation path and typed command inventory. |
| C-G2 shared preview path and non-mutation | Pass | Preview/commit equivalence and immutability tests. |
| C-G3 transaction atomicity | Pass | Multi-command partial-success failure fixture. |
| C-G4 stale preconditions | Pass | Commit, undo, redo, and preview stale-revision tests. |
| C-G5 scenario lock enforcement | Pass | Six-command preview/commit lock matrix and unlock regression. |
| C-G6 exact undo/redo | Pass | Semantic snapshot equality and repeated-cycle tests. |
| C-G7 failed commands do not enter history | Pass | Invalid target/payload and no-op atomicity tests. |
| C-G8 persistence and clean R1C rebuild | Pass | Commit/undo/redo save/load and rebuild integration test. |
| C-G9 snapshot-history scale evidence | Pass | 500-iteration benchmark and 9,702-byte JSON proxy. |
| C-G10 regressions and no scope creep | Pass | 114 local tests plus hosted R2C/R2B/R2A/R1A/R1B/R1C and Workflow Integrity all green; R2-only diff. |

## Known limitations and scope audit

- Undo/redo history and session revision intentionally do not survive restart in R2C.
- The accepted R2B stale-junction reopen limitation remains unchanged.
- The benchmark reports a transparent storage proxy rather than heap allocation accounting.
- Local Windows/MSVC native gates cannot run without `link.exe`/`cl.exe`; the workflow is prepared for hosted verification.
- The initial no-push interpretation was corrected by the control plane: GitHub push/PR operations for the target repository were explicitly authorized while local filesystem/system write restrictions remained unchanged.
- Independent review of the actual PR diff and hosted logs found no implementation blocker. Review #5085930417 returned **PASS WITH CONDITIONS**, with this evidence-only closeout and green final-head CI as the sole remaining condition.
- No R3 or UI work was started.

## Recommendation

`PROCEED_TO_R3`

All R2C implementation gates are green, hosted Linux/Windows qualification and inherited regressions are green, and independent review found no implementation blocker. Final acceptance remains contingent only on this documentation-only closeout commit retaining a green final-head CI matrix before PR #27 is merged. R3 must not start automatically.
