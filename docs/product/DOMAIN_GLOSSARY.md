# Domain Glossary

## Purpose

Provide stable product/domain language before implementation. Code/type names may vary only with good reason; semantic meaning should remain consistent.

## Project

Top-level saved design document containing:
- metadata;
- coordinate/reference context;
- standards-profile pins;
- scenarios;
- presentation/project display state.

A project is not a renderer scene.

## Scenario

One design alternative within a project.

Examples:
- Existing
- Alternative A
- Alternative B

Scenario contains scenario-specific engineering state while sharing project context according to the persistence implementation.

Do not use “Layer” as a synonym for Scenario.

## Network

Semantic road/junction/topology state within a scenario.

A network is not merely a set of visible line graphics.

## Road

A semantic longitudinal facility defined primarily by:
- stable id;
- reference alignment;
- station range;
- cross-section components/lifecycles;
- attachments/features.

Road does not mean pavement polygon.

## Reference Alignment

Canonical longitudinal geometric reference used for road stationing and lateral construction.

Initial primitives:
- line;
- circular arc;
- smooth conceptual curve.

The alignment is not necessarily a lane centerline and is not a renderer path.

## Station / Stationing

Distance coordinate measured along a road/reference alignment.

Canonical unit:
metres.

Display may use engineering notation such as 0+120, but storage/API semantics should remain unambiguous numeric metres unless a later decision changes it.

## Lateral offset

Signed/defined distance perpendicular/normal to the alignment at a station.

Sign convention must be explicitly defined by kernel architecture; never infer left/right from screen coordinates.

## Cross Section

Ordered set of semantic road components at a given station/range.

Examples:
- traffic lane;
- median;
- shoulder;
- curb/gutter;
- sidewalk;
- verge;
- bike lane;
- parking lane.

Cross section is contextual longitudinal state, not a single immutable template for an entire road.

## Component

Generic semantic cross-section element.

A Lane is one kind of component; a Median/Sidewalk may be other component types.

Do not create unrelated geometry types for every visual strip if they share component lifecycle behavior.

## Lane

Traffic-bearing semantic component with attributes such as:
- id;
- width profile;
- direction;
- lane type;
- movement permissions;
- lifecycle along station.

Lane identity persists even if its width varies.

## Lane lifecycle

Longitudinal existence/width behavior of a lane/component.

Examples:
- constant width;
- widen;
- narrow;
- zero → full width (lane add);
- full → zero (lane drop);
- turn pocket.

Turn pockets should use the general lifecycle mechanism.

## Width profile

Ordered station/value definition for component width.

Initial R1 interpolation:
piecewise linear.

## Road configuration / preset

Reusable generator recipe that creates an ordered semantic cross section.

After application:
components are editable semantic objects.

Preset is not an opaque graphic and not permanent source-of-truth.

## Feature

A semantic road/network treatment that is not best represented as a standalone generic prop.

Examples:
- median opening;
- turn pocket command/result;
- bus bay;
- slip-lane treatment;
- project access relationship.

Avoid using Feature as a catch-all for every object.

## Longitudinal feature

A feature whose behavior is defined over road station/range, often by component lifecycle.

Examples:
- widening;
- lane add/drop;
- turn pocket;
- bus bay.

## Turn pocket

A dedicated turning lane region generated through a lane lifecycle.

Typical conceptual parameters:
- movement;
- width;
- taper length;
- storage length;
- anchor/approach.

Do not define it as an arbitrary polygon.

## Taper

Longitudinal transition between widths/lateral states.

A taper can be a general geometric/profile concept, not a special visual asset.

## Storage length

Full-width longitudinal length of a turn/storage lane as defined by the product’s semantic feature.

Exact engineering interpretation/rules may be profile-specific.

## Median

Cross-section component separating traffic/space as defined by road configuration.

Possible presentation/forms:
- raised;
- flush;
- other future types.

## Median opening

Semantic discontinuity/opening in a median over a station range, potentially linked to movement/topology.

Not merely a transparent hole cut in a polygon.

## Access / Project Access / Driveway

Connection from the primary road/network to a site/project/local facility.

In normal product UX, Project Access is a first-class task/feature.

Its geometry may include an access road plus a junction/connection relationship.

## Junction

First-class semantic connection object between roads/approaches.

Contains/relates:
- approaches;
- cut/connection stations;
- corners;
- pavement surface derivation;
- lane connections;
- movements;
- islands/crossings/stop lines.

A junction is not just the union of road polygons.

## Candidate Junction

Non-topological proposal created when geometry suggests roads may connect.

Requires explicit user resolution:
- Create Junction;
- Grade Separated;
- Ignore.

## Approach

Road/lane context entering/leaving a junction at a connection/cut station.

Approach is a junction-relative role, not a separate road type.

## Corner

Local junction boundary geometry between approaches.

Each corner has independent semantic identity and geometry.

## Lane connection

Explicit semantic relationship:
from lane → to lane through a junction/network connection.

May have:
- movement category;
- derived/path geometry;
- provenance/manual override.

## Movement

Traffic movement intent/category.

