# Golden UAT Cases

## Purpose

Translate Golden Workflows into testable product acceptance scenarios. These are human/domain UAT cases, not substitutes for kernel unit/property/fuzz tests.

Use realistic traffic-engineering tasks rather than synthetic UI-tour acceptance.

## Common acceptance principles

For every case:
- the user should not need CAD/Illustrator/Blender;
- engineering dimensions remain inspectable;
- direct manipulation and numeric properties agree;
- 2D/3D derive from the same semantic state where 3D is in scope;
- undo/redo operates on meaningful semantic transactions;
- no restricted basemap is required for success;
- presentation quality must not conceal invalid geometry;
- generated standards-sensitive content exposes source/profile status.

---

# UAT-01 — Project access improvement

Primary product acceptance case.

## User goal

Create a concept for a new project access on a divided road, including a right-turn pocket and median opening, compare an alternative, and export a plan.

## Starting data

Reference:
- user-provided aerial/site plan or permitted basemap;
- known scale or A-B calibration distance.

Concept road:
- divided 4-lane road;
- 2 lanes each direction;
- nominal lane width entered by user;
- median.

Access:
- one project driveway/side road.

Turn pocket:
- right-turn movement;
- user-entered width;
- user-entered taper;
- user-entered storage length.

## Steps

1. Create project in metric/LHT mode.
2. Import and calibrate reference if needed.
3. Draw main road alignment.
4. Apply a 4-lane divided road configuration.
5. Select/edit lane widths and median width.
6. Add project access.
7. Resolve resulting candidate junction explicitly.
8. Add right-turn pocket using parametric command/panel.
9. Add/edit median opening.
10. Add/generate relevant concept markings.
11. Switch to Split 2D/3D.
12. Select the turn pocket in 2D and confirm same semantic object highlights in 3D.
13. Duplicate Alternative A → Alternative B.
14. Modify one meaningful geometry parameter in B.
15. Compare A/B.
16. Export engineering-only plan.
17. Undo/redo a major operation and confirm coherent restoration.

## Must-pass observations

- User never draws a pocket polygon manually.
- Taper/storage dimensions are inspectable and editable.
- Pocket uses one lane lifecycle model.
- Candidate junction requires user confirmation.
- 3D reflects the same width/taper change.
- Existing/Alternative state does not cross-contaminate.
- Engineering-only export works without embedding the reference image.
- Undo/redo does not leave orphan markings/assets/lane fragments.

## Failure examples

- right-turn pocket becomes a detached graphic;
- changing road alignment leaves pocket behind;
- 3D uses stale lane width;
- creating the access silently changes topology before confirmation;
- export fails because basemap cannot be embedded;
- Alt B edits modify Alt A.

---

# UAT-02 — Skewed four-leg intersection improvement

## User goal

Create/refine a skewed four-leg intersection with unequal approach widths, turn lanes, corner radii, movement connections, crossings, and an island.

## Starting geometry

- main road and side road intersect at a non-90° angle;
- at least one approach has an additional turn lane;
- approach widths are unequal.

## Steps

1. Draw/import two road alignments that geometrically cross.
2. Observe Candidate Junction.
3. Choose Create Junction.
4. Review generated approaches/corners.
5. Set different corner radii on at least two corners.
6. Add/modify a turn pocket on one approach.
7. Review lane-to-lane connections.
8. Remove one inappropriate automatic connection and add a valid one.
9. Add a traffic island.
10. Add a crosswalk and stop line.
11. Inspect validation/issues.
12. Move one approach alignment slightly and verify junction regeneration/response.
13. Undo the movement.
14. Inspect synchronized 3D later when available.

## Must-pass observations

- junction is a semantic object, not only pavement union;
- per-corner geometry is independently editable;
- movement connections use stable lane ids;
- geometry crossing and topology remain conceptually separate;
- generated junction content does not silently destroy manual overrides;
- no NaN/self-intersection/disconnected pavement after accepted geometry operations.

