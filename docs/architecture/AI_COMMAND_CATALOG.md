# AI / Automation Semantic Command Catalog

## Purpose

Define product-level typed intentions that manual UI, future AI Copilot, scripts, and import helpers can converge on. This catalog is architectural/product guidance, not a final programming API.

AI must never mutate renderer state or project JSON directly.

## Command proposal lifecycle

```text
natural-language intent / manual UI action
→ resolve semantic target(s)
→ construct typed command parameters
→ validate command coherence
→ build candidate semantic state
→ show human-readable proposal/diff
→ Preview
→ Apply / Cancel
→ committed semantic transaction
→ history / undo / redo
```

For purely manual, low-risk edits, preview may be implicit in normal direct manipulation, but the committed mutation still uses the same semantic path.

---

# 1. Project / scenario commands

## CreateProject

Parameters:
- name;
- unit system;
- traffic side;
- optional coordinate/reference context;
- optional default standard profile.

Ambiguities requiring resolution:
- none if defaults are explicit.

## AddScenario

Parameters:
- project id;
- name;
- source scenario optional.

## DuplicateScenario

Parameters:
- source scenario id;
- new name.

AI examples:
- “ทำ Alternative B จาก A”
- “คัดลอก Existing เป็น Proposed 1”

## RenameScenario

Parameters:
- scenario id;
- new name.

## SetScenarioLock

Parameters:
- scenario id;
- locked boolean.

Use case:
protect Existing from accidental modification.

---

# 2. Reference / map commands

## AddReferenceImage

Parameters:
- file reference;
- display name;
- source/provenance metadata.

## CalibrateReferenceImage

Parameters:
- reference id;
- point A in image/reference space;
- point B;
- known distance in metres;
- optional north/rotation.

Hard rule:
calibration does not silently scale already-authored engineering geometry.

## SetReferenceDisplay

Parameters:
- opacity;
- dim amount;
- saturation;
- contrast;
- visibility;
- lock.

## SetProjectLocation

Parameters:
- coordinate/location context.

## AddBasemapReference

Parameters:
- provider id;
- source/layer;
- location/view context.

Precondition:
provider capability policy permits intended use.

---

# 3. Alignment / road commands

## CreateRoad

Parameters:
- scenario id;
- alignment definition;
- road configuration id or explicit components;
- name optional.

AI examples:
- “วาดถนนตามแนวนี้เป็นถนน 4 ช่องจราจรมีเกาะกลาง”
- future sketch/selection-assisted requests.

Ambiguity:
AI must not invent alignment location from prose alone without map/canvas context.

## EditAlignment

Parameters:
- road id;
- explicit geometry edit;
- affected primitive/control point;
- new position/curve parameter.

## InsertAlignmentPoint

Parameters:
- road id;
- station/segment target;
- point.

## RemoveAlignmentPoint

Parameters:
- road id;
- control point id.

## SplitRoad

Parameters:
- road id;
- split station.

## JoinRoads

Parameters:
- road ids;
- endpoint relationship.

Precondition:
semantic/geometry compatibility validated.

---

# 4. Cross-section commands

## ApplyRoadConfiguration

Parameters:
- road id;
- configuration id;
- target station range if not full road;
- replacement/merge strategy;
- preview required for destructive replacement.

AI:
“เปลี่ยนช่วงนี้เป็นถนน 6 ช่องจราจร”

Must expose what existing components will be changed/removed.

## AddCrossSectionComponent

Parameters:
- road id;
- component type;
- insertion position;
- width;
- direction/type metadata;
- station range/lifecycle if applicable.

## RemoveCrossSectionComponent

Parameters:
- component id;
- affected station range.

## ReorderCrossSectionComponent

Parameters:
- component id;
- new semantic position.

Invalid if ordering would violate component rules.

## SetComponentWidth

Parameters:
- component id;
- width;
- station or station range/profile point.

## SetWidthProfile

Parameters:
- component/lane id;
- ordered profile points;
- interpolation type.

Starter interpolation:
piecewise linear unless later architecture explicitly changes it.

---

# 5. Longitudinal road-feature commands

## AddLaneTransition

Parameters:
- road id;
- lane/component target;
- start station;
- end station;
- start width;
- end width.

General primitive for widening/narrowing.

