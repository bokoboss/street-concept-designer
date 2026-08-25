# Geometry Precision & Tolerance Policy

## Purpose

Geometry correctness must not depend on scattered magic epsilons. This policy separates authored engineering precision, robust computational tolerances, snapping/user interaction, and renderer precision.

## Canonical numeric model

- engineering coordinates and dimensions use `f64`-class precision in metres in the canonical/kernel model;
- project/georeferenced coordinates may be large, so renderer-local origin conversion occurs only at renderer boundaries;
- GPU float precision never determines engineering geometry;
- no global coordinate quantization is allowed as an unreviewed shortcut.

## Four different concepts must remain separate

### 1. Authored engineering value
A deliberate user/profile value such as lane width 3.25 m or corner radius 12 m. Preserve it as entered/declared within normal numeric serialization precision.

### 2. Computational robustness tolerance
Used to classify near-coincident geometry, convergence, parameter bounds, intersection predicates, or generated topology. Must be named, centralized, documented, and tested.

### 3. Interaction/snapping tolerance
A view-dependent selection aid such as snapping the cursor to an endpoint. It may be expressed partly in screen pixels but resolves to an explicit semantic target. It must never silently become the kernel's geometric equality rule.

### 4. Display/render tolerance
Sampling, curve tessellation, mesh subdivision, anti-aliasing, and LOD choices. These may vary by zoom/performance and are derived only.

## Named tolerance families

The kernel should expose policy/configuration rather than scattered literals. Candidate families include:
- coordinate coincidence tolerance;
- station/domain bound tolerance;
- angular/tangent tolerance;
- minimum accepted segment/edge length;
- polygon validity/cleanup tolerance;
- projection convergence tolerance;
- adaptive curve sampling error;
- derived triangulation/area consistency tolerance.

Actual numeric defaults are **not locked in R0**. R1 must establish evidence from canonical/adversarial fixtures and benchmarks.

## Candidate precision scale for investigation

A millimetre-order derived-geometry tolerance may be a reasonable starting experiment for road-concept geometry, but it is not an accepted universal constant. R1 should test values against:
- normal road dimensions;
- 1 mm gap/overlap adversarial cases;
- near tangent intersections;
- very short segments;
- acute/skewed geometry;
- large georeferenced-style coordinate magnitudes.

The result must be documented as a tolerance policy/ADR, not left as code folklore.

## Robust predicates before tolerance inflation

Prefer numerically robust algorithms/predicates and well-defined topology over repeatedly increasing epsilon until tests pass.

A tolerance may classify numerical ambiguity; it must not hide:
- self-intersections;
- disconnected lane topology;
- invalid negative/zero-width lifecycle semantics;
- coincident but semantically distinct roads;
- wrong junction connectivity.

## Curve sampling

Alignment semantics remain analytic/parametric where defined. Sampling is derived.

Adaptive sampling should be based on declared error/curvature criteria rather than arbitrary fixed screen spacing. Separate:
- engineering diagnostic sampling;
- 2D display tessellation;
- 3D mesh tessellation.

Equivalent semantic geometry may have different render tessellation while remaining the same project design.

## Polygon Boolean / cleanup

If a polygon/overlay library requires fixed precision or quantization:
- quantize only derived working geometry, not canonical alignment/component data;
- choose scale explicitly;
- record the transform/precision;
- test area/boundary deviation against source geometry;
- detect collapse of narrow features;
- retain the ability to regenerate from unquantized canonical state.

## Snapping policy

Snapping candidates should be ranked semantically, for example:
- endpoint/node;
- explicit junction/control point;
- intersection;
- midpoint;
- perpendicular/tangent candidate;
- nearest alignment/component edge;
- station/grid/guide.

Screen-space acquisition distance is UX state. Once accepted, the result is an explicit semantic reference/coordinate in engineering space.

Do not infer topology simply because two rendered points visually snap together; topology creation remains an explicit operation.

## Large coordinate policy

Canonical/project coordinates may retain CRS-scale values. Renderers subtract a stable local origin before converting to GPU-friendly values.

Tests should prove:
- point/tangent/projection remain finite at large coordinate magnitude;
- local-origin conversion preserves local distances/shape within renderer acceptance;
- changing view/render origin does not modify canonical geometry;
- exported engineering coordinates use the declared project context, not renderer-local values.

## Degenerate input behavior

Explicitly reject/report rather than auto-fix silently when semantic intent is unclear, including:
- zero-length alignment primitives;
- non-finite inputs;
- negative widths;
- invalid/unsorted station ranges;
- impossible curve parameters;
- topology references to missing lanes/roads.

Automatic cleanup is allowed only when behavior is deterministic, disclosed, and tested.

## Invariant examples

- all accepted coordinates finite;
- primitive lengths positive above declared minimum;
- alignment station function monotonic;
- width >= 0 throughout profile;
- derived boundaries preserve ordering except explicitly allowed merge/zero-width transitions;
- polygons accepted as surfaces have valid winding/holes and no unresolved self-intersection;
- triangulated area is consistent with source surface within declared tolerance;
- no NaN/Inf/degenerate triangles reach accepted renderer snapshot;
- lane connection references are valid and not orphaned.

## Test strategy

Use a combination of:
- exact/simple analytical cases where expected values are known;
- golden numeric fixtures;
- property-based tests;
- adversarial near-degenerate fixtures;
- fuzzing of parsers/geometry operations where applicable;
- cross-implementation comparison if evaluating Rust vs TypeScript;
- benchmark/regression tracking.

Tests must state the tolerance they rely on rather than hiding permissive assertions in helpers.

## Change control

Changing a kernel tolerance can change geometry behavior globally and must be treated as an architecture/engineering-sensitive change:
- explain failing cases motivating the change;
- show new and old outcomes;
- run canonical/adversarial regression matrix;
- scrutinize whether algorithmic correction is preferable;
- document any project-file compatibility implications.

## Architecture invariant

**Tolerance is an explicit, tested numerical policy—not a substitute for correct semantic geometry.**
