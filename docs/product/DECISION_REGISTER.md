# Decision Register

Baseline: 2026-08-27

## Purpose

Prevent brainstorm ideas, research hypotheses, and accepted product decisions from being treated as equivalent.

Statuses:
- **LOCKED** — accepted product/architecture baseline; change requires explicit rationale/review.
- **EVIDENCE_GATED** — preferred hypothesis; R1/prototype/research must decide.
- **DEFERRED** — intentionally postponed; not a current implementation requirement.
- **SOURCE_GATED** — desired, but authoritative source/licensing evidence is required.
- **REJECTED_INITIAL** — explicitly outside initial product or unsafe/inappropriate as a default.
- **OPEN_LATER** — may be reconsidered after core product success.

---

# Product / scope decisions

| Decision | Status | Rationale |
|---|---|---|
| Clean-slate Generation 2 repository | LOCKED | avoid legacy technical debt and schema/architecture coupling |
| Product = rapid street/access/intersection concept designer | LOCKED | closes gap between simple graphics and heavy CAD |
| Standalone desktop product direction | LOCKED | removes Illustrator/CAD dependency and supports professional local work |
| Windows-first | LOCKED | initial user environment; architecture may remain cross-platform capable |
| Traffic Engineer / Transport Planner primary user | LOCKED | drives workflows and acceptance |
| Concept design, not detailed Civil CAD | LOCKED | prevents scope explosion |
| Core editing works offline | LOCKED | online maps are optional reference services |
| Cloud collaboration/account required | REJECTED_INITIAL | not needed for initial value |
| Traffic microsimulation | REJECTED_INITIAL | separate product class/integration opportunity |
| Drainage / grading / BIM | REJECTED_INITIAL | detailed civil-design scope |
| Full swept-path analysis | DEFERRED | later engineering module after stable network/vehicle semantics |
| Vertical alignment / terrain | DEFERRED | reserve architecture hooks; not needed for initial Golden Workflows |

---

# User-effort decisions

| Decision | Status | Rationale |
|---|---|---|
| User must not need Illustrator | LOCKED | core product objective |
| User must not need Blender/3D modelling | LOCKED | asset production belongs to system/development pipeline |
| User must not write code/config JSON for normal use | LOCKED | approachable engineering editor |
| Standard road features generated from intent/parameters | LOCKED | automation-first |
| Manual semantic override available | LOCKED | irregular real sites and engineering judgment |
| Manual polygon drawing as normal turn-pocket workflow | REJECTED_INITIAL | hides engineering semantics and creates fragile geometry |
| Large manual asset-production burden assigned to user | REJECTED_INITIAL | conflicts with product/user constraint |

---

# UX decisions

| Decision | Status | Rationale |
|---|---|---|
| 2D plan is primary authoring surface | LOCKED | best fit for engineering geometry and current competitor gap |
| Split 2D/3D | LOCKED | core product proposition |
| 3D uses same semantic model | LOCKED | prevents divergence |
| Direct manipulation + exact numeric properties | LOCKED | speed + engineering control |
| Cross-section is contextual editor, not sole source-of-truth | LOCKED | longitudinal/intersection work is core |
| Small persistent tool families + contextual actions | LOCKED | avoid CAD modal/tool overload |
| Select and Hand/Pan are distinct | LOCKED | prevent accidental geometry movement |
| Progressive disclosure | LOCKED | keeps normal workflow approachable |
| Layers/Map/Library top-level concepts | LOCKED | required as project complexity grows |
| Exact palette/font/iconography | EVIDENCE_GATED | decide after real prototype visual UAT |
| Fully customizable/dockable workspace | DEFERRED | avoid premature UI complexity |
| Full CAD command line | REJECTED_INITIAL | conflicts with approachability goal |

---

# Road / geometry decisions

