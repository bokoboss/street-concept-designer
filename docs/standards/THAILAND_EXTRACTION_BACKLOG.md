# Thailand Standards / Asset Extraction Backlog

Baseline: 2026-08-27

## Purpose

Convert the source register into a controlled extraction plan. No parameter becomes an authoritative Thailand profile default until the required source record is complete.

## Extraction record required for every encoded rule/asset

```text
authority
documentTitle
documentCode
edition/revision
publication/effectiveDate
page
table/figure/drawing
roadClass/applicability
topic/assetFamily
parameter
value/unit or geometry definition
requirementType
  mandatory | preferred | typical | illustrative | project-specific
notes
sourceUrl/sourceFile
extractedBy
verifiedBy
verificationDate
supersessionStatus
```

If page/table/figure cannot be verified, status remains PENDING and the value must not ship as authoritative.

---

# Priority A — Core pavement markings

Required for early professional plan output.

## A1 — Lane / center / edge lines

Extract:
- line categories/types;
- color;
- width;
- single/double arrangement;
- solid/dashed rules;
- dash/gap/repeat geometry where specified;
- applicability by road/control condition;
- material/spec reference where relevant to representation.

Product outputs:
- procedural marking definitions;
- profile-default parameters;
- rule/advisory references.

Target authorities:
- DOH permanent marking/design manuals;
- DRR permanent marking/standard drawings.

Do not use work-zone marking dimensions as permanent defaults.

Status: PENDING PAGE-LEVEL EXTRACTION.

---

## A2 — Stop / yield lines

Extract:
- shape;
- width/depth;
- placement relative to intersection/crosswalk;
- color;
- applicable control.

Product outputs:
- StopLine generator;
- YieldLine generator;
- junction attachment rules.

Status: PENDING.

---

## A3 — Crosswalks

Extract:
- crosswalk types;
- stripe geometry;
- width;
- orientation/placement;
- relationship to stop line/curb;
- applicability.

Product:
`roadmark.crosswalk.*` procedural generator.

Status: PENDING.

---

## A4 — Direction arrows / pavement symbols

Extract:
- official arrow family;
- geometry/dimensions;
- lane placement/offset;
- combinations;
- U-turn symbol;
- bicycle/motorcycle/bus symbols where applicable.

Product:
vector master shapes generated programmatically.

Status: PENDING.

---

## A5 — Hatch / chevron / gore

Extract:
- boundary rules;
- stripe angle;
- stripe width;
- spacing;
- color;
- use case.

Product:
parametric area-fill generator.

Status: PENDING.

---

# Priority B — Traffic sign face library

Primary DOH source candidate:
Traffic Sign Standard Manual, 2561, Volume 1.

Secondary:
design/installation volume and DRR standard drawings.

## B1 — Regulatory signs

Extract for each high-use sign:
- code/id;
- Thai/English canonical name if defined;
- shape;
- face dimensions;
- colors;
- border;
- symbol/text geometry/source;
- variant sizes/classes;
- applicability;
- mounting references.

Starter signs should prioritize common road/access/intersection concepts rather than complete catalog breadth.

Status: PENDING.

---

## B2 — Warning signs

Same extraction fields.

Starter priority:
- intersection;
- side road;
- curve;
- merge;
- pedestrian/crossing;
- school/urban hazards where applicable;
- U-turn/turning context if officially defined.

Status: PENDING.

---

## B3 — Guide / directional signs

Later starter subset:
- direction arrows;
- destination panel geometry;
- lane/route direction;
- overhead/ground-mounted structure.

Need typography/text-layout source review.

Status: PENDING / lower priority than regulatory/warning.

---

# Priority C — Sign installation / supports

Extract:
- mounting height;
- lateral offset/clearance;
- single/double pole cases;
- overhead/gantry cases;
- orientation;
- road-context applicability.

Product:
semantic support assemblies and future advisory rules.

Important:
support dimensions/installation guidance may differ between authority/road type.

Status: PENDING.

---

# Priority D — Traffic signals

Extract:
- signal head configurations;
- lens/head arrangement;
- mounting;
- pole/mast-arm/gantry;
- pedestrian heads;
- conditions/warrants only if authoritative and in product scope;
- relationship to stop line/crosswalk.

