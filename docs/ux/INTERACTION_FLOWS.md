# Detailed Interaction Flows

These flows translate Golden Workflows into concrete editor behavior. They define product intent, not final pixel layout.

## UX invariants

- 2D plan is the primary engineering authoring view.
- Split 2D/3D is always available without creating a second model.
- Direct manipulation and exact numeric properties edit the same semantic state.
- Meaningful edits become undoable commands.
- Automation proposes/generates semantic objects; manual override remains available.
- Reference imagery/map is visually subordinate and never engineering truth.
- Selection is synchronized across hierarchy, properties, 2D, and 3D.
- Standard features should not require line-by-line drafting.

---

## F01 — New project from map / aerial / site plan

### Entry
`New Project`

### Start choices
- Map / licensed aerial provider
- Import image / site plan
- Blank canvas

### Minimal setup
- project name;
- units: metric default;
- traffic side: Thailand/LHT default;
- optional location/search;
- advanced: CRS/project origin.

Do not force CRS selection for a simple image-calibrated concept.

### Imported image calibration
1. Place/import image.
2. Choose `Calibrate Scale`.
3. Click point A and point B.
4. Enter known real distance in metres.
5. Optional: define north/rotation.
6. Show calibration quality/state clearly.
7. Lock reference layer after calibration by default.

### Basemap capability behavior
If provider terms disallow digitization, tracing/drawing commands must be disabled for that provider with a clear explanation. See `docs/architecture/MAP_BASEMAP_POLICY.md`.

### Reference display controls
- visibility;
- lock;
- opacity;
- Dim Background;
- saturation/contrast;
- labels where supported.

---

## F02 — Draw a road

### Entry
Toolbar: `Road` → `Draw Road`

### Interaction
1. Click start point in 2D.
2. Continue alignment using line/arc/smooth-curve interactions.
3. Finish alignment.
4. Immediately show provisional road using default/configured road preset.
5. Properties opens Road + Cross Section.

### Default road creation
A preset generates editable components, for example:

```text
SW | → | → | MED | ← | ← | SW
```

Preset identity may be retained as provenance/convenience but must not prevent independent component editing.

### Editing
Road selection exposes:
- alignment edit;
- cross section;
- longitudinal features;
- markings;
- road-level metadata;
- advanced stationing.

Alignment handles support rapid direct manipulation. Exact parameters/stations are available in properties when meaningful.

---

## F03 — Cross-section editor

### Context
Open from selected road.

### Layout concept
A compact horizontal strip displays ordered semantic components and widths:

```text
Sidewalk | Bike | Lane → | Lane → | Median | ← Lane | ← Lane | Sidewalk
  2.0       1.5     3.25      3.25      2.0      3.25     3.25       2.0 m
```

### Supported interactions
- click component → properties;
- drag component to reorder where semantically valid;
- Add Component;
- duplicate/remove component where allowed;
- edit width numerically;
- set direction/type;
- apply another configuration/preset with preview before replacing current composition.

### Advanced
Reveal station-based width/lifecycle editing only when needed. Basic users should not see raw section breakpoints during normal constant-width road creation.

### Live views
Cross-section edits update 2D plan and 3D immediately from canonical state.

---

## F04 — Add a right-turn pocket

### Preferred entry paths
- select road → `Turn Pocket`;
- select project access/junction approach → `Add Turn Lane`;
- future AI command describing the same intent.

### Basic panel
```text
Movement       Right Turn
Width          3.00 m
Taper          30 m
Storage        50 m
Anchor         Before selected access/junction
```

### Preview behavior
Changing values previews the resulting lane lifecycle in plan/cross-section/3D without committing history on every transient drag/keystroke.

### Advanced panel
Expose derived/editable stations, for example:
- taper start;
- full-width start;
- storage end;
- connection/end station.

### Apply
One semantic command creates/updates the lane lifecycle. Undo removes the complete operation coherently.

### Prohibition
Never ask the user to draw the pocket boundary polygon manually for the normal workflow.

---

## F05 — Candidate junction

### Trigger
Two road geometries conflict/cross or endpoints are brought within connection tolerance.

### Result
Create a non-topological `Candidate Junction` state/overlay.

Prompt/contextual action:
```text
Candidate junction: Road R01 × Road R02
[Create Junction] [Grade Separated] [Ignore]
```

No network semantics change until the user chooses Create Junction.

### Create Junction
System proposes:
- approach cut/connection locations;
- initial pavement surface;
- per-corner geometry;
- lane movement/connectivity candidates;
- default markings/crossings only where justified by configuration/profile.

The proposal should be inspectable before committing if the resulting change is substantial.

---

## F06 — Refine a junction

### Selection
Selecting a junction exposes categories rather than many permanent toolbar buttons:
- Approaches
- Corners
- Lane Connections / Movements
- Islands
- Median treatments
- Crosswalks
- Stop/Yield lines

### Corner editing
Each corner is independently selectable.

Properties may include:
- corner mode: Auto / Circular / Custom later;
- radius where circular;
- tangent/handle controls;
- source/provenance if generated from a preset/rule.

Drag handles and numeric radius edit the same corner object.

### Lane movement review
Show lane-to-lane connections as temporary overlays/arrows. Allow:
- accept auto proposal;
- add/remove connection;
- set movement category;
- inspect from/to lane ids.

