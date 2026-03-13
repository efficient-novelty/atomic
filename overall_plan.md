The current artifact is a two-layer system—Cubical Agda plus a Haskell synthesis engine—and the existing engine source already isolates the pieces worth donating into the rewrite: `Kolmogorov.hs`, `Telescope.hs`, `MBTTCanonical.hs`, `MBTTEnum.hs`, `MBTTNu.hs`, `StructuralNu.hs`, `CoherenceWindow.hs`, `StrictMinimality.hs`, `AgdaExport.hs`, `RunAbInitio.hs`, `StrictMolecules.hs`, and related modules. The paper also states that the current executable finding is a **strict bounded molecular-first recovery lane** over a fixed MBTT grammar and finite shell inventory, not open-ended atomic invention from scratch. ([GitHub][1])

Your strict notes are the key engineering warning: the old atomic attempt was not mainly failing because raw runtime was too slow, but because the frontier got dominated by generic macro junk before the sparse true eliminator structures could survive. That means the rewrite should preserve the reusable kernel semantics and Agda sidecar, but replace the generator, search loop, storage, and memory model completely. ([GitHub][2])

One more freeze up front: `pen-core/src/atom.rs` should mirror the current MBTT atom family exactly for v1—`App`, `Lam`, `Pi`, `Sigma`, `Univ`, `Var`, `Lib`, `Id`, `Refl`, `Susp`, `Trunc`, `PathCon`, `Flat`, `Sharp`, `Disc`, `Shape`, `Next`, `Eventually`. Any later atom change is an `ast_schema_hash` bump and invalidates frontier resume. ([GitHub][3])

## Frozen decisions

* New repo name: `pen-atomic`
* Language split:

  * **Rust**: all hot-path search, evaluation, storage, resume, reporting
  * **Cubical Agda**: sidecar export/verification only, never in the hot loop
* One semantic mode only: **strict**
* Two output styles only:

  * standard
  * `--debug`
* Search is **CPU-first and deterministic**
* GPU is **optional acceleration only**
* No semantic labels, names, or target IDs inside `pen-core`, `pen-type`, `pen-eval`, or `pen-search`
* No molecular/template crate at all
* No floating-point comparisons in the hot path; bars and `rho` are exact rational/integer comparisons
* Step checkpoints are the stable resume unit
* Frontier checkpoints are resumable only on full compatibility hash match

## Frozen workspace layout