Initial product goal:
semantic assembly and visualization, not timing design.

Status: PENDING.

---

# Priority E — Lighting

Current source identified:
DOH 2026 LED road-lighting design/installation guidance.

Extract:
- pole/luminaire families;
- mounting height;
- arm configurations;
- placement principles;
- spacing only where applicability/photometric context is clear;
- road-class applicability.

Product:
lighting assembly metadata first.

Do not turn typical spacing into a universal default without design context.

Status: SOURCE IDENTIFIED, PAGE EXTRACTION PENDING.

---

# Priority F — Safety / roadside devices

## F1 — Guardrail / barrier

Extract:
- standard assembly/drawing ids;
- segment dimensions;
- post spacing;
- terminal/end treatment where in scope;
- placement/offset guidance;
- authority/applicability.

Product:
procedural repeated assembly.

Status: DRR drawing source identified, extraction pending.

## F2 — Delineators / bollards

Extract:
- size;
- color/reflective surfaces;
- spacing/placement where appropriate;
- application.

Status: PENDING.

## F3 — Pedestrian railing / fence

Lower priority unless repeatedly needed in projects.

Status: PENDING.

---

# Priority G — U-turn / median treatments

Research targets:
- DOH/DRR geometric design guidance;
- standard drawings;
- median-opening/U-turn manuals if current and applicable.

Extract:
- design categories/types;
- semantic geometry elements;
- applicability;
- pocket/storage/taper guidance only with explicit context;
- island/median relationships;
- marking/sign requirements.

Important:
do not encode a single universal “Thai U-turn” dimension.

Product:
general geometric primitive first; profile-guided templates later.

Status: SOURCE DISCOVERY/VERIFICATION NEEDED.

---

# Priority H — Access / intersection geometric guidance

Research targets:
- access-management/intersection design manuals;
- DOH/DRR geometric criteria;
- current road design manuals.

Potential extraction:
- corner radius guidance;
- lane width;
- taper;
- storage;
- channelization;
- island dimensions;
- sight/clearance concepts.

These rules are highly context-sensitive.

Product policy:
prefer advisory range/rule with applicability metadata rather than hard block.

Status: SOURCE DISCOVERY/VERIFICATION NEEDED.

---

# Work-zone materials

DOH/DRR construction traffic-control manuals are useful but must live in a separate future profile:
- `Thailand / DOH / Work Zone`
- `Thailand / DRR / Work Zone`

They must not supply permanent-road defaults.

Status: DEFERRED MODULE.

---

# Asset extraction workflow

For a source-backed visual asset:

1. inspect source PDF page image;
2. record citation/provenance;
3. derive semantic dimensions;
4. create a code/vector generator specification;
5. independently verify generated geometry against source;
6. produce 2D rendering;
7. derive 3D representation if applicable;
8. add automated dimension/provenance tests;
9. mark verification state:
   - DRAFT
   - SOURCE_EXTRACTED
   - GENERATED
   - VERIFIED
   - RELEASED

No manual tracing by the user is required.

---

# Rule extraction workflow

For engineering guidance:

1. capture exact source text/table/figure;
2. identify applicability;
3. classify mandatory/preferred/typical;
4. model condition inputs;
5. encode rule id/version;
6. test boundary cases;
7. show source in UI;
8. allow override only when rule policy permits;
9. project pins profile version.

---

# Source-access constraint

As of this baseline, several identified DOH/DRR PDF endpoints are not reliably retrievable/renderable from the current research environment. Therefore:
- source existence/index information may remain in `THAILAND_SOURCE_REGISTER.md`;
- no unrendered PDF is treated as page-verified;
- no page number/dimension is invented from search snippets;
- extraction resumes only when the actual authoritative page can be visually inspected.

This is a research-access limitation, not a reason to lower the product provenance standard.

---

# First Thailand pack completion target

Before calling the initial pack “Thailand Engineering Starter” aim for verified coverage of:

- lane/center/edge line basics;
- stop line;
- crosswalk;
- directional arrows including U-turn where official;
- hatch/chevron;
- common regulatory sign subset;
- common warning sign subset;
- basic sign supports;
- generic/verified traffic signal assembly basis;
- basic lighting assembly basis;
- guardrail/delineator basis.

Anything else can remain generic or later without blocking the core editor.
