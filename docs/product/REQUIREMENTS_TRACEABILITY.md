# Requirements Traceability Matrix

## Purpose

Connect product requirements to authoritative design/architecture documents, implementation stages, and human acceptance evidence.

This matrix is intentionally higher-level than unit tests.

| ID | Requirement | Primary source | Architecture / design policy | Earliest proof stage | Human/UAT evidence |
|---|---|---|---|---|---|
| P-01 | User does not need Illustrator/CAD for normal concept drafting | PRODUCT_VISION | UX Architecture, Low-Fidelity Workspace | R3 | UAT-01, UAT-05 |
| P-02 | User does not need Blender/manual 3D asset creation | PRODUCT_VISION | Asset Production Pipeline, Starter Asset Catalog | R6 | UAT-05, UAT-07 |
| P-03 | Desktop, Windows-first, offline core editing | DECISION_REGISTER | Technical Architecture / later shell | R2/R3 | install/offline UAT R8/R9 |
| P-04 | Product remains concept-design focused, not detailed Civil CAD | PRODUCT_VISION | FEATURE_SCOPE_MATRIX, ROADMAP | all | scope scrutiny |
| P-05 | Normal Windows install works per-user without Administrator rights | PRODUCT_VISION / DECISION_REGISTER | WINDOWS_DISTRIBUTION_POLICY | R8/R9 | UAT-13 installer profile |
| P-06 | Portable ZIP can run without installation/admin/development toolchain | DECISION_REGISTER | WINDOWS_DISTRIBUTION_POLICY | R8/R9 | UAT-13 Portable Light |
| P-07 | Offline office deployment is qualified | DECISION_REGISTER | WINDOWS_DISTRIBUTION_POLICY | R8/R9 | UAT-13 Offline Installer / Portable Offline |
| P-08 | Project files behave identically across Installed/Portable modes | PROJECT_FILE_SCHEMA | WINDOWS_DISTRIBUTION_POLICY | R8/R9 | UAT-11 + UAT-13 |
| UX-01 | 2D plan is primary engineering authoring surface | UX_ARCHITECTURE | LOW_FIDELITY_WORKSPACE_SPEC | R3 | UAT-01/02/03 |
| UX-02 | Direct manipulation and exact numeric input edit same semantic property | UX_ARCHITECTURE | COMMAND_TRANSACTION_MODEL | R3 | UAT-01, UX Review Gate |
| UX-03 | Select vs Hand/navigation modes are explicit | UX_ARCHITECTURE | LOW_FIDELITY_WORKSPACE_SPEC | R3 | first-time usability / UX gate |
| UX-04 | Advanced station/topology details use progressive disclosure | UX_ARCHITECTURE | LOW_FIDELITY_WORKSPACE_SPEC | R3/R4 | UX gate |
| UX-05 | Contextual actions preferred over CAD-scale tool proliferation | TOOL_TAXONOMY | LOW_FIDELITY_WORKSPACE_SPEC | R3/R4 | UX gate |
| G-01 | Road source-of-truth uses reference alignment + stationing | SEMANTIC_MODEL | GEOMETRY_PRECISION_TOLERANCE_POLICY | R1A | kernel evidence; UAT-03 later |
| G-02 | Initial alignment handles line, arc, smooth conceptual curve | SEMANTIC_MODEL | R1A contract | R1A | R1A fixtures |
| G-03 | Cross section is ordered semantic components | SEMANTIC_MODEL | STARTER_ROAD_CONFIGURATION_CATALOG | R1A/R3 | R1A tests; UAT-05 |
| G-04 | Component/lane widths vary by station | SEMANTIC_MODEL | R1A contract | R1A | R1A fixtures |
| G-05 | Lane add/drop/taper use general lifecycle | SEMANTIC_MODEL | R1A contract | R1A | R1A fixtures; UAT-03 |
| G-06 | Turn pocket uses general lane lifecycle, not polygon overlay | DECISION_REGISTER | AI_COMMAND_CATALOG / Road Configuration Catalog | R1A/R3 | R1A proof; UAT-01 |
| G-07 | Equal semantic input yields deterministic equivalent output | ENGINEERING_CONSTITUTION | precision/tolerance policy | R1A+ | automated evidence |
| J-01 | Junction is first-class semantic object | SEMANTIC_MODEL | R1B contract | R1B | UAT-02 |
| J-02 | Geometry crossing does not silently create topology | SEMANTIC_MODEL | Candidate Junction UX / Selection-Snapping Model | R1B/R4 | UAT-01/02 |
| J-03 | Per-corner junction geometry is independently editable | UX/semantic baseline | R1B | R1B/R4 | UAT-02 |
| J-04 | Lane-to-lane connectivity is explicit semantic data | SEMANTIC_MODEL | R1B | R1B/R4 | UAT-02 |
| R-01 | 2D and 3D derive from one canonical semantic model | PRODUCT_VISION | RENDERER_CONTRACT | R1C | UAT-07 |
| R-02 | Renderer state/caches are disposable and non-authoritative | TECHNICAL_ARCHITECTURE | PROJECT_FILE_SCHEMA | R1C/R2 | rebuild tests, UAT-11 |
| R-03 | Large coordinates use local render-origin strategy | TECHNICAL_ARCHITECTURE | RENDERER_CONTRACT | R1C | large-coordinate fixture / UAT-07 |
| M-01 | Map/reference data is separate from engineering model | MAP_BASEMAP_POLICY | REFERENCE_DATA_QUALITY_MODEL | R3/R7 | UAT-06 |
| M-02 | User image/site plan supports scale calibration | GOLDEN_WORKFLOWS | Low-Fidelity Workspace | R3 | UAT-06 |
| M-03 | Provider digitization/cache/export rights are enforced | MAP_BASEMAP_POLICY | EXPORT_POLICY | R7 | provider capability tests/UAT-06 |
| M-04 | Source quality does not imply false precision | USER_PAINPOINTS | REFERENCE_DATA_QUALITY_MODEL | R3+ | UAT-06 |
| S-01 | Existing and alternatives are first-class scenarios | PRODUCT_VISION | SEMANTIC_MODEL | R6 | UAT-01/03 |
| S-02 | Editing one scenario cannot mutate another | SEMANTIC_MODEL | PROJECT_FILE_SCHEMA | R6 | UAT-01 |
| A-01 | Engineering markings are procedural/semantic | ASSET_SYSTEM | STARTER_ASSET_CATALOG | R5 | UAT-01/02 |
| A-02 | Sign/signal/light systems use semantic assemblies | ASSET_SYSTEM | STARTER_ASSET_CATALOG | R5/R6/R7 | asset QA |
| A-03 | One semantic asset may have 2D + 3D representations | ASSET_SYSTEM | ASSET_METADATA_SCHEMA | R6 | UAT-05/07 |
| A-04 | Repeated assets support deterministic distribution | ASSET_PRODUCTION_PIPELINE | Starter Asset Catalog | R6 | UAT-05 |
| STD-01 | Standards rules/defaults have authority/document/edition/page provenance | STANDARDS_POLICY | STANDARD_PROFILE_SCHEMA | R5/R7 | UAT-10 |
| STD-02 | Project pins standards profile version | STANDARDS_POLICY | PROJECT_FILE_SCHEMA | R5/R7 | UAT-10/11 |
| STD-03 | DOH/DRR are separate profiles, not one universal Thai standard | THAILAND_SOURCE_REGISTER | Thailand Extraction Backlog | R7 | source review |
| STD-04 | No unverified Thai numeric default is presented as authoritative | ENGINEERING_CONSTITUTION | STANDARDS_POLICY | all | scrutiny/UAT-10 |
| CMD-01 | Manual UI, AI, import, automation share one semantic mutation path | TECHNICAL_ARCHITECTURE | COMMAND_TRANSACTION_MODEL | R2+ | command tests |
| CMD-02 | Meaningful edits are undoable atomic transactions | COMMAND_TRANSACTION_MODEL | AI_COMMAND_CATALOG | R2/R3 | UAT-08 |
| CMD-03 | Preview does not mutate canonical state | COMMAND_TRANSACTION_MODEL | UX interaction flows | R2/R3 | UAT-08/09 |
| AI-01 | AI resolves intent to typed commands and shows proposal | PRODUCT_VISION | AI_COMMAND_CATALOG | later | UAT-09 |
| AI-02 | AI never directly edits mesh/SVG/project JSON | DECISION_REGISTER | COMMAND_TRANSACTION_MODEL | all/later | architecture tests/review |
| PERS-01 | Project uses versioned semantic schema | PROJECT_FILE_SCHEMA | DECISION_REGISTER | R2 | UAT-11 |
| PERS-02 | Save/reopen preserves stable semantic ids/state | PROJECT_FILE_SCHEMA | R2 contract later | R2 | UAT-11 |
| PERS-03 | Schema migrations are explicit/tested | PROJECT_FILE_SCHEMA | ROADMAP | R2+ | migration fixtures/UAT-11 |
| EXP-01 | Engineering export works without basemap | EXPORT_POLICY | MAP_BASEMAP_POLICY | R7 | UAT-01/06 |
| EXP-02 | Basemap inclusion only when rights permit | EXPORT_POLICY | MAP_BASEMAP_POLICY | R7 | UAT-06 |
| EXP-03 | High-resolution report-ready plan output | PRODUCT_VISION | EXPORT_POLICY | R7 | GW-06/UAT |
| Q-01 | Geometry work uses golden/adversarial/property/fuzz/benchmark evidence | ENGINEERING_CONSTITUTION | Engineering Development Workflow | R1+ | evidence package |
| Q-02 | Major UI flows pass UX Review Gate and relevant Golden UAT | UX_REVIEW_GATE | GOLDEN_UAT_CASES | R3+ | UAT evidence |
| Q-03 | Agent completion claim alone is insufficient | Engineering workflow | AGENTS.md | all | PR/CI/evidence |
| L-01 | Third-party dependencies require explicit version/license/adoption evidence | dependency governance | DEPENDENCY_LICENSE_REGISTER | R1+ | PR review |
| L-02 | Competitor code/assets are not copied without license rights | research policy | DEPENDENCY_LICENSE_REGISTER / Asset Pipeline | all | review/license evidence |

---

## Traceability rule for future features

A material new feature should identify:
1. product requirement/job;
2. semantic model impact;
3. command behavior;
4. UX flow;
5. standards/license/source impact;
6. roadmap stage;
7. automated acceptance;
8. human UAT case.

If several of these are unknown, the feature is not implementation-ready.

## Gap rule

If implementation discovers a requirement with no authoritative source or acceptance path:
- do not silently decide it in code;
- document the gap;
- route it to product/architecture review;
- update this matrix when resolved.
