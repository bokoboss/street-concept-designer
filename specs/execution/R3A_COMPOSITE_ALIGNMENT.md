# R3A Execution Contract — Composite Alignment Productionization

Status: PLANNED / BLOCKED UNTIL POST-R2 CONTROL-PLANE MERGE AND EXACT BASE RECORD

## Work mode

**STRICT**

Mode rationale:
- changes the accepted engineering alignment public model;
- affects road stationing, sampling, projection, cross-section derivation, topology detection/regeneration, and persistence;
- requires schema migration;
- creates the basis for all future Road Draw/Edit UI.

Mode confidence: HIGH.

## Workspace safety

Target project root:
`D:\R&D\street-concept-designer`

Writable boundary:
target project root only.

External/system writes:
none unless the control plane explicitly authorizes target-repository GitHub push/PR handoff.

No:
- global package install;
- PATH/registry/system config;
- writes to other repositories;
- frontend/Tauri/npm setup.

## Objective

Replace the R1 single-primitive Road-alignment limitation with a production-ready ordered multi-segment reference alignment while preserving one Road identity, one cumulative station domain, deterministic engineering behavior, and backward-compatible project migration.

## Authoritative baseline

Repository:
`bokoboss/street-concept-designer`

Execution base:
**resolve exact accepted `main` SHA only after the post-R2/R3 control-plane PR is merged and record it in the R3A Issue before coding.**

Do not execute this contract from a provisional control-plane branch.

## Research / scrutiny basis

Post-R2 finding:
the current `Alignment` enum owns exactly one Line, CircularArc, or SmoothConceptualCurve.

Rejected workaround:
representing one user road as several Road objects.

Reason:
it breaks Road identity, one station domain, cross-section/lifecycle semantics, history targeting, and later road-relative attachments.

Scrutiny decision:
**GO WITH CONDITIONS**.

## Core semantic design

### Primitive versus production alignment

Retain the accepted primitive families:
- line;
- circular arc;
- smooth conceptual cubic curve.

Production `Alignment` must represent an ordered sequence of segments.

Preferred conceptual shape:

```text
Alignment
  segments[] in authored order
    AlignmentSegment
      stable AlignmentSegmentId
      primitive: Line | CircularArc | SmoothConceptualCurve
  cumulative station index/derived lookup
```

Exact Rust type names may differ if the resulting API is clearer and equally constrained.

### Segment identity

Each segment requires a stable semantic id:
- stable across edits that do not replace that segment identity;
- persisted;
- scoped by Road / Scenario;
- never derived from renderer handles or coordinates.

A migrated schema-v1 single primitive receives a deterministic stable legacy segment id such as `segment-0` or another documented collision-safe equivalent.

Do not add UUID/random dependencies merely for this packet unless a separate dependency decision proves them necessary.

### Continuity

For R3A production acceptance:
- adjacent segment endpoints must coincide within the accepted coordinate tolerance;
- adjacent segment tangents must be continuous within the accepted angular tolerance;
- zero/near-zero segment length is invalid;
- backtracking/reversal or ambiguous station order is invalid.

This deliberately keeps R3A to smooth engineering centerline composition and avoids inventing a separate miter/kink offset algorithm.

If real product evidence later requires intentional tangent discontinuities, add them under a separate explicit geometry design.

### Stationing

- whole Alignment starts at station 0;
- total length = deterministic sum of segment lengths;
- segment local station maps to cumulative road station;
- exact segment-boundary stations are preserved;
- point/tangent/normal/project/sample operate on the whole Alignment;
- boundary dispatch must be deterministic and numerically stable;
- projection tie handling must be deterministic.

### Sampling

Sampling must:
- include every segment boundary exactly;
- retain global monotonic station ordering;
- respect the existing declared chord-error/segment-length budgets within each primitive;
- avoid duplicate or descending station values;
- maintain deterministic point-budget behavior;
- never hide a segment boundary due to adaptive subdivision.

### Projection

Whole-alignment projection must:
- evaluate all relevant segments;
- return global cumulative station;
- choose ties deterministically;
- preserve existing finite-value/tolerance policy;
- include adversarial points near a segment boundary.

## Compatibility requirements

### Cross section

Existing CrossSection station range must equal the whole Alignment station range.

No per-segment cross-section source of truth is introduced.

Existing piecewise width profiles continue to use road-global stationing.

### Road / RoadNetwork

One Road owns one composite Alignment.

RoadId remains unchanged by segment edits.

RoadNetwork replace/regeneration behavior remains the canonical path for affected junctions.

### Junctions

Composite roads must work with:
- candidate detection;
- endpoint meeting;
- true crossing;
- approach extraction;
- junction regeneration;
- stale/manual-connectivity semantics.

Do not rewrite junction topology architecture merely for composite alignment.

### Shared 2D / 3D derivation

R1C derivation must:
- traverse the whole alignment;
- preserve component identities;
- include exact segment boundaries;
- remain deterministic;
- preserve local render-origin policy.

2D and 3D must not implement separate segment stitching logic.

## Persistence / migration

R3A must introduce an explicit new canonical schema version.

Minimum:
- schema v1 remains readable;
- v1 single-primitive alignment migrates deterministically to one-segment production Alignment;
- current writer emits only the new schema version;
- future/malformed schema rejection remains strict;
- exact finite f64 round-trip remains qualified;
- migrated semantic geometry is equivalent to the original v1 project;
- authored junction reconstruction continues to work.

Do not mutate v1 meaning by silently adding curves or standards behavior.

## Command/history integration

Existing R2C:
- AddRoad;
- ReplaceRoad;
- preview/commit;
- atomicity;
- revision;
- undo/redo

