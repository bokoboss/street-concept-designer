# R2B — Versioned Persistence & Migration Evidence

## Disposition

This document records the bounded R2B implementation and qualification. The
working branch is `codex/r2b-persistence-migration`; the PR is intentionally
unmerged and R2C was not started.

The qualification PR is open and intentionally unmerged. This remediation
closes the independent-review F-01 strict-schema finding while preserving the
accepted R2B architecture and behavior. Hosted CI is green for the remediation
head recorded below; the final evidence revision is documentation-only after
that qualification.

## Execution identity and baseline

| Item | Value |
|---|---|
| Repository | `https://github.com/bokoboss/street-concept-designer` |
| Working directory | `D:\\R&D\\street-concept-designer` |
| Accepted R2B execution base | `bbaa4fe5a57e8b754efd27c0d6f9f1a5821f2031` |
| Execution branch | `codex/r2b-persistence-migration` |
| Remote branch at kickoff | 0 ahead / 0 behind the accepted base |
| Accepted `main` containment | `bbaa4fe5a57e8b754efd27c0d6f9f1a5821f2031` is an ancestor of `origin/main` |
| Implementation HEAD | `8fef27c7c58bc680fa30160bfb6812315eb1d623` |
| Hosted qualification HEAD | `a43bb6f2a491ee43ef9beb1f6cb538cbc251f855` |
| Remediation source HEAD | `914d24e70bde68f30f3e2560585b80de1a577b69` |
| Remediation qualification HEAD | `d634c1d42f87ae4f89c2e0db8e6a81eb9718f61d` |
| PR | [#24](https://github.com/bokoboss/street-concept-designer/pull/24), open and intentionally unmerged |

Before implementation, the worktree was clean, `origin` was fetched, the
accepted base was verified, and the branch was checked out from that exact
SHA. Engineering Development Workflow v1.5.0 validation used commit
`9e2494616393f0c397f065db274a0ce572206599` and reported:

```text
VALIDATION PASS (8 managed files, 2 project-owned files)
```

The local toolchain baseline is:

```text
rustc 1.98.0 (88d9e12ae 2026-08-18)
cargo 1.98.0 (797e8a9bc 2026-08-05)
active toolchain: 1.98.0-x86_64-pc-windows-msvc
installed targets: wasm32-unknown-unknown, x86_64-pc-windows-msvc
rust-toolchain.toml: Rust 1.98.0, rustfmt, clippy, wasm32-unknown-unknown
```

The host has no `link.exe` or Windows SDK import libraries. Local native
qualification therefore used the installed WinLibs GNU linker with the
installed GNU Rust toolchain for executable tests; pinned MSVC qualification
is delegated to the Windows CI job. The pinned 1.98 kernel WASM build runs
locally without that native linker dependency.

## Independent review finding and F-01 remediation

The independent review recorded on PR #24 identified that the original
`decode_document` path first deserialized the complete JSON document into
`serde_json::Value` and then called `serde_json::from_value` for the selected
DTO. JSON object entries in that intermediate map used last-write-wins
semantics, so duplicate keys such as `schemaVersion`, `projectId`, or nested
`widthM` could be collapsed before the strict DTO parser saw them. Consequently,
`deny_unknown_fields` alone did not make the previous complete parsing path
fully strict for duplicate object keys.

The remediation introduces a private permissive `SchemaVersionProbe` that
deserializes only the top-level `schemaVersion` field from the original bytes.
It allows unrelated top-level fields, but `schemaVersion` is a known Serde
field, so duplicate versions fail during the probe. Once the version is
resolved, v1 uses `serde_json::from_slice::<ProjectDocumentV1>(bytes)` and
historical v0 uses `serde_json::from_slice::<ProjectDocumentV0>(bytes)` on the
same original bytes before explicit migration. The whole-project
`serde_json::Value` intermediary and both production `from_value` calls were
removed. `serde_json::Value` remains only as the probe's scalar value carrier,
not as an intermediate project document.

Raw JSON regressions cover both `schemaVersion` orders, conflicting and
same-value `projectId`, nested `scenarioId`, engineering `widthM`, and Junction
`radiusM`. Every fixture is assembled as a raw JSON string and decoded from
bytes; no `serde_json::Value` or `json!` construction is used for the duplicate
key input.

## Bounded scope and architecture

R2B is implemented only as semantic snapshot persistence and migration:

```text
kernel (std-only)
    ↑
project-core
    ↑
project-io (Serde DTO boundary and JSON/file APIs)
```

The accepted root kernel and `project-core` dependency direction remain intact.
The kernel has no `serde`, `serde_json`, filesystem, UI, renderer, map, asset,
standards, command, or history dependency. `project-core` remains free of
serialization dependencies.

The implementation does not add commands, sessions, undo/redo, editor/UI
surfaces, `.scd` packaging, ZIP logic, autosave, recovery, locking, backups,
standards profiles, maps, assets, presentation state, export, AI, simulation,
BIM, or detailed CAD behavior. No R2C branch or feature was started.

The pre-coding scrutiny resolved the main risks as follows:

| Risk | R2B decision |
|---|---|
| Serialization leaking into the kernel | Serde derives exist only on `project-io::schema` DTOs. |
| Runtime/private layout becoming schema | Explicit field-by-field conversion uses public semantic getters and validated constructors. |
| Derived geometry persistence | Junction DTOs contain authored intent only; R1C values are rebuilt on request. |
| Lossy `f64` round trip | `serde_json/float_roundtrip`, direct DTO serialization, finite checks, and bit-exact fixtures. |
| Scenario order loss | `Project.scenarios: Vec<Scenario>` is emitted and reconstructed without sorting. |
| JSON map ordering as state | Semantic collections use explicit `Vec` fields; roads/junctions use accepted stable-id kernel order. |
| Junction restore shortcut | Kernel re-detects candidate topology and invokes the normal junction builder. |
| Unnecessary dependency/features | Only exact Serde/Serde JSON pins and the resolved standard graph were adopted. |
| R2C/R3 scope creep | No command/history/UI/container/autosave code was added. |

## Package structure

```text
Cargo.toml
src/                                  street-concept-designer-kernel
crates/project-core/Cargo.toml        street-concept-designer-project-core
crates/project-core/src/lib.rs        Project / Scenario domain semantics
crates/project-io/Cargo.toml          street-concept-designer-project-io
crates/project-io/src/lib.rs          versioned DTOs, conversion, JSON, migration, path I/O
crates/project-io/tests/support/      reusable R2B semantic fixtures
crates/project-io/tests/r2b_persistence.rs
crates/project-io/benches/r2b_persistence.rs
```

## Dependency and license qualification

Research-gate verdict: `GO WITH CONDITIONS`, using the requested exact direct
pins. The full package/license register is in
[`DEPENDENCY_LICENSE_REGISTER.md`](DEPENDENCY_LICENSE_REGISTER.md).

Direct declarations:

```toml
serde = { version = "=1.0.229", features = ["derive"] }
serde_json = { version = "=1.0.151", features = ["float_roundtrip"] }
```

The locked transitive graph is:

| Package | Version | License expression | Role |
|---|---:|---|---|
| `serde` | 1.0.229 | MIT OR Apache-2.0 | Direct DTO traits |
| `serde_json` | 1.0.151 | MIT OR Apache-2.0 | Direct compact JSON codec |
| `serde_core` | 1.0.229 | MIT OR Apache-2.0 | Serde core traits |
| `serde_derive` | 1.0.229 | MIT OR Apache-2.0 | DTO derive proc macro |
| `itoa` | 1.0.18 | MIT OR Apache-2.0 | Integer formatting |
| `memchr` | 2.8.3 | Unlicense OR MIT | JSON byte scanning |
| `zmij` | 1.0.23 | MIT | JSON floating-point formatting |
| `proc-macro2` | 1.0.107 | MIT OR Apache-2.0 | Proc-macro token support |
| `quote` | 1.0.47 | MIT OR Apache-2.0 | Proc-macro token generation |
| `syn` | 3.0.4 | MIT OR Apache-2.0 | Proc-macro syntax parsing |
| `unicode-ident` | 1.0.24 | (MIT OR Apache-2.0) AND Unicode-3.0 | Proc-macro identifier support |

All are registry packages from crates.io with upstream repositories recorded in
the register. No native C library or runtime service is introduced. Runtime
packages compile for native and WASM targets; proc-macro packages are host
build dependencies only. No `preserve_order`, `arbitrary_precision`,
`raw_value`, or `unbounded_depth` feature is enabled.

## Schema v1

`project-io::schema` defines explicit Serde DTOs with one external naming
policy: Rust `snake_case` fields are emitted as `camelCase`, and enum spellings
are explicit. `ProjectDocumentV1` is:

```text
ProjectDocumentV1
  schemaVersion: 1
  projectId
  name
  canonicalUnits: "m"
  trafficSide: "LHT" | "RHT"
  coordinateContext { referenceId?, description? }
  scenarios: ScenarioDocumentV1[]       # authored Vec order
```

Each scenario contains `scenarioId`, `name`, `role`, `locked`, and a network:

```text
NetworkDocumentV1
  roads: RoadDocumentV1[]               # accepted stable-id kernel order
  junctionDefinitions: JunctionDocumentV1[]
```

Road DTOs contain `roadId`, one of:

```text
Line                  { start {x,y}, end {x,y} }
CircularArc           { center {x,y}, radiusM, startAngleRad, sweepAngleRad }
SmoothConceptualCurve { p0, p1, p2, p3 }
```

and an ordered cross-section with `stationRange`, ordered components, ordered
width-profile knots, plus one `laneDirections` entry for every traffic lane.

Junction DTOs contain only `junctionId`, stable ordered `roadIds`, candidate
point/stations/crossing type/relation, `JunctionOptions`, keyed authored corner
radii, `LaneConnectivityMode`, and authored manual lane connections. There are
no placeholders for standards, maps, assets, presentation, export, AI, or
saved views.

## Strictness, validation, and errors

All current schema structs use `#[serde(deny_unknown_fields)]` where a struct
has fields. Current schema enum values are closed; unsupported values fail
typed JSON decoding. These checks now receive the original JSON bytes after
duplicate-safe version probing; the previous whole-document `Value` parse did
not preserve duplicate-key strictness. The public `PersistenceError`
distinguishes:

```text
JsonSyntax
JsonDecode
JsonEncode
MissingSchemaVersion
InvalidSchemaVersion
UnsupportedSchemaVersion
Migration
MalformedPersistenceDto
NonFiniteEngineeringValue
Project(ProjectError)
Kernel(KernelError)
Io (native path API only)
```

The loader parses and identifies the version before constructing any domain
object. It validates finite DTO numbers and IDs, constructs every road through
validated alignment/cross-section/road constructors, creates junctions only
after all roads exist, validates the completed network, then constructs the
Project. A failure returns no partially constructed Project.

Canonical units are explicitly metres (`"m"`); unsupported values such as
`"ft"` fail. Traffic side is explicit and never inferred from coordinates or
CRS. Coordinate context round-trips only the accepted R2A reference identifier
and description.

## Canonical order and deterministic encoding

The canonical encoder is compact UTF-8 JSON from one explicit DTO type. Struct
field order is fixed by the DTO declaration. Scenario order is copied exactly
from `Project.scenarios()`. Cross-section components, width knots, authored
corner-radius overrides, and manual connections preserve their semantic Vec
order. Roads and junctions are already stable-id ordered by the accepted
kernel; the loader inserts them through those same kernel collections. No JSON
object/map ordering is semantic state, and no RFC 8785 claim is made.

The tested deterministic claim is:

> Equal canonical Project state under the pinned schema/serializer policy emits
> equal JSON bytes.

The representative test compares both Project equality and encoded byte
equality after a save/load round trip.

## Exact finite `f64` policy

The domain-to-DTO path explicitly visits alignment coordinates, arc radius and
angles, smooth control points, station ranges, every width knot, candidate
point/stations, junction options, and authored corner radii. Non-finite values
are rejected before encoding. The DTO-to-domain path repeats finite checks
before invoking kernel constructors. JSON syntax/number range failures such as
`1e999`, null numeric fields, and malformed numeric values are errors; NaN and
infinity are never accepted as canonical JSON engineering numbers.

`serde_json` is pinned with `float_roundtrip`. Tests cover large coordinates
near `1e9`, sub-metre stations and widths, arc angles, corner radii, signed
zero, difficult finite bit patterns, and `9_007_199_254_740_991.0`. The test
fixture compares `f64::to_bits()` for alignment parameters, ranges, knots,
candidate values, option radii, and authored corner radii.

## Junction authored-state boundary and reconstruction

The kernel adds the smallest explicit boundary needed by persistence:

```text
AuthoredJunctionSnapshot
  id
  candidate (roads, point, stations, crossing type, relation)
  options
  authored corner-radius overrides
  connectivity mode
  authored manual lane connections
```

It intentionally has no approaches, cut cross-section states, corner frames or
arc samples, pavement surface, active automatic connections, R1C polygons,
render origin, or renderer buffers. `JunctionCandidate::from_parts` is an
inert validated observation constructor. `RoadNetwork::create_junction_from_authored`
is the controlled restore path.

Load order and checks are:

1. Decode current or migrated DTO and validate finite/id structure.
2. Construct all Roads first using existing validated constructors.
3. Validate exactly two stable lexical `roadIds` for each junction and resolve
   both roads from the reconstructed network.
4. Re-detect the current at-grade candidate from those roads.
5. Compare persisted candidate point, stations, crossing classification, and
   relation using the kernel's centralized tolerance policy. A mismatch is
   `KernelError::CandidateStale`.
6. Rebuild approaches, corners, surface, and connectivity through the normal
   kernel builder.
7. Apply authored corner radii by stable `CornerId`, never by position.
8. Restore manual connections when in Manual mode; valid connections become
   active and incompatible authored connections retain
   `ManualConnectivityIncompatible` without automatic fallback.
9. Confirm the resulting authored snapshot matches the persisted intent and
   validate the complete network.

Automatic mode persists only the mode and its options. Active generated
connections are recomputed. Manual mode persists the complete authored
connection definition (`LaneConnectionId`, from/to `ApproachId`, from/to
`ComponentId`, and movement).

### Stale-junction decision

R2B allows a stale junction's authored intent to be saved because the accepted
R1B/R2A network validator permits source-road edits to clear disposable state.
The file contains that authored intent and no stale geometry. On load, R2B
attempts reconstruction against the saved current Roads. If the original
candidate no longer exists, the loader returns `KernelError::CandidateStale`
instead of dropping the junction or inventing topology. The R2B API does not
introduce an unresolved junction representation; this explicit failure is the
safe accepted policy for the current stage.

The stale fixture proves both halves: stale authored state encodes successfully
with no derived fields, and loading a moved-source-road document fails clearly.

## Fixtures and tests

`crates/project-io/tests/r2b_persistence.rs` contains 16 R2B integration tests:

| Test area | Evidence |
|---|---|
| Representative Project | LHT, coordinate context, locked Existing, two Roads, turn-pocket lifecycle, junction, corner override, valid manual connection, editable Alternative A/B, non-lexical ScenarioIds/order. |
| Scenario order | `scenario-z-existing`, `scenario-m-alt-b`, `scenario-a-alt-a` remains exactly ordered. |
| Road/alignment | Line, CircularArc, and SmoothConceptualCurve constructor parameters round-trip; sampled lookup data is absent. |
| CrossSection/WidthKnot | Ordered components and irregular lifecycle knots round-trip exactly. |
| Lane directions | Every traffic-lane direction is emitted and restored. |
| Junction authored state | IDs, candidate facts, options, corner overrides, manual IDs, and active valid connections survive. |
| Automatic connectivity | Active generated connections are regenerated; no `laneConnections` authored field is emitted. |
| Manual incompatibility | Incompatible authored manual intent remains Manual and does not fall back to Automatic. |
| R1C | Fresh 2D/3D-derived outputs before and after load are equal for the representative scenarios. |
| `f64` | Bit-exact authored corpus including large coordinates, signed zero, sub-metre values, angles, widths, stations, and radii. |
| Migration | Synthetic v0 loads through explicit DTO migration; repeated loads and v1 re-encoding are deterministic. |
| Strict/corrupt inputs | Truncated/malformed JSON, missing/future/invalid versions, bad units/traffic, duplicate IDs, missing roads, invalid alignment/width/radius/manual state, candidate mismatch, reordered topology, cache-like fields, huge exponent, and null numbers. |
| Raw duplicate-key strictness | Six raw-byte regressions reject duplicate `schemaVersion` in both orders, conflicting `projectId`, nested `scenarioId`, `widthM`, `radiusM`, and same-value `projectId`. |
| Stale policy | Save succeeds for stale authored junction intent; load returns `CandidateStale`. |
| Path API | Bounded ordinary native read/write round-trip. |

The recursive derived-field test rejects canonical keys corresponding to
pavement vertices, approach frames, corner samples, alignment samples,
DerivedEngineeringSnapshot, render origin, local coordinates, and renderer
buffers. DTO type definitions separately contain no such fields.

## Migration harness

Schema v1 loads directly. Schema `0` is a clearly labelled synthetic
pre-release R2 fixture and was never publicly released. Its only difference is
the representational spelling `canonicalUnits: "metres"`; explicit
`ProjectDocumentV0` decoding migrates that value to the v1 enum `"m"`. No
engineering value, standard, traffic side, ID, or order is invented. The
production migration path does not perform arbitrary JSON string replacement.

Future versions greater than 1 fail as `UnsupportedSchemaVersion`; negative,
fractional, missing, and otherwise invalid version values fail explicitly. A
zero version with any unsupported historical shape/value fails migration rather
than being guessed.

## Qualification commands

Local commands completed so far:

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo +stable-x86_64-pc-windows-gnu check --locked --workspace --all-targets` | PASS with Rust 1.98.0 and the installed WinLibs GNU linker fallback |
| `cargo +stable-x86_64-pc-windows-gnu clippy --locked --workspace --all-targets --all-features -- -D warnings` | PASS with Rust 1.98.0 and the installed WinLibs GNU linker fallback |
| `cargo test --locked --workspace --all-targets --all-features -- --nocapture` | PASS: 89 integration tests; all existing R1/R2A tests and R2B tests |
| `cargo test --locked --package street-concept-designer-project-io --test r2b_persistence -- --nocapture` | PASS: 16 R2B tests, including all raw duplicate-key regressions |
| `cargo bench --locked --package street-concept-designer-project-io --bench r2b_persistence -- --nocapture` | PASS: 2,000 iterations per operation |
| `cargo build --locked --package street-concept-designer-kernel --target wasm32-unknown-unknown --release` | PASS with pinned Rust 1.98.0 |
| `cargo +stable-x86_64-pc-windows-gnu build --locked --package street-concept-designer-project-io --target wasm32-unknown-unknown --release` | PASS with Rust 1.98.0 and the installed GNU fallback host linker |
| Workflow v1.5.0 `setup_project.py validate .` | PASS: 8 managed files, 2 project-owned files |
| Native pinned MSVC test/check | Not runnable locally: no `link.exe`/Windows SDK; hosted Windows job required |

The workspace integration-test count is 89: 73 accepted R1/R2A tests plus 16
R2B tests. Unit-test targets contain no additional test cases.

## Benchmark

The exploratory benchmark uses a non-trivial two-scenario project and measures
2,000 iterations of each operation. The local GNU fallback run produced:

```text
document_bytes=4816
encode Project -> JSON:       15.169 us/op
decode JSON -> Project:       95.204 us/op
v0 -> v1 migration/load:      93.548 us/op
complete in-memory save/load: 106.937 us/op
clean R1C rebuild after load: 193.316 us/op
```

Platform/toolchain: Windows x86_64, Rust 1.98.0 GNU fallback with WinLibs
linker. This is exploratory evidence, not an SLA. Hosted Linux CI provides the
authoritative R2B benchmark output recorded below.

Hosted Linux release benchmark from the remediation R2B workflow run
[#33582934461](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934461),
job [100100900138](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934461/job/100100900138):

```text
target_os=linux target_arch=x86_64 document_bytes=4816 iterations=2000
encode Project -> JSON:       13.164 us/op
decode JSON -> Project:       102.586 us/op
v0 -> v1 migration/load:      100.484 us/op
complete in-memory save/load: 115.758 us/op
clean R1C rebuild after load: 194.684 us/op
```

## CI matrix and gates

The new `R2B Persistence and Migration Qualification` workflow runs on Linux
and Windows/MSVC. It covers formatting, strict workspace Clippy, workspace
check/tests, explicit project-io tests, the R2B benchmark, kernel WASM,
project-io WASM, and a pinned Rust 1.98.0 MSVC release build. Existing R1A,
R1B, R1C, R2A, and Workflow Integrity workflows remain active regressions.

The prior hosted qualification for source head
`a43bb6f2a491ee43ef9beb1f6cb538cbc251f855` remains the accepted R2B baseline
qualification. The remediation qualification for source head
`d634c1d42f87ae4f89c2e0db8e6a81eb9718f61d` is green across R2B, R2A, R1A,
R1B, R1C, and Engineering Workflow Integrity.

| Workflow | Linux job | Windows/MSVC job | Result |
|---|---|---|---|
| R2B Persistence and Migration Qualification, run [33582934461](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934461) | [100100900138](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934461/job/100100900138) | [100100899979](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934461/job/100100899979) | PASS |
| R1A Kernel Qualification, run [33582934379](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934379) | [100100899950](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934379/job/100100899950) | [100100899692](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934379/job/100100899692) | PASS |
| R1B Junction Topology Qualification, run [33582934419](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934419) | [100100900019](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934419/job/100100900019) | [100100899786](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934419/job/100100899786) | PASS |
| R1C Shared Render Qualification, run [33582934424](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934424) | [100100899911](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934424/job/100100899911) | [100100899687](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934424/job/100100899687) | PASS |
| R2A Project Core Qualification, run [33582934387](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934387) | [100100899605](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934387/job/100100899605) | [100100899766](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934387/job/100100899766) | PASS |
| Engineering Workflow Integrity, run [33582934340](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934340) | [100100899447](https://github.com/bokoboss/street-concept-designer/actions/runs/33582934340/job/100100899447) | — | PASS |

| Gate | Evidence/status |
|---|---|
| B-G0 accepted R2A base/workflow | PASS locally and hosted |
| B-G1 explicit schema v1 | PASS by DTO/code/tests |
| B-G2 stable IDs round-trip | PASS by R2B tests |
| B-G3 R1 road/lifecycle state | PASS by R2B tests and R1C comparison |
| B-G4 authored Junction reconstruction | PASS by R2B tests |
| B-G5 clean R1C equivalence | PASS by representative test |
| B-G6 deterministic encoding/order | PASS by byte/order assertions |
| B-G7 explicit v0 migration | PASS by migration tests |
| B-G8 future/malformed rejection | PASS by negative tests |
| B-G9 dependency/license/build matrix | PASS: dependency graph, Linux/Windows/MSVC, kernel/project-io WASM |
| B-G10 no scope creep | PASS by diff/scope audit and hosted Workflow Integrity |

## Known limitations

- The final physical `.scd` container is deliberately not selected.
- Native path APIs use ordinary `read`/`write`; they do not claim atomic save,
  crash safety, backups, autosave, or recovery.
- Stale junctions are saveable but fail load when current topology no longer
  contains the persisted candidate; no unresolved junction representation was
  added in R2B.
- Only the accepted R2A coordinate metadata is persisted; no CRS transform,
  map calibration, camera, or render origin is persisted.
- No standards, map, asset, presentation, export, AI, command, or history
  semantics exist in this schema.
- Non-default `TolerancePolicy` is not canonical persisted project state. R2B
  retains the accepted default-policy reconstruction assumption and makes no
  persistence promise for custom numerical policies; that decision remains
  separate before R2C/R3.
- The local host cannot execute the pinned MSVC native matrix; hosted CI is
  required for that evidence.

## Final review checklist

The remediation source is
`914d24e70bde68f30f3e2560585b80de1a577b69`; the remediation qualification
head is `d634c1d42f87ae4f89c2e0db8e6a81eb9718f61d`. The final evidence
closeout after that qualification changes documentation only. PR
[#24](https://github.com/bokoboss/street-concept-designer/pull/24) is open and
unmerged, the workspace count is 89 integration tests, the duplicate-key
regressions are green locally and hosted, and R2C remains unstarted.