| Decision | Status | Rationale |
|---|---|---|
| Canonical engineering units = metres | LOCKED | metric-native engineering model |
| LHT/RHT configurable; Thailand/LHT default | LOCKED | local relevance without hard-coding |
| Road source-of-truth = reference alignment + semantics | LOCKED | supports arbitrary plan geometry and derived views |
| Station-based longitudinal model | LOCKED | taper/add/drop/feature lifecycle |
| Initial alignment primitives: line + circular arc + smooth conceptual curve | LOCKED FOR R1 | sufficient initial proof; future spiral hook preserved |
| Clothoid/spiral | DEFERRED | architecture hook only until need/UX proven |
| Lane/component width varies by station | LOCKED | fundamental feature |
| Turn pocket uses general lane lifecycle | LOCKED | prevents special-case technical debt |
| Graphics/SVG/mesh as road source-of-truth | REJECTED_INITIAL | violates semantic architecture |
| Fixed single cross-section for entire road | REJECTED_INITIAL | insufficient for real traffic concepts |
| Exact numerical tolerance values | EVIDENCE_GATED | R1 must derive/test rather than invent |
| Kernel implementation language | EVIDENCE_GATED | Rust native/WASM is first hypothesis |
| Rust native/WASM | EVIDENCE_GATED | R1A proof required |
| Specific geometry crates | EVIDENCE_GATED | dependency/adversarial tests required |

---

# Junction / topology decisions

| Decision | Status | Rationale |
|---|---|---|
| Junction = first-class semantic object | LOCKED | topology/movement/corners cannot be just polygon union |
| Geometry crossing != topology | LOCKED | prevents accidental network mutation |
| Candidate Junction → Create / Grade-Separated / Ignore | LOCKED | explicit user authority |
| Per-corner geometry | LOCKED | real intersections are asymmetric |
| Lane-to-lane connectivity stored semantically | LOCKED | future movements/visualization/analysis |
| Auto-generate + manual override | LOCKED | speed without loss of control |
| Automatic silent junction creation | REJECTED_INITIAL | unsafe/opaque |
| T / four-leg / skew cases first | LOCKED FOR CORE | foundation before special cases |
| U-turn treatment | DEFERRED TO POST-GENERAL PRIMITIVES | important Thailand use case but not a separate kernel |
| Roundabout | DEFERRED | dedicated gated module after T/X kernel |

---

# 2D / 3D renderer decisions

| Decision | Status | Rationale |
|---|---|---|
| Renderers consume derived DTO/buffers | LOCKED | renderer never canonical truth |
| Semantic ids survive into renderer selection mapping | LOCKED | synchronized selection |
| Local render origin before GPU float32 | LOCKED POLICY | large-coordinate stability |
| PixiJS 2D | EVIDENCE_GATED/PREFERRED | strong candidate; production prototype must confirm |
| Three.js 3D | EVIDENCE_GATED/PREFERRED | strong candidate; production prototype must confirm |
| Separate 2D and 3D geometry engines | REJECTED_INITIAL | divergence risk |
| Photorealism before engineering 3D | REJECTED_INITIAL | wrong priority |

---

# Project / persistence decisions

| Decision | Status | Rationale |
|---|---|---|
| Versioned semantic project schema | LOCKED | long-term project continuity |
| Renderer cache not source-of-truth | LOCKED | must regenerate deterministically |
| Schema migration explicit/tested | LOCKED | avoid future project breakage |
| Physical project container format | EVIDENCE_GATED | JSON/ZIP/etc should follow real needs |
| Event sourcing required | REJECTED AS REQUIREMENT | command history does not imply event-sourced persistence |
| Autosave/crash recovery | DEFERRED TO PRODUCT BETA BUT REQUIRED BEFORE TRUST | not R1 blocker, later professional requirement |

---

# Command / AI decisions

| Decision | Status | Rationale |
|---|---|---|
| Manual UI/AI/import/automation share one semantic command path | LOCKED | deterministic mutation/undo/validation |
| Preview vs commit separation | LOCKED | smooth editing without history spam |
| AI proposes typed semantic commands | LOCKED | avoids model/renderer corruption |
| AI directly edits project JSON/mesh/SVG | REJECTED_INITIAL | bypasses invariants/audit |
| AI natural-language authoring in initial kernel | DEFERRED | command architecture first |
| AI may propose alternatives later | OPEN_LATER | valuable once scenarios/validation stable |
| Autonomous unreviewed engineering design | REJECTED_INITIAL | human engineering authority retained |

---

# Map / reference decisions