must remain correct with composite Alignment.

R3A does **not** add Road Draw editor-intent commands yet.

A ReplaceRoad containing a changed segment sequence must follow accepted junction invalidation/regeneration semantics.

## API compatibility posture

Backward source compatibility with every R1 internal constructor is secondary to a clean production model.

However:
- retain simple `Alignment::line`, circular-arc, and smooth-curve convenience constructors where they remain architecturally sound;
- simple constructors should create a one-segment production Alignment;
- tests/fixtures should be migrated deliberately rather than by compatibility hacks.

## Out of scope

No:
- Tauri/React/Vite/PixiJS;
- wasm-bindgen production adapter;
- Road Draw UI;
- generic road preset catalog implementation;
- editor-level CreateRoad/EditAlignment command expansion;
- tangent-discontinuous polyline/miter geometry;
- clothoid/spiral;
- vertical alignment;
- maps/reference layers;
- standards values;
- R3B+.

## Required tests / evidence

### A-G0 baseline/scope
- exact accepted base recorded;
- R3A-only diff;
- no frontend/application dependencies;
- workspace clean at handoff.

### A-G1 primitive preservation
For line, arc, smooth curve as one-segment Alignment:
- length;
- point;
- tangent;
- normal;
- projection;
- deterministic sampling

remain equivalent to accepted R1 evidence.

### A-G2 composite canonical fixtures
At minimum:
- line → tangent arc → line;
- line → smooth curve → line;
- multiple mixed segments;
- long/large-coordinate fixture;
- near-minimum valid segment lengths.

Verify exact cumulative station boundaries.

### A-G3 invalid continuity
Reject:
- endpoint gap above tolerance;
- tangent discontinuity above tolerance;
- zero/near-zero segment;
- non-finite primitive;
- duplicate segment id.

Failure must not create partially valid Alignment.

### A-G4 projection/boundary adversarial
Exercise:
- query exactly at boundary;
- nearly equidistant projections on adjacent segments;
- near endpoint;
- large-coordinate local context;
- deterministic tie result.

### A-G5 cross-section / lane lifecycle
Existing and new fixtures prove:
- CrossSection spans total alignment;
- width knots at segment boundaries remain exact;
- lane add/drop/taper across a segment boundary remains deterministic;
- right-turn-pocket lifecycle equivalence is preserved.

### A-G6 junction compatibility
Composite-road fixtures prove:
- candidate detection;
- endpoint meeting;
- true crossing;
- junction create/regenerate;
- road replacement invalidation;
- manual connectivity/stale behavior.

### A-G7 persistence migration
- schema v1 fixtures load;
- deterministic v1 → new-version migration;
- current new-version save/reopen;
- exact finite f64 behavior;
- clean R1C rebuild after migration;
- malformed/future schema rejection;
- no renderer/session history persisted.

### A-G8 R2C history
Transactions containing composite roads prove:
- preview = later commit candidate at same revision;
- failure atomicity;
- undo exact before snapshot;
- redo exact after snapshot;
- divergent commit clears redo;
- stale revision remains stale.

### A-G9 property/adversarial testing
Extend existing deterministic/property-style coverage for:
- cumulative station monotonicity;
- point/tangent finiteness;
- projection bounded to [0,total_length];
- sample station monotonicity and exact boundaries;
- migration round-trip equivalence.

### A-G10 performance
Benchmark representative 3/10/50-segment alignments:
- construction/validation;
- sampling;
- projection;
- R1C rebuild;
- Project clone/preview/commit impact.

Do not introduce complex caching unless evidence shows a blocker.

### A-G11 platform/regression
Required:
- Linux native qualification;
- Windows/MSVC qualification;
- kernel WASM build;
- project-core/project-io/project-session WASM where current workflows require them;
- all R1A/R1B/R1C regressions;
- all R2A/R2B/R2C regressions;
- Engineering Workflow Integrity.

## Independent review

Required: YES.

Independence basis:
fresh context or different reviewer not anchored to the executor narrative, plus deterministic CI evidence.

Review scope:
- segment/station semantics;
- continuity/tolerance use;
- projection tie behavior;
- cross-section/junction regression;
- schema migration;
- R2C atomic/history behavior;
- scope containment.

## Stop / architecture escalation

Stop and report rather than expanding if:
- tangent-continuous composition cannot support the intended R3 authoring without a new geometry family;
- composite stationing forces duplicated cross-section state per segment;
- junction algorithms require renderer-specific stitching;
- schema v1 cannot migrate deterministically;
- performance evidence shows an immediate architecture blocker;
- accepted tolerance policy must be changed to make tests pass;
- tangent-discontinuous roads become necessary to complete R3A.

## Execution routing

When R3A is authorized after exact-base creation:

- Work mode: **STRICT**
- Model: **GPT-5.6 Luna**
- Reasoning effort: **Max**
- Chat: **new Codex chat**
- Workspace: `D:\R&D\street-concept-designer`
- Why sufficient: the control plane has fixed the product abstraction, continuity rule, migration contract, and acceptance matrix; remaining work is difficult but bounded and testable.
- Why not Terra/Sol initially: no unresolved product architecture decision should remain at execution start.
- Escalate to Terra High/Max only if the executor finds a concrete kernel/schema conflict not resolvable within this contract.
- Escalate to Sol only for conflicting protected architecture/numerical evidence after a smallest reproducible case is produced.

## Final recommendation

Exactly one:
- `PROCEED_TO_R3B`
- `REMEDIATE_R3A`
- `ARCHITECTURE_ESCALATION`

Do not start R3B automatically.
