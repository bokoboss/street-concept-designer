# Export Architecture & Policy

## Purpose

Export must communicate engineering concepts reliably without making the project dependent on renderer internals or restricted basemap/provider content.

## Export categories

### Engineering plan export
- 2D plan geometry;
- dimensions/annotations where enabled;
- transparent/neutral background option;
- selected scenario(s);
- engineering/presentation style preset;
- high-resolution raster and later vector output.

### Comparison export
- Existing vs Alternative overlay;
- side-by-side;
- before/after presentation;
- selected annotations/change notes.

### 3D presentation export
- still image from saved/current camera;
- later GLB/video where justified.

### Interoperability export
Later candidates:
- SVG;
- GeoJSON;
- DXF;
- GLB;
- OpenDRIVE.

These are derived adapters, never the canonical project format.

## Basemap independence

Every design must remain exportable without a basemap/reference layer.

Reference inclusion is conditional on provider/source capability and terms:
- allowed: include with required attribution/branding;
- conditional: enforce limits/options;
- forbidden/unknown: disable inclusion and export engineering geometry only.

Never flatten restricted reference imagery into an engineering export merely because it is visible on screen.

## Attribution

When source/provider terms require attribution:
- render it visibly in exports where reference content is included;
- preserve required provider branding rules;
- do not allow style settings to hide mandatory attribution;
- record provider/source identity in export metadata/log where practical.

## Concept-level disclosure

The product should support optional export notes indicating context such as:
- Concept Design / Not for Construction;
- reference source/calibration quality;
- standards profile/version used for advisory checks;
- unresolved validation issues/overrides if the workflow requires disclosure.

Do not automatically imply regulatory approval or detailed-design compliance.

## Determinism

Given the same semantic project state, export settings, asset versions, and renderer/export version, output geometry/content should be reproducibly equivalent. Presentation raster pixels may vary across renderer/GPU versions where unavoidable, but engineering geometry and metadata must remain stable.

## Vector export

SVG/vector export should derive from renderer-independent engineering primitives where possible rather than serializing the live DOM/canvas scene as project truth.

Maintain:
- true engineering dimensions/scale metadata where appropriate;
- semantic grouping/layers where useful;
- text as text when feasible;
- reference imagery as a separate inclusion governed by rights.

## Raster export

Support user-selectable physical/resolution presets suitable for reports/presentations. High-resolution export must not require zooming the interactive screen or changing project geometry.

Avoid screenshot-only architecture; export should render an explicit output viewport/scene.

## 3D export

GLB/glTF later may contain derived road meshes and approved assets. It must not be used as a round-trip authoritative engineering project unless a future importer explicitly reconstructs/validates semantics.

Provider/reference terrain/imagery must only be embedded when rights allow.

## Saved views

Named views/cameras are durable presentation state and can drive repeatable exports. They do not affect engineering geometry.

## Fonts / external resources

Bundled/exported fonts, icons, textures, and 3D assets require appropriate licensing. Export must not silently embed resources whose license forbids redistribution.

## Export validation

Before writing output:
- project semantic state valid enough for requested export;
- selected scenario exists;
- required representations/assets available or clear proxy/fallback chosen;
- provider/reference inclusion policy satisfied;
- attribution requirements satisfied;
- output dimensions/resolution valid;
- no secrets/API keys/provider tokens serialized.

## Failure behavior

If basemap/reference export is prohibited or unavailable:
- do not fail the engineering export;
- clearly explain reference omission;
- offer transparent/neutral engineering output.

If a presentation asset is missing:
- use an approved proxy or report it;
- do not remove/corrupt the semantic project object.

## Export provenance

A future evidence-friendly export may include metadata such as:
- project/scenario id;
- application version;
- schema version;
- standards profile/version;
- export date;
- saved view/export preset;
- reference-provider attribution;
- validation status summary.

This should be configurable so ordinary presentation exports remain clean.

## Architecture invariant

**Export is a derived communication/interoperability product of the semantic design; no third-party basemap or renderer cache may become a prerequisite for preserving the design.**
