# Street Concept Designer

Clean-slate project for a map-first, engineering-aware street and intersection concept design environment.

## Product thesis

**Engineer in 2D. Understand in 3D.**

Street Concept Designer is intended to let traffic engineers and transport planners create real-world street, access, and intersection concepts on top of maps, aerial imagery, and site plans without requiring CAD drafting or 3D-modelling skills.

The application is planned as a standalone desktop product with:

- 2D plan view as the primary engineering authoring surface;
- synchronized live 3D generated from the same semantic model;
- reference-alignment and station-based road geometry;
- component-based cross sections;
- native tapers, lane additions/drops, turn pockets, median treatments, and access geometry;
- first-class junction topology and lane connectivity;
- Existing / Alternative scenario workflows;
- procedural and semantic engineering assets;
- Thailand/LHT-first standards profiles with source provenance;
- future natural-language authoring through typed, previewable, undoable commands.

## Clean-slate rule

This repository must not inherit source code, schemas, tests, or architecture from the legacy `road-concept-builder` prototype by default. Legacy material may be consulted only for lessons learned. Any reuse requires an explicit independent justification.

## Product boundary

This is a rapid concept-design environment, not a Civil 3D/OpenRoads replacement. Detailed grading, drainage, earthworks, BIM, traffic microsimulation, signal optimization, and construction-document production are outside the initial product boundary.

## Authoritative project documents

Read in this order:

1. `ENGINEERING_CONSTITUTION.md`
2. `AGENTS.md`
3. `docs/product/PRODUCT_VISION.md`
4. `docs/product/GOLDEN_WORKFLOWS.md`
5. `docs/ux/UX_ARCHITECTURE.md`
6. `docs/architecture/SEMANTIC_MODEL.md`
7. `docs/architecture/TECHNICAL_ARCHITECTURE.md`
8. `docs/assets/ASSET_SYSTEM.md`
9. `docs/standards/STANDARDS_POLICY.md`
10. `docs/development/AI_OPERATING_MODEL.md`
11. current specification under `specs/`

## Current status

Pre-implementation foundation. No production architecture is considered qualified until the geometry/semantic spike passes its acceptance gate.