---

# UAT-03 — Road widening 4 lanes → 6 lanes

## User goal

Prepare an alternative widening a 4-lane divided road to 6 lanes while preserving reference alignment and producing longitudinal transitions.

## Steps

1. Create Existing: 4-lane divided road.
2. Duplicate as Alternative A.
3. Add one traffic lane per direction over a defined station range.
4. Configure taper/transition at beginning/end where needed.
5. Adjust median/shoulder/sidewalk widths.
6. Inspect plan footprint.
7. Compare Existing vs Alternative A.
8. Select each new lane and inspect lifecycle/width profile.
9. Export overlay/plan.

## Must-pass observations

- lane addition uses station-based lifecycle;
- alignment is not duplicated as unrelated edge polygons;
- width changes are deterministic;
- compare clearly distinguishes scenarios;
- no forced global cross-section change where only a station range changes.

---

# UAT-04 — Median opening / U-turn concept

## User goal

Create a median opening and later a U-turn treatment without introducing a one-off geometry model.

## Steps

1. Create divided road.
2. Select median and add opening.
3. Set opening location/range.
4. Configure permitted movement.
5. Add U-turn pocket when feature becomes available.
6. Enter width/taper/storage.
7. Review movement/connectivity.
8. Inspect generated markings.
9. Change median width and confirm attached feature updates coherently.

## Must-pass observations

- median opening is semantically attached to road/median;
- U-turn pocket uses general lane lifecycle;
- no custom hand-drawn polygon is required;
- geometry remains editable after upstream road changes.

---

# UAT-05 — Urban street / complete-street concept

## User goal

Create a quick urban street alternative including traffic lanes, bike lane, parking, sidewalk/verge, trees, and lighting.

## Steps

1. Draw road alignment.
2. Apply base urban configuration.
3. Open cross-section editor.
4. Add/reorder:
   - traffic lanes;
   - bike lane;
   - parking lane;
   - sidewalk/verge.
5. Enter exact widths.
6. Add trees along an edge at fixed spacing.
7. Add streetlights along an edge at fixed spacing.
8. Bake one distribution and move one exception manually.
9. View in 3D presentation mode later.
10. Export plan.

## Must-pass observations

- cross-section editor is understandable without raw geometry editing;
- array/distribution assets remain deterministic;
- baked exception does not destroy other array items;
- 2D and 3D asset representation belongs to the same semantic object;
- decorative assets do not affect engineering validation unless explicitly relevant.

---

# UAT-06 — Reference image calibration and rights-safe export

## User goal

Use a non-georeferenced site plan accurately enough for concept work and export the engineering design without legal/technical dependency on the reference.

## Steps

1. Import JPG/PNG.
2. Set A and B.
3. Enter known distance.
4. Verify scale indicator.
5. Rotate/set north optionally.
6. Lock reference.
7. Draw measured road/feature.
8. Change reference opacity/dim/saturation.
9. Export with reference excluded.
10. Where permitted, export with reference included.

## Must-pass observations

- engineering dimensions are in metres after calibration;
- display changes do not alter geometry;
- reference lock prevents accidental edits;
- recalibration is explicit and does not silently transform engineering model;
- export availability follows source capability/policy.

---

# UAT-07 — 2D / 3D semantic synchronization

## User goal

Verify that plan and 3D are two views of one model.

## Steps

1. Load road with:
   - curve;
   - median;
   - widening;
   - turn pocket.
2. Open Split view.
3. Select lane in 2D.
4. Confirm same semantic id selected in 3D.
5. Change width numerically.
6. Confirm 2D/3D regenerate.
7. Drag supported 2D handle.
8. Confirm properties and 3D update.
9. Delete renderer cache/rebuild in diagnostic environment later.
10. Confirm equivalent output.

## Must-pass observations

