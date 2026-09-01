# R1B Junction Geometry & Topology Evidence

## Scope and decision

R1B implements the bounded concept-stage junction contract from
`specs/execution/R1B_JUNCTION_TOPOLOGY.md` on top of the accepted R1A base.
The result is renderer-free Rust semantic/kernel code.  It demonstrates that
an XY crossing is not network topology: detection returns an inert
`JunctionCandidate`, while only an explicit `RoadNetwork::create_junction`
call creates a first-class `Junction`.

Recommendation at this evidence capture: `PROCEED_TO_R1C`.  The required
hosted Windows/MSVC, Linux/WASM, and workflow-integrity checks are green.  The
implementation is intentionally not a claim of standards-grade junction
design.

## Reproducibility

- Accepted execution base: `fc384b531d9f3a04790cbd9606dc71af3b310a0e`.
- Execution branch: `codex/r1b-junction-topology`.
- Implementation verification commit: `3cc7549` (`R1B: add junction geometry
  and topology kernel`); the branch was clean at the accepted base before R1B
  changes.
- Hosted CI verification SHA: `ded5b2325972490ee4c87d220c7f7f78628db9ac`.
- Workflow validation: Engineering Development Workflow v1.5.0, validated
  with `setup_project.py validate`; result was `VALIDATION PASS` for the
  managed/project-owned workflow files.
