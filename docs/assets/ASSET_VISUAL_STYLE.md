# Asset Visual Style

## Principle

Asset style must support two distinct needs without creating two separate semantic libraries:

1. **Engineering clarity** — readable plan/3D geometry, real dimensions, minimal visual noise.
2. **Presentation quality** — enough material/context detail to communicate a convincing concept to clients and stakeholders.

One semantic asset may have engineering and presentation representations/modes.

## 2D engineering style

### Geometry
- use true/declared physical footprint where relevant;
- prefer crisp vector/procedural geometry;
- simplify details that do not help identification or engineering reading;
- preserve important silhouette/dimensions such as vehicle length/width, pole position, barrier footprint, tree canopy footprint where relevant;
- avoid perspective effects in top-view symbols.

### Scale behavior
Some representation detail may change by zoom level, but the physical attachment point and footprint remain stable.

At small scale:
- simplify detail;
- preserve category recognition;
- avoid text baked into generic icons where localization/standards matter.

At large scale:
- reveal markings, outlines, mounting/support details, and engineering anchors where useful.

## Road markings

Markings are not decorative illustrations. Their geometry should remain parameter-driven and standards-profile-aware. Rendering may add anti-aliasing/material effects, but engineering dimensions remain independent of visual styling.

## Sign assets

2D plan representation should distinguish:
- sign face/support location;
- facing/orientation;
- support/gantry footprint when relevant.

Detailed sign artwork belongs to sign-face representation rather than the plan footprint.

## Vehicle assets

Engineering mode:
- recognizable but visually restrained;
- accurate length/width/class;
- clear heading;
- top-view footprint suitable for movement/queue illustrations later.

Presentation mode:
- richer 3D model/material variation;
- avoid brand/trademark dependency unless licensed;
- generic vehicle families are preferred for bundled assets.

## Trees / vegetation

Engineering mode:
- simple trunk anchor + canopy footprint;
- canopy size metadata;
- avoid overly dense visual textures that hide road geometry.

Presentation mode:
- richer tree meshes/materials are allowed;
- use LOD/instancing for repeated planting;
- deterministic distribution where saved.

## Signals / lights / roadside devices

Prioritize silhouette and mounting geometry over decorative detail. Repeated devices should remain lightweight enough for large scenes.

## 3D engineering mode

Use neutral, low-noise materials:
- clear pavement/components;
- visible lane markings;
- readable curb/median/sidewalk edges;
- restrained asset colors unless color has engineering/traffic-control meaning;
- selection and validation overlays remain legible.

Avoid photorealism that makes semantic geometry harder to inspect.

## 3D presentation mode

May add:
- higher-quality PBR materials;
- vegetation variation;
- vehicles/people;
- lighting/shadows/environment;
- context buildings;
- richer ground treatment.

Presentation styling must not mutate engineering dimensions or validation.

## Consistency rules

- consistent coordinate axes and forward/up conventions;
- consistent scale in metres;
- consistent pivot conventions by asset family;
- consistent thumbnail camera/background;
- consistent material naming;
- consistent selection bounds/footprints;
- semantic color/status meaning defined centrally rather than per asset.

## Quality target

The bundled starter library should feel coherent and professional before it feels large. Prefer a small consistent set of correctly dimensioned assets over hundreds of visually inconsistent objects.
