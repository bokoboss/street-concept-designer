# Starter Asset Catalog

This catalog defines capability targets, not final artwork. Standards-sensitive assets require source verification before they are presented as official.

## Procedural road markings

Priority A:
- solid lane line;
- dashed lane line;
- double line;
- edge line;
- stop line;
- yield line;
- through arrow;
- left/right-turn arrows;
- U-turn arrow;
- combination movement arrows;
- zebra crosswalk;
- hatch/chevron/gore area;
- parking stall;
- motorcycle box / advanced stop area;
- bicycle symbol.

Each marking should be parameterized by geometry such as width, dash/gap, stripe spacing, orientation, repeat, offset, and standards profile.

## Semantic traffic-control assemblies

Priority A:
- generic sign face + single pole;
- warning/regulatory starter sign shapes;
- signal pole;
- signal head;
- pedestrian signal head;
- streetlight;
- bollard;
- delineator;
- guardrail;
- concrete barrier;
- traffic cone.

Later:
- mast arm;
- gantry;
- guide-sign assemblies;
- bus-stop shelter;
- railing/fence variants.

## Vehicle props

Starter physical classes:
- sedan;
- pickup/SUV;
- motorcycle;
- van;
- city bus;
- coach;
- rigid truck;
- articulated truck.

Vehicle assets should preserve real dimensions and usable orientation/pivots. A 2D footprint/top symbol and 3D representation belong to the same semantic asset record.

## Context / landscape

- small / medium / large tree families;
- shrub/planting proxy;
- grass/ground material;
- generic people;
- simple building massing;
- bench/basic street furniture.

## Placement behavior

Every prop should support point placement. Suitable categories should additionally support:
- along-path spacing;
- curb/edge-relative offset;
- alternating sides;
- area distribution;
- deterministic seed where distribution is persisted;
- bake/explode to individually editable objects.

## Asset production responsibility

The user is not expected to create any starter artwork or model manually. Engineering assets should be generated through code/vector/parametric assembly. Complex presentation assets may be created through automated tooling or sourced from permissively licensed libraries with explicit provenance and normalization.
