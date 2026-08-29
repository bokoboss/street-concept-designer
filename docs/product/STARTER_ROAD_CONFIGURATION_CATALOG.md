# Starter Road Configuration Catalog

## Purpose

Define the reusable configurations/presets the product should offer without turning presets into source-of-truth or pretending unverified dimensions are authoritative standards.

A configuration is a **generator recipe**:
- generates ordered semantic components;
- may provide suggested values;
- generated components are independently editable;
- can be applied to full road or station range where supported;
- configuration identity may be retained as provenance/convenience;
- manual changes never need to preserve the original preset shape.

## Standards rule

Until a specific authority/profile is page-verified:
- configuration structure may ship as GENERIC;
- exact numeric defaults must be clearly generic/user-editable, not labeled DOH/DRR standard;
- profile-specific presets are source-gated.

---

# A. Core generic road configurations

## RC-01 — Two-Lane Undivided

Semantic composition:

```text
Roadside | Lane → | ← Lane | Roadside
```

Variants:
- no shoulder;
- shoulder;
- sidewalk/verge;
- urban curb version.

Primary uses:
- side roads;
- project accesses;
- local/rural road concepts.

Priority: P0.

---

## RC-02 — Four-Lane Undivided

```text
Roadside | → | → | ← | ← | Roadside
```

Optional:
- center line/flush center treatment;
- roadside shoulder/curb;
- sidewalk/verge.

Priority: P1.

---

## RC-03 — Four-Lane Divided

```text
Roadside | → | → | MEDIAN | ← | ← | Roadside
```

Optional roadside components:
- shoulder;
- curb/gutter;
- verge;
- sidewalk;
- bicycle facility.

Primary use:
GW-01 project access improvement.

Priority: P0.

---

## RC-04 — Six-Lane Divided

```text
Roadside | → | → | → | MEDIAN | ← | ← | ← | Roadside
```

Primary use:
road widening alternatives.

Priority: P0/P1.

---

## RC-05 — Urban Local Street

```text
Sidewalk/Verge | Parking? | → | ← | Parking? | Sidewalk/Verge
```

Optional:
- bike facility;
- planting;
- curb.

Priority: P1.

---

## RC-06 — Urban Complete Street

Base composition example:

```text
SW | Verge | Bike | Parking | → | ← | Parking | Bike | Verge | SW
```

All optional/parameterized.

Purpose:
fast streetscape/urban concept communication, not a universal standard.

Priority: P1.

---

## RC-07 — Project Access / Driveway

```text
Roadside | Inbound lane(s) | Outbound lane(s) | Roadside
```

Options:
- one-in/one-out;
- two-in/one-out;
- one-in/two-out;
- median/island;
- gate/guardhouse later as context asset.

Priority: P0.

Important:
access semantics include connection to parent road/junction; it is not merely a small road graphic.

---

## RC-08 — One-Way Street

```text
Sidewalk/Edge | → [→ ...] | Edge/Sidewalk
```

Optional:
- parking;
- bike lane;
- bus lane.

Priority: P1.

---

# B. Component families available to configurations

Road configurations compose from:
- TrafficLane
- Median
- Shoulder
- Curb/Gutter
- Sidewalk
- Verge
- BicycleLane
- ParkingLane
- TransitLane
- Barrier
- Drain/Channel later if product scope justifies
- CustomSemanticComponent later

Each component has:
- id;
- type;
- side/order;
- width/lifecycle;
- direction where relevant;
- material/presentation metadata;
- standards/profile provenance where relevant.

---

# C. Longitudinal feature generators

These are not presets; they transform/add semantic lifecycles.

## GF-01 — Lane Widening / Narrowing

Inputs:
- target lane/component;
- start/end stations;
- start/end widths.

Output:
piecewise-linear width profile.

Priority: P0.

---

## GF-02 — Lane Add

Inputs:
- parent road;
- side/direction;
- start/taper station;
- full-width station;
- target width;
- downstream range/end.

Output:
new lane identity whose lifecycle emerges zero → full width.

Priority: P0.

---

## GF-03 — Lane Drop

Inverse lifecycle:
full width → zero.

Priority: P0.

---

## GF-04 — Turn Pocket

Inputs:
- movement;
- target road/approach;
- width;
- taper;
- storage;
- anchor.

