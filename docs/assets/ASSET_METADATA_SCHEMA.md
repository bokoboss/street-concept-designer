# Asset Metadata Schema Baseline

## Purpose

Define durable metadata needed to make built-in, generated, AI-assisted, and licensed-source assets reliable across 2D/3D views without depending on filenames or manual cleanup.

## Asset identity

Each asset definition should conceptually include:

```text
assetId
assetVersion
family
category
name
semanticTags[]
physical
representations
placement
provenance
license
performance
verification
```

Stable semantic ids are preferred over display names or file paths.

## Families

At minimum:
- procedural road/component;
- procedural marking;
- semantic assembly;
- prop;
- distribution/preset.

## Physical metadata

Where meaningful:
- canonical unit = metres;
- length/width/height;
- footprint/bounding box;
- anchor/pivot;
- forward/up orientation;
- mounting height/offset range;
- canopy/trunk or support footprint;
- vehicle class/dimensions;
- collision/clearance proxy where later needed.

Do not fabricate dimensions for standards-sensitive assets. Generic proxies must be marked generic.

## Representations

An asset may reference:
- generated 2D vector/procedural representation;
- static 2D plan symbol/footprint;
- 3D GLB/glTF representation;
- thumbnail/preview;
- LOD variants;
- engineering-mode vs presentation-mode representation settings.

Each representation should record its own version/hash/source where useful so visual updates do not force project semantic ids to change.

## Placement metadata

Examples:
- allowed placement: point / path / edge / area / assembly child;
- default attachment type;
- allowed host categories;
- default orientation behavior;
- lateral/vertical offset semantics;
- repeat/spacing capability;
- canBake/explode;
- deterministic distribution seed requirement.

## Semantic attachment

When an asset belongs relative to a road/junction, project instances should prefer semantic attachment over raw XY alone:
- road id + station + lateral offset;
- lane/sidewalk/edge reference;
- junction approach/corner;
- area/polygon host.

World position can be derived and may be cached, but attachment semantics help assets remain coherent when road geometry changes.

## Provenance

Record, as applicable:
- createdBy: procedural / project / AI-assisted / external;
- source title/url/repository/provider;
- author/creator;
- source date/version;
- transformation/normalization pipeline version;
- original file hash;
- derivative hash/version;
- standards profile/source refs for authority-specific assets.

## License metadata

Conceptual fields:
- SPDX identifier when applicable;
- license name/url;
- attribution text;
- commercial-use status;
- modification/redistribution constraints;
- share-alike/notice requirements;
- bundledAllowed yes/no/conditional;
- source-file redistribution allowed yes/no/conditional;
- review date;
- reviewer/status.

Unknown rights mean `not approved for bundling` until reviewed.

## Verification status

Suggested categories:
- verified engineering/generated;
- verified authority/profile asset;
- generic presentation proxy;
- licensed presentation asset;
- unverified development-only;
- blocked/license unknown.

UI should not present a generic proxy as an authority-verified engineering device.

## Performance metadata

For 3D/render assets:
- triangle/vertex count;
- material count;
- texture count/resolution;
- approximate GPU memory where useful;
- instancing compatibility;
- LOD levels;
- animation/bone information if later needed.

Performance limits may vary by asset family and product stage.

## Sign assembly example

A traffic sign may separate:
- sign-face semantic definition;
- face artwork/vector representation;
- support assembly;
- mounting metadata;
- 2D orientation symbol;
- 3D assembly representation.

This avoids duplicating every sign face for every support type.

## Vehicle example

```text
asset.vehicle.bus.city.12m
  family: prop
  category: vehicle.bus
  physical: 12.0 x 2.5 x ... m
  plan2D: generated/symbol
  model3D: <normalized GLB>
  semanticTags: [bus, transit, heavy-vehicle]
  license/provenance: ...
```

Future swept-path/simulation modules may use vehicle engineering metadata but must not assume presentation mesh dimensions are authoritative without verification.

## Asset instances vs asset definitions

Definition = reusable library metadata/representations.
Instance = project placement/attachment/transform/overrides.

Project instances reference stable definition id/version. Updating a library asset's representation should not silently change engineering dimensions in old projects if dimensions are behaviorally significant.

## Missing asset behavior

If a presentation representation is missing:
- retain semantic project object;
- show proxy/bounding representation where practical;
- report missing representation;
- do not delete the instance or corrupt road geometry.

## Normalization QA

Before an external/generated representation is approved:
- validate dimensions/units;
- orientation/pivot;
- materials/textures;
- plan footprint;
- provenance/license;
- performance;
- deterministic load;
- expected selection bounds;
- engineering/presentation classification.

## Architecture invariant

**An asset is a semantic, sourced definition with representations—not just a GLB/SVG filename.**
