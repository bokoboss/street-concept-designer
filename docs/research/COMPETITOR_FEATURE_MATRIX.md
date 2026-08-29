# Competitor Feature Matrix

Research synthesis baseline: 2026-08-27

Legend:
- ✓ strong/core observed capability
- ◐ partial/limited/secondary capability
- — not a core observed capability for the compared product/use case
- ? not established by current research

This matrix is comparative research, not a procurement evaluation and not permission to copy code/assets/UI.

| Capability | Streetcraft | Streetmix | Remix Streets | 3DStreet | RoadRunner | CityEngine Street Designer | InfraWorks | Street Concept Designer target |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| Start from aerial/map context | ✓ | — | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| User image/site-plan calibration | ✓ | — | ◐ | ◐ | ◐ | ◐ | ◐ | ✓ |
| Simple blank-canvas start | ◐ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Metric-native engineering model | ◐ | ◐ | ✓ | ◐ | ✓ | ✓ | ✓ | ✓ |
| LHT/RHT configurable | ◐ | ◐ | ◐ | ◐ | ✓ | ✓/context-dependent | ✓ | ✓ |
| Fast preset/template workflow | ✓ | ✓ | ◐ | ✓ | ◐ | ✓ | ✓ | ✓ generator-based |
| Preset resolves to editable semantics | — | ✓/section components | ◐ | ✓/managed street components | ✓ | ✓ | ✓ | ✓ |
| Primary plan-view road authoring | graphics workflow | — | ✓ | ◐/3D scene | ✓ | ✓ | ✓ | ✓ |
| Cross-section component editor | — | ✓ | ◐ | ✓ | ◐ | ✓ | ✓ | ✓ contextual |
| Arbitrary reference alignment | graphics/freeform | — | ✓ | ◐ | ✓ | ✓ | ✓ | ✓ |
| Line/arc/smooth road geometry | graphics | — | ✓ | ◐ | ✓ | ✓ | ✓ | ✓ |
| Longitudinal lane width change | graphic/manual | — | ✓ | ◐ | ✓ | ✓ | ✓ | ✓ |
| Taper / lane add/drop | graphic/manual | — | ✓ | ◐ | ✓ | ✓ | ✓ | ✓ core |
| Parametric turn pocket | graphics | — | ✓/editing | ✓/components limited | ✓ | ✓ | ✓ | ✓ core |
| Semantic lane identity/direction | — | section-level | ◐ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Semantic junction object | graphics | — | ✓ conceptual | ◐ | ✓ | ✓ | ✓ | ✓ |
| T / X / skew junction support | graphic presets | — | ✓ | ◐ | ✓ | ✓ | ✓ | ✓ |
| Per-corner junction geometry | manual graphics | — | ✓ | ◐ | ✓ | ✓ | ✓ | ✓ |
| Lane-to-lane connectivity | — | — | ? | ◐ | ✓ | ◐/network semantics | ◐ | ✓ |
| Explicit geometry ≠ topology confirmation | — | — | ? | — | automatic-heavy | ? | ? | ✓ target |
| Median / median opening | graphics | ✓ section | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| U-turn concept workflow | graphic assets | — | ◐ | ◐ | ◐ | ◐ | ◐ | NEXT on general primitives |
| Roundabout | graphic preset | — | ◐ | ◐ | ✓ | ✓ | ✓ | gated later module |
| Procedural road markings | graphics kit | section symbol | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Traffic sign/signal semantic assets | graphics | — | ◐ | ✓ | ✓ | ✓/procedural | ✓ | ✓ assemblies |
| Point asset placement | graphics | section props | ◐ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Along-path/edge repeated assets | manual graphics | — | ◐ | ◐ | ✓ | ✓ procedural | ✓ | ✓ |
| Area asset distribution | manual graphics | — | — | ◐ | ✓ | ✓ procedural | ✓ | ✓ |
| 2D plan + live synchronized 3D | external/manual | — | limited | 3D primary | 3D/road scene | ✓ | ✓ | ✓ core |
| One semantic model for 2D/3D | — | — | ? | ✓/managed scene concepts | ✓ | ✓ | ✓ | ✓ mandatory |
| Engineering 3D mode | — | — | — | ◐ | ✓ | ✓ | ✓ | ✓ |
| Presentation 3D mode | — | — | — | ✓ | ✓ | ✓ | ✓ | ✓ later |
| Geospatial CRS/context | image scale | — | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ progressively |
| Scenario/alternative comparison | manual copies | street share/variants | ✓ | scene/layers | ✓ workflow-dependent | ✓ scenario/manual | ✓ proposals | ✓ first-class |
| Before/after presentation | ✓ | ◐ | ✓ | ✓ | ◐ | ◐ | ✓ | ✓ |
| Saved views/cameras | graphics workflow | — | ◐ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Engineering advisory rules | — | — | —/planning context | — | domain tools | procedural/rules | design analysis | ✓ source-pinned advisory |
| Standards provenance/version pinning | — | — | — | — | specialized domain | rule packages | standards/configuration | ✓ explicit target |
| Local semantic project save | Illustrator file | ✓ web/share model | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| High-res plan export | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Vector export | ✓ via Illustrator | ✓/image/SVG context | ✓ | ◐ | ✓ | ✓ | ✓ | ✓ later |
| GLB/3D export | — | — | — | ✓ | ✓/interchange | ✓ | ✓ | ✓ later |
| OpenDRIVE interoperability | — | — | — | — | ✓ | — | — | later architecture-aware |
| Natural-language scene/edit command | — | — | — | ✓ | — | — | — | later typed-command Copilot |
| AI bypasses semantic model | n/a | n/a | n/a | product-specific | n/a | n/a | n/a | explicitly prohibited |
| No external graphics/CAD dependency | — Illustrator required | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Designed for traffic-engineering concept speed | ✓ visual | ✓ section | ✓ | ✓ visualization | ✓ automated-driving scene | ✓ urban/procedural | ✓ infrastructure concept | ✓ primary product identity |