Output:
general lane lifecycle attached to approach/road.

Priority: P0.

Variants:
- right turn;
- left turn;
- U-turn later.

---

## GF-05 — Median Width Transition

Inputs:
- median id;
- stations;
- start/end width.

Priority: P0/P1.

---

## GF-06 — Median Opening

Inputs:
- median/road;
- anchor station;
- opening range;
- movement permissions.

Output:
semantic median discontinuity/feature with connectivity consequences when linked to junction/U-turn semantics.

Priority: P0.

---

## GF-07 — Bus Bay

After core lifecycle is qualified.

Inputs:
- side;
- bay width;
- entry transition;
- stopping length;
- exit transition;
- station/anchor.

Output:
roadside/traffic-lane lifecycle using general component transition logic.

Priority: P1/NEXT.

---

## GF-08 — Slip Lane

After junction kernel qualification.

Inputs:
- source approach;
- target exit;
- width;
- corner/channelizing geometry;
- island relationship.

Output:
semantic road/lane connection integrated with junction topology.

Priority: P1/NEXT.

---

# D. Junction generator catalog

## JG-01 — Project Access Junction

Input:
- main road;
- access road/driveway;
- connection location;
- initial corner modes.

Output candidate:
- approach/cut stations;
- two access-side corners;
- lane-connection proposal;
- optional median relationship.

Priority: P0.

---

## JG-02 — T Junction

Input:
- continuing road;
- terminating road;
- geometric crossing/endpoint relationship.

Output:
- 3 approaches;
- corners;
- pavement surface;
- lane connection proposal.

Priority: P0.

---

## JG-03 — Four-Leg Junction

Output:
- 4 approaches;
- 4 independent corners;
- lane connection proposal.

Priority: P0.

---

## JG-04 — Skew Junction

Not a separate semantic class if the general T/X generator supports arbitrary approach angle.

Qualification:
must include skewed canonical/adversarial fixtures.

Priority: P0 as capability.

---

## JG-05 — Divided × Undivided

Must support:
- median interaction;
- approach offsets;
- independent carriageway/lane relationships if model requires.

Priority: P0/P1.

---

## JG-06 — Divided × Divided

Higher complexity:
- medians on both roads;
- wider junction footprint;
- movement/median interactions.

Priority: P1 after base T/X.

---

## JG-07 — Roundabout

Dedicated later module.

Must not be approximated as a giant circular graphic template.

Future semantic needs likely include:
- central island;
- circulatory lanes;
- entries/exits;
- splitter islands;
- entry/exit geometry;
- lane connections.

Priority: DEFERRED/NEXT after core junction qualification.

---

# E. Roadside configuration snippets

Instead of multiplying whole-road presets combinatorially, allow reusable component snippets:

- Urban Curb + Sidewalk
- Shoulder + Verge
- Bike Lane + Buffer
- Parking Lane
- Planting Verge
- Barrier Edge
- Transit Lane

User can add a snippet to an existing road configuration and edit the generated components.

This avoids a preset library explosion.

---

# F. Saved/custom configurations

Later user workflow:
1. select road cross section;
2. Save Configuration;
3. name it;
4. choose scope:
   - project;
   - user library;
   - organization library later;
5. reuse in new road.

Saved configuration stores semantic component recipe, not renderer paths.

---

# G. Configuration preview

Before destructive application to a non-empty road:
- show current vs proposed cross-section;
- state what components will be added/removed/replaced;
- warn about longitudinal features/manual overrides affected;
- Apply / Cancel.

Simple initial road creation may apply configuration immediately with normal undo.

---

# H. Configuration naming

Prefer descriptive names:
- “4-Lane Divided — Generic”
- “Urban 2-Lane + Parking — Generic”
- “Project Access 1-In / 1-Out”

Profile-specific:
- “DOH … — <edition/profile>” only after verified source-backed configuration exists.

Never name a generic concept preset “Thai Standard 4-Lane Road”.

---

# I. Acceptance criteria

A configuration system is successful if:
- a first-time user can create common road structures quickly;
- irregular roads can diverge from presets without fighting the tool;
- longitudinal features do not require new whole-road preset variants;
- rendering is regenerated from semantic components;
- preset application is undoable;
- saved configurations remain independent of viewport/renderer state;
- profile-specific values expose provenance.
