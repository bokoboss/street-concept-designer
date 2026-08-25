# UX Architecture

## Character

Modern professional engineering editor: calm, precise, compact, premium, and less intimidating than CAD. Not a SaaS dashboard and not a game editor.

## Primary workspace

- top: project/scenario/view/export controls;
- left: persistent Layers / Map / Library navigation;
- center: dominant viewport;
- right: contextual Properties / Issues / AI Copilot;
- compact tool strip: Select, Hand, Road, Junction, Marking, Measure, Snapping;
- scenario strip: Existing / Alt A / Alt B / +.

## View modes

- 2D Plan;
- Split 2D + 3D;
- Full 3D;
- Cross Section as contextual editor/helper.

2D is the primary engineering authoring surface. Selection is synchronized across views.

## Editing model

- direct manipulation for speed;
- exact numeric properties for engineering control;
- live preview during edit;
- commit meaningful operations as commands;
- deterministic undo/redo with readable history;
- contextual tools based on selected semantic object;
- progressive disclosure for stationing, lane profiles, topology, and advanced properties.

## Navigation safety

Select and Hand/Pan are distinct explicit modes. 3D navigation separates selection from orbit/pan/move. Accidental object movement must be difficult.

## Junction interaction

When roads geometrically conflict, show a candidate junction. The user explicitly chooses Create Junction, Grade Separated, or Ignore. No silent topology mutation.

## Validation

Use an Issues panel with severity, target, source/rule id, and click-to-zoom. Warnings are advisory unless geometry is internally impossible.

## AI interaction

Natural-language requests appear as proposed semantic commands. The UI must show a human-readable diff/parameters before Apply. AI-generated changes must share the same undo/redo path as manual changes.