- Rust toolchain: `1.98.0-x86_64-pc-windows-msvc`, `rustc 1.98.0 (88d9e12ae
  2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.
- Targets checked locally: `wasm32-unknown-unknown` and the pinned MSVC
  target.  A locally installed `stable-x86_64-pc-windows-gnu` toolchain was
  used to execute the native tests because this host does not expose
  `link.exe`; the hosted Windows workflow remains the authoritative MSVC test.
- New third-party dependencies: none.  `Cargo.lock` remains dependency-free;
  surface construction uses a small std-only monotonic-chain convex hull.

## Semantic model

- `Road` wraps one accepted R1A `Alignment` and `CrossSection`, retains stable
  `RoadId`, and supports explicit lane travel direction relative to increasing
  station.  Unspecified concept-stage lanes are bidirectional.
- `RoadNetwork` stores roads in lexical id order and stores only explicitly
  created junctions.  Candidate detection is immutable.
- `Junction` owns connected road ids, explicit approaches, independent
  corners, semantic lane connections, and derived surface state.
- Stable approach ids are `<road-id>::start`/`<road-id>::end`; corner ids are
  derived from the stable approach pair; generated lane-connection ids are
  derived from junction, approach, lane, and movement ids.  They do not depend
  on floating-point coordinates or insertion order.
- Ignore and grade-separated outcomes remain disconnected.  Duplicate and
  stale candidates are rejected.

## Geometric strategy

- Candidate detection samples each R1A alignment with
  `SamplingOptions::from_policy`, intersects the derived segments, and
  interpolates bounded stations.  Existing centralized coordinate and station
  tolerances classify endpoint meetings; no new scattered epsilon is used.
- Approaches are cut by station side and use the source alignment tangent and
  left-hand normal.  No screen-left/screen-right convention is used.
- Each adjacent approach pair receives an independent circular
  radius-style corner.  Five deterministic arc samples retain the semantic
  radius while avoiding a standards/compound/swept-path claim.
- The pavement surface is derived from width-aware approach mouths and corner
  arc samples, then reduced to a deterministic convex envelope.  The public
  `PavementSurface` validator requires finite vertices, positive area above
  the existing minimum-length-derived area threshold, counter-clockwise
  winding, and a simple polygon.  Invalid or degenerate geometry is rejected.

## Topology and lane strategy

- `CrossingRelation::AtGrade` is required for creation.  A
  `GradeSeparated` candidate can be observed but cannot create a junction.
- `CandidateDisposition::Ignore` is inert and cannot create a junction.
- Automatic proposals enumerate valid incoming-to-outgoing lane pairs, classify
  left/through/right from station-derived approach headings, and omit U-turns.
  Proposals are stored as semantic `LaneConnection` values and can be wholly
  replaced through validated `Junction::replace_lane_connections`.
- Missing approaches, missing/inactive/non-traffic lanes, duplicate connection
  ids, U-turn pairs, and movement-category mismatches are rejected.
- Replacing a source road clears approaches, corners, surface, and lane
  connections and marks the junction `Stale`.  Explicit regeneration rebuilds
  from current candidate geometry while preserving corner radii by stable
  corner id where possible.

## Independent review record

- Review boundary: actual R1B diff, accepted R1A source/tests/CI, execution
  contract, tolerance policy, evidence, and the complete local verification
  results.  The review specifically challenged geometry/topology separation,
  sampled-crossing classification, stable ids, surface validity, lane
  compatibility, invalidation, and scope containment.
- Independence basis: a fresh-context pass over the implementation artifact
  followed by deterministic reruns of formatter, clippy, native tests, pinned
  MSVC check, WASM build, workflow validation, and release benchmark.
- Review-discovered issue fixed: the first candidate implementation treated a
  sampled polyline vertex as an alignment endpoint.  An interior crossing that
  happened to fall on a sample boundary could therefore be mislabeled as an
  endpoint meeting.  Classification now uses the interpolated station against
  the source alignment domain, with a regression test.
- Review-discovered issue fixed: manual lane replacement initially validated
  endpoints and U-turn exclusion but accepted a mismatched left/right/through
  label.  Replacement now recomputes the expected category from approach
  headings and rejects mismatches, with a regression test.
- Maintainability issue fixed: the corner constructor was reshaped around a
  frame value after strict clippy rejected the initial eight-argument helper.
- Final review decision: `PASS`; no unresolved material implementation finding
  remains.  Hosted CI closed the local MSVC-linker evidence gap.

## Fixture coverage

The integration suite in `tests/r1b_junction.rs` covers:

| Fixture | Evidence |
| --- | --- |
| J-01 90-degree T | `t_and_four_leg_junctions_have_valid_deterministic_derived_geometry` |
| J-02 skewed T | `skewed_t_fixture_keeps_station_side_semantics_and_valid_surface` |
| J-03 90-degree four-leg | `t_and_four_leg_junctions_have_valid_deterministic_derived_geometry` |
| J-04 skewed four-leg | `skewed_and_unequal_width_junctions_remain_finite_and_valid` |
| J-05 unequal approach widths | `skewed_and_unequal_width_junctions_remain_finite_and_valid` |
| J-06 endpoint meeting vs true crossing | `endpoint_meeting_and_near_miss_are_distinct` |
| J-07 Ignore | `ignore_and_grade_separation_never_create_topology` |
| J-08 GradeSeparated | `ignore_and_grade_separation_never_create_topology` |
| J-09 divided-to-undivided | `divided_to_undivided_cross_sections_are_supported_without_topology_inference` |

## Adversarial, determinism, property, and fuzz-style coverage

The suite exercises acute/near-tangent lines, near misses, near-coincident
endpoints, large coordinates, very short alignment rejection, invalid radii,
self-intersecting and degenerate public surfaces, duplicate creation, missing
lanes, incompatible manual connections, source-road movement/width changes,
and repeated detection/generation.  The generated 64-offset corpus in
`property_and_fuzz_style_candidate_corpus_is_repeatable_and_controlled`
repeats candidate detection and surface validation for each case without a
property-testing dependency.

No accepted path emits a non-finite candidate, corner, surface, or lane
connection.  The convex-hull construction means accepted derived surfaces are
simple by construction and are revalidated at their public boundary.

## Verification commands and results

Expected project commands:

```text
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features -- --nocapture
cargo bench --locked --bench r1b_junction
cargo build --locked --target wasm32-unknown-unknown --release
```

Local results captured during implementation:

- Formatting: pass.
- Clippy with `-D warnings`: pass.
- GNU native execution: all 24 accepted R1A tests and all 16 R1B tests passed
  (40 integration tests total).
- Pinned MSVC `cargo check --all-targets`: pass; native test execution was
  unavailable on this host solely because `link.exe` is not installed, and the
  hosted MSVC test completed successfully.
- WASM `cargo check --target wasm32-unknown-unknown`: pass; hosted WASM
  release artifact build completed successfully.
- Release benchmark (GNU executable on this host; 2,000 iterations): candidate
  detection `314.44 us/op`, T generation `334.95 us/op`, four-leg generation
  `667.80 us/op`, connectivity lookup `20,000 ops in 5.2355 ms`, and
  regeneration after width change `1,025.13 us/op`.  These measurements are
  exploratory and are not a performance blocker; the hosted matrix remains
  authoritative for the pinned MSVC target.

## CI and workflow

- `.github/workflows/r1b-kernel.yml` preserves Linux native tests, strict lint,
  release R1B benchmark, WASM release build, and pinned Windows/MSVC
  format/lint/check/test/release-build coverage.
- Existing R1A workflow remains present and continues to qualify the accepted
  alignment/lane behavior.
- Workflow-integrity validation remains pinned to the repository's v1.5.0
  workflow commit.
- PR #13 hosted checks for SHA `ded5b2325972490ee4c87d220c7f7f78628db9ac`
  completed successfully: [R1B Linux/WASM run](https://github.com/bokoboss/street-concept-designer/actions/runs/33453810321),
  [R1B Windows/MSVC run](https://github.com/bokoboss/street-concept-designer/actions/runs/33453810321),
  [R1A qualification run](https://github.com/bokoboss/street-concept-designer/actions/runs/33453810209),
  and [workflow-integrity run](https://github.com/bokoboss/street-concept-designer/actions/runs/33453810246).

## Limitations and explicit non-goals

This phase intentionally does not add commands/transactions, persistence,
renderer or UI behavior, map/satellite integration, roadway standards,
signalization, channelization, swept paths, simulation, roundabouts, or U-turn
engines.  Pairwise alignment candidates and convex concept-stage surfaces are
the bounded R1B model.  A future phase may replace the surface derivation only
with a separately justified semantic/geometry design; R1B does not conceal
that limitation behind renderer output or tolerance inflation.
