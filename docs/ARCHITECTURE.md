# Architecture

`pen-atomic` is a deterministic strict-only workspace with a single hot-path language:
Rust. Agda is reserved for export and verification.

## Current engine state

The current executable surface is intentionally narrow and honest:

- `pen-cli run` and `pen-cli resume` perform real live atomic bootstrap search through
  step 10 and deterministic reference replay beyond that.
- Both paths write the same real run artifacts, checkpoints, reports, and telemetry
  expected by the frozen storage contract.
- The frontier search engine, shard-backed working state, and resume-from-frontier path
  are still scaffolded rather than fully implemented.

## Dependency graph

```text
pen-core
   ^
pen-type        pen-store
   ^               ^
pen-eval          |
   ^               |
   └────── pen-search ──────┐
            ^       ^       |
         pen-accel  |     pen-agda
                \\  |       /
                  pen-cli
```

## Rules

- `pen-core` contains the frozen anonymous AST, IDs, exact arithmetic helpers, and
  canonical structural helpers.
- `pen-store` owns on-disk contracts and memory-governor surfaces only.
- `pen-search` orchestrates search and resume, but does not own semantic truth.
- `pen-cli` owns human presentation and post-hoc reporting only.
- `pen-agda` never influences acceptance decisions.

## Artifact flow

1. `pen-cli run` or `pen-cli resume` writes `run.json`, `config.toml`, telemetry, step
   summaries, and step checkpoints under `runs/<run-id>/`.
2. `pen-cli inspect` reads those frozen artifacts without mutating them.
3. `pen-cli export-agda` or `xtask export-reference-agda` turns accepted steps into
   Agda stubs plus an export manifest.
4. Optional Agda verification logs stay next to the generated modules and never feed
   back into search ranking or acceptance.
