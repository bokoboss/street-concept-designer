# Competitor Research Synthesis

Verified: 2026-08-25

Purpose: identify interaction patterns, semantic architecture ideas, and failure modes worth learning from. This is research only; no competitor code/assets are to be copied without explicit license review.

## Streetcraft Studio

Official source: https://www.streetcraft.studio/

Observed strengths:
- starts from aerial/satellite imagery;
- auto-scale workflow from a known real-world distance;
- very fast road/intersection/roundabout/pavement-marking placement;
- before/after communication and high-resolution export;
- large preset library reduces drafting effort.

Observed constraints:
- requires Adobe Illustrator;
- static-visual workflow rather than semantic network model;
- toolkit is internally built around 1 px = 1 ft and 11 ft default lanes even though Auto Scale can use metric references;
- traffic simulation is external.

Takeaway for us:
- keep the map/image-first speed and scale-calibration experience;
- replace static graphics/presets with metric-native semantic generators;
- no external graphics editor dependency.

## Streetmix

Official sources:
- https://streetmix.net/
- https://docs.streetmix.net/

Observed strengths:
- extremely approachable component-based street cross-section editing;
- immediate visual feedback;
- segments/components communicate intent clearly;
- reusable component mental model supports lanes, sidewalks, parking, trees, lamps, etc.

Important architecture lesson:
Streetmix documentation describes street segments as assemblies of components and notes that naive variant combinations can create data-complexity growth. Newer schema work moves toward composable component definitions rather than enumerating every visual variant.

Takeaway for us:
- adopt the ease of rearranging cross-section components;
- keep presets as generators of editable semantic components;
- do not make a single cross-section the full road source-of-truth because longitudinal changes/intersections are core to our product.

## Remix Streets

Official sources:
- https://ridewithvia.com/solutions/remix/streets
- https://ridewithvia.com/resources/how-to-communicate-with-accuracy-and-clarity-using-remix-streets-advanced-editing
- https://ridewithvia.com/resources/visual-storytelling-made-better-with-remix-streets-advanced-editing-2-0

Observed strengths:
- plan/network-view conceptual design rather than cross-section only;
- map/satellite context and measurement;
- lane tapers, transitions, curves, street corners, intersections;
- drag-based corner/radius editing;
- crosswalks, stop bars, lane extensions;
- strong alternative/stakeholder communication workflow.

Takeaway for us:
- plan view must be the primary authoring environment;
- taper/transition/intersection geometry is core, not advanced polish;
- direct manipulation should coexist with exact numeric engineering values;
- concept-design speed is a legitimate product objective separate from detailed CAD.

## 3DStreet

Official sources:
- https://www.3dstreet.com/docs/
- https://www.3dstreet.com/docs/3dstreet-editor/overview-3dstreet-editor/
- https://www.3dstreet.com/docs/managed-street/overview-managed-street/
- https://www.3dstreet.com/docs/key-features/geospatial/

Observed strengths:
- interactive 3D street scene editor;
- precise property panel plus layer-oriented editing;
- geospatial location/context;
- Managed Street procedural representation;
- glTF export and custom GLB asset upload;
- scene/layer organization has become increasingly important as scenes grow.

Takeaway for us:
- 3D should be generated procedurally from the same semantic street data;
- asset scale/orientation/provenance should be normalized so drag-and-drop placement is reliable;
- layers, geospatial controls, and properties must have clear top-level information architecture;
- do not let scene-object manipulation become the authoritative road model.

## MathWorks RoadRunner

Official sources:
- https://www.mathworks.com/help/roadrunner/index.html
- https://www.mathworks.com/help/roadrunner/ref/roadplantool.html
- https://www.mathworks.com/solutions/automated-driving/roadrunner-tutorial/customizing-lanes.html
- https://www.mathworks.com/help/roadrunner/ref/road.html
- https://www.mathworks.com/help/driving/ref/lane.html

