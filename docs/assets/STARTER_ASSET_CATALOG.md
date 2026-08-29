# Starter Asset Catalog

## Purpose

Define the first coherent asset library needed for the Golden Workflows. This is a production backlog/schema input, not a request for the user to manually draw or model assets.

Every bundled asset follows:
- stable semantic id;
- physical units in metres where applicable;
- 2D representation;
- 3D representation or procedural generator where applicable;
- placement/attachment semantics;
- provenance/license metadata;
- standards verification state where relevant.

Priority:
- P0 — needed for core engineering workflows;
- P1 — needed for strong professional/presentation workflow soon after core;
- P2 — later breadth/polish.

Verification:
- GENERIC — no jurisdiction-specific geometry claim;
- SOURCE_GATED — must not ship as authoritative Thailand profile until source/page verification;
- PRESENTATION — non-engineering decorative asset.

---

# A. Procedural road markings

These should be generated from parameters, not stored as raster artwork.

| ID | Name | Priority | 2D | 3D | Verification |
|---|---|---:|---|---|---|
| roadmark.line.solid | Solid line | P0 | procedural vector | decal/mesh | SOURCE_GATED when profile-defaulted |
| roadmark.line.dashed | Dashed line | P0 | procedural vector | decal/mesh | SOURCE_GATED |
| roadmark.line.double_solid | Double solid | P0 | procedural vector | decal/mesh | SOURCE_GATED |
| roadmark.line.solid_dashed | Solid + dashed pair | P1 | procedural vector | decal/mesh | SOURCE_GATED |
| roadmark.edge | Edge line | P0 | procedural vector | decal/mesh | SOURCE_GATED |
| roadmark.stop_line | Stop line | P0 | procedural rectangle/path | decal/mesh | SOURCE_GATED |
| roadmark.yield_line | Yield line / triangles | P1 | procedural repeated symbol | decal/mesh | SOURCE_GATED |
| roadmark.crosswalk.zebra | Zebra crossing | P0 | procedural stripe generator | decal/mesh | SOURCE_GATED |
| roadmark.hatch.chevron | Hatch / chevron | P0 | procedural area fill | decal/mesh | SOURCE_GATED |
| roadmark.gore | Gore marking | P1 | procedural boundary/fill | decal/mesh | SOURCE_GATED |
| roadmark.parking.stall | Parking stall | P1 | procedural | decal/mesh | SOURCE_GATED/project-specific |
| roadmark.motorcycle.box | Motorcycle box | P1 | procedural/vector symbol | decal/mesh | SOURCE_GATED |
| roadmark.bicycle.symbol | Bicycle symbol | P1 | vector/procedural stencil | decal/mesh | SOURCE_GATED |
| roadmark.text.custom | Custom road text | P2 | text-to-vector | decal | CUSTOM/non-authoritative |

## Required marking parameters

Common:
- color;
- width;
- material/reflective class later;
- station/attachment;
- lateral offset;
- orientation;
- profile provenance.

Repeated markings:
- dash length;
- gap length;
- repeat phase.

Crosswalk:
- overall width;
- stripe width;
- gap;
- crossing orientation;
- attachment to approach/junction.

Hatch:
- boundary;
- stripe angle;
- stripe width;
- gap.

---

# B. Pavement arrows / stencils

Prefer parameterized vector master shapes that can render both 2D and 3D decals.

| ID | Name | Priority | Verification |
|---|---|---:|---|
| roadmark.arrow.through | Through arrow | P0 | SOURCE_GATED |
| roadmark.arrow.left | Left-turn arrow | P0 | SOURCE_GATED |
| roadmark.arrow.right | Right-turn arrow | P0 | SOURCE_GATED |
| roadmark.arrow.through_left | Through + left | P1 | SOURCE_GATED |
| roadmark.arrow.through_right | Through + right | P1 | SOURCE_GATED |
| roadmark.arrow.left_right | Left + right | P2 | SOURCE_GATED |
| roadmark.arrow.uturn | U-turn arrow | P1 | SOURCE_GATED |
| roadmark.arrow.merge_left | Merge left | P1 | SOURCE_GATED |
| roadmark.arrow.merge_right | Merge right | P1 | SOURCE_GATED |
| roadmark.stencil.bus | BUS stencil | P2 | SOURCE_GATED/project-specific |
| roadmark.stencil.bike | Bicycle stencil | P1 | SOURCE_GATED |

Asset production:
- programmatically generate SVG/vector from verified dimensions;
- create thumbnail automatically;
- derive 3D decal/mesh from same master vector;
- do not redraw manually in Illustrator.

---

# C. Traffic signs — semantic face + support assemblies

A traffic sign is not one monolithic GLB.

## Face families

| ID family | Description | Priority | Verification |
|---|---|---:|---|
| traffic.sign.regulatory.* | regulatory/prohibitory/mandatory | P1 | SOURCE_GATED |
| traffic.sign.warning.* | warning | P1 | SOURCE_GATED |
| traffic.sign.guide.* | guide/directional starter | P2 | SOURCE_GATED |
| traffic.sign.information.* | information/service | P2 | SOURCE_GATED |
| traffic.sign.custom | project/custom concept sign | P2 | CUSTOM |

