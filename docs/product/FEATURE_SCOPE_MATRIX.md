# Feature Scope Matrix

Research-informed baseline: 2026-08-25

This matrix prevents feature enthusiasm from overriding product sequencing. `NOW` means required to prove/build the core product, not necessarily all in the next coding packet.

## Core semantic / engineering capabilities

| Capability | Priority | Rationale |
|---|---|---|
| Reference alignment + stationing | NOW | Foundation for arbitrary roads and all longitudinal geometry |
| Component-based cross section | NOW | Core ease-of-use + semantic road structure |
| Variable width / lane lifecycle | NOW | Required for taper, widening, lane add/drop, turn pockets |
| T / four-leg / skew junction semantics | NOW | Core intersection concept use case |
| Explicit lane connectivity / movements | NOW | Needed for correct junction semantics and future visualization |
| Per-corner junction geometry | NOW | Real intersections require asymmetric control |
| Median / median opening | NOW | High-value Thailand/TIA workflow |
| Project access / driveway | NOW | Primary Golden Workflow |
| Right/left-turn pockets | NOW | Primary traffic-engineering concept feature |
| U-turn treatment | NEXT | Important Thailand use case; build after general lane/junction primitives prove stable |
| Bus bay / slip lane | NEXT | High-value feature using the same longitudinal/junction primitives |
| Roundabout | NEXT | First-class special module after T/X junction kernel qualification |
| Vertical alignment | LATER | Reserve architecture hooks; not needed for initial concept workflows |
| Terrain / grading | LATER | Useful context but not core concept-authoring proof |
| Full swept-path analysis | LATER | Separate engineering module with vehicle/standards depth |
| Traffic microsimulation | EXCLUDED_INITIAL | Different product class; may integrate externally later |
| Signal timing optimization | EXCLUDED_INITIAL | Calculation/simulation scope, not concept geometry core |
| Drainage / earthworks / BIM | EXCLUDED_INITIAL | Detailed civil design scope |

## Reference / map / import

| Capability | Priority | Rationale |
|---|---|---|
| Blank canvas | NOW | Independent core authoring |
| User image / site-plan import | NOW | Lowest-risk real-world reference workflow |
| Known-distance scale calibration | NOW | Required for non-GIS aerial/site plans |
| Reference layer opacity/dim/lock | NOW | Critical plan readability and accidental-edit prevention |
| Real CRS / georeference context | NEXT | Needed for robust map/provider/import workflows after kernel coordinate proof |
| MapLibre renderer/provider abstraction | NEXT | Map-first product capability; provider terms must be enforced |
| OSM-derived context import | NEXT | Useful starting context but must remain non-authoritative/editable |
| Licensed satellite/aerial provider | NEXT | Provider depends on digitization/export/cache rights |
| Organization WMS/XYZ | NEXT | Common professional GIS integration pattern |
| GeoTIFF | LATER/NEXT | Valuable once CRS/import pipeline is mature |
| DXF import | LATER | Useful interoperability; must not dictate canonical model |
| DWG direct import | LATER | Licensing/SDK complexity; DXF/intermediate may suffice first |
| Google satellite tracing | NOT SUPPORTED BY DEFAULT | Current Google Maps Platform terms prohibit tracing/digitizing from satellite content |

## Authoring / UX

| Capability | Priority | Rationale |
|---|---|---|
| 2D plan editor | NOW | Primary authoring surface |
| Direct manipulation + exact properties | NOW | Speed without giving up engineering control |
| Cross-section contextual editor | NOW | Streetmix-level component ease applied to a plan-based model |
| Select vs Hand/Pan explicit modes | NOW | Prevent accidental editing/navigation confusion |
| Contextual tool actions | NOW | Avoid CAD tool overload |
| Undo/redo semantic transactions | NOW | Fundamental editor reliability and AI safety |
| Human-readable history | NEXT | Valuable once command layer is functioning |
| Layers / hierarchy / lock / visibility | NOW/NEXT | Reference + scenarios + assets become unmanageable without it |
| Issues panel + click-to-zoom | NEXT | Required when advisory validation lands |
| Keyboard shortcuts | NEXT | Professional efficiency after stable actions exist |
| Customizable workspace | LATER | Avoid premature editor complexity |
| Full CAD command line | NOT PLANNED | Conflicts with product accessibility goal |

## 3D / presentation

| Capability | Priority | Rationale |
|---|---|---|
| Shared-model 3D proof | NOW (R1 proof) | Must prove no divergent 2D/3D models |
| Live synchronized engineering 3D | NEXT | Core product proposition after kernel qualification |
| Pavement/curb/median/sidewalk 3D | NEXT | Derived from semantic road components |
| Engineering 3D mode | NEXT | Geometry inspection/communication |
| Presentation 3D mode | NEXT/LATER | Richer materials/context after engineering view is stable |
| Simple building massing | NEXT/LATER | Useful context; procedural and low-risk |
| Saved camera views | NEXT/LATER | High presentation value |
| Rich vegetation/vehicles/people | LATER | Presentation polish; use normalized assets/instancing |
| Night mode | LATER | Presentation option only |
| Traffic movement animation | LATER | Requires stable lane connectivity; visualization, not simulation initially |