```text
pen-atomic/
├── Cargo.toml                         # workspace members, shared release profiles
├── rust-toolchain.toml                # pinned stable toolchain channel
├── .cargo/
│   └── config.toml                    # cargo aliases only; no machine-specific target-cpu flags
├── README.md                          # repo entrypoint, strict-only philosophy, quickstart
├── LICENSE
│
├── configs/
│   ├── default.toml                   # portable defaults for any modern PC
│   ├── desktop_16gb.toml              # canonical 16 GB profile and hot-path budget
│   ├── cpu_only.toml                  # deterministic CPU-only profile
│   └── debug.toml                     # verbose diagnostics profile
│
├── docs/
│   ├── ARCHITECTURE.md                # overall system architecture
│   ├── DONOR_MAP.md                   # Haskell/Agda → Rust mapping and explicit non-port list
│   ├── SEARCH_CONTRACT.md             # admissibility, objective, prune classes, determinism rules
│   ├── CHECKPOINTS.md                 # checkpoint/resume contract and compatibility hashes
│   ├── MEMORY_MODEL.md                # resident-memory budget and governor states
│   ├── FRONTIER_STATE_V1.md           # fixed binary layout for frontier state records
│   └── AGDA_SIDECAR.md                # export and verification path
│
├── schemas/
│   ├── run_manifest_v1.schema.json
│   ├── step_checkpoint_v1.schema.json
│   ├── frontier_manifest_v1.schema.json
│   └── telemetry_event_v1.schema.json
│
├── agda/
│   ├── README.md                      # how generated exports are checked
│   ├── Prelude.agda                   # shared support definitions
│   ├── StepWitness.agda               # witness wrapper module template
│   └── Generated/
│       └── .gitkeep
│
├── crates/
│   ├── pen-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                 # public re-exports
│   │       ├── ids.rs                 # ExprId, ClauseId, StepId, StateId, ObligationSetId, BlobId
│   │       ├── rational.rs            # exact rational helpers for bar/rho comparisons
│   │       ├── atom.rs                # frozen MBTT atom enum
│   │       ├── expr.rs                # ExprNode and expression façade
│   │       ├── interner.rs            # compact immutable arenas and deduplicating interners
│   │       ├── clause.rs              # ClauseRec, ClauseRole, telescope-entry primitives
│   │       ├── telescope.rs           # anonymous candidate/library telescope representation
│   │       ├── library.rs             # library snapshot and d=2 active window views
│   │       ├── capability.rs          # structural capability flags derived from AST shape
│   │       ├── encode.rs              # prefix-free κ bit encoding and clause-cost accounting
│   │       ├── canonical.rs           # canonical keys and structural quotienting
│   │       ├── hash.rs                # stable hashes/fingerprints for blobs and dedupe
│   │       ├── pretty.rs              # debug pretty-printers only
│   │       └── stats.rs               # cheap structural counters for reporting/bounds
│   │
│   ├── pen-type/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                 # public checker API
│   │       ├── context.rs             # local contexts and library references
│   │       ├── infer.rs               # bidirectional inference
│   │       ├── check.rs               # bidirectional checking
│   │       ├── normalize.rs           # normalization / weak-head normalization
│   │       ├── equality.rs            # judgmental equality
│   │       ├── obligations.rs         # atomic proof-theoretic obligations on prefixes
│   │       ├── admissibility.rs       # exact-band admissibility and sealability checks
│   │       └── connectivity.rs        # connectedness / reference-window checks
│   │
│   ├── pen-eval/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                 # evaluator API
│   │       ├── bar.rs                 # d=2 bar dynamics and exact threshold arithmetic
│   │       ├── coherence.rs           # coherence-window computations and active interface
│   │       ├── nu.rs                  # native structural ν evaluator
│   │       ├── nu_trace.rs            # human-readable ν trace for debug mode
│   │       ├── bounds.rs              # nu_min / nu_max prefix bounds for branch-and-bound
│   │       ├── scc.rs                 # SCC extraction over support graphs
│   │       ├── minimality.rs          # semantic minimality and detachable-subbundle pruning
│   │       ├── halting.rs             # local halting logic and stop conditions
│   │       └── audit.rs               # omitted-candidate summaries and near-miss digests
│   │
│   ├── pen-store/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                 # storage façade
│   │       ├── layout.rs              # authoritative on-disk structs, magic bytes, versions
│   │       ├── manifest.rs            # run/step/frontier manifest structs
│   │       ├── checkpoint.rs          # write/read checkpoint orchestration
│   │       ├── frontier.rs            # frontier checkpoint packing/unpacking
│   │       ├── shard.rs               # fixed-width shard I/O for hot/cold frontier pages
│   │       ├── queue.rs               # disk-backed queue abstraction over shards
│   │       ├── blob.rs                # content-addressed pack files for AST/type/trace blobs
│   │       ├── sqlite.rs              # metadata DB only; never hot-path candidate lookup
│   │       ├── spill.rs               # spill/compaction policy under memory pressure
│   │       ├── memory.rs              # resident-memory governor and threshold actions
│   │       ├── telemetry.rs           # NDJSON events and summaries
│   │       ├── migrate.rs             # schema migrations for old checkpoints
│   │       └── checksum.rs            # integrity checks for manifests and shard files
│   │
│   ├── pen-search/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                 # search façade
│   │       ├── config.rs              # runtime config structs and defaults
│   │       ├── state.rs               # PrefixState and FrontierStateRec builders
│   │       ├── priority.rs            # deterministic priority-key construction
│   │       ├── frontier.rs            # in-RAM frontier window management
│   │       ├── enumerate.rs           # next-clause generation over atomic MBTT space
│   │       ├── expand.rs              # state expansion and obligation updates
│   │       ├── branch_bound.rs        # sound prune checks and incumbent handling
│   │       ├── dedupe.rs              # canonical-key dedupe and diversity buckets
│   │       ├── diversify.rs           # obligation-bucket quotas / anti-junk retention
│   │       ├── scheduler.rs           # worker scheduling and shard assignment
│   │       ├── worker.rs              # per-worker batch loop and scratch arenas
│   │       ├── accept.rs              # final acceptance and step-sealing logic
│   │       ├── resume.rs              # resume policy: frontier if compatible, else step
│   │       └── motif.rs               # learned atomic motifs, disabled in early phases
│   │
│   ├── pen-accel/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs                 # optional accelerator façade
│   │   │   ├── backend.rs             # backend selection and capability probe
│   │   │   ├── pack.rs                # compact array packing for GPU batch jobs
│   │   │   ├── cpu_fallback.rs        # CPU reference implementations
│   │   │   ├── wgpu.rs                # optional portable GPU backend
│   │   │   ├── kernel_bounds.rs       # batch optimistic-bound kernels
│   │   │   ├── kernel_hash.rs         # batch fingerprint kernels
│   │   │   └── kernel_filter.rs       # cheap structural filter kernels
│   │   └── shaders/
│   │       ├── bounds.wgsl
│   │       ├── hash.wgsl
│   │       └── filter.wgsl
│   │
│   ├── pen-agda/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs                 # export façade
│   │       ├── export.rs              # accepted-step → Agda artifact conversion
│   │       ├── render.rs              # text rendering into Agda modules
│   │       ├── manifest.rs            # export manifests and dependency graph
│   │       └── verify.rs              # shell-out / orchestration for Agda checks
│   │
│   └── pen-cli/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs                # binary entrypoint
│           ├── cli.rs                 # clap definitions
│           ├── cmd_run.rs             # fresh strict run
│           ├── cmd_resume.rs          # resume compatible checkpoint
│           ├── cmd_inspect.rs         # inspect checkpoints, manifests, telemetry
│           ├── cmd_export_agda.rs     # export accepted states to Agda
│           ├── output.rs              # standard vs debug console output
│           └── report.rs              # post-hoc labels and human-readable reports only
│
├── xtask/
│   ├── Cargo.toml
│   └── src/main.rs                    # import fixtures, validate schemas, generate docs
│
├── tests/
│   ├── fixtures/
│   │   ├── trajectory/                # frozen acceptance fixtures and legacy step states
│   │   ├── checkpoints/               # sample v1 checkpoint corpus
│   │   └── libraries/                 # canonical library snapshots
│   └── integration/
│       ├── atomic_bootstrap.rs        # empty-library → early-step recovery tests
│       ├── resume_roundtrip.rs        # frontier/step checkpoint compatibility tests
│       ├── deterministic_replay.rs    # same result on repeated runs
│       └── agda_export.rs             # Agda sidecar export smoke tests
│
├── scripts/
│   ├── run_desktop_16gb.sh            # wrapper for canonical desktop profile
│   ├── resume_latest.sh               # resume newest interrupted run
│   └── compare_runs.py                # compare telemetry and accepted trajectories
│
└── runs/
    └── .gitkeep
```

