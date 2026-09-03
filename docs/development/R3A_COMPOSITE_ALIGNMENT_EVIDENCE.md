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

The R3A workflow adds Linux native/WASM and Windows/MSVC qualification. Hosted
run ids and the pull-request id must be appended after the branch is pushed;
this record must not be read as a claim that hosted qualification or
independent review has completed.