Movement overlays should not permanently clutter normal plan mode.

---

## F07 — Median opening / U-turn treatment

### Entry
Select divided road/median → `Median Opening` or `U-turn Treatment`.

### System behavior
- create semantic opening on the median/road station range;
- preserve road/lane identities;
- optionally create a U-turn pocket through the same lane lifecycle used by other turn pockets;
- generate relevant markings only through profile-aware generators.

### Controls
Basic:
- opening location;
- opening length/width semantics appropriate to final domain model;
- permitted movement;
- pocket enabled;
- pocket width/taper/storage.

Advanced geometry details remain hidden until needed.

---

## F08 — Add markings

### Preferred flow
1. Select lane / approach / junction / area.
2. Open contextual `Marking` action or Library.
3. Choose marking type.
4. System auto-orients and attaches it semantically.
5. Adjust offset/repeat/dimensions if needed.

### Marking attachment
Prefer semantic attachment such as road station/lateral offset/lane target rather than free world XY where the relationship is meaningful.

### Manual escape hatch
A user may place a custom concept marking manually, but it must be visibly distinguishable in metadata from profile/generated authoritative markings.

---

## F09 — Place assets

### Point placement
Choose asset → click plan position → system uses semantic orientation/scale defaults.

### Along path / edge
Example:
```text
Asset       Street Light
Reference   Left sidewalk edge
Spacing     25 m
Offset      0.50 m
Range       station 0+020 to 0+220
Alternate   optional
```

### Area distribution
Suitable for trees/landscape/context. Persist deterministic seed/rules if regenerated.

### Bake / explode
Procedural distribution can become individual semantic props for manual exceptions without losing provenance.

---

## F10 — Layers and object hierarchy

### Persistent top-level groups
Typical project:
```text
Reference
  Aerial / map
  Site plan
Existing
  Roads
  Junctions
  Assets
Alternative A
  Roads
  Junctions
  Markings
  Assets
Annotations
```

### Controls
- visibility;
- lock;
- reorder where meaningful;
- group/nesting;
- multi-select later;
- search/filter for larger projects.

Engineering hierarchy and display layers may not always be identical; UI should not force all semantic relationships into a flat layer list.

---

## F11 — Existing / Alternatives

### Scenario strip
`Existing | Alternative A | Alternative B | +`

### Duplicate
Create Alternative B from a chosen scenario using storage mechanics hidden from the product-level API.

### Compare modes
- overlay;
- side by side;
- before/after slider where meaningful;
- 3D compare later;
- change summary later.

### Protection
Existing scenario may be locked/reference-only for selected workflows to prevent accidental modification.

---

## F12 — Split 2D / 3D

### Selection sync
Click lane/corner/asset in either view:
- same semantic object selected;
- hierarchy/properties sync;
- counterpart highlights in other view.

### Editing policy
Initial production direction:
- full engineering authoring in 2D;
- 3D supports navigation, selection/inspection, and selected simple direct manipulation only when its semantics are unambiguous;
- do not invent a second 3D-only geometry state.

### 3D navigation
Separate modes/actions for selection, orbit, pan, and camera movement to reduce accidental object editing.

---

## F13 — Validation / Issues

Issues panel items include:
- severity;
- concise description;
- object/target;
- rule/source id;
- applicable standard profile/reference when relevant;
- override/status where allowed.

Selecting an issue zooms/highlights the affected object.

Warnings are advisory unless the semantic/geometry state is internally impossible.

---

## F14 — AI Copilot change proposal

Example user request:
> Add a 3.0 m right-turn lane with 30 m taper and 50 m storage before this access.

AI resolves intent to typed command parameters, not renderer geometry.

UI shows:
```text
Proposed change
+ Right-turn lane
  Road/approach: ...
  Width: 3.00 m
  Taper: 30 m
  Storage: 50 m
[Preview] [Apply] [Cancel]
```

If target or intent is ambiguous, AI identifies the ambiguity rather than guessing silently.

Applied AI commands enter the same undo/history/audit path as manual operations.

---

## F15 — Export / presentation

### Saved views
Allow named plan/camera views such as:
- Overall Plan
- Project Access
- Junction Detail
- Driver/Eye-Level View

### Export modes
- engineering geometry only;
- transparent/neutral background;
- reference map included only if provider policy allows;
- presentation 3D;
- Existing vs Proposed comparison.

### Basemap policy
Provider attribution and export restrictions are enforced by capability metadata. A restricted basemap must never prevent exporting engineering geometry without the basemap.

---

## Error/recovery principles

- avoid destructive modal flows;
- autosave/recovery later should preserve canonical project state, not renderer cache;
- invalid numeric input should remain understandable and never silently alter engineering values;
- geometry generation failure should preserve last valid model/transaction and report the failed command;
- undo/redo should operate on semantic transactions, not pixel movements disconnected from domain state.

## First usability benchmark

A first-time traffic engineer should be able to perform a simplified GW-01 concept without CAD knowledge:

1. import/calibrate a reference;
2. draw/apply a 4-lane divided road configuration;
3. add a project access;
4. add a right-turn pocket parametrically;
5. inspect live 3D;
6. create an alternative;
7. export a plan.

The exact time target should be measured during UX prototype testing rather than invented in R0.
