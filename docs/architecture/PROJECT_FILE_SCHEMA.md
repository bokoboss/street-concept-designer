# Project File & Schema Baseline

## Purpose

Define what must persist as project truth before implementation chooses the physical file/container format. The goal is to avoid renderer-cache persistence, silent schema drift, and migration debt.

## Core rule

The canonical project document stores semantic engineering/project state plus durable presentation/reference metadata. Derived 2D/3D geometry caches are optional and disposable.

## Logical project document

A project should conceptually contain:

```text
ProjectDocument
  schemaVersion
  projectId
  metadata
  units / trafficSide
  coordinateContext
  standardsProfiles[]
  referenceLayers[]
  scenarios[]
  assetReferences[]
  savedViews[]
  presentationSettings
  provenance / migration metadata
```

### Metadata
Examples:
- project name;
- description;
- created/modified timestamps;
- application/schema version;
- optional author/organization fields.

### Coordinate context
Keep engineering coordinate interpretation explicit:
- canonical unit: metres;
- optional CRS identifier/details;
- project/local origin;
- georeference/calibration metadata;
- never infer coordinate meaning solely from numeric magnitude.

### Standards profiles
Store pinned profile identity/version/reference, not merely `Thailand` or `latest`.

### Reference layers
Store source/calibration/display/provenance/capability metadata according to `MAP_BASEMAP_POLICY.md`. Do not persist third-party tile pixels unless provider/file policy expressly allows it.

### Scenarios
Each scenario contains semantic design state such as roads, junctions, markings, engineering assets, annotations, and scenario metadata. The product API should not expose storage mechanics such as full-copy vs delta as user-facing semantics.

### Presentation state
May include durable layer visibility, saved views/cameras, engineering-vs-presentation mode settings, asset presentation variants, and export presets where useful. Presentation state must not change engineering validation results.

## Semantic identity

Persistent objects require stable ids independent of renderer ids and display names. Renaming a road must not change its identity. Derived geometry should carry/reference semantic ids.

## Schema versioning

Requirements:
- explicit top-level `schemaVersion`;
- migrations are deterministic and testable;
- migration never silently applies new engineering standards or reinterprets project assumptions;
- unsupported future schema versions fail clearly rather than partially loading;
- old fixtures remain in regression tests once real versions exist.

## Standards-version behavior

Opening an old project under a newer application must preserve its pinned standards profile until the user explicitly chooses an update/migration. Application upgrade and standards upgrade are separate concepts.

## Numeric representation

Persist engineering values in explicit SI/metre semantics unless a field has another declared physical dimension. UI display units are presentation preference and must not alter stored engineering meaning.

Avoid serializing formatted strings such as `3.25 m` as the authoritative numeric value.

## Commands/history

R0 does not require event sourcing. The project may persist canonical snapshot state and optionally selected history/audit information. Undo history persistence is a later product decision.

If command records are persisted, they require their own versioning and must not replace the canonical schema merely for implementation convenience.

## Reference files / attachments

User-provided imagery/site plans may require packaging with the project for portability. The architecture should support a logical project package containing:
- canonical project document;
- permitted local reference files;
- optional user assets;
- optional disposable caches/thumbnails.

The physical container may later be a directory, ZIP-like package, or another format. Do not lock it until persistence/export requirements are tested.

Externally licensed/provider-hosted content must obey provider terms and should usually be referenced rather than copied into a package unless storage rights exist.

## Asset references

Built-in assets should be referenced by stable semantic asset id/version. User/imported assets require provenance and a strategy for embedding vs external references. Missing presentation assets must not corrupt core road geometry.

## Derived caches

Possible caches:
- sampled alignment geometry;
- 2D render primitives;
- 3D meshes;
- thumbnails;
- provider tile caches where permitted.

Rules:
- never authoritative;
- version/hash against source state when retained;
- safe to delete;
- application can rebuild equivalent output from canonical state;
- corrupted/old cache does not rewrite semantic state.

## Save atomicity / recovery

Production persistence should eventually provide:
- atomic save/replace where feasible;
- crash-safe temporary/recovery strategy;
- autosave/recovery independent of renderer cache;
- clear dirty/unsaved state;
- backup/migration safety before destructive schema conversion.

Exact implementation waits for the product persistence stage.

## Validation on load

Loading has layers:
1. file/container integrity;
2. schema/version validity;
3. semantic referential integrity;
4. geometry coherence/invariants;
5. advisory engineering validation.

A standards advisory warning is not the same as file corruption.

## Privacy / secrets

Do not store provider API keys, account tokens, or secrets inside ordinary project documents. Store provider identifiers/configuration references separately through secure application configuration.

## Minimal future canonical example

Illustrative only, not a locked JSON schema:

```json
{
  "schemaVersion": 1,
  "projectId": "project-...",
  "units": "m",
  "trafficSide": "LHT",
  "coordinateContext": {},
  "standardsProfiles": [],
  "referenceLayers": [],
  "scenarios": [
    {"id": "scenario-existing", "roads": [], "junctions": []},
    {"id": "scenario-alt-a", "roads": [], "junctions": []}
  ]
}
```

## Acceptance requirements before persistence implementation

- semantic model types/ids are sufficiently stable;
- scenario semantics are defined independently from storage optimization;
- reference-layer licensing/storage rules are implemented;
- schema migration test strategy exists;
- save/load roundtrip determinism/equivalence can be tested;
- renderer caches are demonstrably removable.

## Architecture invariant

**A project file records the design and its declared context—not the current renderer's memory layout.**
