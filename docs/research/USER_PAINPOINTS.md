# User Pain-Point Research

Research date: 2026-08-25

This document supplements official product research with community/user anecdotes. Community sources are directional evidence, not authoritative engineering facts.

## P1 — Strong demand for a quick plan-view equivalent of Streetmix

Community example:
- https://www.reddit.com/r/urbanplanning/comments/p0pjl7

Users describe Streetmix as effective for communicating sections to non-technical stakeholders but explicitly ask for a similarly quick and visually clear **plan-view** tool for intersections and street layouts.

Product implication:
- plan-view authoring is not an optional extension of a cross-section product;
- primary value is rapid longitudinal/intersection communication while retaining engineering structure.

## P2 — Non-CAD users struggle to keep aerial-image concepts clean and to scale

Community examples:
- https://www.reddit.com/r/gis/comments/1p7emua/how_to_make_a_street_redesign_layout_as_a/
- https://www.reddit.com/r/urbandesign/comments/101or3u

Users trying to sketch over overhead imagery resort to GIMP/image editors, styluses, or CAD suggestions. Repeated pain points include:
- difficult scale control;
- messy manual lane/road-line drawing;
- CAD learning cost;
- inability to quickly build intersections rather than just typical sections.

Product implication:
- known-distance image calibration must be easy;
- road/lane/marking creation should be semantic/procedural, not freehand line drawing;
- normal use cannot assume Illustrator/CAD/Blender skills;
- exact dimensions should remain available without forcing a CAD command workflow.

## P3 — The dangerous middle: visually realistic but engineering-unrealistic designs

Community example:
- https://www.reddit.com/r/urbanplanning/comments/jr56go

A professional-planning discussion warns that existing infrastructure is irregular and that a simplified tool can become harmful if it produces drawings that look realistic enough to imply feasibility without sufficient geometry/engineering basis.

Product implication:
- never market the canvas as unconstrained visual painting;
- engineering-looking geometry must have explicit dimensions/semantics;
- reference/aerial data should show accuracy/provenance limitations;
- validation should distinguish `geometrically coherent`, `advisory engineering warning`, and `presentation-only`;
- presentation realism must never imply standards compliance;
- allow export notes/metadata describing concept-level status when appropriate.

## P4 — Base/reference data may be useful but not survey-grade

The same professional discussion notes that municipal vector maps, aerials, and existing 3D models may still be insufficient for actual design and can require survey/georeferenced drawings for higher-precision work.

Product implication:
- distinguish reference/calibrated/survey-quality source status;
- do not assign false precision because the geometry model uses metres;
- later allow authoritative survey/DXF/GIS input without changing the core road semantics;
- project/export should be able to disclose reference-source quality.

## P5 — Professional tools can create tool-mode knowledge overhead

Community example:
- https://www.reddit.com/r/matlab/comments/1kyga7y/roadrunner_project_roads_only_projects_my/

A RoadRunner user attempting a terrain projection used the Maneuver tool when road projection needed to occur in Road Plan first. This is a small example of a broader risk visible in professional editors: users must understand which specialist tool owns a particular operation.

RoadRunner official documentation also exposes many separate road/lane/junction tools, including Custom Junction, Corner, Junction Surface, Maneuver, lane add/width/marking/carve, etc.

Product implication:
- prefer contextual actions after semantic selection over proliferating dedicated modes;
- show current mode explicitly when a mode is necessary;
- group advanced specialist actions under object context;
- one semantic property should not require users to know which internal subsystem owns it.

## P6 — Tool continuity / project persistence matters

Community example:
- https://www.reddit.com/r/urbandesign/comments/1afo0d4

A user evaluating a Streetmix-derived tool immediately asks for JSON export/reimport if saving is restricted. Even concept-design tools become frustrating if work cannot be reliably resumed or moved.

Product implication:
- semantic project files and schema versioning are core product infrastructure, not an export afterthought;
- autosave/crash recovery should arrive before the product is trusted for real projects;
- cloud accounts are not required for continuity; local project ownership should work first.

## P7 — Quick vs good is perceived as a trade-off

Community examples:
- https://www.reddit.com/r/LandscapeArchitecture/comments/1lcw5b3
- https://www.reddit.com/r/urbanplanning/comments/jr56go

Users often assume they must choose between:
- quick but schematic;
- accurate/good but CAD-heavy.

Product opportunity:
The core differentiation is to narrow that trade-off through semantic generators:
- fast intent-level operations;
- exact numeric values available;
- engineering model under the visuals;
- presentation output derived automatically.

The product should not promise detailed-design certainty; it should provide **rapid concept geometry whose assumptions are explicit and inspectable**.

## P8 — Existing road/intersection context is irregular

Professional feedback notes real existing infrastructure rarely matches a small finite preset catalog.

Product implication:
- presets/configurations must be generators, not rigid templates;
- manual semantic override is mandatory;
- arbitrary alignment/skew/unequal approach widths must be supported progressively;
- import/tracing should produce editable objects rather than forcing users into predefined intersection diagrams.

## UX guardrails derived from pain points

1. Plan view is primary; cross section is a contextual editor.
2. Start from reference context but expose reference quality/rights limitations.
3. Make scale/calibration obvious.
4. Use automation for standard geometry but preserve exact numeric editing.
5. Prefer contextual actions over specialist-tool proliferation.
6. Separate navigation and editing modes visibly.
7. Never equate visual realism with engineering validity.
8. Keep local project save/recovery first-class.
9. Presets create editable semantics rather than constrain irregular sites.
10. Communicate concept-level assumptions and validation provenance.

## Research caution

Community anecdotes help identify friction and product opportunities but cannot establish engineering criteria or universal user behavior. They should inform UX hypotheses that are later validated with real product UAT and canonical traffic-engineering workflows.