| Decision | Status | Rationale |
|---|---|---|
| Map renderer separated from provider | LOCKED | technical/licensing independence |
| Basemap is reference, never engineering truth | LOCKED | avoids false authority |
| User image/site-plan import + scale calibration | LOCKED | reliable initial real-world workflow |
| MapLibre GL JS | EVIDENCE_GATED/PREFERRED | renderer candidate only |
| OSM-derived data can be context/import, user-verified | SOURCE_GATED | open data/licensing + non-survey accuracy |
| OSMF public tile service as permanent unrestricted backend | REJECTED_INITIAL | service policy/capacity restrictions |
| Google satellite as default tracing/digitization source | REJECTED_INITIAL under current terms | current terms restrict derived tracing/digitizing use |
| Licensed satellite/aerial provider | SOURCE_GATED | provider must permit digitization/export/cache behavior |
| Generic offline “download area” for arbitrary provider | REJECTED_INITIAL | terms-sensitive |

---

# Assets decisions

| Decision | Status | Rationale |
|---|---|---|
| Procedural road/marking assets | LOCKED | dimensions/standards/provenance |
| Semantic assemblies for signs/signals/lights | LOCKED | reusable 2D/3D + metadata |
| Complex presentation props may use normalized GLB | LOCKED | practical art pipeline |
| One semantic asset can have 2D + 3D representations | LOCKED | selection/placement consistency |
| User manually produces SVG/GLB as prerequisite | REJECTED_INITIAL | conflicts with user constraint |
| Core traffic-control assets copied from competitors | REJECTED_INITIAL | license/correctness risk |
| Generic CC0 presentation assets | SOURCE_GATED/PREFERRED | useful after provenance/normalization |
| Large realistic library before kernel/core workflow | REJECTED_INITIAL | wrong sequencing |
| Custom user GLB import | DEFERRED | useful later escape hatch |

---

# Standards decisions

| Decision | Status | Rationale |
|---|---|---|
| StandardProfile is versioned and project-pinned | LOCKED | no silent rule drift |
| Authority/document/edition/page/figure provenance | LOCKED | traceability |
| DOH/DRR profiles are distinct | LOCKED | avoid false universal “Thailand” standard |
| Hard-code Thai values across geometry source | REJECTED_INITIAL | governance/maintenance risk |
| Numeric Thailand defaults without page-level verification | REJECTED_INITIAL | false authority risk |
| Geometry coherence vs advisory engineering validation separated | LOCKED | concept constraints need overrides |
| Full compliance certification claim | REJECTED_INITIAL | outside concept-product assurance |

---

# Scenarios / comparison decisions

| Decision | Status | Rationale |
|---|---|---|
| Existing / Alternative scenarios are first-class | LOCKED | traffic consultancy workflow |
| Storage implementation hidden from semantic API | LOCKED | snapshot/delta choice may evolve |
| Duplicate scenario | LOCKED FOR PRODUCT | high-value simple workflow |
| Overlay comparison | LOCKED FOR PRODUCT | primary concept communication |
| Side-by-side / slider / 3D compare | DEFERRED | add after basic scenario system |

---

# Export decisions

| Decision | Status | Rationale |
|---|---|---|
| Engineering-only export independent of basemap | LOCKED | rights/product resilience |
| Include basemap only when provider permits | LOCKED | license compliance |
| Attribution automatically enforced when required | LOCKED | provider/data compliance |
| High-resolution plan export | LOCKED FOR PRODUCT |
| SVG/vector export | DEFERRED/NEXT | useful, but not source-of-truth |
| GLB/OpenDRIVE/DXF/GeoJSON | DEFERRED | interoperability after core model stabilizes |

---

# Decisions that R1 must return

R1 should provide evidence for, not silently decide:
- kernel language final choice;
- exact crates/libraries;
- native/WASM build feasibility;
- tolerance defaults;
- sampling strategy details;
- Boolean/offset/triangulation strategy;
- performance baseline;
- production DTO details;
- any architecture contradiction discovered by adversarial geometry.

## Change discipline

A coding implementation must not rewrite a LOCKED decision simply because a library/API makes another approach easier.

If a LOCKED decision proves technically unsound:
1. stop;
2. provide a minimal failing case/evidence;
3. propose alternatives;
4. scrutinize architecture;
5. update this register only through an explicit accepted decision change.
