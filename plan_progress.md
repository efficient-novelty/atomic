# Plan Progress

Last updated: 2026-03-13

## Objective

Build the frozen v1 `pen-atomic` Rust-first workspace in this repository, starting from stable contracts first:

1. workspace and tooling scaffolding
2. `pen-core`
3. `pen-store`
4. schemas and round-trip tests
5. `pen-type`
6. `pen-eval`
7. `pen-search`
8. `pen-cli`
9. `pen-agda`
10. `pen-accel`

## Assumptions

- This repository root will host the `pen-atomic` workspace even though the folder name is currently `atomic`.
- Existing `skills/` and `tex/` content stays in place unless a later task proves it should move.
- The workspace currently pins GNU for continuity, but the canonical `stable-x86_64-pc-windows-msvc` toolchain is now installed and verified on this machine.

## Plan

| ID | Task | Status | Done when | Learnings |
| --- | --- | --- | --- | --- |
| P0 | Create execution plan and baseline assumptions | done | `plan_progress.md` exists with ordered phases and constraints | Repo currently contains only docs/context plus the new `.gitignore`; the usable Rust toolchain today is `stable-x86_64-pc-windows-gnu`. |
| P1 | Scaffold the frozen workspace tree and root project files | done | Root `Cargo.toml`, `rust-toolchain.toml`, `.cargo/config.toml`, configs, docs, schemas, crates, tests, scripts, and placeholder keep files exist | The repo now matches the frozen top-level layout closely enough to start real implementation without reworking the tree. |
| P2 | Make the workspace compile with minimal crate stubs | done | `cargo check --workspace` succeeds with placeholder APIs across all crates | The repo-wide check passes after pinning the workspace toolchain to `stable-x86_64-pc-windows-gnu`, which matches the working linker path on this PC. |
| P3 | Implement `pen-core` v1 foundations | done | Frozen atom enum, ID newtypes, exact rational helpers, core module exports, and baseline tests exist | The frozen atom order, exact prefix-bit base costs, and rational normalization are now executable and covered by unit tests. |
| P4 | Implement `pen-store` manifests and checkpoint contracts | done | Run/step/frontier manifest structs and serialization tests exist | `pen-store` now has explicit run, step, and frontier manifest structs with round-trip tests grounded in the frozen JSON examples. |
| P5 | Add schema artifacts and round-trip validation tests | done | JSON schema files and tests cover manifest/checkpoint round-trips | The schema files are now generated from the Rust types through `xtask`, and the manifest/checkpoint round-trip tests still pass on the typed payloads. |
| P6 | Pull donor semantics needed for `pen-core` and `pen-store` parity | in_progress | Exact source-backed notes or ports exist for encoding, telescope, canonicalization, and compatibility hashing | Donor-backed Rust ports now cover the MBTT AST, telescope/reference fixtures, canonical keys, conservative checkpoint payloads, and structural capability detection. |
| P7 | Implement `pen-type` and `pen-eval` strict foundations | done | Conservative typing, normalization/equality skeletons, exact bar arithmetic, and `nu` stubs/tests exist | `pen-eval` now carries donor-backed d-bonacci windowing, exact rational `bar`/`rho` arithmetic, structural minimality helpers, and a structural `nu` evaluator that reproduces the frozen 15-step donor totals under test. |
| P8 | Implement deterministic `pen-search` resume/search scaffolding | in_progress | Runtime config, resume policy, frontier state record, and deterministic scheduling skeleton compile and are tested | `pen-search` now parses the frozen TOML profiles, packs/unpacks the 64-byte frontier record, computes stable priority keys, manages a compact sortable frontier window, and enforces the checkpoint resume matrix under test. |
| P9 | Implement `pen-cli`, `pen-agda`, and `xtask` integration surfaces | pending | CLI commands, Agda export shell, and repo maintenance tasks compile | Pending |
| P10 | Harden with fixtures, deterministic replay, and documentation updates | pending | Integration tests and architecture docs cover the frozen contracts | Pending |

## Progress Log

### 2026-03-13

- Completed `P0`.
- Completed `P1`.
- Completed `P2`.
- Completed `P3`.
- Completed `P4`.
- Completed `P7`.
- Current focus: the remaining `P6` donor-backed parity work and `P8` deterministic search scaffolding.
- Tooling update: Visual Studio Build Tools with VC tools are installed, and the MSVC Rust toolchain is verified with both a standalone smoke build and `cargo +stable-x86_64-pc-windows-msvc check --workspace`.
- Tooling update: the repository default toolchain has now been switched back to MSVC and the workspace, `pen-core`, `pen-eval`, and `pen-search` test suites are green under MSVC.
- Learning from `P1`: it is safe to scaffold the frozen workspace directly in this repo root while leaving `skills/` and `tex/` untouched.
- Learning from `P2`: a generic `stable` pin resolves to MSVC on this Windows host, so the workspace must currently pin the GNU toolchain to remain buildable in this non-elevated environment.
- Learning from `P3`: the atom family, exact prefix costs, and rational arithmetic are all precise enough in the local skill context to implement without donor-source ambiguity.
- Learning from `P4`: the frozen manifest examples are precise enough to encode now, but the nested checkpoint payloads need exact donor-backed structural types before their schemas should be tightened.
- Learning from the donor repo: the Haskell convenience layer carried entry names, but the Rust hot path can stay faithful to the semantics while remaining anonymous and structural.
- Learning from `P5`: generating schemas from the Rust types is much safer than hand-maintaining placeholder JSON, especially while `pen-core` structures are still actively being ported.
- Learning from the start of `P7`: the conservative checker ports cleanly because it depends only on scope, binders, and library-window sizes, not on the full dependent-equality engine.
- Learning from tooling verification: this repo still defaults to GNU only because [`rust-toolchain.toml`](C:\DEV\atomic\rust-toolchain.toml) is pinned there; the machine itself is now ready for MSVC builds once we switch that override.
- Learning from the finished `P7`: the donor structural-`nu` formulas port cleanly as long as we preserve anonymous structural inputs and use capability-bearing library entries instead of semantic names or hard-coded labels.
- Learning from the start of `P8`: the frozen search contracts are strong enough to implement and test config parsing, frontier record packing, and resume policy decisions before any real enumeration or worker logic exists.
