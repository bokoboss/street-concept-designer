# Low-Fidelity Workspace Specification

## Purpose

Define the editor structure and interaction hierarchy before production UI work. This is not a visual mockup or final component library; it is a product-behavior specification for later implementation and UAT.

## UX principles

- 2D plan is the primary engineering authoring view.
- 3D is synchronized and derived from the same semantic model.
- The viewport dominates the application.
- Contextual actions are preferred over a large permanent CAD-style toolbar.
- Direct manipulation and exact numeric input edit the same semantic state.
- Reference layers are visibly subordinate to engineering geometry.
- Current tool/navigation mode is always obvious.
- The normal workflow must not require CAD, Illustrator, Blender, GIS, or programming expertise.
- Advanced station/topology/standards detail is available progressively, not forced into first-time workflows.

---

## S00 — Welcome / Project Hub

Primary actions:
- New Project
- Open Project
- Recent Projects

Do not require login/cloud account.

A recent-project card may show:
- project name;
- last modified time;
- location/reference thumbnail if available;
- schema/application version;
- recovery/autosave status later.

---

## S01 — New Project

```text
┌──────────────────────────────────────────────────────┐
│ New Project                                          │
├──────────────────────────────────────────────────────┤
│ Start from                                           │
│                                                      │
│ [ Map / Aerial ] [ Import Image / Site Plan ]        │
│ [ Blank Canvas ]                                     │
│                                                      │
│ Project name        __________________________        │
│ Units               Metric                           │
│ Traffic side        ● Left   ○ Right                 │
│ Location            Optional                         │
│                                                      │
│ Advanced ▸ CRS / origin                              │
│                                      [Create Project]│
└──────────────────────────────────────────────────────┘
```

Rules:
- Thailand/LHT + metric are sensible defaults, not hard-coded global assumptions.
- Do not force CRS selection for a simple calibrated image project.
- Online basemap choices are filtered by provider capability/policy.

---

## S02 — Image Calibration

```text
┌──────────────────────────────────────────────────────┐
│ Calibrate Reference Image                            │
├──────────────────────────────────────────────────────┤
│ 1. Click point A                                     │
│ 2. Click point B                                     │
│ 3. Known distance: [ 25.000 ] m                      │
│                                                      │
│ Optional                                             │
│ Rotation / North: [ Set ]                            │
│                                                      │
│ Status: Calibrated                                   │
│ Estimated source quality: User reference / unknown   │
│                                      [Apply] [Cancel] │
└──────────────────────────────────────────────────────┘
```

After calibration:
- reference layer is locked by default;
- scale state is visible;
- user can later recalibrate explicitly;
- recalibration must not silently distort already-authored engineering geometry.

---

## S03 — Main Workspace

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Project ▼   Existing ▼   Undo Redo     2D | Split | 3D        Export       │
├──────────────┬──────────────────────────────────────────────┬────────────────┤
│              │                                              │                │
│ LAYERS       │                                              │ PROPERTIES     │
│ MAP          │                                              │                │
│ LIBRARY      │                MAIN VIEWPORT                 │ Selected:      │
│              │                                              │ Road R-01      │
│              │         map / aerial / site plan             │                │
│              │          + engineering design                │ Geometry       │
│              │                                              │ Cross Section  │
│              │                                              │ Features       │
│              │                                              │ Markings       │
│              │                                              │ Validation     │
│              │                                              │                │
│              │                                              ├────────────────┤
│              │                                              │ AI COPILOT     │
├──────────────┴──────────────────────────────────────────────┴────────────────┤
│ V Select │ H Hand │ Road │ Junction │ Marking │ Measure │ Snap ▼           │
├──────────────────────────────────────────────────────────────────────────────┤
│ Existing │ Alternative A │ Alternative B │ +                               │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Top bar
Contains only project-level and view-level actions:
- project/file;
- active scenario;
- undo/redo;
- 2D/Split/3D;
- export;
- later focus/presentation mode.

Do not put road/junction micro-tools in the top bar.

### Left rail/panel
Persistent domains:
- Layers
- Map / References
- Library

The left panel is not a tool dump.

### Right inspector
Contextual categories depend on selected semantic object.

Examples:
- Road: Geometry / Cross Section / Features / Markings / Validation
- Junction: Approaches / Corners / Movements / Islands / Crossings / Validation
- Asset: Placement / Representation / Metadata
- Reference: Source / Calibration / Display / Rights

### Bottom tool strip
Small set of high-frequency modes/actions:
- Select
- Hand
- Road
- Junction
- Marking
- Measure
- Snap

Specialized actions appear contextually after selection.

### Scenario strip
Always shows active design context:
- Existing
- Alternative A
- Alternative B
- +

---

## S04 — Road Draw Mode

