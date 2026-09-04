# Pascal Editor / Nature Plugin Prior-Art Review

Date: 2026-09-03

## Decision supported

Determine whether the Pascal Editor ecosystem contains architecture or implementation patterns worth carrying into Street Concept Designer without changing the current R3 scope, weakening the semantic/kernel boundary, or prematurely adopting a 3D/plugin stack.

## External evidence inspected

Repositories:

- Pascal Editor: https://github.com/pascalorg/editor
  - inspected snapshot: `19327a98748db383b394643494f9fa3af220906d`
  - repository license: MIT
- Pascal Nature / Trees plugin: https://github.com/pascalorg/plugin-trees
  - inspected snapshot: `56c847fe329722a76af5fcf4e823aae9fa911d34`
  - repository license: MIT
  - procedural tree dependency declared by the plugin: `@dgreenheck/ez-tree`, MIT according to the plugin repository

Important inspected areas included:

- Pascal repository/package separation between core scene contracts, viewer, editor tools/UI, node definitions, CLI/MCP, and shared UI;
- registry-driven node definitions;
- pure parametric geometry builders;
- dirty/change-driven geometry rebuild;
- 2D floor-plan representations separate from 3D representations;
- selection/object registry patterns;
- procedural vegetation generation;
- deterministic geometry variants;
- instanced rendering;
- per-node selection proxies for collectively rendered instances;
- declarative parametric inspectors;
- placement footprints/capabilities;
- runtime versus baked/export representation.

This record treats those repositories as prior art and implementation evidence only. It does not import their product assumptions into this project.

## Verified project constraints

Street Concept Designer already requires:

- canonical semantic engineering state above all renderer state;
- one shared engineering derivation for 2D and 3D;
- Rust kernel/project/session ownership;
- renderer objects and caches to remain disposable;
- manual UI, AI, import, and automation to converge on typed semantic transactions;
- user workflows that do not require Illustrator, Blender, or manual asset production;
- Windows-first desktop/offline operation;
- rich 3D and large asset libraries only after the engineering editor is trustworthy.

Therefore a useful external pattern must fit those constraints rather than replace them.

## High-value patterns

### 1. Pure representation builders

Pascal's registry distinguishes node data from functions that build render geometry. Builders are intended to be deterministic/pure and operate in local space.

Street Concept Designer should carry the same high-level principle into future asset and presentation work:

```text
semantic/project state
        ↓
accepted kernel / derived engineering snapshot
        ↓
representation builder
        ↓
2D primitive or 3D mesh/buffer
```

For engineering roads/junctions, the builder must consume accepted derived engineering geometry rather than recalculate lane edges, topology, widths, or junction semantics in Three.js.

### 2. Geometry-variant state versus per-instance state

The Nature plugin separates parameters that change generated mesh geometry from parameters that can be expressed by an instance transform.

Example concept:

```text
geometry variant:
  species / seed / foliage / shape / LOD

instance:
  semantic id / position / rotation / scale
```

This is valuable for future trees, vehicles, people, street furniture, lights, and other repeated presentation assets.

Street Concept Designer should make this distinction explicit enough that a transform edit does not regenerate expensive geometry unnecessarily.

### 3. Deterministic bounded variation

The Nature plugin uses deterministic seeds and a bounded seed pool so many visually varied assets can still share geometry variants.

For Street Concept Designer, any persisted generated distribution or presentation variation should be reproducible across save/reopen/rebuild/export. Bounded variant pools are a preferred performance technique where visual quality remains acceptable.

### 4. Variant caching and instanced rendering

Repeated assets with the same geometry variant can share geometry/material resources and render through instancing/batching.

This is particularly relevant to future R6/R8 scenes containing hundreds of:

- trees;
- vehicles;
- motorcycles;
- people;
- bollards;
- delineators;
- streetlights;
- repeated street furniture.

Instancing is a renderer optimization only. It must not collapse semantic object identity.

### 5. Selection identity through optimized rendering

The Nature plugin demonstrates a useful hybrid: collective instanced rendering for scale plus lightweight per-node/proxy behavior for selection and outline.

Street Concept Designer may use direct instance-index → semantic-id mapping, proxy render objects, or another proven method. Whichever method is chosen, the authoritative selection value remains the semantic id and a renderer handle remains disposable.

### 6. One semantic asset, multiple representations

The same Pascal plant node can provide a distinct 2D floor-plan symbol and a richer 3D representation.

This directly supports the existing Street Concept Designer asset contract:

```text
semantic asset
  ├── physical dimensions / metadata
  ├── 2D plan representation
  ├── 3D representation
  ├── thumbnail
  └── export representation
```

2D should not need to project the 3D mesh, and the 3D mesh must not become the source of engineering dimensions.

### 7. Declarative capability / parametric descriptors

Pascal node definitions can declare properties, capabilities, placement rules, inspector fields, tools, and render contributions without hard-coding every kind into the host.

A future Street Concept Designer internal asset-definition registry could similarly drive:

- property inspector fields;
- placement affordances;
- 2D/3D representation choice;
- instancing policy;
- asset provenance/metadata;
- typed AI/tool descriptions.

However all persistent changes must still produce normal Street Concept Designer semantic commands/transactions. A descriptor must not gain a bypass around `ProjectSession`.

### 8. Runtime representation versus export representation

Optimized runtime rendering may use instancing, LODs, proxies, caches, or lightweight materials while export uses a deterministic baked/static representation.

This separation is compatible with the current renderer architecture as long as both are reproducible from the same semantic state and representation definitions.

### 9. Change-driven/incremental regeneration

Pascal's dirty-node systems are useful evidence that selective rebuild can materially reduce renderer work.

Street Concept Designer should not copy this mechanism directly. Its later optimization should originate from semantic transaction/change sets and must prove equivalence against a clean full rebuild.

## Patterns not adopted

### Pascal scene/store as canonical project state

Not applicable. Street Concept Designer already has a Rust canonical Project/ProjectSession boundary. React/Zustand-style state may own ephemeral UI/view state only.

### Renderer-side engineering geometry

Not applicable for roads, lane lifecycles, junction semantics, topology, or standards-sensitive geometry. Three.js/PixiJS consume derived outputs; they do not decide the engineering design.

### Public plugin ecosystem during the core roadmap

Deferred. The useful current lesson is registry-driven internal composition, not a third-party plugin API or marketplace. Public plugin/API work remains post-core/post-R9 unless separately justified.

### Pascal Editor as a framework/base or code fork

Rejected. The product domain, persistence model, transaction model, desktop constraints, and engineering correctness requirements differ materially.

### WebGPU as an automatic production choice

Not adopted. Pascal proves viability in its environment, not compatibility with Street Concept Designer's Windows office-PC/Portable target. WebGL/WebGPU choice remains evidence-gated and must be qualified on representative hardware.

### CSG as the primary road/junction construction method

Rejected for canonical road/junction engineering geometry. CSG may be useful for presentation/context objects but must not replace station/cross-section/topology-derived engineering geometry.

## Roadmap implications

### R3

No implementation scope expansion.

Use the prior art only to reinforce:

- semantic state versus renderer state;
- declarative UI/representation thinking;
- direct semantic selection identity;
- future-proof representation contracts.

Three.js, vegetation, asset instancing, and plugin infrastructure remain out of R3.

### R4-R5

Allow internal descriptors/registries to emerge only where they simplify repeated semantic features, markings, or attachments. Do not create a general plugin system merely in anticipation of future assets.

### R6

Before rich 3D/starter assets are accepted, explicitly qualify:

- pure/deterministic representation builders;
- semantic asset → distinct 2D/3D representations;
- geometry-variant versus instance-state separation;
- deterministic variation seeds/pools;
- variant cache and instancing/batching;
- selection identity for batched/instanced assets;
- runtime versus baked/export representation;
- Three.js renderer choice and WebGL/WebGPU compatibility on representative Windows hardware.

### R8

Harden:

- asset LOD/instancing thresholds;
- GPU resource disposal/cache lifecycle;
- incremental/change-set regeneration versus clean rebuild;
- large-scene selection/outline behavior;
- renderer settle/readiness;
- office-PC performance and renderer fallback behavior.

## Licensing implication

Both inspected Pascal repositories are MIT-licensed, so they are permissive references. This review does not authorize copying code. Any future direct code reuse or dependency adoption still requires the normal dependency/license register, attribution/notice handling, compatibility review, and project-specific acceptance evidence.

## Research verdict

**GO WITH CONDITIONS**

Use Pascal Editor and the Nature plugin as architecture/prior-art references for future representation, asset, selection, and performance design.

Conditions:

1. no R3 scope expansion;
2. no Pascal scene/store ownership of canonical engineering state;
3. no renderer-side reimplementation of road/junction engineering semantics;
4. no public plugin API before a separately justified later gate;
5. Three.js/WebGPU/ez-tree or any other dependency remains separately evidence-gated;
6. deterministic semantic identity and reproducible project output remain mandatory through batching, instancing, LOD, and export.
