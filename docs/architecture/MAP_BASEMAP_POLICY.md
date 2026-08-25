# Map / Basemap Architecture & Licensing Policy

Verified research date: 2026-08-25

## Product requirement

Map, satellite/aerial imagery, and imported site plans are reference layers that support engineering authoring. They are never canonical engineering geometry.

The application must separate:

1. **Map renderer** — draws geospatial/reference layers;
2. **Basemap provider** — supplies tiles/images/data under provider-specific terms;
3. **Reference layer** — project-level instance with transform/opacity/display metadata;
4. **Engineering model** — independently authored semantic roads/junctions/assets.

## Renderer candidate

MapLibre GL JS is a strong candidate because its style/source model supports independent map sources and layers, including vector TileJSON, raster tiles, and raster/WMS-style sources.

Official references:
- https://maplibre.org/maplibre-style-spec/
- https://maplibre.org/maplibre-style-spec/sources/

MapLibre is a rendering technology, not a license to use any particular basemap. Every provider/source still requires its own terms and provenance review.

## Provider capability contract

A provider/reference-source record should declare capabilities/policy such as:

```text
providerId
sourceType              vector | raster | wms | xyz | local-image | local-geotiff | other
attribution
termsUrl
licenseId
allowInteractiveDisplay
allowDigitization       yes | no | unknown
allowExportWithOverlay  yes | no | conditional | unknown
allowPersistentCache    yes | no | conditional | unknown
allowOfflinePrefetch    yes | no | conditional | unknown
cachePolicy
requiresOnlineAccess
requiresApiKey
requiresVisibleBranding
lastTermsReviewDate
```

Unknown must be treated conservatively. A provider with `allowDigitization = no` must not be offered as a tracing/reference source for creating engineering geometry.

## Critical distinction: OSM data vs OSM-hosted tiles

OpenStreetMap data is open under ODbL subject to attribution/share-alike obligations for applicable database uses, but OSM Foundation-hosted tile services are separately capacity- and policy-constrained.

Official references:
- OSM copyright/license: https://www.openstreetmap.org/copyright
- raster tile policy: https://operations.osmfoundation.org/policies/tiles/
- vector tile policy: https://operations.osmfoundation.org/policies/vector/

The standard `tile.openstreetmap.org` service:
- is best-effort and has no SLA;
- requires visible attribution and compliant caching;
- prohibits bulk prefetch/download/offline-area features;
- may block clients that violate policy;
- explicitly recommends alternative providers or self-hosting when product needs exceed the service policy.

Therefore:
- do not hard-code OSMF tile endpoints as the only production basemap;
- do not equate open OSM data with unrestricted tile-server use;
- offline workflows should use a provider/data pipeline that expressly supports offline use or self-hosted data/tiles.

## Google Maps Platform / satellite imagery

Current Google Maps Platform policy is incompatible with our core satellite-tracing workflow unless a specific agreement/product exception explicitly permits it.

Official references:
- Map Tiles API policies: https://developers.google.com/maps/documentation/tile/policies
- Google Maps Platform Terms of Service / current terms landing page: https://cloud.google.com/maps-platform/terms/

Published Google Maps Platform terms prohibit creating content from Google Maps Content and explicitly give tracing/digitizing roadways/building outlines from Maps JavaScript API satellite basemap as an example. Map Tiles API policy also restricts unauthorized prefetching/caching/storage and offline uses.

Policy for this project:
- do **not** implement Google satellite imagery as a default engineering digitization/tracing source;
- do not cache/export Google imagery except where the governing current agreement explicitly permits it;
- any future Google integration must undergo a fresh terms review and be capability-restricted in code;
- never make product viability depend on Google satellite availability.

## Preferred reference-source strategy

### A. User-provided image / site plan

Core requirement and safest baseline when the user has legitimate rights to the reference.

Support:
- PNG/JPEG initially;
- known-distance A–B scale calibration;
- rotation/north alignment;
- opacity/dim/saturation controls;
- lock/reference-only state;
- later control-point georeferencing where useful.

The project should store the user's reference file or a project-managed copy only according to explicit project-file policy.

### B. Open / licensed vector map data

OSM-derived data can support road/building context when used in compliance with ODbL and attribution requirements. Prefer provider/self-hosting strategies appropriate to expected product load and offline requirements.

Do not treat imported OSM geometry as engineering truth. It is editable/reference starting data requiring engineer verification.

### C. Licensed satellite/aerial provider

The product architecture should allow a commercial or organizational imagery provider whose license explicitly permits the intended interactive display, tracing/digitization, caching/offline behavior, and export workflow.

Provider onboarding requires a documented terms/capability review before enabling engineering digitization.

### D. Organization/project WMS/XYZ/GeoTIFF

Support future organizational GIS imagery/services where the user/client already has rights. Preserve service/source metadata and attribution requirements.

## Reference-layer model

A project reference layer should store engineering-independent metadata such as:
- stable layer id;
- provider/source id;
- source URI/file reference;
- source type;
- CRS/georeference metadata when known;
- calibration transform when image-based;
- visible/locked;
- opacity;
- dim/brightness/contrast/saturation display controls;
- z-order;
- attribution/terms snapshot/reference;
- cache/export capability state.

Do not store provider tile pixels as project truth.

## UX requirements

Map/reference panel should expose:
- provider/source selector;
- map vs satellite/aerial where licensed;
- opacity;
- Dim Background;
- saturation/contrast;
- labels toggle when source supports it;
- lock;
- calibration/georeference status;
- attribution visibly where required.

If a provider forbids digitization, the UI must prevent/disable tracing workflows on that provider rather than relying on a hidden legal note.

## Export policy

Engineering geometry/export must remain possible without embedding the basemap.

Export pipeline should support:
- engineering-only transparent/neutral-background export;
- reference/basemap included only when provider terms permit;
- mandatory attribution rendered into exported output when required;
- an explicit warning/disabled option when the reference layer cannot legally be exported.

This separation is important because a valid engineering design must not become non-exportable merely because a particular map provider is restricted.

## Offline policy

Core semantic project editing should work offline. Online basemap availability is an optional reference service.

Offline basemap support must only be enabled for:
- user-provided local imagery;
- self-hosted/offline datasets;
- providers explicitly permitting offline packages/prefetch/cache.

Do not implement generic `Download this area` functionality against arbitrary providers.

## Acceptance gates for adding a provider

Before a provider becomes built-in:
1. official terms/license reviewed and date recorded;
2. digitization/tracing permission classified;
3. cache/offline policy classified;
4. export/screenshot/overlay policy classified;
5. attribution/branding requirements implemented;
6. API-key/secrets handling reviewed;
7. automated capability tests prevent prohibited operations;
8. provider can be removed/switched without schema migration of canonical engineering geometry.

## Architecture invariant

**Basemap provider terms may change product capabilities, but must never change the canonical engineering model.**