Entry:
- Road → Draw Road
- keyboard shortcut later

Visible state:
- active mode badge: “Draw Road”
- cursor/guide appropriate to alignment creation
- Esc cancels current unfinished road

Interaction:
1. click start point;
2. add/edit alignment geometry;
3. finish;
4. road is immediately generated using selected/default road configuration;
5. road becomes selected and inspector opens.

The editor must not require drawing left/right pavement edges manually.

---

## S05 — Road Inspector

```text
ROAD R-01

Geometry
  Alignment        [Edit]
  Length           245.30 m
  Start station    0+000
  End station      0+245.30

Cross Section
  Urban Arterial — 4 Lane Divided ▼
  [Open Cross-Section Editor]

Longitudinal Features
  Turn lanes       1
  Lane transitions 2
  Median openings  1
  [Add Feature]

Markings
  Generated        Yes
  [Review]

Validation
  1 Advisory
```

Basic mode avoids exposing raw station-profile tables unless the user opens Advanced.

---

## S06 — Cross-Section Editor

```text
Cross Section — Road R-01

 SW    BIKE      →       →      MED      ←       ←      SW
┌────┬───────┬───────┬───────┬─────┬───────┬───────┬────┐
│2.0 │ 1.5   │ 3.25  │ 3.25  │2.0  │ 3.25  │ 3.25  │2.0 │
└────┴───────┴───────┴───────┴─────┴───────┴───────┴────┘

[ + Add Component ]                         Advanced ▸
```

Interactions:
- click component → property editor;
- drag/reorder if semantic rules permit;
- numeric width edit;
- duplicate/remove where allowed;
- direction/type edit;
- preset/configuration can be applied through preview.

A preset generates editable semantic components and is never an opaque graphic.

---

## S07 — Turn Pocket Panel

```text
Add Turn Pocket

Movement          Right Turn
Target            Road R-01 / approach A
Width             [ 3.00 ] m
Taper             [ 30.0 ] m
Storage           [ 50.0 ] m
Anchor            Before Access A

Advanced ▸
  Taper start     0+120
  Full width      0+150
  End             0+200

[Preview] [Apply] [Cancel]
```

Preview:
- plan, cross section, and 3D update from candidate semantic state;
- no undo/history item until Apply.

Apply:
- one human-readable transaction;
- no manual pocket polygon drafting.

---

## S08 — Candidate Junction

Non-blocking contextual overlay:

```text
Candidate Junction
Road R-01 × Road R-02

[Create Junction] [Grade Separated] [Ignore]
```

Important:
- geometric crossing alone never changes topology;
- ignoring a candidate is explicit state/provenance where useful;
- creating a junction launches a semantic proposal, not an opaque polygon union.

---

## S09 — Junction Inspector

```text
JUNCTION J-01

Approaches        4

Corners
  NE    12.00 m
  SE     8.00 m
  SW    10.00 m
  NW    12.00 m

Lane Connections
  Auto proposal: 12
  [Review Movements]

Islands
  1
  [Add]

Crossings
  2
  [Add]

Stop / Yield
  [Add]

Validation
  2 Advisories
```

Selecting a corner exposes local geometry controls and a viewport handle.

---

## S10 — Lane Movement Review

Temporary overlay only when requested:
- from-lane → to-lane curves/arrows;
- movement class: left / through / right / U-turn / custom later;
- source lane and destination lane ids visible in inspector;
- warnings for disconnected/duplicate/impossible connections.

Do not permanently clutter plan view with all movement paths.

---

## S11 — Map / Reference Panel

```text
MAP / REFERENCE

Source
  Satellite / Aerial Provider ▼
  or Imported Site Plan

Opacity          70%
Dim Background   25%
Saturation      -30%
Contrast          5%

☑ Labels
☑ Lock Reference

Calibration
  Georeferenced / Calibrated / Unscaled

Rights
  Digitization       Allowed / Not allowed / Unknown
  Export with map    Allowed / Conditional / Not allowed
  Offline cache      Allowed / Conditional / Not allowed
```

If digitization is not permitted:
- Road/Trace authoring against that source is disabled or clearly restricted by policy.

---

## S12 — Layers / Hierarchy

```text
Reference
  👁 🔒 Aerial
  👁 🔒 Existing Site Plan

Existing
  👁 Roads
  👁 Junctions
  👁 Assets

Alternative A
  👁 Roads
  👁 Junctions
  👁 Markings
  👁 Assets

Annotations
```

Required concepts:
- visibility;
- lock;
- nested grouping;
- reorder where meaningful;
- semantic hierarchy is not forced to equal visual layer hierarchy.

---

## S13 — Asset Library