- no separate renderer-owned lane width;
- selection ids match;
- 3D is regenerable from semantic state;
- large-coordinate project remains visually stable after local render origin is applied.

---

# UAT-08 — Undo / redo / history integrity

## User goal

Trust the editor during real design exploration.

## Script

Perform:
1. create road;
2. apply configuration;
3. add access;
4. create junction;
5. add turn pocket;
6. change corner radius;
7. add crosswalk;
8. place asset array.

Then undo each operation one by one and redo all.

## Must-pass observations

- no orphan semantic children;
- no partial transaction states;
- redo yields equivalent semantic state;
- history descriptions reflect user intent;
- transient preview events do not flood history;
- failed/invalid operation never enters history.

---

# UAT-09 — AI proposal safety

Applicable only after typed command layer and AI integration exist.

## User request

“เพิ่มช่องเลี้ยวขวากว้าง 3 เมตร taper 30 เมตร storage 50 เมตร ก่อนทางเข้าโครงการนี้”

## Must-pass behavior

1. AI resolves selected/explicit road/access.
2. If target is ambiguous, it asks/resolves before proposing.
3. Proposal shows:
   - target;
   - movement;
   - width;
   - taper;
   - storage.
4. Preview uses candidate semantic state.
5. Cancel leaves canonical state unchanged.
6. Apply enters normal transaction history.
7. Undo works exactly like the equivalent manual command.

## Hard failures

- AI edits mesh/SVG/project JSON directly;
- AI selects a target solely from visual phrases without stable id/context;
- AI silently fabricates standard values;
- AI applies without review when meaningful engineering ambiguity exists.

---

# UAT-10 — Standards advisory provenance

Applicable after StandardProfile implementation.

## User goal

Use guidance without allowing software to falsely certify the design.

## Steps

1. Pin a verified standards profile/version.
2. Enter a dimension outside preferred range but still geometrically coherent.
3. Observe advisory.
4. Open Details.
5. Inspect:
   - authority;
   - document;
   - edition;
   - page/table/figure;
   - rule id;
   - applicability;
   - severity.
6. Override/keep design with rationale if policy allows.
7. Reopen project later.

## Must-pass observations

- project remains pinned to original profile version;
- later profile updates do not silently alter existing project interpretation;
- warning is distinct from invalid geometry;
- source is traceable.

---

# UAT-11 — Project save/reopen/schema integrity

Applicable when persistence exists.

## Steps

1. Build a non-trivial GW-01 project.
2. Save.
3. Close app.
4. Reopen.
5. Compare canonical semantic state.
6. Regenerate 2D/3D.
7. Remove any disposable renderer cache and reopen/rebuild.
8. Upgrade through one test schema migration when migrations exist.

## Must-pass observations

- semantic ids remain stable;
- renderer cache is not required;
- project schema/version is explicit;
- migration is deterministic and tested;
- source/reference provenance survives.

---

# UAT-12 — Performance perception on normal concept project

Performance budgets will be evidence-driven rather than invented, but subjective acceptance still matters.

## Scene

At least:
- several roads;
- one four-leg junction;
- turn pockets;
- markings;
- basic asset arrays;
- map/reference layer;
- split 2D/3D later.

## User operations

- pan/zoom;
- select;
- drag alignment handle;
- edit width;
- add/remove turn pocket;
- switch scenarios;
- switch 2D/Split/3D.

## Acceptance

- interaction feels immediate for normal editing;
- high-cost robust regeneration may use preview-during-drag and commit-on-release;
- application explains long work if an exceptional operation cannot be immediate;
- no operation freezes the UI due to avoidable synchronous presentation work.

---

# UAT evidence template

For each UAT run record:
- app/build/version/commit;
- platform/hardware;
- project/fixture;
- exact steps;
- pass/fail by required observation;
- screenshots/video references if relevant;
- defects/issues;
- engineering judgment notes;
- accepted limitations;
- final UAT decision.

## Gate principle

