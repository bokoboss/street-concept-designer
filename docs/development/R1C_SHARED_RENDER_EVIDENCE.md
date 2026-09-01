# R1C Shared 2D / 3D Derivation Evidence

## Scope and disposition

R1C proves a renderer-neutral boundary from the accepted R1A/R1B semantic
kernel to diagnostic 2D data and diagnostic 3D buffers.  This document records
the bounded remediation of the three independent-review findings on PR #16:
semantic width-breakpoint fidelity, globally unambiguous lane-connection
identity, and rejection/removal of degenerate connection-guide geometry.

The implementation uses one owned `DerivedEngineeringSnapshot`.  Road and
component engineering geometry is derived once in f64 project coordinates;
the 2D and 3D adapters consume those same records.  The adapters perform only
local-origin conversion, primitive construction, float32 conversion, and
deterministic fan triangulation.  The remediation does not add a production
renderer, desktop shell, browser runtime, map layer, asset system,
serialization format, or editor interaction model.

The qualified remediation gates pass.  The recommendation is
`PROCEED_BEYOND_R1`, with geometric lane-movement paths explicitly remaining a
later, deliberately specified feature rather than an invented R1C diagnostic.

## Reproducibility

- Exact accepted R1C execution base: `f73db356537636af2b378ddddd61b3dbfa85018a`.
- Previous independently reviewed PR #16 HEAD: `c4a4ab3431060590268afa40d70532e903d0f487`.
- Qualified remediation implementation HEAD: `7f73deb4d59295dfd1394ca163944cf5fd20a58`.
- Execution branch: `codex/r1c-shared-render-proof`.
- Pull request: [#16 — R1C: prove shared 2D and 3D derivation boundary](https://github.com/bokoboss/street-concept-designer/pull/16)
  (open; not merged).
- Issue: #15, R1C shared 2D/3D derivation and renderer boundary proof.
- Engineering Development Workflow: v1.5.0.  The local and hosted integrity
  checks passed; local validation reported `VALIDATION PASS (8 managed files,
  2 project-owned files)`.
- Rust toolchain: `1.98.0-x86_64-pc-windows-msvc`, `rustc 1.98.0
  (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.
- Local native execution used the installed
  `stable-x86_64-pc-windows-gnu` toolchain because this host does not expose
  `link.exe`.  Pinned MSVC format, lint, and check paths passed locally;
  native MSVC tests and release linking are hosted-workflow responsibilities.
- Local GNU native release and `wasm32-unknown-unknown` release builds passed.

The evidence-only documentation follow-up is intentionally separate from the
qualified implementation HEAD above.  The hosted run set below is attached to
the code/test commit that implements the remediation.

## Dependency and license decision

No third-party Rust, JavaScript, geometry, triangulation, renderer, or test
dependencies were added.  `Cargo.lock` remains dependency-free.  R1C uses
only the Rust standard library and the accepted kernel modules, so there is no
new package license or attribution decision.  PixiJS and Three.js remain
candidate later production decisions and are not adopted by this proof.

## Independent-review remediation record

### F-01 — alignment-only sampling skipped authored width knots

The reviewed implementation derived each road as:

```text
Alignment::sample(...)
→ CrossSection::states_at only at alignment stations
```

The accepted alignment sampler is driven by chord error and maximum segment
length, while each `PiecewiseLinearWidthProfile` has independent semantic
`WidthKnot.station_m` breakpoints.  On the default 200 m straight-road grid,
an authored lifecycle knot could fall between alignment stations (for example,
between the 18.75 m and 25 m samples), causing a widening or narrowing state
to be evaluated away from its authored breakpoint.  This could shift add/drop,
taper, storage, and slope-change behavior even when 2D and 3D agreed with one
another.

`derive_road` now builds one deterministic station grid per road from the
sorted/deduplicated union of:

```text
accepted Alignment::sample stations
+ every WidthKnot.station_m from every cross-section component
```

Near-equal candidates are merged only with the centralized
`TolerancePolicy::station_bound_m`.  When an alignment sample and authored
knot collide within that bound, the authored station is retained.  Authored
knot values are copied, never snapped to the alignment grid; canonical
profiles are not changed.  At every merged station the implementation
evaluates alignment point, tangent, normal, the complete ordered
`CrossSection::states_at`, and all shared component boundaries.  Each emitted
`DerivedComponentStrip` also records its exact shared start/end stations.

The mandatory lifecycle fixture is a straight 200 m road with:

```text
0.0   → 0.0
23.4  → 0.0
41.7  → 3.25
87.3  → 3.25
103.6 → 0.0
200.0 → 0.0
```

`non_grid_aligned_lifecycle_knots_are_preserved_in_shared_station_grid`
proves that all authored stations occur exactly in the shared samples, widths
equal the authored values, the first positive strip starts exactly at 23.4 m,
the full-width state remains at exactly 87.3 m, the final positive strip ends
exactly at 103.6 m, no strip exists outside that lifecycle interval, both
adapters consume the same strip records, and repeated derivation is equal.

`non_grid_aligned_slope_changes_and_shared_samples_match_canonical_state`
adds the irregular nonzero-slope profile:

```text
0.0   → 3.0
37.3  → 5.0
63.8  → 4.0
120.0 → 4.0
```

It proves every knot is in the shared grid with its exact width while the
alignment contributes additional stations.  The curved-road case compares
the complete accepted circular-arc sample set with the merged grid, so width
knots add stations without removing chord-error stations.

### F-02 — unscoped LaneConnection renderer identity

The reviewed renderer identity was:

```text
SemanticRef::LaneConnection(LaneConnectionId)
```

R1B correctly permits local authored ids and validates duplicates within one
junction.  Therefore two valid junctions could both contain
`manual-connection`, which collided in the renderer-facing identity.

R1B's authored identity model is unchanged.  R1C now scopes the renderer
reference by its owning junction:

```rust
SemanticRef::LaneConnection {
    junction_id: JunctionId,
    connection_id: LaneConnectionId,
}
```

`DerivedLaneConnection` retains this scoped semantic reference in the shared
snapshot, but deliberately has no geometry.  The regression
`lane_connection_refs_are_junction_scoped_when_geometry_is_deferred` creates
two separate valid junctions, installs the same validated local id
`manual-connection` in each, proves the two references differ, proves each
junction retains only its own reference, proves deterministic rebuild, and
proves the 2D/3D emitted reference sets remain equal.  Since connection-path
geometry is deferred under F-03, neither adapter reports a geometric match for
these refs; the test also proves one junction's ref cannot match the other's.

### F-03 — zero-length lane-connection guide geometry

The reviewed guide was constructed as:

```text
[from.center(), junction.candidate().point(), to.center()]
```

Accepted R1B defines each approach center from the junction candidate point,
so the guide was effectively `[P, P, P]`.  Existing line validation checked
only count and finiteness, allowing a zero-length selectable primitive.

The selected remediation is the preferred bounded option: R1C removes
lane-connection guide geometry entirely.  R1B remains the owner of semantic
connectivity.  The snapshot retains explicit junction-scoped connection
references, while `PrimitiveRole::LaneConnectionGuide`, connection point
storage, and both 2D/3D connection-guide emission are removed.  A vehicle path,
turning-radius trajectory, or arbitrary diagnostic movement line is not
invented in R1C.  Geometric movement-path derivation and connection selection
remain deferred until a later specification provides a meaningful design.

Line validation is strengthened regardless of that choice:

- f64 2D polylines require at least one adjacent pair farther apart than the
  declared coordinate-coincidence tolerance;
- float32 3D `LineStrip` validation checks the actual indexed strip segments
  and requires at least one positive local XY segment above the explicitly
  named `FLOAT32_LOCAL_COORDINATE_TOLERANCE_M` of 1 mm;
- finite values, index ranges, and existing polygon/triangle checks remain in
  force.

The R1C junction/adversarial assertions inspect every emitted 2D polyline and
3D line strip, confirm road alignments and R1B corner curves remain valid, and
confirm no fake LaneConnection primitive is emitted.

## Shared snapshot and semantic references

`src/render.rs` provides the following owned, renderer-neutral contract:

- `DerivedEngineeringSnapshot`: schema metadata, explicit render origin,
  project-space extents, derived roads, and current non-stale junctions.
- `DerivedRoad`: merged shared-station `SamplePoint` alignment records and
  ordered `DerivedComponent` records.
- `DerivedComponent`: scoped `SemanticRef`, component kind, every evaluated
  `DerivedComponentSample` including zero-width lifecycle states, and finite
  `DerivedComponentStrip` polygons for renderable station intervals.
- `DerivedJunction`: accepted R1B pavement surface, copied corner arcs, and
  junction-scoped semantic connection references without fabricated paths.
- `DerivedEngineeringSnapshot` records own their `Vec` data and hold no references to
  mutable network internals or renderer objects.

The shared selection identity is:

```text
Road(RoadId)
RoadComponent { road_id: RoadId, component_id: ComponentId }
Junction(JunctionId)
Corner(CornerId)
LaneConnection { junction_id: JunctionId, connection_id: LaneConnectionId }
```

The road id scopes a component id and the junction id scopes a connection id.
Renderer numeric handles and coordinate nearest-neighbour matching are not
canonical identity.  The snapshot contract version is `2` because the
renderer-facing connection identity and connection DTO geometry contract were
changed by this remediation.

## Shared road/component derivation

`DerivedEngineeringSnapshot::derive` obtains accepted alignment sampling
stations, merges all component width-knot stations, and evaluates each merged
station exactly once for the shared road/component records.  The complete
ordered cross-section is centred on the sampled alignment and each component's
left/right boundary is computed in f64 project space from the shared sample
normal.  No adapter evaluates a width profile or reconstructs a component
offset.

Adjacent shared boundary samples form one strip interval.  Intervals with zero
width at both ends are omitted.  A zero-to-full or full-to-zero interval is
retained as a valid triangle after consecutive duplicate boundary points are
removed; no microscopic filler triangles are created.  Component samples
remain present even when their renderable strip list is empty, so lifecycle
state and renderable geometry are not conflated.

The direct invariant
`every_shared_component_sample_matches_canonical_alignment_and_profile`
re-evaluates the source alignment and complete cross-section at every shared
sample.  It proves station and width equality, exact source alignment-point
agreement, and policy-bounded shared boundary agreement.

## Junction derivation

The snapshot reads current accepted R1B `Junction` data.  It validates and
copies the existing `PavementSurface` and `Corner::arc_points`, and retains
the accepted lane-connection semantic records with junction scope.  It does
not derive movement-path geometry.

When R1B invalidates a junction, its surface/corners/active connections are
cleared and the stale junction is omitted from the disposable geometry
snapshot.  This prevents an old surface from reaching either adapter.

## Diagnostic adapters

`derive_diagnostic_2d` produces owned `Diagnostic2D` data containing:

- `Polyline2D` road alignment references;
- `Polygon2D` shared component strips;
- `Polygon2D` accepted junction surfaces;
- `Polyline2D` accepted corner curves.

It emits no lane-connection guide primitive.  All 2D coordinates are f64
render-local values, with the snapshot origin and line tolerance carried on
the DTO.  Each emitted primitive carries a `SemanticRef` and a
`PrimitiveRole`.

`derive_diagnostic_3d` produces owned `Diagnostic3D` data containing
`MeshPrimitive3D` records with semantic references, roles, topology, local
`[f32; 3]` positions, and u32 indices.  Road/component/junction surfaces are
flat z=0 diagnostic geometry.  Alignment and corner diagnostics use line-strip
buffers.  It emits no lane-connection guide mesh.

The 3D adapter never queries `Alignment`, `CrossSection`, `Road`, `Junction`,
or lane profiles.  It consumes only the snapshot's already-derived points.

## Triangulation and validity

Bounded R1C polygons are triangulated with a deterministic fan
`[0, i, i + 1]`.  R1B junction surfaces are validated convex concept-stage
surfaces; component strips are bounded station intervals.  `MeshPrimitive3D::validate`
checks finite positions, index ranges, positive triangle area, consistent
positive XY winding, and positive indexed LineStrip length.  2D line validation
uses the centralized coordinate-coincidence tolerance.  No triangulation
dependency was needed.

Project-space polygon area is accumulated relative to the first vertex to avoid
large-coordinate cancellation.  This is a numerical validation detail, not a
change to canonical coordinates.

## Local origin and float32 evidence

Canonical snapshot records remain f64 metres in project coordinates.  An
explicit `SnapshotMetadata::render_origin` is carried with the snapshot, and
both adapters apply:

```text
p_local = p_project - origin
```

in f64 before the 3D adapter converts positions to f32.  `with_render_origin`
copies the disposable snapshot and changes only metadata; project geometry,
extents, and semantic references remain unchanged.

The large-coordinate fixture uses project coordinates around
`1_000_000_000 m` with 0.375 m and 0.625 m offsets.  It demonstrates that the
incorrect `f32(project) - f32(origin)` sequence loses the sub-metre offset,
while f64 subtraction followed by f32 conversion preserves it within the
named diagnostic bound `FLOAT32_LOCAL_COORDINATE_TOLERANCE_M` (1 mm).

## Selection synchronization and lifecycle

Both diagnostic DTOs expose semantic-ref lookup.  R1C claims geometric
selection parity for road, road component, junction, and corner references.
The shared component trace compares each 2D polygon and 3D mesh vertex
sequence with the exact upstream `DerivedComponentStrip` record; it does not
independently regenerate similar shapes.

Two roads both use the local component id `lane-1`.  Their references are
distinct `RoadComponent { road_id: road-a, component_id: lane-1 }` and
`RoadComponent { road_id: road-b, component_id: lane-1 }`, and both remain
selectable independently in 2D and 3D.  LaneConnection refs are retained in
the shared snapshot with junction scope, but geometric movement-path
selection is explicitly not claimed in R1C because F-03 removes its
fabricated guide.

The test derives snapshot, 2D, and 3D objects inside a scope, drops them, and
performs a clean derivation from the same semantic network.  Owned values are
equal on rebuild and no global cache is required.  A road width replacement
through `RoadNetwork::replace_road` changes both diagnostic representations
while preserving semantic refs and without reusing stale render objects.
R1B's explicit invalidation/regeneration path remains the owner of junction
regeneration; R1C adds no command or undo system.

## Canonical fixtures and coverage

The R1C integration suite is `tests/r1c_shared_render.rs` and reuses accepted
R1A fixture helpers where applicable.

| Requirement | Test evidence |
| --- | --- |
| R-01 straight road | `straight_and_curved_roads_trace_one_shared_component_derivation` |
| R-02 curved road and union preservation | same test, complete accepted arc station subset |
| R-03 variable-width lane | `variable_width_add_drop_and_right_turn_pocket_use_general_component_lifecycle` |
| R-04 lane add/drop | same test plus `non_grid_aligned_lifecycle_knots_are_preserved_in_shared_station_grid` |
| R-05 right-turn pocket | same test, general `TrafficLane` profile |
| F-01 lifecycle breakpoints | `non_grid_aligned_lifecycle_knots_are_preserved_in_shared_station_grid` |
| F-01 irregular slope changes | `non_grid_aligned_slope_changes_and_shared_samples_match_canonical_state` |
| Shared DTO source invariant | `every_shared_component_sample_matches_canonical_alignment_and_profile` |
| J-01 T junction | `t_four_leg_and_skewed_junctions_trace_accepted_r1b_geometry` |
| J-02 four-leg junction | same test |
| J-03 skewed junction | same test |
| Scoped duplicate connection id | `lane_connection_refs_are_junction_scoped_when_geometry_is_deferred` |
| Semantic id parity | `selection_refs_are_scoped_and_parity_is_shared_between_2d_and_3d` |
| Duplicate local component id | same test, `road-a`/`road-b` `lane-1` |
| Junction/corner selection parity | `t_four_leg_and_skewed_junctions_trace_accepted_r1b_geometry` |
| Large origin and f64→local→f32 | `large_project_coordinates_use_f64_origin_then_float32_local_buffers` |
| Origin-only change | `changing_origin_only_changes_local_adapters_and_keeps_shared_ids` |
| Cache discard/full rebuild | `disposable_rebuild_is_deterministic_and_semantic_update_has_no_stale_geometry` |
| Semantic update | same test, 3.5 m→4.5 m road width |
| Finite/non-degenerate buffers | every R1C test calls snapshot/DTO validation where applicable |
| Repeated deterministic derivation | `bounded_width_and_origin_matrix_remains_finite_and_deterministic` |

The R1C suite contains 12 integration tests.  The full local all-targets run
contains 56 integration tests: 24 accepted R1A tests, 20 accepted R1B tests,
and 12 R1C tests.  Existing R1A/R1B adversarial, property-style, and
fuzz-style tests remain in the regression total.

## Verification commands and local results

Commands used:

```text
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features -- --nocapture
cargo bench --locked --bench r1c_shared_render -- --nocapture
cargo build --locked --target wasm32-unknown-unknown --release
python <workflow-checkout>/scripts/setup_project.py validate .
```

On the local Windows host, the pinned MSVC format, strict Clippy, and
all-target check passed.  The GNU all-targets run passed all 56 integration
tests and all deterministic benchmark targets.  The GNU native release build,
WASM release build, and R1C release benchmark passed.  Local MSVC test and
release-link execution remains hosted because `link.exe` is unavailable here.

Hosted verification for qualified remediation implementation HEAD
`7f73deb4d59295dfd1394ca163944cf5fd20a58`:

- [R1C workflow run 33473220784](https://github.com/bokoboss/street-concept-designer/actions/runs/33473220784)
  passed [Windows/MSVC job 99747039277](https://github.com/bokoboss/street-concept-designer/actions/runs/33473220784/job/99747039277)
  and [Linux/native+WASM job 99747039468](https://github.com/bokoboss/street-concept-designer/actions/runs/33473220784/job/99747039468).
- [R1A regression workflow run 33473220767](https://github.com/bokoboss/street-concept-designer/actions/runs/33473220767)
  passed [Windows/MSVC job 99747038931](https://github.com/bokoboss/street-concept-designer/actions/runs/33473220767/job/99747038931)
  and [Linux job 99747039109](https://github.com/bokoboss/street-concept-designer/actions/runs/33473220767/job/99747039109).
- [R1B regression workflow run 33473220769](https://github.com/bokoboss/street-concept-designer/actions/runs/33473220769)
  passed [Linux job 99747038994](https://github.com/bokoboss/street-concept-designer/actions/runs/33473220769/job/99747038994)
  and [Windows/MSVC job 99747039198](https://github.com/bokoboss/street-concept-designer/actions/runs/33473220769/job/99747039198).
- [Engineering Workflow Integrity run 33473220814](https://github.com/bokoboss/street-concept-designer/actions/runs/33473220814)
  passed [validation job 99747039184](https://github.com/bokoboss/street-concept-designer/actions/runs/33473220814/job/99747039184).

## Exploratory benchmark

Benchmark target: `benches/r1c_shared_render.rs`; fixture: three roads plus
one accepted junction, with two lanes on the main road.  Iterations: 1,000 per
operation.  Local observation on Windows x86_64, GNU target, Rust 1.98.0:

| Operation | Reviewed baseline c4a4ab3 | Remediation 7f73deb |
| --- | ---: | ---: |
| Shared snapshot | 164.22 us/op | 185.09 us/op |
| 2D DTO from shared snapshot | 122.90 us/op | 100.39 us/op |
| 3D buffers from shared snapshot | 167.91 us/op | 294.75 us/op |
| Complete clean rebuild | 385.64 us/op | 437.62 us/op |
| Semantic width update + rebuild | 588.45 us/op | 662.13 us/op |

These are architectural observations, not product SLAs.  The remediation
correctly adds semantic station evaluations and removes connection-guide
geometry; no optimization was performed merely to recover the previous
microbenchmark numbers.  The update fixture uses the accepted R1B junction
regeneration path before deriving the new snapshot.

## Qualification gates

| Gate | Result | Evidence |
| --- | --- | --- |
| C-G0 accepted R1A/R1B base and workflow | PASS | accepted base, local validation, and hosted runs above |
| C-G1 one snapshot feeds 2D and 3D | PASS | shared strip trace and adapter signatures |
| C-G2 globally stable semantic ids | PASS | road/component tests plus junction-scoped LaneConnection regression |
| C-G3 no duplicate engineering algorithms | PASS | only shared road derivation evaluates widths/offsets; adapters consume DTOs |
| C-G4 large coordinates/local origin | PASS | 1e9-coordinate and origin-change tests |
| C-G5 deterministic clean rebuild | PASS | lifecycle and discard/rebuild equality tests |
| C-G6 semantic selection | PASS | road/component/junction/corner selection is proven; connection geometric selection is explicitly not claimed |
| C-G7 finite/non-degenerate geometry | PASS | exact lifecycle strips, source-state invariant, strengthened 2D/3D line validation, and hosted tests |
| C-G8 benchmark observations | PASS | refreshed 1,000-iteration benchmark above |
| C-G9 production recommendation | PASS | bounded recommendation below |
| C-G10 no scope creep | PASS | scope audit below |

## Production-boundary recommendation

Evidence-backed recommendations from R1C remediation:

- Keep accepted Rust as the engineering kernel.  The corrected shared station
  grid preserves semantic lifecycle breakpoints and alignment sampling while
  one owned f64 snapshot feeds both representations.
- Keep the Rust/shared DTO boundary semantic and geometry-rich enough to carry
  project-space f64 records, extents, render-origin metadata, component
  ownership, junction/corner identities, and junction-scoped connection refs.
- Keep movement-path geometry out of R1C until a later specification defines a
  meaningful diagnostic or engineering path.  Do not infer selection geometry
  from zero-length or arbitrary approach-to-candidate lines.
- Let a future application boundary consume a versioned snapshot/command
  contract, but defer serialization until application runtime and persistence
  needs are specified.
- PixiJS remains a reasonable later 2D candidate and Three.js a reasonable
  later 3D candidate because their future adapters can consume diagnostic
  geometry and shared semantic ids; neither is adopted here.
- R1C proves the safe Rust-side WASM compilation path, not browser/JS binding
  ergonomics, serialization costs, worker transfer design, or production
  memory/performance.  Those remain unproven.
- Project-to-render-local conversion belongs at the derived render boundary,
  after canonical f64 derivation and before any f32/GPU conversion.  It is
  view/render state, not a semantic edit.
- `SemanticRef` should remain the shared identity carried between 2D, 3D,
  hierarchy, properties, and later command/selection state.  Renderer handles
  should remain disposable indexes.

Still-unproven production decisions include frontend framework integration,
serialization format, JS bindings, browser worker architecture, GPU upload
policy, view-dependent tessellation, hit-test acceleration, and incremental
regeneration equivalence at production scene sizes.

## Limitations and scope audit

R1C intentionally keeps the R1B convex concept-stage junction surface and
sampled corner curves.  It does not claim standards-grade curb returns,
swept-path validation, vertical alignment, terrain, markings, assets, maps,
simulation, export, persistence, commands, undo, React, Tauri, PixiJS,
Three.js, MapLibre, WebView2, browser runtime behavior, or R2 editor work.
The 3D proof is a flat z=0 diagnostic buffer proof.  Line diagnostics use
line-strip buffers; future renderers may choose richer line/material
representations without changing semantic geometry.

No material R1A/R1B redesign was required.  No duplicate renderer-side
engineering calculation, renderer framework, coordinate quantization, or
third-party dependency was introduced.  PR #16 remains open and was not
merged.  No R2 branch or application-shell work was started.
