# R1B Junction Geometry & Topology Evidence

## Scope and decision

R1B implements the bounded concept-stage junction contract from
`specs/execution/R1B_JUNCTION_TOPOLOGY.md` on top of the accepted R1A base.
The result is renderer-free Rust semantic/kernel code.  It demonstrates that
an XY crossing is not network topology: detection returns an inert
`JunctionCandidate`, while only an explicit `RoadNetwork::create_junction`
call creates a first-class `Junction`.

The R1B remediation is qualified and the recommendation at this evidence
capture is `PROCEED_TO_R1C`.  The required hosted Windows/MSVC, Linux/WASM,
R1A, and workflow-integrity checks are green.  The implementation is
intentionally not a claim of standards-grade junction design.

## Reproducibility

- Accepted execution base: `fc384b531d9f3a04790cbd9606dc71af3b310a0e`.
- Execution branch: `codex/r1b-junction-topology`.
- Original R1B implementation verification commit: `3cc7549` (`R1B: add
  junction geometry and topology kernel`).
- Remediation parent/reviewed HEAD: `c7aedb36053a695b25f17e2c7e10ddc76ec8b785`.
- Remediation implementation commit: `810057298739e85534de7b8e69b15ae0e2062650`
  (`R1B: preserve authored junction intent`).
- Final semantic/evidence commit: `c3edf4004188514268c6f49a10e9a9529aaac197`.
- Hosted CI verification SHA for the final semantic/evidence state:
  `c3edf4004188514268c6f49a10e9a9529aaac197`.
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
- `Junction` owns connected road ids and authored intent separately from
  derived state: stable-id keyed corner-radius values, a lane-connectivity
  mode, and any manual lane-connection set are semantic; approaches, corner
  arc samples, pavement surface, and active validated connections are derived.
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
  `LaneConnectivityMode::Automatic` regenerates those proposals from current
  approaches; they are not treated as authored overrides.
- `Junction::replace_lane_connections` switches the junction to
  `LaneConnectivityMode::Manual`, validates the replacement, and retains the
  exact semantic set separately from the active derived connections.
- Missing approaches, missing/inactive/non-traffic lanes, duplicate connection
  ids, U-turn pairs, and movement-category mismatches are rejected.
- Replacing a source road clears approaches, corners, surface, and active lane
  connections and marks the junction `Stale`; authored radius values, mode, and
  manual intent remain. Explicit regeneration rebuilds current geometry and
  reapplies only matching corner ids. Unmatched authored corner entries remain
  inspectable and are never applied to another corner by position.
- Automatic mode regenerates deterministic proposals. Compatible manual mode
  revalidates and retains the exact authored set without adding automatic
  connections. If manual intent is incompatible with current lanes/approaches,
  active connections stay empty and regeneration returns
  `ManualConnectivityIncompatible` with the authored set retained for
  resolution.

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
- Remediation finding F-01: the original implementation stored an edited radius
  only in the live `Corner`; invalidation cleared that geometry, so regeneration
  used the default/initial radius. The remediation stores radii separately by
  stable `CornerId`, updates that authored map from `set_corner_radius`, and
  rebuilds only disposable arc samples.
- Remediation finding F-02: the original implementation stored a manual
  replacement only in active `lane_connections`; invalidation cleared it and
  regeneration silently returned automatic proposals. The remediation stores
  connectivity mode and manual intent separately, revalidates compatible
  manual sets, and exposes an explicit incompatible status with no active
  fallback when validation fails.
- Remediation regression coverage proves one and multiple corner overrides,
  compatible manual preservation, incompatible manual retention without auto
  fallback, automatic deterministic regeneration, and complete derived-state
  clearing while stale.
- Final remediation review decision: `PASS`; the two authored-state findings
  are closed and no unresolved material implementation finding remains.
  Hosted CI closed the local MSVC-linker evidence gap.

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
| Authored corner persistence | `source_road_changes_clear_derived_state_and_regenerate_deterministically`, `multiple_authored_corner_overrides_survive_compatible_road_edit` |
| Compatible manual connectivity | `manual_connectivity_survives_compatible_road_edit_without_auto_fallback` |
| Incompatible manual connectivity | `incompatible_manual_connectivity_is_explicit_and_has_no_auto_fallback` |

## Adversarial, determinism, property, and fuzz-style coverage