## AddTurnPocket

Parameters:
- road/approach id;
- movement: left/right/U-turn where semantically supported;
- side/direction;
- width;
- taper length;
- storage length;
- anchor: junction/access/station;
- optional offset/advanced stations.

AI examples:
- “เพิ่มช่องเลี้ยวขวากว้าง 3 เมตร taper 30 เมตร storage 50 เมตรก่อนทางเข้า”
- “ทำช่องเลี้ยวซ้ายยาวเก็บรถ 40 เมตร”

Required proposal:
- target road/approach;
- movement;
- width;
- taper;
- storage;
- derived stations.

Hard rule:
uses general lane lifecycle, never a hand-drawn pocket polygon.

## RemoveTurnPocket

Parameters:
- lane/feature id.

## AddMedianOpening

Parameters:
- road/median id;
- center/anchor station;
- opening range/geometry parameters;
- movement permissions.

## EditMedianOpening

Parameters:
- opening id;
- changed semantic parameters.

---

# 6. Access / junction commands

## AddProjectAccess

Parameters:
- parent/context road;
- access alignment or connection point;
- access road/cross-section configuration;
- connection side;
- name optional.

May produce Candidate Junction rather than immediately creating topology.

## CreateJunctionFromCandidate

Parameters:
- candidate id;
- selected connection intent;
- optional initial corner/default configuration.

Proposal may include:
- approach cut stations;
- initial corners;
- lane-connection candidates;
- derived pavement surface.

## IgnoreJunctionCandidate

Parameters:
- candidate id;
- reason optional.

## MarkGradeSeparated

Parameters:
- candidate id;
- relationship metadata.

## SetCornerRadius

Parameters:
- junction id;
- corner id;
- radius.

AI:
“มุมตะวันออกเฉียงเหนือปรับ radius เป็น 12 เมตร”

Must resolve named corner to stable id.

## SetCornerGeometry

Parameters:
- corner id;
- geometry mode;
- mode-specific parameters.

Starter:
- Auto
- Circular radius
- Custom later

## AddLaneConnection

Parameters:
- junction id;
- from lane id;
- to lane id;
- movement category;
- optional path-control parameters.

## RemoveLaneConnection

Parameters:
- connection id.

## SetAllowedMovement

Parameters:
- lane/approach id;
- movement set.

## AddTrafficIsland

Parameters:
- junction/approach context;
- semantic placement/shape parameters.

Do not default to arbitrary freehand polygon if a semantic splitter/channelizing island generator can represent intent.

---

# 7. Marking commands

## AddLaneMarking

Parameters:
- target lane/boundary/road;
- marking type;
- station range;
- profile/style.

## AddStopLine

Parameters:
- approach/lane group;
- station/offset;
- width/profile.

## AddCrosswalk

Parameters:
- approach/junction;
- crossing location;
- width;
- profile.

## AddArrowMarking

Parameters:
- lane id;
- arrow type;
- station;
- profile.

## AddHatchedArea

Parameters:
- semantic target/boundary;
- hatch parameters;
- profile.

AI must not claim a marking is standard-compliant if source profile is unverified.

---

# 8. Asset commands

## PlaceAsset

Parameters:
- asset id;
- scenario;
- attachment: world point or semantic road-relative target;
- orientation;
- representation options.

AI:
“วางต้นไม้ตรงนี้”
“ใส่รถบัสหนึ่งคันตรงทางเข้า”

## PlaceAssetsAlongEdge

Parameters:
- asset id;
- reference edge/path;
- start/end station;
- spacing;
- offset;
- side/alternation;
- deterministic seed/options.

AI:
“วางไฟถนนทุก 25 เมตรตามขอบทางด้านซ้าย”

## DistributeAssetsInArea

Parameters:
- asset id;
- area;
- density/spacing;
- deterministic seed;
- exclusions.

## BakeAssetDistribution

Parameters:
- distribution id.

Result:
individual semantic props while preserving provenance.

## ReplaceAsset

Parameters:
- selected asset(s);
- replacement asset id;
- retain attachment/transform policy.

---

# 9. Layer / display commands

These mainly affect persistent presentation state, not engineering semantics.

## SetLayerVisibility
## SetLayerLock
## SetLayerOrder
## SetEngineeringPresentationMode
## SaveCameraView
## FitSelection