## Interpretation

### Streetcraft contributes
- aerial-first speed;
- scale-calibration mental model;
- immediate before/after graphics;
- large starter vocabulary.

We should **not** inherit:
- Illustrator dependency;
- static graphic source-of-truth;
- imperial-template assumptions.

### Streetmix contributes
- approachable component-based cross section;
- fast drag/reorder mental model;
- visual clarity for non-CAD users.

We should **not** inherit:
- cross-section-only product boundary.

### Remix Streets contributes
- plan-view conceptual editing;
- tapers/transitions/intersection refinement;
- strong alternative/stakeholder workflow.

We should push further with:
- explicit semantic kernel;
- synchronized 3D;
- traffic-engineering topology/provenance.

### 3DStreet contributes
- procedural street-to-3D mindset;
- geospatial context;
- model library;
- layers/properties organization;
- natural-language editing precedent.

We should push further with:
- plan-first engineering authoring;
- exact road/junction semantics;
- deterministic typed commands;
- stronger separation of presentation from engineering validity.

### RoadRunner contributes
- road reference-line/lane semantics;
- lane carving/marking tools;
- junction/maneuver connectivity;
- strong programmatic scene model.

We should avoid:
- exposing a similarly large specialist-tool surface to normal concept users.

### CityEngine contributes
- street configurations;
- direct + inspector lane editing;
- procedural/network representation;
- per-corner radius and network-cleanup lessons.

We should focus more narrowly on:
traffic-engineering concept tasks rather than general procedural city generation.

### InfraWorks contributes
- component roads;
- longitudinal component start/end;
- transition/taper model;
- exact + direct component editing;
- conceptual infrastructure workflow.

We should exclude:
- grading/drainage/earthworks/BIM breadth from the core product.

---

# Product gap to own

The target is not merely the union of competitor checkmarks.

The intended unique combination is:

```text
Streetcraft / Streetmix approachability
        +
Remix plan-view concept speed
        +
RoadRunner / CityEngine / InfraWorks semantic geometry
        +
3DStreet immediate visual communication
        +
traffic-engineering topology, scenarios, provenance
        +
Thailand/LHT-first workflow
        +
automation / AI through deterministic semantic commands
```

## Key differentiators that should survive scope pressure

1. **Plan-first engineering authoring without CAD burden.**
2. **One model produces plan, section, and live 3D.**
3. **Turn lanes/tapers/junctions are semantic, not painted graphics.**
4. **Existing/alternatives are part of the normal workflow.**
5. **Standards guidance is traceable/advisory rather than falsely authoritative.**
6. **The user is not expected to produce 2D/3D art manually.**
7. **AI operates through the same deterministic command layer as manual editing.**
8. **Reference/basemap quality and rights are explicit, not hidden.**
