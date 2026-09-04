# Asset Production Pipeline

## Objective

The product and development process must not depend on the user manually drawing SVG artwork, modelling GLB assets, editing textures, or preparing asset metadata. Asset production is an engineering/development responsibility handled through procedural generation, AI-assisted tooling, or licensed-source normalization.

## Decision tree

For every needed asset:

1. Can it be represented correctly as procedural engineering geometry?
   - If yes: generate from semantic parameters in code.
2. If not, can a simple vector/parametric assembly represent it?
   - If yes: generate source vector/mesh programmatically and keep semantic dimensions.
3. If not, can an AI-assisted asset workflow generate an acceptable source with clear rights/provenance?
   - If yes: generate, inspect, normalize, and record provenance.
4. If not, find a permissively licensed source.
   - verify license;
   - store source/author/license/attribution;
   - normalize scale/orientation/pivot/materials/LOD;
   - never rely on a remote asset that can disappear silently.
5. If none of these paths are acceptable, mark the asset blocked rather than assigning manual production work to the user.

## Procedural representation contract

For generated or normalized presentation assets, separate expensive geometry generation from cheap instance placement whenever the asset family permits it.

A future representation builder should be deterministic for its declared geometry inputs:

```text
geometry variant spec
  -> deterministic builder or normalized model lookup
  -> reusable geometry/material resources
```

Per-instance placement then supplies semantic identity plus transform/attachment data without regenerating the variant.

When randomized presentation is persisted:
- record the seed or chosen stable variant;
- prefer bounded variant pools when that preserves acceptable visual diversity;
- never regenerate a materially different scene on save/reopen merely because a runtime RNG changed.

Renderer variant caches and instance buffers are disposable. Deleting them must allow a deterministic clean rebuild from project/asset state.

Runtime representation may differ from export representation. For example, an editor may render many trees/vehicles with instancing while an export path emits a deterministic baked/static representation. Both paths must preserve physical scale, semantic ownership/provenance, and equivalent visible placement.

Selection/picking of instanced assets must resolve directly to the semantic object id through instance metadata/index mapping or a proven proxy mechanism; do not make renderer mesh identity canonical.

## Pipeline A — Procedural markings

Examples:
- solid/dashed/double lane lines;
- edge lines;
- stop/yield lines;
- crosswalks;
- arrows/stencils;
- hatch/chevron/gore;
- parking stalls;
- motorcycle/bicycle markings.

Source of truth:
- semantic type;
- physical dimensions;
- dash/spacing/repeat parameters;
- target attachment;
- standards/profile provenance.

Outputs:
- 2D render primitives/vector paths;
- optional 3D/decal/mesh representation from the same parameters.

Never store a PNG as the engineering definition of a standard marking.

## Pipeline B — Semantic assemblies

Examples:
- traffic signs;
- signal poles/heads;
- streetlights;
- bollards/delineators;
- barriers/guardrails;
- gantries.

Model as reusable parts where practical.

Example sign assembly:
- face geometry;
- border/background;
- symbol/text layer;
- support type;
- mounting height/offset;
- 2D symbol/footprint;
- 3D generated assembly.

This allows a sign face to be reused on single-pole, double-pole, mast-arm, or gantry supports without duplicating the semantic asset.

## Pipeline C — Generated simple 3D props

Suitable for code-generated low/medium-detail assets:
- bollards;
- cones;
- simple signs/poles;
- basic signal heads;
- barriers;
- guardrail segments;
- streetlight proxies;
- building massing;
- simple benches/street furniture.

Requirements:
- real-world dimensions;
- deterministic geometry generation for equal declared inputs where generation is procedural;
- explicit geometry-variant inputs versus per-instance transform inputs when repeated use is expected;
- bounded/reproducible variation strategy when randomized appearance is persisted;
- consistent world orientation;
- predictable pivot;
- efficient geometry;
- deterministic generation;
- reusable 2D plan footprint.

## Pipeline D — Complex presentation props

Examples:
- realistic vehicles;
- people;
- trees/vegetation;
- detailed shelters;
- complex urban furniture.

Preferred order:
1. permissively licensed existing asset with clear provenance;
2. AI-assisted generation with rights/provenance review;
3. procedural proxy if realism is not necessary.

The project should not spend early engineering effort creating AAA-quality art when a clean proxy adequately supports engineering/presentation workflows.

## 2D/3D representation contract

One semantic asset record may reference multiple representations:

```text
asset.vehicle.bus.city12
  physical dimensions
  semantic tags
  plan2D representation
  model3D representation
  thumbnail
  source/license/provenance
```

Selection, placement, scenario state, and engineering attachment belong to the semantic object, not to a particular SVG/GLB representation.

## Normalization gate for imported/generated 3D assets

Before bundling:
- verify units/metres;
- verify bounding dimensions;
- orient forward/up consistently;
- set pivot/origin intentionally;
- inspect normals/materials;
- reduce unnecessary mesh/material complexity;
- validate texture paths/format;
- record triangle/material/texture metadata;
- create/verify plan footprint;
- generate thumbnail;
- verify license/provenance;
- test deterministic loading in target renderer.

## Asset identifiers

Use semantic stable ids rather than filenames, for example:
- `vehicle.car.sedan.generic`
- `vehicle.bus.city.12m`
- `traffic.sign.warning.generic`
- `traffic.signal.head.3aspect`
- `roadmark.arrow.through`
- `roadmark.crosswalk.zebra`
- `street.light.single_arm`
- `landscape.tree.medium.generic`

Version representations independently from project object ids where possible.

## Thailand profile workflow

Standards-sensitive traffic assets must be generated from a verified authority/profile:
- DOH;
- DRR;
- project/custom;
- other future jurisdictions.

Do not create one ambiguous `thai` asset whose dimensions mix multiple authorities/documents.

## QA gates

Each bundled engineering asset should pass relevant checks:
- dimensions and orientation;
- semantic metadata completeness;
- rendering at typical 2D/3D scales;
- attachment/placement behavior;
- deterministic generation/load;
- source/license/provenance completeness;
- standard-profile verification status.

Presentation props additionally need performance/LOD checks where appropriate.

## User-facing principle

The user selects intent and engineering parameters. The system handles geometry/artwork/model generation. Manual asset editing is an advanced escape hatch, not a prerequisite for normal use.


## Prior-art note

Pascal Editor / `plugin-trees` is retained as prior art for deterministic procedural props, geometry variant caching, instancing, 2D/3D representation separation, and runtime-versus-baked rendering. See `docs/research/PASCAL_EDITOR_PRIOR_ART_REVIEW.md`. No Pascal dependency is authorized by this note.
