# R1C Shared 2D / 3D Derivation Evidence

## Scope and disposition

R1C proves a renderer-neutral boundary from the accepted R1A/R1B semantic
kernel to diagnostic 2D data and diagnostic 3D buffers.  It does not add a
production renderer, desktop shell, browser runtime, map layer, asset system,
serialization format, or editor interaction model.

The implementation uses one owned `DerivedEngineeringSnapshot`.  Road and
component engineering geometry is derived once in f64 project coordinates;
the 2D and 3D adapters consume those same records.  The adapters perform only
local-origin conversion, primitive construction, float32 conversion, and
deterministic fan triangulation.

The hosted qualification matrix completed successfully.  The recommendation
is `PROCEED_BEYOND_R1`.

## Reproducibility

- Exact accepted R1C execution base: `f73db356537636af2b378ddddd61b3dbfa85018a`.
- Execution branch: `codex/r1c-shared-render-proof`.
- Qualified implementation/evidence baseline HEAD: `d8adb81301a9ba9d7fc966b46963ac977376b0ce`.
- Pull request: [#16 — R1C: prove shared 2D and 3D derivation boundary](https://github.com/bokoboss/street-concept-designer/pull/16)
  (open; not merged).
- Issue: #15, R1C shared 2D/3D derivation and renderer boundary proof.
- Engineering Development Workflow: v1.5.0; local
  `setup_project.py validate .` result: `VALIDATION PASS (8 managed files,
  2 project-owned files)`.
- Rust toolchain: `1.98.0-x86_64-pc-windows-msvc`, `rustc 1.98.0
  (88d9e12ae 2026-08-18)`, `cargo 1.98.0 (797e8a9bc 2026-08-05)`.
- Local native execution used the installed
  `stable-x86_64-pc-windows-gnu` toolchain because this host does not expose
  `link.exe`.  The pinned MSVC `check`, format, and lint paths are locally
  compilable; native MSVC tests and release linking are hosted-workflow
  responsibilities.
- `wasm32-unknown-unknown` release build completed locally after installing
  the target for the installed GNU toolchain.

## Dependency and license decision

No third-party Rust, JavaScript, geometry, triangulation, renderer, or test
dependencies were added.  `Cargo.lock` remains dependency-free.  R1C uses
only the Rust standard library and the accepted kernel modules, so there is
no new package license or attribution decision.  The eventual PixiJS and
Three.js choices remain candidate production decisions and are not adopted by
this proof.

## Shared snapshot and semantic references

`src/render.rs` adds the following owned, renderer-neutral contract:

- `DerivedEngineeringSnapshot`: schema metadata, explicit render origin,
  project-space extents, derived roads, and current non-stale junctions.
- `DerivedRoad`: accepted sampled `SamplePoint` alignment records and ordered
  `DerivedComponent` records.
- `DerivedComponent`: scoped `SemanticRef`, component kind, every evaluated
  `DerivedComponentSample` (including zero-width states), and finite
  `DerivedComponentStrip` polygons for renderable station intervals.
- `DerivedJunction`: accepted R1B pavement surface, copied corner arcs, and
  optional active lane-connection guide records.
- `DerivedSnapshot` records contain owned `Vec` data and no references to
  mutable network internals or renderer objects.

`SemanticRef` is the shared selection identity:

```text
Road(RoadId)
RoadComponent { road_id: RoadId, component_id: ComponentId }
Junction(JunctionId)
Corner(CornerId)
LaneConnection(LaneConnectionId)
```

The road id scopes a component id.  Renderer numeric handles and coordinate
nearest-neighbour matching are not canonical identity.

## Shared road/component derivation

`DerivedEngineeringSnapshot::derive` samples each accepted `Alignment` once
using `SamplingOptions::from_policy`.  At every shared station it evaluates
the accepted `CrossSection::states_at` once, sums the ordered component widths,
centres the complete cross-section on the sampled alignment, and computes each
component's left/right boundary in f64 project space from the shared sample
normal.

Adjacent shared boundary samples form one strip interval.  Intervals with
zero width at both ends are omitted.  A zero-to-full or full-to-zero interval
is retained as a valid triangle after consecutive duplicate boundary points
are removed; no microscopic filler triangles are created.  Component samples
remain present even when their renderable strip list is empty, so lifecycle
state and renderable geometry are not conflated.

The accepted general `TrafficLane` component is used for variable lanes,
lane add/drop, and the right-turn pocket.  There is no renderer-only pocket
type and no adapter-side width-profile evaluation.

## Junction derivation

The snapshot reads current accepted R1B `Junction` data.  It validates and
copies the existing `PavementSurface`, `Corner::arc_points`, and active
`LaneConnection` endpoints.  A guide is only a diagnostic polyline through
the existing approach centres and candidate point; it does not infer or
rebuild connectivity.

When R1B invalidates a junction, its surface/corners/active connections are
cleared and the stale junction is omitted from the disposable geometry
snapshot.  This prevents an old surface from reaching either adapter.

## Diagnostic adapters

`derive_diagnostic_2d` produces owned `Diagnostic2D` data containing:

- `Polyline2D` road alignment references;
- `Polygon2D` component strips;
- `Polygon2D` accepted junction surfaces;
- `Polyline2D` corner curves and optional lane-connection guides.

All 2D coordinates are f64 render-local values, with the snapshot origin
carried on `Diagnostic2D`.  Each primitive carries a `SemanticRef` and a
`PrimitiveRole`.

`derive_diagnostic_3d` produces owned `Diagnostic3D` data containing
`MeshPrimitive3D` records with semantic references, roles, topology, local
`[f32; 3]` positions, and u32 indices.  Road/component/junction surfaces are
flat z=0 diagnostic geometry.  Alignment, corner, and guide diagnostics use
line-strip buffers.

The 3D adapter never queries `Alignment`, `CrossSection`, `Road`, `Junction`,
or lane profiles.  It consumes only the snapshot's already-derived points.

## Triangulation and validity

The bounded R1C polygons are triangulated with a deterministic fan
`[0, i, i + 1]`.  R1B junction surfaces are validated convex concept-stage
surfaces; component strips are bounded station intervals.  `MeshPrimitive3D::validate`
checks finite positions, index ranges, positive triangle area, and consistent
positive XY winding.  Line strips require at least two positions.  No
triangulation dependency was needed.

Project-space polygon area is accumulated relative to the first vertex to
avoid large-coordinate cancellation.  This is a numerical validation detail,
not a change to canonical coordinates.

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
`1_000_000_000 m` with 0.375 m and 0.625 m offsets.  The test demonstrates
that the incorrect `f32(project) - f32(origin)` sequence loses the sub-metre
offset, while f64 subtraction followed by f32 conversion preserves it within
the named diagnostic bound `FLOAT32_LOCAL_COORDINATE_TOLERANCE_M` (1 mm).

## Selection synchronization

Both diagnostic DTOs expose semantic-ref lookup.  The R1C tests select road,
road component, junction, and corner references and assert matches in both
outputs.  The shared component trace test compares each 2D polygon and 3D
mesh vertex sequence with the exact upstream `DerivedComponentStrip` record;
the test does not independently regenerate similar shapes.

Two roads both use the local component id `lane-1`.  Their references are
distinct `RoadComponent { road_id: road-a, component_id: lane-1 }` and
`RoadComponent { road_id: road-b, component_id: lane-1 }`, and both remain
selectable independently in 2D and 3D.

## Disposable rebuild and semantic update

The test derives snapshot, 2D, and 3D objects inside a scope, drops them, and
performs a clean derivation from the same semantic network.  Owned values are
equal on rebuild and no global cache is required.

A road width is replaced through `RoadNetwork::replace_road` with the same
road/component identities.  A clean snapshot, 2D DTO, and 3D buffers are then
derived.  Matching semantic references remain stable, the component geometry
changes in both representations, and no old render object is reused.  R1B's
explicit invalidation/regeneration path remains the owner of junction
regeneration; R1C adds no command or undo system.

## Canonical fixtures and coverage

The R1C integration suite is `tests/r1c_shared_render.rs` and reuses the
accepted R1A fixture helpers where applicable.

| Requirement | Test evidence |
| --- | --- |
| R-01 straight road | `straight_and_curved_roads_trace_one_shared_component_derivation` |
| R-02 curved road | same test, accepted circular arc |
| R-03 variable-width lane | `variable_width_add_drop_and_right_turn_pocket_use_general_component_lifecycle` |
| R-04 lane add/drop | same test, zero/full lifecycle samples and valid strips |
| R-05 right-turn pocket | same test, general `TrafficLane` profile |
| J-01 T junction | `t_four_leg_and_skewed_junctions_trace_accepted_r1b_geometry` |
| J-02 four-leg junction | same test |
| J-03 skewed junction | same test |
| Semantic id parity | `selection_refs_are_scoped_and_parity_is_shared_between_2d_and_3d` |
| Duplicate local component id | same test, `road-a`/`road-b` `lane-1` |
| Junction/corner selection parity | `t_four_leg_and_skewed_junctions_trace_accepted_r1b_geometry` |
| Large origin and f64→local→f32 | `large_project_coordinates_use_f64_origin_then_float32_local_buffers` |
| Origin-only change | `changing_origin_only_changes_local_adapters_and_keeps_shared_ids` |
| Cache discard/full rebuild | `disposable_rebuild_is_deterministic_and_semantic_update_has_no_stale_geometry` |
| Semantic update | same test, 3.5 m→4.5 m road width |
| Finite/non-degenerate buffers | every R1C test calls snapshot/DTO validation where applicable |
| Repeated deterministic derivation | `bounded_width_and_origin_matrix_remains_finite_and_deterministic` |

The bounded matrix varies widths and large local origins without a testing
dependency.  Existing R1A/R1B adversarial, property-style, and fuzz-style
tests remain unchanged and continue to run.

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

The local GNU native run passed 52 integration tests: 24 accepted R1A tests,
20 accepted R1B tests, and 8 R1C tests.  Strict Clippy and formatting passed.
The local WASM release build passed.  The local pinned MSVC check/lint path is
available, but test/release linking requires the hosted Windows runner's
`link.exe`.

Hosted verification for qualified implementation/evidence baseline HEAD
`d8adb81301a9ba9d7fc966b46963ac977376b0ce`:

- [R1C workflow run 33470156173](https://github.com/bokoboss/street-concept-designer/actions/runs/33470156173)
  passed both [Linux/native+WASM job 99738098207](https://github.com/bokoboss/street-concept-designer/actions/runs/33470156173/job/99738098207)
  and [Windows/MSVC job 99738098391](https://github.com/bokoboss/street-concept-designer/actions/runs/33470156173/job/99738098391).
- [R1A regression workflow run 33470156276](https://github.com/bokoboss/street-concept-designer/actions/runs/33470156276)
  passed [Linux job 99738098048](https://github.com/bokoboss/street-concept-designer/actions/runs/33470156276/job/99738098048)
  and [Windows/MSVC job 99738098396](https://github.com/bokoboss/street-concept-designer/actions/runs/33470156276/job/99738098396).
- [R1B regression workflow run 33470156108](https://github.com/bokoboss/street-concept-designer/actions/runs/33470156108)
  passed [Linux job 99738097814](https://github.com/bokoboss/street-concept-designer/actions/runs/33470156108/job/99738097814)
  and [Windows/MSVC job 99738098096](https://github.com/bokoboss/street-concept-designer/actions/runs/33470156108/job/99738098096).
- [Engineering Workflow Integrity run 33470156322](https://github.com/bokoboss/street-concept-designer/actions/runs/33470156322)
  passed [validation job 99738098213](https://github.com/bokoboss/street-concept-designer/actions/runs/33470156322/job/99738098213).

## Exploratory benchmark

Benchmark target: `benches/r1c_shared_render.rs`; fixture:
three roads plus one accepted junction, with two lanes on the main road.
Iterations: 1,000 per operation.  Local observation on Windows x86_64,
GNU target, Rust 1.98.0:

```text
shared snapshot:                  164.22 us/op
2D DTO from shared snapshot:      122.90 us/op
3D buffers from shared snapshot:  167.91 us/op
complete clean rebuild:           385.64 us/op
semantic width update + rebuild:  588.45 us/op
```

These are architectural observations, not product SLAs.  The update fixture
uses the accepted R1B junction regeneration path before deriving the new
snapshot.

## Qualification gates

| Gate | Result | Evidence |
| --- | --- | --- |
| C-G0 accepted R1A/R1B base and workflow | PASS | exact base above, local validation, and hosted runs 33470156276, 33470156108, 33470156322 |
| C-G1 one snapshot feeds 2D and 3D | PASS | shared component trace and adapter signatures |
| C-G2 stable semantic ids | PASS | `SemanticRef` parity tests |
| C-G3 no duplicate engineering algorithms | PASS | only `derive_road` evaluates widths/offsets; adapters consume snapshot |
| C-G4 large coordinates/local origin | PASS | 1e9-coordinate and origin-change tests |
| C-G5 deterministic clean rebuild | PASS | discard/rebuild equality test |
| C-G6 semantic selection | PASS | road/component/junction/corner lookup tests |
| C-G7 finite/non-degenerate geometry | PASS locally | snapshot and DTO validation plus all fixture tests |
| C-G8 benchmark observations | PASS | 1,000-iteration benchmark above |
| C-G9 production recommendation | PASS | recommendation below |
| C-G10 no scope creep | PASS | scope audit below |

## Production integration recommendation

Evidence-backed recommendations from R1C:

- Keep accepted Rust as the engineering kernel.  R1C shows that accepted
  alignment, component lifecycle, junction, and topology state can produce
  one owned f64 snapshot for both representations without renderer state
  becoming truth.
- Keep the Rust/shared DTO boundary semantic and geometry-rich enough to carry
  project-space f64 records, extents, render-origin metadata, component
  ownership, junction/corner identities, and optional connection guides.
- Let a future application boundary consume a versioned snapshot/command
  contract, but defer the serialization mechanism until the application
  runtime and persistence needs are specified.
- PixiJS remains a reasonable later 2D candidate because its adapter can
  consume the diagnostic polygons/polylines and shared semantic ids.
- Three.js remains a reasonable later 3D candidate because its adapter can
  consume flat mesh buffers and the same semantic ids.
- R1C proves the safe Rust-side WASM compilation path, not browser/JS binding
  ergonomics, serialization costs, worker transfer design, or production
  memory/performance.  Those remain unproven.
- Project-to-render-local conversion belongs at the derived render boundary,
  after canonical f64 derivation and before any f32/GPU conversion.  It is
  view/render state, not a semantic edit.
- `SemanticRef` should be the shared identity carried between 2D, 3D,
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
Three.js, MapLibre, WebView2, or browser runtime behavior.  The 3D proof is a
flat z=0 diagnostic buffer proof.  Line diagnostics use line-strip buffers;
future renderers may choose richer line/material representations without
changing semantic geometry.

No material R1A/R1B redesign was required.  No duplicate renderer-side
engineering calculation, renderer framework, coordinate quantization, or
third-party dependency was introduced.

The qualified implementation HEAD above passed the complete hosted matrix.
The PR remains open and was not merged.
