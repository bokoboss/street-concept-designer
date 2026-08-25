# Asset System

## Principle

The user is not expected to draw 2D symbols or model 3D assets manually. Core assets must be generatable, bundled, or importable through an automated pipeline.

## Asset families

### Procedural Road
Lane surfaces, curbs, medians, sidewalks, verge, barriers, and other geometry generated from semantic parameters.

### Procedural Marking
Solid/dashed/double lines, stop lines, yield lines, crosswalks, hatches, arrows, parking stalls, stencils, and repeated markings generated parametrically.

### Semantic Assembly
Traffic signs, signal poles/heads, streetlights, gantries, bollards, delineators, guardrails, and similar items assembled from meaningful parts and metadata.

### Prop
Vehicles, trees, people, benches, shelters, and context objects represented by a 2D plan symbol/footprint plus 3D model where needed.

### Distribution
Rules that place assets along paths/edges or inside areas, with optional bake/explode to individual editable objects.

## Common metadata

- stable asset id;
- family/category;
- physical dimensions;
- semantic tags;
- orientation/pivot;
- 2D representation;
- 3D representation;
- source;
- author/creator;
- license;
- attribution;
- version;
- standards/profile provenance where relevant;
- LOD/triangle/texture metadata for 3D assets.

## Production policy

1. Prefer procedural generation for engineering geometry/markings.
2. Prefer internally created vector/parametric assets for standards-sensitive traffic content.
3. Use permissively licensed external assets only with machine-readable provenance.
4. Never copy proprietary competitor assets.
5. Normalize scale, orientation, pivot, metadata, and performance before bundling.

## Initial library priorities

Markings, arrows, crosswalks, signs, signals, lights, barriers/guardrails, bollards/delineators, sedan, pickup/SUV, motorcycle, bus, truck, trees, and simple building massing.