Initial categories may include:
- left;
- through;
- right;
- U-turn;
- custom/other later.

Movement direction must be road/travel-direction aware, never screen-left/right based.

## Island

Semantic channelizing/refuge/traffic island associated with road/junction geometry.

Some islands may be generated; exact types can expand later.

## Marking

Engineering/presentation road-surface marking with semantic/parametric definition.

Examples:
- lane line;
- stop line;
- crosswalk;
- arrow;
- hatch.

A standard marking is not merely a PNG.

## Generated marking

Marking created from a road/junction/profile generator and linked to source/provenance.

User override status must be preserved.

## Custom marking

User/project concept marking without claim that it comes from a standards profile.

## Asset

Reusable semantic library definition that can have one or more representations.

Examples:
- vehicle;
- tree;
- sign assembly;
- signal assembly;
- streetlight.

Asset definition is distinct from placed Asset Instance.

## Asset Instance / Prop

Placed occurrence of an asset in a scenario.

May attach by:
- world coordinates;
- road station/lateral offset;
- edge/path/area distribution.

## Semantic assembly

Asset generated/organized from meaningful parts.

Examples:
- sign face + support;
- traffic signal pole + heads;
- streetlight pole + arm + luminaire.

## Distribution

Procedural placement rule for repeated asset instances.

Examples:
- lights every 25 m;
- bollards every 1.5 m;
- trees along sidewalk;
- vegetation in area.

A distribution uses deterministic rules/seed when persistence requires repeatability.

## Bake / Explode Distribution

Convert a procedural distribution into individual semantic instances for exception editing.

Provenance should remain.

## Reference Layer

Non-canonical context used to locate/understand the design.

Examples:
- aerial image;
- site plan;
- basemap;
- survey drawing;
- GIS layer.

Reference layer does not become engineering truth merely by being visible.

## Basemap Provider

External/local data/service source under specific technical/license/capability rules.

Map renderer and provider are separate concepts.

## Reference quality

Recorded confidence/provenance class for a reference:
- unscaled;
- calibrated;
- georeferenced;
- project reference;
- survey/authoritative.

This is separate from mathematical precision of authored geometry.

## Layer

Display/organization construct controlling visibility/lock/order/grouping.

Layer is not automatically a Scenario or semantic parent relation.

## Presentation state

Visual properties not defining engineering geometry.

Examples:
- materials;
- camera;
- layer visibility;
- reference dimming;
- presentation asset variation.

## Engineering state

Canonical semantic content that defines the design and must survive renderer/cache replacement.

## Renderer DTO / Derived geometry

Disposable/reproducible representation used by 2D/3D renderer.

Examples:
- polyline buffer;
- polygon triangles;
- mesh vertices;
- selection render ids.

Must be derivable from semantic state.

## Command

Typed semantic mutation intent.

Example:
SetComponentWidth.

## Transaction

One human-level committed operation potentially containing several semantic changes.

Example:
AddRightTurnPocket may create/update multiple related objects but undo as one operation.

## Preview / Candidate state

Temporary calculated state shown before committing a transaction.

Must not silently become canonical state.

## Validation — internal geometry

Determines whether semantic/geometry state is coherent enough to exist.

Examples:
- finite values;
- non-negative widths;
- valid stations;
- non-self-intersecting required surfaces.

## Validation — engineering advisory

Checks a coherent design against selected guidance/standards/profile.

Normally:
warning/advisory, with provenance and override policy.

Do not confuse this with internal invalidity.

## StandardProfile

Versioned set of source-backed rule/asset defaults for a particular authority/context/edition.

Examples:
- Thailand / DOH / <edition>
- Thailand / DRR / <edition>
- Generic
- Project Custom

## Provenance

Machine-readable origin/authority/source information for generated rules/assets/reference data.

## Override

Intentional user deviation from generated/profile default or advisory guidance.

Must not be silently erased by regeneration.

## Existing

Scenario representing current/reference design state when modeled semantically.

May be locked for protection.

## Alternative

Scenario representing proposed design variant.

## Engineering 3D

Low-noise 3D representation optimized for inspecting road/junction geometry and markings.

## Presentation 3D

Richer materials/assets/environment generated from same engineering state for communication.

## AI Copilot

Intent-to-command assistant.

It may:
- explain;
- resolve target;
- propose typed commands;
- preview alternatives.

It must not bypass the semantic transaction/validation system.

---

# Terms to avoid or use carefully

## “AutoCAD-like”
Avoid as a product goal. Individual precision interactions may be familiar, but the product intentionally reduces CAD workflow burden.

## “Standard”
Never use without source/profile context for an engineering value.

Prefer:
- Generic default
- Recommended by <profile/source>
- Project custom

## “Accurate”
Distinguish:
- exact authored dimension;
- geometric computation precision;
- reference/source positional accuracy;
- standard compliance.

## “Connected”
Clarify whether it means:
- geometry touches/intersects;
- topology exists;
- lane connection exists.

## “Left / Right”
Always clarify semantic frame:
- road side;
- travel direction;
- turning movement;
- UI/screen position.

Code/domain logic must not rely on screen left/right for traffic semantics.