## Support assemblies

| ID | Description | Priority |
|---|---|---:|
| traffic.sign.support.single_pole | single pole | P1 |
| traffic.sign.support.double_pole | double pole | P2 |
| traffic.sign.support.mast | mast/cantilever | P2 |
| traffic.sign.support.gantry | overhead gantry | P2 |

## Sign schema intent

Face:
- family/code;
- shape;
- physical face dimensions;
- background/border/color;
- symbol/text composition;
- orientation/facing;
- authority/profile/source.

Support:
- support type;
- pole dimensions;
- mounting height;
- lateral offset;
- foundation footprint proxy where useful.

Representations:
- 2D plan symbol/face direction;
- generated low-poly 3D assembly;
- high-quality 3D later if needed.

---

# D. Traffic signals

Semantic assembly first; no timing simulation in starter scope.

| ID | Description | Priority | Verification |
|---|---|---:|---|
| traffic.signal.head.vehicle.3aspect | red/amber/green head | P1 | SOURCE_GATED for exact housing/mounting |
| traffic.signal.head.arrow | arrow head | P2 | SOURCE_GATED |
| traffic.signal.head.pedestrian | pedestrian signal head | P2 | SOURCE_GATED |
| traffic.signal.support.pole | simple pole | P1 | SOURCE_GATED/generic proxy initially |
| traffic.signal.support.mast_arm | mast arm | P1 | SOURCE_GATED/generic proxy initially |
| traffic.signal.support.gantry | gantry | P2 | SOURCE_GATED |

Assembly metadata:
- pole/support;
- head list;
- head orientation;
- mounting positions;
- semantic controlled approach/movement later;
- profile/source.

Early 3D may use generated engineering proxies; detailed photorealistic signal equipment is not required.

---

# E. Safety / channelization / roadside devices

| ID | Name | Priority | Production | Verification |
|---|---|---:|---|---|
| safety.bollard.flexible | flexible bollard | P1 | procedural/simple mesh | SOURCE_GATED/generic mode |
| safety.delineator.post | delineator post | P1 | generated/simple GLB | SOURCE_GATED |
| safety.cone | traffic cone | P1 | procedural/simple GLB | GENERIC/presentation unless work-zone profile |
| safety.barrier.concrete | concrete barrier | P1 | procedural segment | SOURCE_GATED/generic mode |
| safety.guardrail.wbeam | W-beam guardrail | P1 | procedural repeated assembly | SOURCE_GATED |
| safety.railing.pedestrian | pedestrian railing | P2 | procedural repeated assembly | SOURCE_GATED/custom |
| safety.fence.generic | generic fence | P2 | procedural repeated assembly | GENERIC |
| safety.curb.separator | modular curb separator | P2 | procedural repeated assembly | SOURCE_GATED/custom |

Placement:
- point;
- along-edge/path;
- repeated spacing;
- start/end station;
- offset;
- orientation.

---

# F. Lighting / poles

| ID | Name | Priority | Production | Verification |
|---|---|---:|---|---|
| street.light.single_arm | single-arm streetlight | P1 | parametric assembly | SOURCE_GATED for Thailand profile |
| street.light.double_arm | double-arm streetlight | P1 | parametric assembly | SOURCE_GATED |
| street.light.highmast | high mast | P2 | assembly | SOURCE_GATED |
| utility.pole.generic | generic utility pole | P2 | simple mesh | GENERIC |

Parametric lighting metadata:
- pole height;
- arm length;
- arm angle;
- luminaire;
- mounting orientation;
- lateral offset;
- source/profile.

Photometric design is outside starter asset scope.

---

# G. Transit / street furniture

| ID | Name | Priority | Production | Verification |
|---|---|---:|---|---|
| transit.bus_stop.sign | bus-stop sign | P2 | semantic sign assembly | custom/profile |
| transit.bus_shelter.basic | simple bus shelter | P2 | generated/basic GLB | GENERIC |
| furniture.bench.basic | bench | P2 | simple GLB | PRESENTATION |
| furniture.bin.basic | bin | P2 | simple GLB | PRESENTATION |
| furniture.bike_rack.basic | bike rack | P2 | simple GLB | PRESENTATION |

These must not delay road/junction geometry.

---

# H. Vehicles

Vehicles are semantic props with accurate class dimensions and coherent top/3D representations. Use generic unbranded models.

## P0/P1 starter classes

| ID | Class | Priority | 2D | 3D |
|---|---|---:|---|---|
| vehicle.car.sedan.generic | passenger car/sedan | P0 | generated/vector top view | normalized GLB/proxy |
| vehicle.car.suv.generic | SUV | P1 | vector | GLB/proxy |
| vehicle.pickup.generic | pickup | P0/P1 | vector | GLB/proxy |
| vehicle.motorcycle.generic | motorcycle | P0/P1 | vector | GLB/proxy |
| vehicle.van.generic | van | P1 | vector | GLB/proxy |
| vehicle.bus.city.12m | city bus ~class identifier | P0/P1 | vector | GLB/proxy |
| vehicle.bus.coach.generic | coach | P1 | vector | GLB/proxy |
| vehicle.truck.rigid.medium | rigid truck | P0/P1 | vector | GLB/proxy |
| vehicle.truck.articulated.generic | articulated truck | P1 | vector | GLB/proxy |

