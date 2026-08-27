# UX Review Gate

## Purpose

Define how a screen/flow/editor change is reviewed before it becomes accepted product behavior.

A visually polished UI can still fail if it forces manual drafting, hides engineering state, creates mode confusion, or bypasses semantic commands.

## Review decision

Use:
- PASS
- PASS WITH CONDITIONS
- REVISE
- BLOCK

A major editor flow should not be accepted from screenshots alone. It must be checked against a real task.

---

# Gate 1 — Task fit

Questions:
- What Golden Workflow step does this screen/action serve?
- Can a traffic engineer understand the next action?
- Does the UI reduce manual drafting or simply move CAD complexity into another panel?
- Is the normal path shorter than the advanced/manual escape hatch?
- Is a new permanent tool justified by frequent use?

Block if:
- feature has no clear user job;
- task requires technical internals irrelevant to the user;
- implementation convenience dictates the interaction.

---

# Gate 2 — Engineering state clarity

The user should be able to tell:
- what semantic object is selected;
- which scenario is active;
- whether they are editing engineering state, presentation state, or a reference layer;
- what dimensions/parameters will change;
- whether a value is authored, generated, or standards-derived.

Block if:
- selection highlight is only renderer-local;
- property panel edits something different from canvas drag;
- a decorative object looks like authoritative engineering geometry without metadata.

---

# Gate 3 — Mode safety

Check:
- current Select/Hand/Draw/etc mode visible;
- Esc behavior consistent;
- camera navigation does not accidentally move engineering objects;
- destructive action cannot occur from an ambiguous drag;
- locked/reference layers cannot be accidentally edited.

Block if:
- navigation and editing gestures conflict;
- roads can be moved while user intended to pan/orbit;
- mode is hidden and only discoverable from cursor behavior.

---

# Gate 4 — Direct manipulation + exact input

For engineering geometry:
- common edits should be possible by intuitive handle/drag;
- exact value should also be available;
- both update the same semantic property;
- units are visible;
- temporary input/preview is distinguished from committed value.

Block if:
- user must choose between visual editing and accurate editing;
- drag updates pixels but numeric field updates separate geometry.

---

# Gate 5 — Progressive disclosure

Normal user should see only what is needed.

Advanced concepts can include:
- raw station profile points;
- lane ids;
- topology connection ids;
- tolerance diagnostics;
- provider capability metadata;
- source-page detail;
- mesh/render stats.

These should remain accessible without dominating normal workflow.

Block if:
- first-time road creation requires advanced station tables;
- every property is shown at once;
- critical action is hidden inside unrelated advanced settings.

---

# Gate 6 — Automation transparency

For generated actions:
- show what will be created/changed;
- show key parameters;
- Preview where meaningful;
- generated content remains editable;
- user overrides are protected from silent regeneration.

AI additionally:
- target is explicit;
- ambiguity is surfaced;
- assumptions are visible;
- Apply/Cancel is clear.

Block if:
- automatic behavior silently changes topology;
- AI directly changes geometry without typed proposal;
- regeneration deletes overrides without warning.

---

# Gate 7 — Error and recovery

Check:
- invalid numeric value remains understandable;
- failed geometry does not corrupt last valid project;
- error identifies affected object;
- Issues can zoom/select target;
- Undo/Redo is available after valid commits;
- transient preview does not pollute history.

Block if:
- an error leaves partial geometry;
- user needs to restart or manually repair JSON;
- popup disappears without actionable context.

---

# Gate 8 — 2D / 3D consistency

For features shown in both views:
- same semantic object id;
- same dimensions;
- same scenario;
- selection sync;
- 3D cache can be rebuilt;
- presentation settings do not change engineering data.

Block if:
- 3D has an independent width/radius;
- editing in one view produces stale other view;
- switching view changes canonical state.

---

# Gate 9 — Map / reference safety

