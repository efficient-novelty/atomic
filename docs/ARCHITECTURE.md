# Architecture

`pen-atomic` is a deterministic strict-only workspace with a single hot-path language:
Rust. Agda is reserved for export and verification.

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
