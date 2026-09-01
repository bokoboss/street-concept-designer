# R2A Project / Scenario Domain Core Evidence

## Disposition

R2A is qualified for the bounded implementation in PR [#21](https://github.com/bokoboss/street-concept-designer/pull/21).
The recommendation is `PROCEED_TO_R2B`. R2B persistence, R2C command/history, and R3 UI were not started.

## Execution identity

| Item | Evidence |
|---|---|
| Repository | `https://github.com/bokoboss/street-concept-designer` |
| Working directory | `D:\\R&D\\street-concept-designer` |
| Accepted execution base | `a6fb7ccb27f153d9e8373f8a51b645e0b531784a` |
| Execution branch | `codex/r2a-project-scenario-core` |
| Qualification/implementation HEAD | `9989142043ea4d1d43a28cd3a69d1f9fc66d470d` |
| PR | [#21](https://github.com/bokoboss/street-concept-designer/pull/21), open and intentionally unmerged |
| Workflow source | v1.5.0, commit `9e2494616393f0c397f065db274a0ce572206599` |

The branch was created from the accepted base and was 0 commits ahead/behind at kickoff. `git fetch origin` was run before branch selection; `a6fb7ccb27f153d9e8373f8a51b645e0b531784a` was verified as an ancestor of `origin/main`, and the checked-out branch initially resolved exactly to that SHA.

## Toolchain

Local Windows baseline:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
host: x86_64-pc-windows-msvc
cargo 1.98.0 (797e8a9bc 2026-08-05)
active toolchain: 1.98.0-x86_64-pc-windows-msvc
installed targets: wasm32-unknown-unknown, x86_64-pc-windows-msvc
rust-toolchain.toml: Rust 1.98.0, rustfmt, clippy, wasm32-unknown-unknown
```

Engineering Workflow v1.5.0 validation:

```text
python <workflow-v1.5.0>/scripts/setup_project.py validate .
VALIDATION PASS (8 managed files, 2 project-owned files)
```

The local host has no `link.exe`, so native test execution could not be linked locally. The hosted Windows/MSVC job is the authoritative MSVC test/build result. Local `cargo check --all-targets`, strict Clippy, formatting, and the kernel WASM release build passed.

## Package and dependency boundary

The smallest workspace conversion was used; the accepted root package and `src/` layout were preserved.

```text
Cargo.toml                         root package + workspace
src/                               street-concept-designer-kernel
crates/project-core/Cargo.toml     street-concept-designer-project-core
crates/project-core/src/lib.rs     project/scenario domain API
crates/project-core/tests/         R2A fixtures and integration tests
crates/project-core/benches/       R2A exploratory benchmark
```

Dependency direction is one-way:

```text
street-concept-designer-project-core
                ↓ path dependency
street-concept-designer-kernel
```

The kernel has no third-party runtime dependencies and no reverse dependency. `cargo tree --workspace --locked --offline` reports only the two local packages. No `serde`, `serde_json`, filesystem, Tauri, React, renderer, map, standards, or asset dependency was added. No dependency/license register entry was needed because there were no new third-party dependencies.

The root kernel remains independently checkable/buildable, and its explicit WASM artifact is still qualified with:

```text
cargo build --locked --package street-concept-designer-kernel \
  --target wasm32-unknown-unknown --release
```

## Semantic design

### Project and scenario identity

`ProjectId` and `ScenarioId` are caller-supplied newtypes. They reject empty values and any Unicode whitespace using the same predicate as the accepted R1 semantic ids. They derive deterministic value equality, ordering, and hashing. IDs are independent of names, paths, timestamps, renderer handles, and random generation.

`ScenarioId` is unique within one `Project`; scenario display names are not keys. Scenarios are stored in lexical `ScenarioId` order so equivalent construction input has deterministic project equality.

### Project

`Project` contains only:

```text
id: ProjectId
name: String
traffic_side: TrafficSide
coordinate_context: CoordinateContext
scenarios: Vec<Scenario>
```

There is no active-scenario tab, selection, camera, viewport, hover, renderer cache, or presentation state in the R2A engineering root. `Project::new` is an intermediate empty shell; `Project::validate` rejects it until a scenario is added. `Project::with_scenario` and `Project::from_scenarios` provide validated construction paths.

### Scenario and roles

`Scenario` contains its own value-owned `RoadNetwork`, stable id, display name, `ScenarioRole`, and lock flag. The only roles are `Existing` and `Alternative`; `Alternative A/B/C` remain names. Each scenario has an immutable public `network()` view. No `network_mut()` or equivalent unrestricted mutable network accessor is exposed.

`TrafficSide` explicitly supports `LeftHand` and `RightHand`. It is never inferred from geography, coordinates, or screen direction. `CoordinateContext` is deliberately limited to optional reference/CRS identifier and descriptive string metadata. It performs no transformations and does not store the R1C render-local origin.

### Lock and mutation boundary

Lock state is stored and readable with `Scenario::is_locked`. Ordinary metadata/network mutations through `Scenario` or `Project` reject `ScenarioLocked` when locked:

- rename scenario;
- remove scenario;
- replace the complete scenario network.

Explicit lock/unlock control is allowed through `set_locked` / `set_scenario_locked`. Duplication is a read-and-create operation and accepts an explicit destination lock value. There is no command history or transaction engine in R2A; committed-command enforcement is an R2C responsibility.

Network replacement validates the incoming `RoadNetwork` before assignment and is the only R2A engineering-state replacement path. The project API exposes no mutable scenario reference.

## RoadNetwork validation strategy

The kernel received one additive read-only `RoadNetwork::validate(&TolerancePolicy)` API. It checks accepted road/range/profile finiteness and coherence, id uniqueness, junction road references, authored-radius/connection identity, current non-stale surface/approach/corner/connection structure, and stale-state disposal shape.

It deliberately accepts a stale junction after `replace_road`: authored intent may remain while disposable approaches, corners, surface, and active connections are cleared. It does not require render caches or rebuild disposable geometry as a validation prerequisite, and it performs no standards advisory checks. The validator does not redesign `Road`, `Junction`, lane lifecycle, topology, or R1C snapshot semantics.

## Scenario duplication policy

`Project::duplicate_scenario(source_id, new_id, name, role, locked)` takes all destination identity/display/role/lock choices explicitly.

The duplicate:

1. receives the caller-supplied new `ScenarioId`;
2. deep-clones the source `RoadNetwork` using ordinary `Clone` value semantics;
3. retains corresponding local `RoadId`, `JunctionId`, `ComponentId`, `CornerId`, and `LaneConnectionId` values;
4. uses the caller-selected display name, role, and lock state;
5. inserts deterministically by destination scenario id.

Initial R1C `DerivedEngineeringSnapshot` values are equal when derived with the same render origin and policy because R1C snapshots intentionally do not contain `ScenarioId`. A later network replacement in the duplicate changes only the duplicate; the source remains byte/value-equal to its pre-duplication state.

## Project-scoped identity

The kernel `SemanticRef` remains unchanged and scenario-local. `project-core` adds:

```rust
ProjectSemanticRef {
    scenario_id: ScenarioId,
    semantic_ref: SemanticRef,
}
```

It derives `Eq` and `Hash`, so `Existing/R01/lane-1` and `Alternative/R01/lane-1` are distinct deterministic keys even though their embedded R1 semantic refs are equal. `Project::semantic_ref` additionally verifies that the scenario scope exists; the standalone wrapper constructor remains useful for application-level typed references.

## Canonical fixtures

Reusable integration-test fixture helpers are in `crates/project-core/tests/support/mod.rs`.

| Fixture | Coverage |
|---|---|
| P-01 Minimal Existing | One project, one locked Existing scenario, one simple road |
| P-02 Non-trivial Existing | Two roads, explicit at-grade junction, two lane components, station-based turn-pocket lifecycle, accepted R1 geometry |
| P-03 Existing + Alternative | P-02 duplicated to editable `Alternative A`, with equal local lineage ids and initially equal R1C snapshots |

## Adversarial and regression fixtures

The 12 R2A integration tests cover:

- empty and whitespace ProjectId/ScenarioId;
- project validation with no scenarios;
- duplicate ScenarioId insertion and duplicate-destination duplication;
- missing source scenario duplication;
- project/scenario renames preserving identity;
- Existing/Alternative roles and explicit lock state;
- locked rename/network replacement/removal rejection;
- last-scenario removal rejection;
- same local `R01/lane-1` in two scenarios and hash-set distinction;
- large coordinates around `1e9` with explicit render origin;
- stale R1 junction derived state accepted by validation and omitted from a fresh snapshot;
- repeated duplicate construction/equality and equal validation results;
- source/duplicate network isolation;
- R1C snapshot, 2D diagnostic, and 3D diagnostic derivation from scenario-owned networks;
- malformed coordinate-context options;
- public-API boundary behavior without privacy bypasses.

No malformed fresh `RoadNetwork` can be constructed through the accepted public kernel API without bypassing Rust privacy. The stale-after-source-edit fixture is therefore the valid public-API coherence adversary used for R2A.

## Qualification commands and results

Local commands that passed:

```text
cargo fmt --all -- --check                                  PASS
cargo check --workspace --all-targets --locked --offline     PASS
cargo clippy --workspace --all-targets --all-features \
  --locked --offline -- -D warnings                          PASS
cargo build --locked --package street-concept-designer-kernel \
  --target wasm32-unknown-unknown --release                  PASS
workflow v1.5.0 setup_project.py validate .                  PASS
```

Hosted PR qualification for commit `9989142043ea4d1d43a28cd3a69d1f9fc66d470d`:

| Workflow | Run / jobs | Result |
|---|---|---|
| Engineering Workflow Integrity | [run 33493193058](https://github.com/bokoboss/street-concept-designer/actions/runs/33493193058), job `99809252302` | PASS |
| R2A Project Core Qualification | [run 33493193295](https://github.com/bokoboss/street-concept-designer/actions/runs/33493193295); Linux job `99809252958`, Windows/MSVC job `99809253179` | PASS |
| R1A Kernel Qualification regression | [run 33493193202](https://github.com/bokoboss/street-concept-designer/actions/runs/33493193202); jobs `99809253021`, `99809253149` | PASS |
| R1B Junction Topology Qualification regression | [run 33493193079](https://github.com/bokoboss/street-concept-designer/actions/runs/33493193079); jobs `99809252479`, `99809252719` | PASS |
| R1C Shared Render Qualification regression | [run 33493193186](https://github.com/bokoboss/street-concept-designer/actions/runs/33493193186); jobs `99809252888`, `99809253222` | PASS |

Hosted R2A commands passed on Linux and Windows/MSVC:

```text
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo check --locked --workspace --all-targets
cargo test --locked --workspace --all-targets --all-features -- --nocapture
cargo test --locked --package street-concept-designer-project-core \
  --all-targets --all-features -- --nocapture
cargo build --locked --workspace --release                  # Windows/MSVC
cargo build --locked --package street-concept-designer-kernel \
  --target wasm32-unknown-unknown --release                  # Linux/WASM
```

The hosted workspace test run passed all 56 accepted R1 tests plus all 12 R2A tests: 68 implementation tests total, with zero failures. The explicit project-core test command passed all 12 R2A tests. The Windows job used pinned `1.98.0-x86_64-pc-windows-msvc`, rustc 1.98.0, and Cargo 1.98.0.

## Benchmark observations

The R2A benchmark is `crates/project-core/benches/r2a_project_core.rs`; it is exploratory and not an SLA. Linux hosted release output (`x86_64`, Rust 1.98.0):

| Operation | Iterations | Observation |
|---|---:|---:|
| Clone representative project + duplicate scenario | 2,000 | 40.69 µs/op |
| Validate five-scenario project | 2,000 | 30.01 µs/op |
| Construct/hash project-scoped semantic refs | 20,000 | 55.38 ns/op |

The benchmark completed successfully and showed no R2A architectural blocker. These numbers are machine/run observations only.

## Gate record

| Gate | Result | Evidence |
|---|---|---|
| A-G0 exact base/workflow | PASS | exact base, branch, toolchain, and workflow validation above |
| A-G1 renderer-independent Project/Scenario root | PASS | `project-core` contains value semantics only; no renderer fields/dependencies |
| A-G2 stable unique scenario identity | PASS | newtype validation, deterministic id ordering, duplicate-id tests |
| A-G3 lineage-preserving isolated duplication | PASS | P-03, local id assertions, snapshot equality, replacement isolation |
| A-G4 project-scoped semantic identity | PASS | `ProjectSemanticRef` equality/hash test across same local ref |
| A-G5 explicit traffic/coordinate context | PASS | `TrafficSide`, string-only `CoordinateContext`, render-origin separation tests |
| A-G6 deterministic validation | PASS | equal projects produce equal values and validation results |
| A-G7 accepted R1 derivation compatibility | PASS | R1C snapshot/2D/3D derivation tests from both scenario networks |
| A-G8 adversarial identity/isolation coverage | PASS | 12 R2A tests, including stale, lock, large-coordinate, and repeated cases |
| A-G9 benchmark observation | PASS | release benchmark completed with no blocker |
| A-G10 no persistence/command/UI scope creep | PASS | dependency/source/CI/scope audit below |

## R1 preservation

The accepted R1 source and tests remain in place. The only kernel source change is the additive read-only network validator. R1A, R1B, and R1C hosted workflows all passed on the R2A commit. The R1C `SemanticRef` type and shared snapshot derivation were not changed to add project scope; the wrapper remains above the kernel.

## Package-layout and scope audit

Review findings:

- no R1 source move or rewrite was needed;
- no kernel-to-project dependency or project-domain knowledge entered the kernel;
- no public project-core mutable network escape hatch exists;
- persistence DTOs, JSON, serde, migration, filesystem I/O, and physical containers are absent;
- command enums, transactions, revisions, history, undo, redo, and preview state are absent;
- no UI/session state, renderer cache, map, standards, assets, export, Tauri, React, PixiJS, or Three.js code was introduced;
- R1 stale derived-state semantics are preserved;
- no random ids, global counters, timestamps, or hidden mutable registries were introduced.

## Known limitations and recommendations

- The local Windows host cannot link native binaries because Visual Studio `link.exe` is not installed; hosted Windows/MSVC qualification passed and remains authoritative.
- The R2A coordinate context is metadata only. Transformations, calibration, georeferencing, and map providers remain later scope.
- Lock enforcement is intentionally domain-operation level. The typed committed-command/history contract belongs to R2C.
- `ProjectSemanticRef` adds scenario scope but does not claim that every embedded local `SemanticRef` points to an existing object; `Project::semantic_ref` verifies the scenario scope only.
- The project shell can be temporarily empty while being built; deterministic validation rejects it. Validated constructors ensure normal project roots contain a scenario.
- Network validation is a bounded R1 coherence check, not standards compliance or a replacement for future command validation.

Architecture recommendation: preserve this package direction and add the separately gated persistence DTO/JSON boundary above `project-core` in R2B. Recommendation: `PROCEED_TO_R2B`.