## Crate ownership and donor mapping

The donor map should be explicit.

* Port conceptually:

  * `Kolmogorov.hs` → `pen-core/{atom,encode}.rs`
  * `Telescope.hs`, `MBTTCanonical.hs`, parts of `Capability.hs` → `pen-core/{telescope,library,canonical,capability}.rs`
  * `TelescopeCheck.hs` → `pen-type/{infer,check,admissibility,connectivity}.rs`
  * `MBTTNu.hs`, `StructuralNu.hs`, `CoherenceWindow.hs`, `StrictMinimality.hs`, `AbInitioPolicy.hs` → `pen-eval/{nu,nu_trace,coherence,minimality,bar}.rs`
  * `MBTTEnum.hs`, parts of `Parallel.hs`, `RunAbInitio.hs` → `pen-search/*`
  * `AgdaExport.hs` → `pen-agda/*`
* Do **not** port into the hot path:

  * `StrictMolecules.hs`
  * `MCTS.hs`
  * `RunAcceptance*`
  * any shell inventory or molecular candidate layer

That split matches the current repo structure and the paper’s own distinction between reusable MBTT machinery and the present strict bounded molecular-first executable lane. ([GitHub][4])

## Dependency graph

Keep the dependency graph acyclic and simple:

```text
pen-core
   ↑
pen-type        pen-store
   ↑               ↑
pen-eval          │
   ↑               │
   └────── pen-search ──────┐
            ↑       ↑       │
         pen-accel  │     pen-agda
                \   │       /
                  pen-cli
```

Rules:

* `pen-core` never depends on anything internal
* `pen-store` depends on `pen-core` only
* `pen-search` owns orchestration, never semantic evaluation
* `pen-cli` owns human presentation and post-hoc labels
* `pen-agda` never participates in acceptance decisions

