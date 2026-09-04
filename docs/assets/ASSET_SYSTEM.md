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
- LOD/triangle/texture metadata for 3D assets;
- deterministic variation/variant metadata where generated appearance is persisted;
- instancing/batching eligibility and geometry-variant key inputs where relevant.

## Representation architecture

One semantic asset may have multiple derived representations, but representation objects never become the asset's canonical identity.

Conceptually:

```text
semantic asset
  ├── physical dimensions / semantic metadata
  ├── plan2D representation
  ├── model3D representation
  ├── thumbnail
  └── export/baked representation
```

The 2D symbol does not need to be a projection of the 3D mesh. The 3D mesh does not define authoritative engineering dimensions.

### Geometry variant versus instance state

Repeated presentation assets should distinguish fields that require new geometry from fields that can be expressed by a per-instance transform or lightweight runtime state.

Typical geometry-variant inputs:
- asset/preset/species/model family;
- deterministic variation seed;
- shape/detail/LOD parameters that materially alter vertices;
- geometry-affecting style options.

Typical instance inputs:
- semantic object id;
- project/scenario placement;
- road/station/lateral attachment when applicable;
- position/rotation/scale;
- visibility/presentation overrides that do not require new geometry.

This distinction is an optimization contract, not a new semantic source-of-truth.

### Deterministic variation and batching

Persisted generated distributions/variations must be reproducible. Prefer recorded seeds and, where visually acceptable, bounded variant pools so many instances can share geometry.

Renderers may cache variants and use instancing/batching. Optimized draw representation must preserve a deterministic mapping to semantic ids for selection, inspection, scenario state, and export.

A per-object renderer proxy may be used for picking/outline/direct manipulation when an instanced renderer cannot expose those affordances cleanly, but the proxy is disposable renderer state.

### Internal definition registry

A future internal asset-definition contract may declaratively describe:
- semantic kind/category;
- physical metadata;
- editable parameters;
- placement/attachment capabilities;
- plan2D builder/reference;
- model3D builder/reference;
- variant key/instancing policy;
- provenance/license;
- inspector/tool/AI descriptors.

All committed edits still pass through normal semantic commands/transactions. This internal composition mechanism does not imply a public plugin API.

## Production policy

1. Prefer procedural generation for engineering geometry/markings.
2. Prefer internally created vector/parametric assets for standards-sensitive traffic content.
3. Use permissively licensed external assets only with machine-readable provenance.
4. Never copy proprietary competitor assets.
5. Normalize scale, orientation, pivot, metadata, and performance before bundling.

## Initial library priorities

Markings, arrows, crosswalks, signs, signals, lights, barriers/guardrails, bollards/delineators, sedan, pickup/SUV, motorcycle, bus, truck, trees, and simple building massing.


## Prior-art reference

See `docs/research/PASCAL_EDITOR_PRIOR_ART_REVIEW.md` for the 2026-09-03 review of Pascal Editor and its Nature plugin. The useful patterns are pure representation builders, variant/instance separation, deterministic bounded variation, instancing, 2D/3D multi-representation, and semantic selection through optimized rendering. The Pascal scene/store architecture and public plugin model are not adopted.
