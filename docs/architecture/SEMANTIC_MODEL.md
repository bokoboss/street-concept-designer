# Semantic Model

## Canonical project hierarchy

Project
- metadata
- coordinate/reference context
- pinned standards profile
- scenarios[]
  - network
    - roads[]
    - junctions[]
    - features[]
  - markings[]
  - assets[]
  - annotations[]
- presentation state

UI selection, camera state, hover state, DOM nodes, meshes, and renderer caches are not canonical engineering content.

## Road

A road owns:
- stable id;
- reference alignment;
- station range;
- cross-section/lane sections;
- longitudinal component profiles;
- semantic attachments;
- derived geometry cache references only where useful.

## Alignment

Initial primitives:
- line;
- circular arc;
- smooth conceptual curve.

Required API conceptually includes station length, pointAt(s), tangentAt(s), normalAt(s), project(point), and stable sampling.

## Cross-section

Ordered components may include traffic lane, median, shoulder, curb/gutter, sidewalk, verge, bicycle lane, parking lane, transit lane, barrier, and other future components.

A preset generates components; the generated components become editable semantic objects.

## Lane lifecycle

Lane width is a station-based profile. A lane can naturally appear/disappear through zero-to-full-width transitions. Turn pockets, lane adds/drops, widening, and tapering must use this general mechanism rather than special-case polygons.

## Junction

A junction is first-class and separates:
- participating roads/approaches;
- cut/connection stations;
- per-corner geometry;
- pavement surface derivation;
- islands/crossings/stop lines;
- lane-to-lane connections;
- allowed movements.

Geometry does not imply topology.

## Attachments

Road-relative assets/markings should prefer road/station/lateral coordinates when appropriate so they remain attached when geometry changes.

## Scenarios

Existing and alternatives share project context but contain scenario-specific engineering state. Implementation may later choose snapshot, structural sharing, or command-based delta after evidence; the semantic API must not expose storage mechanics.