Primary categories:
- Markings
- Signs
- Signals
- Safety / Roadside
- Lighting
- Vehicles
- Landscape
- Context Buildings
- User Library later

Search is semantic:
“bus”, “crosswalk”, “warning sign”, “tree”, “guardrail”.

Each card may show:
- thumbnail;
- canonical name;
- dimensions/type;
- jurisdiction/profile badge when standards-sensitive;
- verification/provenance status.

Do not expose filesystem filenames as the main UX.

---

## S14 — Asset Placement

Point placement:
- select asset;
- click location;
- default scale/orientation comes from asset metadata.

Along-edge placement:
```text
Street Light Array

Reference       Left sidewalk edge
Start           0+020
End             0+220
Spacing         25.0 m
Offset          0.50 m
Alternate side  Off

[Preview] [Apply]
```

Area distribution:
- trees/vegetation/context;
- deterministic seed stored if regeneration must be stable.

Bake:
- converts procedural distribution to individually editable semantic props.

---

## S15 — Split 2D / 3D

```text
┌───────────────────────────────────────────────┐
│ 2D PLAN                 │ LIVE 3D             │
│                         │                     │
│ selected Lane L2        │ selected Lane L2    │
│                         │                     │
└───────────────────────────────────────────────┘
```

Requirements:
- same selected semantic id;
- same candidate/committed model;
- no separate 3D-only lane widths/geometry;
- 3D camera/navigation never changes engineering state.

Initial product direction:
- author comprehensively in 2D;
- select/inspect in 3D;
- allow only 3D direct manipulation whose semantic meaning is unambiguous.

---

## S16 — Issues / Validation

```text
ISSUES

⚠ Lane width advisory
  Road R-01 / Lane L2
  2.75 m
  Profile: DOH / ... / version
  [Zoom] [Details] [Override]

✕ Invalid geometry
  Junction J-01 / Corner NE
  Self-intersection
  [Zoom] [Fix]
```

Distinguish:
- internal geometry invalidity;
- engineering advisory;
- informational/profile note;
- presentation/reference warning.

A visually plausible design is not automatically validated engineering geometry.

---

## S17 — AI Copilot Panel

The AI panel is optional and subordinate to the editor.

Example:
```text
You:
Add a 3 m right-turn lane with 30 m taper and
50 m storage before this access.

Proposed change:
+ Turn pocket
  Road R-01
  Movement: Right
  Width: 3.00 m
  Taper: 30 m
  Storage: 50 m

[Preview] [Apply] [Cancel]
```

AI must:
- resolve stable semantic targets;
- construct typed commands;
- expose ambiguity;
- never directly edit canvas/mesh/project JSON.

---

## S18 — Alternatives

Scenario strip:
```text
Existing | Alternative A | Alternative B | +
```

Actions:
- duplicate;
- rename;
- lock Existing;
- compare overlay;
- side-by-side later;
- before/after slider later;
- 3D compare later.

Implementation storage mechanics must remain hidden from product semantics.

---

## S19 — Export

```text
Export

Output
  ○ Plan image
  ○ High-resolution raster
  ○ Vector plan
  ○ 3D view / later

Background
  ● Engineering only
  ○ Transparent
  ○ Include reference layer  [only if permitted]

Scenario
  Alternative A

View
  Overall Plan ▼

Resolution / scale
  ...

Attribution
  [automatically required when applicable]

[Export]
```

Engineering export must remain possible even if a basemap cannot legally be embedded.

---

## Focus / Presentation mode

One action hides non-essential panels while preserving current view.

Useful for:
- client review;
- screenshot/export preparation;
- large 3D presentation view.

Focus mode changes UI chrome only, never engineering state.

---

## Navigation and keyboard baseline

Initial conceptual shortcuts:
- V — Select
- H — Hand/Pan
- Esc — cancel current transient operation / leave subtool
- Delete — delete selected editable object with confirmation where high impact
- Ctrl/Cmd+Z — Undo
- Ctrl/Cmd+Shift+Z or Ctrl/Cmd+Y — Redo
- F — Fit selection/view later

Exact shortcuts are not final until UX prototype testing.

---

## First-time-user success criterion

Without prior CAD training, a traffic engineer should understand how to:
1. create/open a project;
2. import/calibrate a site reference;
3. draw a road;
4. edit its cross section;
5. add a turn pocket parametrically;
6. inspect the same object in 3D;
7. duplicate an alternative;
8. export a plan.

If a first-time user must understand internal stations, renderer layers, topology ids, mesh concepts, or project JSON for this flow, the UX has failed.

## Items intentionally not visually locked

Do not treat the following as final until a real prototype is visually tested:
- exact colors;
- exact typography/font;
- panel widths;
- icon family;
- control heights;
- dark/light default;
- final keyboard map;
- docking/customization system.
