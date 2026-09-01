# R2B — Versioned Persistence & Migration

## Objective

Persist and restore the accepted R2A Project/Scenario semantic state without making runtime Rust memory layout or renderer-derived geometry the canonical file schema.

## Entry

Requires R2A accepted/merged.

Record exact accepted `main` base before first change.

## Scope

- persistence/schema DTO layer;
- explicit schemaVersion;
- canonical JSON document encoding for R2;
- domain ↔ persistence conversion;
- stable id round trip;
- Road / Alignment / CrossSection semantic persistence;
- Junction authored-state persistence and derived-state reconstruction;
- scenario/project persistence;
- deterministic canonical ordering;
- migration harness;
- unsupported future schema rejection;
- string/bytes round trip;
- basic path save/load if bounded and reliable;
- R1C rebuild equivalence after load;
- dependency/license evidence.

## Physical container decision

R2B does **not** lock the final `.scd` container.

The R2 canonical semantic document may be JSON.

Future packaging may contain:
- project JSON;
- permitted local references;
- user assets;
- disposable caches.

Do not build ZIP/package logic in R2B.

## Serialization dependency candidate

Preferred candidate:
- serde;
- serde_json.

Before adoption:
- verify exact current crates/versions;
- verify SPDX/license;
- verify source repository;
- verify native/MSVC/WASM compatibility for the chosen use;
- record dependency rationale and lockfile change.

Do not add unrelated persistence/database frameworks.

## Schema boundary

Prefer explicit persistence DTOs or a similarly clear boundary.

Do not simply add Serialize/Deserialize to every internal type if that would:
- persist derived fields;
- expose memory layout as schema;
- make migrations dependent on private implementation details.

## Schema v1 minimum

Conceptually:

```text
ProjectDocumentV1
  schemaVersion = 1
  projectId
  name
  canonicalUnits = "m"
  trafficSide
  coordinateContext
  scenarios[]
    scenarioId
    name
    role
    locked
    network
      roads[]
      junctionDefinitions[]
```

Persist only fields actually implemented in R2.

Do not add misleading empty "future" objects as though supported.

## Road persistence

Persist semantic definitions needed to reconstruct:
- RoadId;
- Alignment primitive + parameters;
- ordered CrossSection components;
- ComponentId;
- ComponentKind;
- WidthProfile station range + knots;
- R1B lane-direction metadata where required by Road.

Do not persist R1 sampled alignment arrays.

## Junction persistence

This is a mandatory R2B risk.

Persist **authored semantic junction intent**, not disposable R1B derived geometry.

Required authored information must be sufficient to restore:
- JunctionId;
- participating Road ids / explicit topological relationship;
- candidate/crossing relationship needed for reconstruction;
- authored per-corner radius overrides by stable CornerId;
- connectivity mode;
- manual LaneConnection definitions/intents;
- semantic options required to reconstruct equivalent current junction state.

Do not persist as canonical truth:
- Approach derived frames;
- corner sampled arc points;
- PavementSurface vertices;
- active auto-generated lane connections if they are pure proposals that can be regenerated;
- R1C geometry.

If the accepted R1B API cannot export/import authored Junction state cleanly, add the smallest explicit authored-state snapshot/restore API. Do not serialize private derived fields.

## Load order

Recommended:

1. parse container/document;
2. identify schema version;
3. migrate persistence DTO to current DTO;
4. validate basic numeric/id structure;
5. construct Project;
6. construct Scenario RoadNetworks with Roads first;
7. restore authored Junction definitions;
8. regenerate/revalidate disposable R1B geometry/connectivity;
9. run Project semantic validation;
10. return canonical project;
11. R1C snapshot derives only on request.

## Canonical ordering

Define deterministic encoded ordering.

Preserve semantic order where order matters:
- Scenario user order;
- CrossSection component order;
- WidthKnot order.

For unordered/set-like collections use deterministic stable-id ordering in the persistence DTO/encoder.

Equivalent Project state should serialize to equivalent canonical JSON output under the declared ordering policy.

## Numeric rules

Reject/non-serialize non-finite engineering values.

Canonical engineering values are numeric metres, not formatted strings.

Traffic side must round-trip explicitly.

Do not infer missing engineering values silently.

## Migration harness

Current schema: v1.

Create one labelled **pre-release R2 v0 migration fixture** for test evidence.

The migration should be semantics-preserving and assumption-free.

Preferred v0 difference: a purely representational field spelling/shape that carries the same engineering information, for example:
- canonical units encoded as `"metres"` instead of v1 `"m"`;
- or another explicit non-semantic structural change.

Do not use a migration that invents LHT/RHT, widths, ids, or standards.

Required:
- v0 fixture → v1 deterministic;
- running migration twice via supported path does not drift;
- future version > current fails explicitly;
- malformed schema fails explicitly.

## Save/load APIs

At minimum prove:
- encode to bytes/string;
- decode from bytes/string.

A bounded file-path API may be included.

Do not claim crash-safe atomic replacement/autosave unless actually proven. Those remain later beta/release work.

## Round-trip acceptance

Representative non-trivial Project:
- Existing + Alternative;
- shared lineage ids across scenarios;
- roads;
- junction;
- corner override;
- manual connectivity where valid;
- variable-width/pocket profile.

Prove:
- save → load preserves canonical semantic equality;
- stable ids preserved;
- scenario lock/role preserved;
- no renderer-derived field required;
- R1C clean snapshot after load is equivalent to pre-save snapshot for each scenario;
- deleting any diagnostic/cache output has no effect.

## Negative tests

- unknown future schema;
- missing required field;
- duplicate ids;
- invalid/nonfinite number;
- malformed width profile;
- missing junction road;
- invalid manual lane connection;
- corrupted enum value;
- truncated JSON;
- renderer-cache-like extra data must not become canonical state.

## Success gates

B-G0 accepted R2A base/workflow.
B-G1 schemaVersion/current DTO explicit.
B-G2 stable ids survive round trip.
B-G3 R1 semantic road/lifecycle state survives.
B-G4 authored Junction state survives; derived geometry reconstructed rather than deserialized.
B-G5 R1C clean rebuild equivalent before/after load.
B-G6 canonical deterministic encoding/order.
B-G7 migration harness deterministic and assumption-free.
B-G8 future/malformed inputs fail clearly with no partial state.
B-G9 dependency/license/build matrix acceptable.
B-G10 no command/UI/container/autosave/R3 scope creep.

## Evidence

Create:
`docs/development/R2B_PERSISTENCE_MIGRATION_EVIDENCE.md`

Final recommendation:
- `PROCEED_TO_R2C`
- `REMEDIATE_R2B`
- `ARCHITECTURE_ESCALATION`

Do not start R2C automatically.