## Runtime artifact layout

Source tree is one half. The other half is the on-disk run layout.

```text
runs/<run-id>/
├── run.json                           # authoritative run manifest
├── config.toml                        # exact config used for this run
├── meta.sqlite3                       # metadata index only
├── telemetry.ndjson                   # append-only events
├── reports/
│   ├── latest.txt                     # standard-mode latest summary
│   ├── latest.debug.txt               # debug-mode latest summary
│   └── steps/
│       ├── step-00-summary.json
│       ├── step-01-summary.json
│       └── ...
├── checkpoints/
│   ├── steps/
│   │   ├── step-00-empty.cbor.zst
│   │   ├── step-01-universe.cbor.zst
│   │   └── ...
│   └── frontier/
│       └── step-10/
│           └── band-04/
│               ├── frontier.manifest.json
│               ├── hot-000.bin.zst
│               ├── hot-001.bin.zst
│               ├── cold-000.bin.zst
│               ├── cold-001.bin.zst
│               ├── dedupe-000.bin.zst
│               ├── dedupe.manifest.json
│               └── cache.bin.zst
└── agda/
    ├── manifest.json
    ├── Step10.agda
    ├── Step10.verify.log
    └── ...
```

Rules:

* `run.json` is always human-readable JSON
* step checkpoints are immutable, compressed, and self-contained
* frontier checkpoints are resumable, disposable working state
* `meta.sqlite3` is metadata only, never the source of truth for acceptance semantics
* `telemetry.ndjson` is append-only and diffable

## Frozen checkpoint schema

### 1. `run.json`

This is the top-level manifest.

```json
{
  "schema_version": 1,
  "run_id": "2026-03-13T14-32-21Z-desktop16",
  "status": "running",
  "created_utc": "2026-03-13T14:32:21Z",
  "updated_utc": "2026-03-13T17:05:09Z",
  "workspace_version": "0.1.0",
  "compat": {
    "ast_schema_hash": "blake3:...",
    "type_rules_hash": "blake3:...",
    "evaluator_hash": "blake3:...",
    "search_semantics_hash": "blake3:...",
    "store_schema_hash": "blake3:..."
  },
  "host": {
    "os": "linux",
    "arch": "x86_64",
    "logical_cpus": 16,
    "ram_bytes": 17179869184
  },
  "config": {
    "path": "config.toml",
    "sha256": "..."
  },
  "position": {
    "completed_step": 9,
    "active_step": 10,
    "active_band": 4,
    "frontier_epoch": 17
  },
  "artifacts": {
    "telemetry": "telemetry.ndjson",
    "reports_dir": "reports",
    "checkpoints_dir": "checkpoints"
  }
}
```

### 2. Step checkpoint

Step checkpoints are the **stable resume unit**. They must be self-contained, so you can change search code and still resume from the last accepted step.

```json
{
  "schema_version": 1,
  "run_id": "2026-03-13T14-32-21Z-desktop16",
  "step_index": 9,
  "accepted_utc": "2026-03-13T16:58:11Z",
  "compat": {
    "ast_schema_hash": "blake3:...",
    "type_rules_hash": "blake3:...",
    "evaluator_hash": "blake3:...",
    "search_semantics_hash": "blake3:..."
  },
  "objective": {
    "bar": { "num": 401, "den": 100 },
    "exact_clause_kappa": 4,
    "bit_band": { "min": 76, "max": 84 }
  },
  "accepted": {
    "candidate_hash": "blake3:...",
    "canonical_hash": "blake3:...",
    "bit_kappa": 78,
    "clause_kappa": 4,
    "nu": 17,
    "rho": { "num": 17, "den": 4 },
    "overshoot": { "num": 66, "den": 100 },
    "shape_fingerprint": "0x...",
    "support_fingerprint": "0x..."
  },
  "library_snapshot": {
    "window_depth": 2,
    "entries": [
      {
        "step": 1,
        "candidate_hash": "blake3:...",
        "telescope": { "clauses": [ /* self-contained */ ] }
      }
    ]
  },
  "near_misses": [
    {
      "candidate_hash": "blake3:...",
      "canonical_hash": "blake3:...",
      "bit_kappa": 79,
      "clause_kappa": 4,
      "nu": 18,
      "status": "bar_clear_higher_overshoot"
    }
  ],
  "stats": {
    "frontier_scanned": 4123312,
    "typed_prefixes": 991281,
    "sound_prunes": 3400012,
    "heuristic_drops": 120398
  }
}
```