Observed strengths:
- road layout based on a 2D reference curve/reference lane;
- roads and lanes have explicit semantic objects and travel direction;
- dedicated lane add/width/marking/carve tools;
- intersections, maneuver paths, signals, signs, props, GIS/aerial context;
- OpenDRIVE and 3D-scene interoperability;
- programmatic authoring APIs enable automated scene generation/testing.

Takeaway for us:
- reference alignment/lane semantics are proven mature patterns;
- turn lanes should be represented as lane geometry/lifecycle, not painted polygons;
- explicit lane connectivity is valuable even before simulation;
- borrow semantic depth, not RoadRunner's full modal-tool complexity;
- unlike RoadRunner's default automatic-junction behavior, our editor should require explicit confirmation before geometry crossing becomes topology.

## ArcGIS CityEngine Street Designer

Official sources:
- https://doc.arcgis.com/en/cityengine/latest/help/street-designer-overview.htm
- https://doc.arcgis.com/en/cityengine/latest/help/street-designer-street-configuration.htm
- https://doc.arcgis.com/en/cityengine/latest/help/street-designer-edit-lanes.htm
- https://doc.arcgis.com/en/cityengine/latest/whats-new/cityengine-whats-new.htm

Observed strengths:
- edit/add/remove lane tools;
- reusable street configurations;
- lane width/direction properties;
- direct manipulation plus Inspector editing;
- CityEngine 2026 adds per-corner curb radii, node cleanup/merge/remove tools, improved selection, and additional transit/pedestrian lane content.

Takeaway for us:
- reusable road configurations are valuable but must resolve to editable components;
- per-corner junction geometry belongs in the semantic model;
- node cleanup becomes important once imports/tracing/network editing are supported;
- selection hierarchy/visual feedback needs deliberate design as network complexity grows.

## Autodesk InfraWorks

Official sources:
- https://help.autodesk.com/cloudhelp/ENG/InfraWorks-RoadsandHighways/files/GUID-75F9DBE1-08B7-4644-9A90-3B212727C13B.htm
- https://help.autodesk.com/cloudhelp/ENU/InfraWorks-RoadsandHighways/files/GUID-CB9A5DDF-B391-4977-80AA-064BE5C8CB24.htm
- https://help.autodesk.com/cloudhelp/ENU/InfraWorks-RoadsandHighways/files/GUID-0FE326CE-D7E1-4FD0-907E-CA868175D42D.htm

Observed strengths:
- component roads assembled from lanes, curbs, gutters, medians, shoulders, sidewalks;
- custom reusable road assemblies;
- components can start/end along a road;
- Transition In/Out creates linear tapering along station ranges;
- direct in-canvas grips plus exact values;
- component-road annotations expose dynamic geometry values.

Takeaway for us:
- longitudinal component start/end and width transition are first-class primitives;
- exact parameter editing and canvas manipulation should update the same semantic data;
- avoid importing InfraWorks' detailed grading/drainage/earthworks scope into the core concept product.

## Cross-product synthesis

Patterns to adopt:
1. map/aerial/site-plan context from the first workflow step;
2. reference-alignment based roads;
3. component-based cross sections with reusable configurations;
4. station-based longitudinal transitions;
5. plan-view direct manipulation plus exact numeric editing;
6. semantic junctions and per-corner controls;
7. explicit lane movement/connectivity data;
8. layer/property organization for complex scenes;
9. synchronized procedural 3D;
10. automated/reusable asset placement and export-ready presentation.

Patterns to avoid:
1. graphics/SVG/mesh as engineering truth;
2. static preset libraries as the permanent data model;
3. cross-section-only authoring;
4. automatic topology mutation without user confirmation;
5. modal-tool proliferation that forces users to learn a CAD-sized toolbar;
6. hard-coded jurisdiction-specific values throughout geometry code;
7. presentation assets mixed into engineering validation;
8. scope creep into detailed civil design before core concept workflows are excellent.

## Product gap we should own

The intended gap is a tool with Streetcraft/Streetmix-level approachability, Remix-style plan editing, RoadRunner/CityEngine/InfraWorks-style semantic geometry, and 3DStreet-style immediate 3D communication—focused specifically on rapid traffic-engineering street/intersection concepts rather than detailed CAD or simulation.