Check:
- reference status/quality visible when relevant;
- source can be locked/dimmed;
- display changes never affect engineering geometry;
- provider digitization/export restrictions enforced;
- unscaled source cannot imply metre accuracy.

Block if:
- reference provider terms are hidden but prohibited operations remain enabled;
- pixel coordinates become canonical engineering coordinates without calibration/context.

---

# Gate 10 — Standards provenance

When an engineering advisory appears:
- severity clear;
- rule/source identifiable;
- authority/profile/version available;
- exact source detail accessible;
- override policy clear;
- warning distinguished from geometry invalidity.

Block if:
- UI says “not standard” without source/applicability;
- generic default is presented as Thai regulation;
- profile update silently changes old project.

---

# Gate 11 — Scenarios

Check:
- active scenario unmistakable;
- Existing can be protected;
- duplicate/rename intuitive;
- editing B cannot alter A;
- compare mode does not accidentally edit both.

Block if:
- layer visibility is confused with scenario semantics;
- scenario state is only presentation grouping.

---

# Gate 12 — Asset workflow

Check:
- library uses semantic categories/search;
- asset arrives correctly scaled/oriented;
- user does not need external art tools;
- road-relative placement is available where useful;
- source/license/verification status available;
- presentation props do not affect engineering rules unless explicitly modeled.

Block if:
- user must resize every vehicle/tree manually;
- engineering marking is just an imported PNG;
- files/folders are the primary asset UX.

---

# Gate 13 — Accessibility / keyboard

Applicable UI should:
- have visible focus;
- not rely on color alone;
- expose semantic object tree/properties outside custom canvas;
- support keyboard access to critical actions;
- respect reduced motion where applicable;
- maintain readable target sizes and numeric controls.

Target:
WCAG 2.2 AA for applicable UI surfaces.

Canvas rendering itself may require paired semantic UI rather than pretending every rendered primitive is DOM-accessible.

---

# Gate 14 — Information density / visual hierarchy

Check:
1. viewport dominant;
2. current selection/tool obvious;
3. properties/issues accessible;
4. reference imagery subordinate;
5. active scenario clear;
6. secondary metadata not visually louder than engineering geometry.

Block if:
- dashboard cards consume the editor;
- decorative chrome reduces useful viewport substantially;
- every panel competes for attention.

---

# Gate 15 — First-time comprehension

Run without explaining the UI verbally.

For the current milestone, ask a first-time tester to perform one canonical task.

Observe:
- where they hesitate;
- what they expect to click;
- whether terminology is understood;
- whether they notice current mode/scenario;
- whether they discover exact input;
- whether automation feels predictable.

Do not fix confusion only by adding tooltips. Reconsider hierarchy/workflow first.

---

# Required UX evidence by milestone

## Low-fidelity
- task-flow walkthrough;
- screen/wireframe;
- anticipated states/errors;
- no pixel-perfect requirement.

## Interactive prototype
- canonical task completion;
- mode/selection behavior;
- exact input/direct manipulation;
- scenario/reference behavior as applicable.

## Production alpha
- real UAT;
- keyboard/accessibility checks;
- error/undo/recovery;
- screenshot/visual regression where stable.

## Beta
- multi-project use;
- performance;
- field feedback;
- accessibility/visual polish;
- issue closure evidence.

---

# Review template

```text
Feature / flow:
Golden Workflow:
Build / commit:

Task-fit:
Engineering-state clarity:
Mode safety:
Direct + exact editing:
Progressive disclosure:
Automation transparency:
Error/recovery:
2D/3D consistency:
Reference/map:
Standards:
Scenario:
Assets:
Accessibility:
Visual hierarchy:

Findings:
- blocker:
- high:
- medium:
- low:

Decision:
PASS / PASS WITH CONDITIONS / REVISE / BLOCK
```

## Principle

A good Street Concept Designer UI is not the one with the most controls visible. It is the one that lets the engineer express intent quickly while keeping geometry, assumptions, source quality, and automation inspectable.