AI examples:
- “ซ่อน existing buildings”
- “dim แผนที่ลง 40%”
- “บันทึกมุมนี้ชื่อ Site Entrance”

Display commands must never alter engineering dimensions.

---

# 10. Validation / explain commands

## ExplainSelection

Read-only:
summarize selected semantic object, dimensions, relationships, provenance, and active warnings.

## ExplainIssue

Read-only:
- why warning/error exists;
- affected object;
- source/rule;
- possible correction options;
- distinction between geometry validity and advisory standard guidance.

## ProposeFix

Produces candidate command(s); never directly applies unless explicitly confirmed according to product policy.

## KeepDesignWithOverride

Parameters:
- issue/rule id;
- object id;
- rationale;
- actor metadata.

Allowed only for advisory rules that permit override.

---

# 11. Comparison / analysis commands

## CompareScenarios

Read-only result:
- semantic changes;
- geometry/width changes;
- asset changes;
- validation difference later;
- optional quantitative metrics later.

## SummarizeAlternative

Read-only:
human-readable design changes and assumptions.

No traffic-performance claims unless a validated analysis module provides them.

---

# 12. Export commands

## ExportPlan

Parameters:
- scenario/view;
- output format;
- scale/resolution;
- reference inclusion;
- attribution mode;
- engineering/presentation style.

Must enforce basemap/reference rights.

## ExportView3D

Later:
- camera view;
- output type;
- presentation mode.

## ExportModel

Later:
- GLB/other format;
- semantic-to-export selection;
- excludes canonical semantics unless format supports them.

---

# 13. AI ambiguity classes

AI should explicitly detect these ambiguity categories.

## Target ambiguity
“ถนนนี้”, “มุมนี้”, “ช่องทางซ้าย”
Resolution:
- active selection;
- hover/context;
- ask user if more than one plausible target.

## Direction ambiguity
“ซ้าย/ขวา”
May mean:
- screen left/right;
- travel direction;
- geometric side;
- turning movement.

AI must resolve against road direction/LHT semantics, not assume screen orientation.

## Station/location ambiguity
“ก่อนทางเข้า”
Needs:
- target access;
- upstream direction;
- anchor station.

## Engineering-value ambiguity
“ทำ taper ให้เหมาะสม”
AI must not fabricate a standard value unless a verified rule/profile can justify it.
Safer actions:
- ask for value;
- propose a clearly labeled user-adjustable concept assumption;
- cite profile if verified.

## Scope ambiguity
“ทำทางแยกให้ดีขึ้น”
Too broad for a direct command.
AI should decompose/propose alternatives rather than silently redesign the junction.

---

# 14. AI authority levels

## Level 0 — Explain/read
No mutation.

## Level 1 — Fill/transform explicit request
User supplies target and values.
Example:
“set radius to 12 m.”

## Level 2 — Generate deterministic semantic feature
User supplies design intent and key values.
Example:
turn pocket with explicit taper/storage.

## Level 3 — Propose design options
AI may propose alternatives with stated assumptions.
Requires user review before any commit.

## Level 4 — Autonomous engineering design
Not planned as normal product behavior.

---

# 15. Proposal UI requirements

Every non-trivial AI mutation proposal should expose:
- command summary;
- target object(s);
- changed parameters;
- generated/deleted objects;
- assumptions;
- standards source if invoked;
- warnings/validation changes;
- Preview / Apply / Cancel.

For a multi-command alternative:
- group into one human-level transaction when appropriate;
- allow inspecting constituent changes;
- do not hide destructive changes.

---

# 16. Audit/provenance

Applied AI transactions may record:
- actor/source = AI-assisted;
- original user intent text optionally;
- typed resolved parameters;
- model/service metadata only if useful and privacy/product policy permits;
- standard/profile references used.

Project semantics must remain usable without replaying the LLM conversation.

---

# 17. Testing principle

Equivalent typed commands created manually and through AI must produce equivalent deterministic semantic outcomes.

AI testing focuses on:
- target resolution;
- ambiguity handling;
- typed command correctness;
- preview/cancel/apply behavior;
- no bypass of domain validation;
- no fabricated standards claims.

The geometry kernel is tested independently of natural-language parsing.