The suite exercises acute/near-tangent lines, near misses, near-coincident
endpoints, large coordinates, very short alignment rejection, invalid radii,
self-intersecting and degenerate public surfaces, duplicate creation, missing
lanes, incompatible manual connections, authored corner/manual connectivity
retention, source-road movement/width changes, and repeated
detection/generation.  The generated 64-offset corpus in
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
- GNU native execution: all 24 accepted R1A tests and all 20 R1B tests passed
  (44 integration tests total), including the six remediation behaviors.
- Pinned MSVC `cargo check --all-targets`: pass; native test execution was
  unavailable on this host solely because `link.exe` is not installed, and the
  hosted MSVC test completed successfully.
- WASM `cargo check --target wasm32-unknown-unknown`: pass; hosted WASM
  release artifact build completed successfully.
- Post-remediation release benchmark (GNU executable on this host; 2,000
  iterations): candidate detection `312.43 us/op`, T generation `341.09 us/op`,
  four-leg generation `699.21 us/op`, connectivity lookup `20,000 ops in
  5.5979 ms`, and regeneration after width change `1,429.79 us/op`.  A repeat
  measured regeneration at `1,729.45 us/op`; these exploratory timings include
  host variance and the added bounded authored-state bookkeeping, and show no
  architectural performance blocker.  The hosted matrix remains authoritative
  for the pinned MSVC target.

## CI and workflow

- `.github/workflows/r1b-kernel.yml` preserves Linux native tests, strict lint,
  release R1B benchmark, WASM release build, and pinned Windows/MSVC
  format/lint/check/test/release-build coverage.
- Existing R1A workflow remains present and continues to qualify the accepted
  alignment/lane behavior.
- Workflow-integrity validation remains pinned to the repository's v1.5.0
  workflow commit.
- PR #13 hosted checks for remediation SHA
  `810057298739e85534de7b8e69b15ae0e2062650` completed successfully:
  [R1B Linux/WASM run](https://github.com/bokoboss/street-concept-designer/actions/runs/33456313520),
  [R1B Linux/WASM job](https://github.com/bokoboss/street-concept-designer/actions/runs/33456313520/job/99696907431),
  [R1B Windows/MSVC run](https://github.com/bokoboss/street-concept-designer/actions/runs/33456313520),
  [R1B Windows/MSVC job](https://github.com/bokoboss/street-concept-designer/actions/runs/33456313520/job/99696907584),
  [R1A qualification run](https://github.com/bokoboss/street-concept-designer/actions/runs/33456313504),
  [R1A Linux job](https://github.com/bokoboss/street-concept-designer/actions/runs/33456313504/job/99696907495),
  [R1A Windows/MSVC job](https://github.com/bokoboss/street-concept-designer/actions/runs/33456313504/job/99696907506),
  and [workflow-integrity run](https://github.com/bokoboss/street-concept-designer/actions/runs/33456313462)
  ([job](https://github.com/bokoboss/street-concept-designer/actions/runs/33456313462/job/99696907457)).

## Remediation gate disposition

| Gate | Result | Evidence |
| --- | --- | --- |
| B-G0 | PASS | Accepted R1A base, local workflow validation, and hosted R1A checks |
| B-G1 | PASS | Inert candidate detection regressions |
| B-G2 | PASS | Explicit junction creation regressions |
| B-G3 | PASS | T, four-leg, skewed, unequal-width, and deterministic geometry tests |
| B-G4 | PASS | Independent corner tests plus one/multiple authored-radius persistence tests |
| B-G5 | PASS | Stable lane endpoints, automatic proposals, and manual connectivity tests |
| B-G6 | PASS | Compatible/incompatible regeneration and stale derived-state tests |
| B-G7 | PASS | Adversarial finite/surface/orphan-connection coverage |
| B-G8 | PASS | 64-offset property/fuzz-style corpus and repeatability checks |
| B-G9 | PASS | Post-remediation release benchmark; no architectural blocker |
| B-G10 | PASS | Scope audit confirms no UI, map, standards, simulation, roundabout, or R1C work |

## Limitations and explicit non-goals

This phase intentionally does not add commands/transactions, persistence,
renderer or UI behavior, map/satellite integration, roadway standards,
signalization, channelization, swept paths, simulation, roundabouts, or U-turn
engines.  Pairwise alignment candidates and convex concept-stage surfaces are
the bounded R1B model.  A future phase may replace the surface derivation only
with a separately justified semantic/geometry design; R1B does not conceal
that limitation behind renderer output or tolerance inflation.
