# Command & Transaction Model

## Purpose

All meaningful engineering edits should pass through one deterministic command/transaction layer. Manual UI, keyboard actions, future AI Copilot, import transformations, and scripted automation must converge on the same semantic mutation path.

This prevents:
- separate AI/manual behavior;
- renderer-driven state corruption;
- fragmented undo logic;
- hidden side effects;
- unreviewable automation.

## State categories

### Canonical engineering state
Persistent project content:
- roads/alignments;
- semantic cross sections/lane lifecycles;
- junctions/topology;
- engineering markings/assets;
- scenarios;
- standards-profile pins;
- engineering annotations where applicable.

### Presentation/project display state
Persistent when useful but not engineering truth:
- layer visibility;
- materials/presentation settings;
- saved cameras;
- basemap/reference display settings;
- presentation asset variations.

### Ephemeral UI state
Not project engineering content:
- hover;
- current mouse position;
- open menu;
- transient selection rectangle;
- drag preview;
- uncommitted property text;
- temporary measurement overlays.

Selection may be session state even when restored for convenience; it must not influence engineering output.

## Command characteristics

A committed command should have, conceptually:
- command type/version;
- stable command id;
- target/project/scenario ids;
- explicit parameters in engineering units;
- preconditions/expected target state when relevant;
- deterministic validation;
- semantic result/change set;
- human-readable summary;
- provenance/actor metadata where useful;
- undo/inverse strategy;
- optional source intent metadata (manual, import, AI proposal, generator).

Do not store DOM nodes, renderer handles, screen pixels, or raw LLM prose as authoritative command parameters.

## Preview vs commit

Interactive editing often needs high-frequency preview without polluting undo history.

Preferred pattern:

```text
Begin interaction
  -> transient preview(s)
  -> validate candidate semantic state
  -> Commit one meaningful command
```

Examples:
- dragging a corner radius handle generates visual/geometry previews but commits one final `SetCornerGeometry` operation;
- typing lane width may update preview after valid parse but commits at edit completion/confirmation;
- AI proposal previews a complete semantic transaction before Apply.

Preview must never become an independent shadow model that diverges from canonical semantics.

## Transaction

A user-intent operation may require several lower-level semantic edits but should undo as one transaction.

Example `AddRightTurnPocket` may internally:
- create a lane/component lifecycle;
- add width-profile points;
- attach it to a target approach/road;
- generate profile-aware markings;
- update derived connectivity candidates.

The history should expose one meaningful user operation unless advanced inspection is requested.

## Candidate/proposal transaction

Potentially large automatic changes should be generated against a candidate state:

```text
base semantic state
  + proposed transaction
  -> candidate state
  -> validation/change summary
  -> preview
  -> Apply or Cancel
```

This is mandatory for AI-authored changes and recommended for substantial junction/configuration regeneration.

## Preconditions and stale edits

Commands operating on a target whose relevant state changed since proposal generation should fail/rebase explicitly rather than silently applying to the wrong geometry.

AI proposals in particular should identify stable semantic ids plus relevant expected state/version, not rely on phrases such as “the road on the left”.

## Undo / redo

Requirements:
- deterministic semantic reversal;
- no dependence on current screen/render state;
- generated children/attachments revert coherently with their parent transaction;
- redo recreates equivalent semantic state;
- failed commands do not enter history;
- transient preview does not enter history;
- imported/AI/generated operations are not privileged and undo normally.

Implementation may use inverse commands, snapshots, immutable structural sharing, or another proven approach; R0 does not lock the storage mechanism.

## Human-readable history

History should describe user intent, for example:
- Draw Road R01
- Change lane width to 3.25 m
- Add right-turn pocket (30 m taper, 50 m storage)
- Create junction J01
- Set NE corner radius to 12 m
- Add crosswalk to Approach A
- Duplicate Alternative A as Alternative B

Avoid exposing low-level renderer mutations as history items.

## Command families — product-level intent

Likely families include:

### Project / scenario
- CreateProject
- AddScenario
- DuplicateScenario
- RenameScenario
- SetStandardsProfile

### Alignment / road
- CreateRoad
- EditAlignment
- SplitRoad
- JoinRoad
- ApplyRoadConfiguration

### Cross section / longitudinal
- AddComponent
- RemoveComponent
- ReorderComponent
- SetComponentWidth
- SetWidthProfile
- AddLaneTransition
- AddTurnPocket
- AddMedianOpening

### Junction / topology
- CreateJunctionFromCandidate
- IgnoreJunctionCandidate
- SetCornerGeometry
- Add/RemoveLaneConnection
- SetMovement
- AddIsland
- AddCrosswalk

### Markings / assets
- AddMarking
- UpdateMarkingParameters
- PlaceAsset
- CreateAssetDistribution
- BakeAssetDistribution

### Reference / presentation
- AddReferenceLayer
- CalibrateReferenceImage
- SetReferenceDisplay
- AddSavedView

Exact class/type structure is an implementation decision. These names define semantic intent only.

## Generated content ownership

Generated markings/assets should retain provenance to the generator/profile/parent semantic object.

When a user overrides generated content, the model must distinguish:
- regenerated/default state;
- user override;
- custom concept/manual object;
- standards/profile source.

Regeneration must not silently erase intentional user overrides.

## AI command path

AI is an intent parser/planner, not an alternate mutation engine.

Required path:

```text
Natural-language request
 -> resolve semantic targets
 -> build typed command/transaction proposal
 -> validate
 -> explain change
 -> preview
 -> user Apply / Cancel
 -> normal history + undo
```

If required target/parameter meaning is ambiguous, ask/resolve through UI context rather than silently guessing engineering intent.

AI output must never directly edit:
- renderer mesh;
- SVG paths;
- DOM/Canvas state;
- project JSON text;
- database rows bypassing domain validation.

## Import path

Importers should also produce validated semantic transactions or a candidate project model, not inject renderer-native data as canonical content.

An OSM/DXF/future import should preserve source provenance and require user verification where the source is not authoritative engineering geometry.

## Validation integration

Two distinct checks are needed:

### Command validity
Can this operation produce a coherent semantic state?
Examples: target exists, station is valid, widths are non-negative.

### Engineering advisory validation
Is the resulting coherent state within a selected standard/guidance profile?
Examples: lane width/taper advisory warning.

An advisory standards warning usually should not reject a coherent command; an internally impossible geometry should.

## Determinism and serialization

Where commands are persisted/replayed:
- versions must be explicit;
- engineering numbers/units must be unambiguous;
- ordering must be deterministic;
- random presentation operations use recorded seeds when result persistence matters;
- project migration must not silently reinterpret old command semantics.

R0 does not require event sourcing. Canonical project state may remain snapshot-based while commands provide editing/history semantics.

## Testing requirements

Command-layer tests should eventually prove:
- command + undo returns equivalent semantic state;
- command + undo + redo returns equivalent post-command state;
- preview does not mutate canonical state;
- invalid commands do not partially apply;
- multi-step transaction is atomic from user history perspective;
- AI/manual construction of equivalent typed command yields equivalent result;
- generated-child ownership survives parent edits/undo;
- stale target/precondition behavior is explicit and deterministic.

## Architecture invariant

**Every editor path may differ in how it captures intent, but all committed engineering changes must converge on one validated semantic transaction system.**
