# Selection, Hit Testing & Snapping Model

## Purpose

Define interaction semantics before renderer implementation so selection/snapping remains consistent across 2D, 3D, hierarchy, and future AI/contextual actions.

## Selection source-of-truth

Application-level selection stores semantic ids/sub-entity ids, not renderer handles.

Examples:
- road;
- alignment control point;
- lane/component;
- junction;
- junction corner;
- lane connection;
- marking;
- asset instance;
- reference layer.

Renderers translate hit results to semantic ids, then shared application state drives properties/hierarchy/highlights.

## Selection hierarchy

Objects can have parent/child relationships. A click should prefer the most useful editable semantic target for the active context rather than always the deepest mesh primitive.

Example hierarchy:
```text
Road R01
  Alignment
    Control Point P3
  Lane L2
  Median M1
  Marking M7
```

Context/tool mode may alter which levels are selectable, but current mode/scope must be visible.

## Select vs navigation

Persistent core modes:
- Select/Edit;
- Hand/Pan/navigation.

3D additionally separates object selection from orbit/pan/camera movement. Navigation gestures should minimize accidental geometry edits.

Temporary keyboard modifiers may provide pan/orbit without changing permanent mode, but the committed editing state remains explicit.

## Hit testing

Renderer hit test may use spatial indexes/mesh picking, but final result returns:
- semantic target id;
- optional sub-entity id;
- world/project position;
- hit category;
- priority/depth metadata.

Do not store hit-test indexes as project state.

## Ambiguous selection

When multiple plausible semantic objects overlap:
- first click may select highest-priority context object;
- repeated click/cycle or a small chooser can access alternatives;
- hierarchy panel provides deterministic fallback;
- locked/reference layers are not editable selection targets unless explicitly requested.

Avoid precision-dependent pixel hunting.

## Selection priority principles

Priority depends on context, but common ordering may prefer:
1. active edit handle/control point;
2. explicitly selectable engineering feature (corner/lane/marking);
3. parent road/junction;
4. presentation asset;
5. reference layer only in reference-edit mode.

The exact priority table should be UAT-tested.

## Multi-selection

Not required for R1, but architecture should allow a set of semantic ids and a primary selection. Batch property actions must declare which properties are compatible across selected objects.

## Snapping is not topology

A snap is an interaction aid producing an explicit candidate target/coordinate. It **never automatically means**:
- create junction;
- merge road nodes;
- connect lanes;
- share topology.

Example:
```text
cursor snaps to Road B endpoint
→ endpoint coordinate chosen
→ if semantic connection is possible, system may create a Candidate Junction/Connection
→ user explicitly confirms topology
```

## Snap candidate families

Potential families:
- endpoint/control point;
- midpoint;
- geometric intersection;
- nearest point on alignment;
- perpendicular projection;
- tangent candidate;
- station;
- guide/reference line;
- grid;
- junction/approach anchor.

Implement only those justified by current stage/workflow.

## Snap candidate record

Conceptually:
```text
candidateId
kind
targetSemanticId
projectPoint
station/parameter if relevant
score/priority
visualCue
wouldCreateCandidateTopology yes/no
```

Candidate is ephemeral UI state until accepted.

## Acquisition tolerance

Cursor acquisition may use a screen-space radius for usability. It must convert/query against project geometry without turning screen pixels into engineering equality.

Separate:
- UI acquisition radius;
- geometry-kernel computational tolerance;
- topology/merge criteria.

See `GEOMETRY_PRECISION_TOLERANCE_POLICY.md`.

## Candidate ranking

Rank semantically, not only by Euclidean distance. For example an endpoint slightly farther from cursor may outrank an arbitrary nearest edge when drawing a connecting road.

Ranking inputs may include:
- active tool intent;
- target type priority;
- screen/project distance;
- alignment direction/tangent compatibility;
- lock/visibility state;
- topology implications.

Do not silently choose a topology-changing candidate merely because it scores highest; confirmation rules still apply.

## Visual feedback

Before accepting snap:
- show target cue/icon;
- distinguish endpoint/intersection/perpendicular/etc.;
- optionally show station/distance;
- distinguish `snap only` from `connection candidate` where relevant.

The user should understand what will happen before click/commit.

## Exact numeric entry

Numeric constraints/coordinates may bypass cursor snapping but still resolve to semantic/project values through the same command layer. Direct manipulation and exact entry must converge on equivalent model state.

## 2D/3D selection synchronization

Selection is shared semantic state. Snapping/editing is primarily 2D in initial product phases; 3D selection is supported for inspection. Future 3D snapping/editing must use semantic/project geometry, not raw triangle coordinates as authoritative edits.

## Reference layers

Basemap/image references may support calibration/control-point interaction in a dedicated mode. Ordinary road drawing does not select tile/image pixels as semantic objects.

## Testing

Eventually test:
- same semantic object selected from 2D/3D/hierarchy;
- locked/reference layers cannot be edited accidentally;
- overlapping target cycling deterministic;
- snap priority examples;
- screen zoom changes acquisition behavior but not accepted engineering coordinate after same target chosen;
- snap-to-crossing does not create topology without explicit command;
- stale snap candidate invalidates if geometry changes before commit;
- hit-test cache rebuild does not change semantic ids.

## Architecture invariant

**Selection identifies semantic objects; snapping proposes geometric intent; topology remains an explicit engineering action.**