### 3. Frontier checkpoint

Frontier checkpoints are working state. They are only resumable when the compatibility hashes and record layout match exactly.

```json
{
  "schema_version": 1,
  "run_id": "2026-03-13T14-32-21Z-desktop16",
  "step_index": 10,
  "band_index": 4,
  "frontier_epoch": 17,
  "base_step_checkpoint": "../../steps/step-09-hopf.cbor.zst",
  "resume_compatible": {
    "ast_schema_hash": "blake3:...",
    "type_rules_hash": "blake3:...",
    "evaluator_hash": "blake3:...",
    "search_semantics_hash": "blake3:...",
    "record_layout_id": "frontier_state_v1"
  },
  "counts": {
    "hot_states": 2241132,
    "cold_states": 15208744,
    "dedupe_keys": 10677731
  },
  "files": {
    "hot_shards": ["hot-000.bin.zst", "hot-001.bin.zst"],
    "cold_shards": ["cold-000.bin.zst", "cold-001.bin.zst"],
    "dedupe_segments": ["dedupe-000.bin.zst"],
    "cache_blob": "cache.bin.zst"
  },
  "memory_snapshot": {
    "rss_bytes": 8642146304,
    "hot_frontier_bytes": 2147483648,
    "interner_bytes": 1744830464,
    "dedupe_bytes": 1107296256,
    "cache_bytes": 671088640
  },
  "scheduler": {
    "worker_count": 12,
    "priority_heads": [101, 117, 128, 144],
    "spill_generation": 23
  }
}
```

### 4. Resume rules

Freeze these rules in `docs/CHECKPOINTS.md` and `pen-search/src/resume.rs`.

* Same `ast_schema_hash` + `type_rules_hash` + `evaluator_hash` + `search_semantics_hash` + `record_layout_id`

  * resume from frontier checkpoint
* Same AST/type/evaluator hashes, different search semantics

  * discard frontier, resume from last step checkpoint
* Same AST/type, different evaluator

  * resume from last step checkpoint and re-evaluate from there
* Different AST schema

  * no automatic resume; migration or replay required

## Frozen frontier state record

`docs/FRONTIER_STATE_V1.md` is authoritative. The binary record is fixed-width and little-endian.

```text
FrontierStateRecV1 = 64 bytes

offset  size  field
0       8     state_id
8       8     parent_state_id
16      4     last_clause_id
20      4     obligation_set_id
24      8     shape_hash64
32      8     support_hash64
40      2     nu_lower_bound
42      2     nu_upper_bound
44      2     bit_kappa_used
46      2     clause_kappa_used
48      2     depth
50      1     step_index
51      1     band_index
52      2     flags
54      4     priority_key
58      2     worker_hint
60      4     reserved
```

Important detail:

* this record stores **truncated 64-bit routing hashes**
* full 128-bit canonical fingerprints live in the dedupe segments
* acceptance never trusts the truncated hashes alone

## Frozen hot-path memory budget

### Host assumption

Canonical target profile: **16 GB RAM desktop**.

### Process budget

* OS / background reserve: **4.0 GiB**
* hard engine cap: **12.0 GiB RSS**
* emergency checkpoint threshold: **11.5 GiB**
* pressure threshold: **10.5 GiB**
* soft threshold: **9.0 GiB**
* target steady-state resident set: **7.5–8.5 GiB**

### Budget by resident component

```text
2.25 GiB  hot frontier window
2.00 GiB  interned expr/clause/obligation arenas
1.25 GiB  dedupe keys and quotient filters
0.75 GiB  evaluator/bound/normalization caches
0.75 GiB  worker scratch arenas
0.50 GiB  spill + shard I/O buffers
0.25 GiB  checkpoint staging + telemetry buffers
0.75 GiB  emergency slack / fragmentation margin
-----------------------------------------------
8.50 GiB  target steady-state
```

### Frozen record-size assumptions

```text
ExprNode                 32 bytes
ClauseRec                32 bytes
FrontierStateRec         64 bytes
DedupeKey128             16 bytes
BoundCacheRec            24 bytes
Worker scratch chunk     64 MiB per worker
```

### Worker defaults