A stage is not accepted because the agent says “implemented”. Relevant Golden UAT cases must become runnable and eventually pass at the milestone where their prerequisites exist.


---

# UAT-13 — Windows installer / portable office-PC deployment

Applicable during R8/R9 packaging qualification.

## User goal

Use the same Street Concept Designer project on a normal office computer whether conventional installation is allowed or not.

## Test profiles

### Profile A — Per-user Installer

Machine/account:
- supported Windows x64;
- standard non-Administrator user;
- no development toolchain assumed.

Steps:
1. launch signed Setup artifact;
2. install for current user without elevation;
3. launch application;
4. create/open a project;
5. save project outside the application installation folder;
6. close/reopen;
7. uninstall;
8. confirm project file remains;
9. reinstall/upgrade qualified version;
10. reopen the same project.

Must pass:
- no administrator privilege;
- no Node/Rust/Python prerequisite;
- WebView2 missing-runtime case handled according to installer profile;
- project files survive uninstall.

### Profile B — Offline Installer

Machine:
- no internet during setup.

Steps:
1. disconnect network;
2. run Offline Setup;
3. install without fetching runtime/content from the internet;
4. launch;
5. create/save/open local project;
6. use local reference image;
7. verify core 2D editing;
8. verify local 3D when available.

Must pass:
- setup does not require Microsoft/provider CDN access;
- core application works offline after install.

### Profile C — Portable Light

Machine/account:
- standard user;
- compatible system WebView2 already present;
- no installation performed.

Steps:
1. extract Portable ZIP to writable local folder;
2. run `StreetConceptDesigner.exe`;
3. verify no installer/UAC elevation flow;
4. create/open/save project;
5. import local site-plan image;
6. perform representative road edit;
7. close/reopen;
8. move/copy Portable application folder to another qualified local folder;
9. launch again;
10. open same external project file.

Must pass:
- no installation;
- no admin;
- no development runtime/toolchain;
- no Windows service/local HTTP server dependency;
- project semantics independent of executable location;
- writable WebView2/app-data strategy works.

### Profile D — Portable Offline

Machine:
- no installed WebView2 assumption;
- network disconnected.

Steps:
1. extract Portable Offline ZIP;
2. launch;
3. verify bundled/runtime strategy is used;
4. create/open/save project;
5. restart application;
6. verify local 2D/3D/reference-image operations;
7. inspect runtime/app version diagnostics.

Must pass:
- no installation/admin;
- no internet;
- bundled runtime starts correctly;
- runtime servicing/version is identifiable;
- package does not rely on a hidden system installation.

## Path matrix

At least test:
- ASCII local path;
- Thai/Unicode folder name;
- path containing spaces;
- reasonably deep/long project path;
- application folder and project folder separated.

Network shares are not required as a WebView2 UDF target and should not be treated as a default supported runtime-data path.

## Enterprise/security observations

Record behavior under:
- Microsoft Defender/SmartScreen;
- signed publisher identity;
- Controlled Folder Access where practical;
- blocked/non-writable runtime-data folder;
- corporate proxy/no internet.

The application must not bypass enterprise security policy. A blocked condition should produce an actionable diagnostic where technically possible.

## Cross-mode project compatibility

Create a project in one qualified distribution mode and open it in another.

Must pass:
- same schema/semantic state;
- stable ids;
- no installation-mode-specific fields in canonical project content;
- renderer/cache rebuild succeeds.

## Failure examples

- Portable launches only on a developer PC because Node/Rust is installed;
- executable opens a localhost server that corporate firewall policy blocks;
- application requires admin to write beside the executable;
- WebView2 UDF cannot be created and user sees only a blank window;
- Portable mode writes canonical projects into a disposable app/cache directory;
- uninstall deletes user project files;
- Portable Offline contains a fixed runtime with no identifiable/update servicing policy;
- unsigned binary is treated as acceptable release evidence despite office policy failures.
