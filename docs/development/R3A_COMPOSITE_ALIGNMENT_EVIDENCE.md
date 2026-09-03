# R3A Composite Alignment Execution Evidence

This record is execution evidence for R3A Issue #31. It is not independent
review or final engineering acceptance.

## Baseline and scope

- Accepted base: `43bb019906240dd49edf735d32ef969ab498cee3`
- Execution branch: `codex/r3a-composite-alignment`
- Initial worktree: clean; target branch and `origin/main` both resolved to the
  accepted base before implementation.
- Scope: kernel composite alignment, R1C shared derivation compatibility,
  junction compatibility, schema-v2 persistence with v1/v0 migration, R2C
  transaction coverage, qualification tests, and bounded benchmarks.
- Excluded: UI, Tauri/React/Vite/PixiJS, wasm-bindgen, maps, standards values,
  editor-intent commands, and R3B+ work.

## Implemented semantic contract

- `Alignment` owns one validated ordered segment sequence and one cumulative
  station domain.
- Every segment has a persisted stable id and one accepted primitive: line,
  circular arc, or smooth conceptual curve.
- Construction rejects duplicate ids, gaps above the existing coordinate
  tolerance, tangent discontinuities above the existing angular tolerance,
  invalid lengths, and non-finite inputs.
- Whole-alignment point/tangent/normal/projection/sampling operations map local
  primitive stations to deterministic global stations. Sampling emits every
  segment boundary exactly once.
- Existing `Road`, `CrossSection`, lane lifecycle, junction, shared 2D/3D
  derivation, and R2C command/history paths remain the semantic owners.
- The canonical writer emits schema v2 segment records. Schema v1 single
  primitive records receive deterministic id `segment-0`; the synthetic
  pre-release v0 fixture remains readable.

## Local qualification

The local executable run used the installed full MinGW distribution through a
command-scoped Cargo linker override because the default Rust GNU toolchain
could not find `libgcc_eh`/`libgcc`. No repository or system configuration was
changed.

Command:

```text
cargo +stable-x86_64-pc-windows-gnu clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo +stable-x86_64-pc-windows-gnu test --locked --workspace --all-targets --all-features -- --nocapture
```

Result: pass. The run included all inherited R1/R2 tests and the R3A suites:

- kernel composite alignment: 8 passed;
- project-io composite persistence/migration: 4 passed;
- project-session composite history/persistence: 3 passed;
- inherited workspace tests: pass.

Representative local benchmark results (Windows GNU, Rust 1.98.0):

| segments | construction/validation | sampling | projection | R1C rebuild |
| ---: | ---: | ---: | ---: | ---: |
| 3 | 3.679 us/op | 3.365 us/op | 626.150 ns/op | 11.907 us/op |
| 10 | 11.899 us/op | 9.184 us/op | 1,823.630 ns/op | 28.846 us/op |
| 50 | 68.968 us/op | 48.298 us/op | 8,288.060 ns/op | 140.375 us/op |

ProjectSession local benchmarks also ran for 3/10/50 segments, covering
Project clone, preview, and commit paths.

## Hosted qualification

Pull request [#32](https://github.com/bokoboss/street-concept-designer/pull/32)
is open against `main` and remains unmerged.  The runtime implementation is
in commit `60517c05743d22d3e95f972425e6ffbc23a0f250`; the current PR head also
contains the documentation-only follow-up commit
`3b55e07541b82d13d8bead70248f7261fca4dce6`.

The qualification runs for both commits completed successfully:

- implementation commit R3A Composite Alignment Qualification: run `33738726949`
  ([Linux/WASM/benchmarks and Windows/MSVC](https://github.com/bokoboss/street-concept-designer/actions/runs/33738726949));
- implementation commit Engineering Workflow Integrity: run `33738726978`;
- implementation commit inherited R1A/R1B/R1C qualification: runs `33738726910`, `33738726929`,
  `33738726938`;
- implementation commit inherited R2A/R2B/R2C qualification: runs `33738726930`, `33738726943`,
  `33738726950`.
- documentation follow-up commit R3A qualification: run `33739301696`;
- documentation follow-up commit inherited/workflow qualification: runs
  `33739301713`, `33739301678`, `33739301646`, `33739301671`, `33739301667`,
  `33739301723`, `33739301648`.

All listed runs completed successfully.  The final PR check state is
authoritative for the current head because this evidence record is itself
versioned on the branch.  This is execution evidence, not independent review
or final engineering acceptance; the PR was intentionally left unmerged.