* `workers = min(logical_cpus - 2, 12)`
* `worker_scratch_arena = 64 MiB`
* on a 16-thread machine: default **12 workers**
* scratch budget at default: **768 MiB**

### Memory governor states

Freeze these states in `pen-store/src/memory.rs`.

**Green** `< 7.5 GiB`
Normal operation.

**Yellow** `7.5–9.0 GiB`
Background spill enabled. Optional caches may grow only if hit rate stays above threshold.

**Orange** `9.0–10.5 GiB`
Stop enlarging hot frontier. Start aggressive shard spill. Shrink diversity bucket quotas by 25%. Flush dedupe deltas every few seconds.

**Red** `10.5–11.5 GiB`
Disable optional caches. Shrink worker arenas from 64 MiB to 32 MiB. Force frontier compaction. Write frontier checkpoint immediately.

**Black** `> 11.5 GiB`
Pause search, flush checkpoint, compact. If still above hard cap after compaction, abort cleanly and leave last good frontier/step checkpoint intact.

### Generic scaling policy for non-16GB hosts

`configs/default.toml` should scale from total RAM:

* reserve `max(2 GiB, min(4 GiB, 25% of RAM))`
* set hard cap to `total_ram - reserve`
* set soft cap to `75% of hard`
* set steady-state target to `70% of hard`

That keeps the engine portable across standard PCs while preserving the same semantics.

## Frozen `desktop_16gb.toml`

```toml
[mode]
strict = true
debug = false

[search]
until_step = 15
workers = "auto"
max_workers = 12
frontier_mode = "obligation_guided"
learned_motifs = false

[objective]
exact_clause_policy = "strict"
window_depth = 2
selector = "minimal_positive_overshoot"

[memory]
reserve_for_os_gib = 4.0
target_rss_gib = 8.5
soft_rss_gib = 9.0
pressure_rss_gib = 10.5
emergency_rss_gib = 11.5
hard_rss_gib = 12.0
hot_frontier_gib = 2.25
intern_gib = 2.0
dedupe_gib = 1.25
cache_gib = 0.75
spill_buffers_gib = 0.50
checkpoint_buffers_gib = 0.25
worker_arena_mib = 64

[checkpoint]
root = "runs"
step_on_accept = true
frontier_interval_sec = 180
retain_frontier_generations = 3
compression = "zstd"

[accel]
backend = "auto"         # cpu if no compatible accelerator
verify_on_cpu = true     # always true
```

## Non-negotiable implementation invariants

1. **No semantic names in hot-path crates.**
   Post-hoc labels live only in `pen-cli/src/report.rs`.

2. **No floating-point ranking in the hot path.**
   All threshold checks use integer/rational arithmetic.

3. **No mutable AST trees in frontier states.**
   Frontier states store only IDs and counters.

4. **Step checkpoints are self-contained.**
   You can always resume from the last completed step.

5. **Frontier checkpoints are conditional.**
   They are a speed feature, not a correctness dependency.

6. **GPU is never the source of truth.**
   Any accelerated filter or bound must be rechecked on CPU before acceptance or sound prune.

7. **Every prune is classified.**

   * sound impossibility prune
   * ranking-neutral quotient/dedupe
   * heuristic frontier-shaping drop

8. **No direct port of molecular search.**
   The old molecular lane is intentionally excluded from the new hot path.

## What should be implemented first

In this exact order:

1. `pen-core`
2. `pen-store`
3. `schemas/*` + round-trip tests
4. `pen-type`
5. `pen-eval`
6. `pen-search`
7. `pen-cli`
8. `pen-agda`
9. `pen-accel`

That order gives you the stable AST, storage, and resume contract before you write any expensive search code.

The right next move is to scaffold exactly this tree and implement `pen-core`, `pen-store`, `desktop_16gb.toml`, and the checkpoint schema tests before touching the enumerator.

[1]: https://github.com/efficient-novelty/mechanization "GitHub - efficient-novelty/mechanization · GitHub"
[2]: https://raw.githubusercontent.com/efficient-novelty/mechanization/main/strict_intelligence_plan.md "raw.githubusercontent.com"
[3]: https://raw.githubusercontent.com/efficient-novelty/mechanization/main/engine/src/Kolmogorov.hs "raw.githubusercontent.com"
[4]: https://github.com/efficient-novelty/mechanization/tree/main/engine/src "mechanization/engine/src at main · efficient-novelty/mechanization · GitHub"