## Markings / signs / road furniture

| Capability | Priority | Rationale |
|---|---|---|
| Procedural lane/edge/stop lines | NEXT | Core engineering visual output |
| Turn arrows / U-turn arrows | NEXT | Traffic concept communication |
| Zebra crossing / hatch / chevron | NEXT | Junction/road-feature communication |
| Semantic sign assembly schema | NEXT | Create structure before full sign library |
| Signal assembly schema | NEXT | Future-proof without signal timing scope |
| Generic lights/bollards/barriers | NEXT | Small generated/semantic starter set |
| Thailand DOH/DRR verified asset packs | NEXT, source-gated | High product relevance; require exact provenance before authoritative geometry |
| Large realistic prop library | LATER | Consistency/provenance more valuable than quantity |
| User GLB/custom asset import | LATER | Useful escape hatch after built-in pipeline is stable |

## Scenarios / project lifecycle

| Capability | Priority | Rationale |
|---|---|---|
| Semantic local project format | NOW/NEXT | Real work must be resumable; renderer cache cannot be source of truth |
| Schema versioning | NOW/NEXT | Avoid future migration debt |
| Existing / Alternative scenarios | NEXT | Core traffic-consulting workflow |
| Duplicate scenario | NEXT | Simple high-value alternative workflow |
| Overlay comparison | NEXT | Core communication |
| Side-by-side comparison | NEXT/LATER | Useful presentation mode |
| Before/after slider | NEXT/LATER | Strong visual communication |
| Autosave/crash recovery | NEXT | Required before real professional trust |
| Cloud sync/accounts | LATER/NOT INITIAL | Not needed for standalone core value |
| Real-time collaboration | LATER | High complexity, not core single-user workflow |

## Validation / standards

| Capability | Priority | Rationale |
|---|---|---|
| Internal geometry invariants | NOW | Kernel correctness |
| Provenance-aware StandardProfile schema | NEXT | Needed before standards guidance |
| Advisory lane/taper/radius/etc rules | NEXT, source-gated | Valuable only with verified authority/applicability |
| Project pinning of standard version | NEXT | Prevent silent interpretation drift |
| Engineer override + rationale | NEXT | Concept work has real constraints/exceptions |
| Full compliance certification | NOT PLANNED INITIAL | Product is concept design, not statutory compliance engine |

## AI / automation

| Capability | Priority | Rationale |
|---|---|---|
| Typed command architecture | NOW/NEXT | Required foundation even before LLM integration |
| Automation-first semantic generators | NOW/NEXT | Reduces manual drafting burden |
| Natural-language command proposal | LATER/NEXT after commands stable | User-friendly but must not bypass validated transactions |
| Explain selection / issues | LATER | Low-risk AI assistance after domain model exists |
| Generate alternative proposal | LATER | High value once scenarios/commands/validation mature |
| Autonomous engineering design without review | NOT PLANNED | Conflicts with human domain authority and engineering accountability |

## Export / interoperability

| Capability | Priority | Rationale |
|---|---|---|
| Engineering plan image export | NEXT | Core report/presentation use |
| Transparent/neutral background export | NEXT | Keeps design independent of basemap licensing |
| Reference-included export with provider policy gate | NEXT | Important map-first output but terms-sensitive |
| High-resolution raster | NEXT | Report/presentation |
| SVG/vector plan export | NEXT/LATER | Useful but renderer-independent semantics remain source of truth |
| GLB 3D export | LATER | Presentation/interoperability once 3D stable |
| GeoJSON | LATER | GIS integration |
| DXF | LATER | Professional interoperability |
| OpenDRIVE | LATER | Semantic interoperability/simulation ecosystem |
| Video | LATER | Presentation only |

## Sequencing principle

Build depth in the Golden Workflows before breadth in the catalog. A small set of reliable primitives that can create an irregular real-world access/intersection concept is more valuable than many templates/assets that cannot be edited semantically.


## Windows distribution

| Capability | Priority | Rationale |
|---|---|---|
| Per-user Windows installer without admin | NOW FOR RELEASE ARCHITECTURE / R8-R9 IMPLEMENTATION | common office deployment path |
| Offline per-user installer | R8-R9 | site/office computers may lack setup-time internet |
| Portable Light ZIP | R8-R9 REQUIRED | no-install use on office PCs with compatible WebView2 |
| Portable Offline ZIP | R8-R9 EVIDENCE-GATED REQUIRED OUTCOME | no-install/no-internet constrained use; exact fixed-runtime packaging must be qualified |
| Trusted code signing | R8-R9 REQUIRED | enterprise/SmartScreen deployment quality |
| Microsoft Store distribution | LATER/OPTIONAL | not required for core standalone workflow |
| Windows ARM64 | LATER | x64 first |
| macOS/Linux packages | LATER | architecture may remain portable but not initial gates |