Important:
- numeric physical dimensions must be stored as data, not inferred from mesh scale;
- starter ids describe generic classes, not legal/design-vehicle standards;
- later swept-path/design-vehicle modules require separately verified engineering vehicle profiles.

Potential use before swept path:
- presentation;
- scale reference;
- queue/movement visualization later.

---

# I. Landscape

| ID | Name | Priority | 2D | 3D | Notes |
|---|---|---:|---|---|---|
| landscape.tree.small.generic | small canopy tree | P1 | procedural canopy circle/symbol | instanced GLB | PRESENTATION |
| landscape.tree.medium.generic | medium tree | P1 | canopy footprint | instanced GLB | PRESENTATION |
| landscape.tree.large.generic | large tree | P2 | canopy footprint | instanced GLB | PRESENTATION |
| landscape.palm.generic | palm | P2 | footprint | GLB | PRESENTATION |
| landscape.shrub.generic | shrub | P2 | area/point symbol | instanced GLB | PRESENTATION |
| landscape.grass.area | grass material area | P2 | area fill | material | PRESENTATION |

Metadata:
- trunk anchor;
- canopy diameter;
- nominal height;
- collision/clearance relevance flag later;
- LOD/instancing compatibility.

Engineering mode prioritizes canopy footprint clarity over visual realism.

---

# J. Buildings / context

| ID | Name | Priority | Production |
|---|---|---:|---|
| context.building.massing | simple massing building | P1/P2 | procedural footprint extrusion |
| context.wall.generic | wall | P2 | path extrusion |
| context.gate.basic | gate | P2 | simple assembly |
| context.curb_existing_proxy | existing curb/reference proxy | P2 | procedural |

Initial building workflow:
- draw/import footprint;
- enter height;
- extrude simple mass;
- neutral engineering material.

Do not build architectural BIM features.

---

# K. Road component materials / presentation

P1/P2 material presets:
- asphalt neutral;
- concrete;
- curb concrete;
- sidewalk concrete/pavers generic;
- grass;
- island/median surface;
- marking white/yellow/other profile-controlled;
- building neutral.

Material presets are presentation properties and must not encode engineering dimensions.

---

# L. 2D-only engineering symbols / overlays

These are UI/editor overlays rather than placed project props.

P0:
- alignment control point;
- tangent handle;
- station marker;
- dimension line;
- selected lane highlight;
- direction arrow overlay;
- candidate junction marker;
- lane-connection movement overlay;
- snap glyphs;
- warning/error marker;
- locked/reference-only state.

They should be generated through the editor renderer/UI system rather than asset-library files where practical.

---

# M. Thailand starter pack build order

Do not create a single mixed “Thai” folder without authority/version context.

Recommended sequence:

## TH-0 — generic engineering generators
Can be built before standards extraction:
- line generator;
- crosswalk generator;
- hatch generator;
- arrow rendering pipeline;
- sign-face/assembly engine;
- signal assembly engine;
- lighting assembly engine;
- barrier/guardrail assembly engine.

Defaults remain generic/unverified.

## TH-1 — DOH profile
After page-level verification:
- core lane/edge/stop markings;
- arrows/stencils;
- high-use regulatory signs;
- high-use warning signs;
- selected guide signs;
- standard sign supports;
- signal/lighting/safety assets where source coverage is adequate.

## TH-2 — DRR profile
Separate verified defaults/asset definitions where DRR differs.

## TH-CUSTOM
Project/company-specific values with explicit provenance and no false authority claim.

---

# N. First built-in library target

The first useful library should be small and coherent.

Recommended first complete bundle:

Engineering:
- solid/dashed/double line;
- stop line;
- zebra crossing;
- hatch/chevron;
- through/left/right/U-turn arrows;
- generic sign assembly;
- generic signal assembly;
- bollard;
- concrete barrier;
- guardrail;
- streetlight.

Presentation:
- sedan;
- pickup;
- motorcycle;
- bus;
- rigid truck;
- three generic tree sizes;
- simple building massing.

This is enough to make GW-01/GW-02/GW-05 look complete without spending months on art production.

---

# O. Asset acceptance gate

An asset is not “done” merely because it renders.

Required where applicable:
- semantic id stable;
- physical dimensions present;
- orientation/pivot convention valid;
- 2D representation valid;
- 3D representation/generator valid;
- metadata schema passes;
- thumbnail generated;
- placement modes tested;
- deterministic generation/loading;
- performance reasonable;
- license/provenance complete;
- standards profile/source status explicit;
- no user manual-art prerequisite.

## Prohibition

Do not import/copy competitor application assets simply because they are publicly viewable. Public visibility is not a license.
