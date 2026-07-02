# Run Instructions: Verifying the Evaluator Derivation Changes (2026-07-02)

**Audience:** the AI agent (or human) on the machine with the Rust toolchain.
**Author of the changes:** Claude session on Halvor's machine (no Rust toolchain
available there — the edits are eye-verified but **uncompiled**).
**Supervised by:** Halvor. Escalate to him at every decision point marked ⚠.

---

## 1. Context — read this before touching anything

The evaluator's class formulas in `crates/pen-eval/src/nu.rs` were audited
against the philosophical definitions of ν and κ, and derived from first
principles. Read, in this order (skim is fine, but read §5 of the third one
carefully — it is your job description):

1. `docs/EVALUATOR_DERIVATION.md` — the six-principle Schema Calculus and audit table.
2. `docs/LEMMA_L1_D_SQUARED.md` — proof that ν_H per path constructor is 1 + d²
   (d monodromies + d(d−1) variation fillers), **summed per constructor**.
3. `docs/HSPACE_ENUMERATION.md` — the hand enumeration settling the
   `(post+1)/2` term, and the list of code changes plus required verification.

Two code changes landed in `crates/pen-eval/src/nu.rs` (the ONLY file edited):

- **Change A (behavior-identical, a naming/documentation refactor):** the
  three occurrences of `(post + 1) / 2` in the Hit ν_C paths now route through
  a documented `canonical_operation_count(post)`. Same arithmetic. Any
  behavioral difference whatsoever is a bug in the refactor.
- **Change B (behavior-changing off-trace):** ν_H now computes
  `path_count + Σ dᵢ²` (per-constructor, new profile field `path_dim_sq_sum`)
  instead of `path_count + max(d)²`, in all four computation paths (profile
  method `base_nu_h`, the fast-path merge, `compute_nu_h`, and the incremental
  single-clause context), **and** the Hit upper-bound estimator
  (`hit_total` + both callers) now bounds the square-sum instead of max² —
  the old bound would be unsound (under-estimating, hence over-pruning) under
  the corrected rule.
  **Expected impact: zero on every selected Genesis step** (each of the 15
  reference telescopes has at most one path constructor, so Σd² = max d²
  identically). Divergence is possible only for multi-path-constructor
  candidates in the live enumeration bands — i.e., off-trace.

## 2. Ground rules

- **Principle over preservation.** If the strict trace changes under the
  corrected rule, the corrected rule wins. Do NOT adjust formulas, weights, or
  counts to restore the old trace. That would be exactly the tuning this whole
  exercise exists to eliminate. ⚠ A trace change is an immediate escalation to
  Halvor with a diff report — not a revert, not a fix.
- Do not edit the three docs listed above except to append run results.
- Do not modify test fixtures by hand. Fixture regeneration, if needed, is a
  Halvor decision (⚠).
- Keep all run artifacts; nothing gets deleted.

## 3. Verification sequence

Work from the repo root (`pen/atomic`). Use the pinned toolchain
(`rust-toolchain.toml` handles this automatically).

### Step 0 — sanity: what changed

```bash
git status
git diff --stat
git diff crates/pen-eval/src/nu.rs
```

Confirm the only modified source file is `crates/pen-eval/src/nu.rs` (plus new
docs `docs/EVALUATOR_DERIVATION.md`, `docs/LEMMA_L1_D_SQUARED.md`,
`docs/LEMMA_L2_R_SQUARED.md`, `docs/HSPACE_ENUMERATION.md`, and this file).
Read the diff of `nu.rs` end to end — it should match §4 of
`HSPACE_ENUMERATION.md` exactly: struct field `path_dim_sq_sum`, its
accumulation next to `max_path_dimension`, the two `base_nu_h` sites, the
`compute_nu_h` body, the merge-site local, `canonical_operation_count` and its
three call sites, and the `hit_total`/`hit_upper_*` bound changes.

### Step 1 — compile

```bash
cargo build -p pen-eval
```

If this fails: the errors will be mechanical (the edits were made without a
compiler). Fix ONLY syntax/type-level issues while preserving the semantics
stated in the docs — e.g. reference/deref mismatches on `dimension`, a missed
`u32` annotation on the `sum()`, an unused-variable warning. Log every fix you
make in your handoff. If a fix would require changing *which quantity is
computed*, stop and escalate ⚠.

### Step 2 — pen-eval unit tests

```bash
cargo test -p pen-eval
```

Expected: **all green**, because:
- `reference_sequence_matches_donor_structural_totals` — every reference
  telescope has ≤ 1 path constructor, so all 15 totals are unchanged;
- `structural_nu_fast_path_matches_legacy_helper_composition` and
  `single_clause_context_matches_full_structural_nu` — both sides of each
  consistency pair were updated identically;
- `dct_meta_theorem_bonuses_fire`, `native_nu_trace_is_deterministic` — the
  Synthesis-class paths (`detect_infinitesimal_shift` etc.) were not touched
  (they already used Σd² over library entries).

If a pen-eval test fails: diagnose whether it encodes the OLD max² rule or a
multi-constructor expectation. A test that asserts the old rule on a
multi-constructor telescope is now asserting refuted mathematics — report it,
propose the corrected expected value with the arithmetic shown, and escalate ⚠
before changing any test.

### Step 3 — workspace tests

```bash
cargo test --workspace
```

Focus areas: `pen-search` engine tests and `tests/integration/*` (trajectory
fixtures, deterministic replay, resume round-trips). Expected: green, with one
allowed class of exception — tests that hardcode ν values or orderings for
**multi-path-constructor non-reference candidates** (retained-evidence
orderings, frontier snapshots, enumeration counts). Any such failure is
Change B working as derived. Document each: test name, old value, new value,
and why the candidate is multi-constructor. Escalate the list ⚠; do not
silently update fixtures.

### Step 4 — V2b: the strict-lane rerun (the decisive check)

Run the strict canonical lane end to end with a fresh run-id (do NOT overwrite
existing runs):

```bash
cargo run --release -p pen-cli -- run \
  --config configs/strict_canon_guarded.toml \
  --root runs --run-id v2b-l1-sigma-dsq
```

(If wall-clock or memory limits bite, `configs/desktop_16gb.toml` /
`configs/cpu_only.toml` are the fallbacks the README documents; note which
config you used. `resume … --until-step 15` is available if interrupted.)

Then compare against the current canon:

```bash
cargo run -p pen-cli -- inspect runs/v2b-l1-sigma-dsq
```

and diff the accepted trajectory against
`tests/fixtures/trajectory/reference_steps_until_15.json`, checking per step:
`label`, `clause_kappa`, `nu`, `rho`, `objective_bar`, and both hashes.

**Outcome A (expected): identical 15-step trajectory, identical values.**
The Σd² correction is confirmed trace-neutral; the trace is now certified
under the derived evaluator. Record this in the handoff and append a dated
confirmation line to `docs/HSPACE_ENUMERATION.md` §5 and
`docs/EVALUATOR_DERIVATION.md` (V2b: PASSED, run-id, hashes).

**Outcome B: the trajectory or any value differs.** ⚠ STOP. Produce a
step-by-step diff report (first divergent step; the candidate that newly wins
or loses; its clause list, ν split, and whether it is multi-path-constructor;
band and margin arithmetic). Do not revert Change B. Do not update fixtures.
Deliver the report to Halvor — under the ground rules, the derived rule
stands and the *fixtures and downstream documents* (Appendix B, the paper,
the book chapters) would need coordinated revision, which is his call.

### Step 5 — optional but valuable while you have the toolchain

1. **Multi-constructor probe (cheap, sharpens the V2b interpretation):**
   write a small unit test constructing a two-path-constructor telescope
   (e.g. formation + PathCon(1) + PathCon(2)) and assert
   `compute_nu_h = 2 + (1 + 4) = 7` (old rule would give `2 + 4 = 6`).
   This pins Change B's intent in the test suite permanently.
2. **Grammar-ablation lanes:** if time permits, rerun
   `configs/grammar_ablation_baseline.toml` with a fresh run-id and confirm
   the ablation conclusions (Steps 1–14 replay, Step-15 stall) are unaffected.

## 4. Handoff format

Follow the repo's handoff convention: append a dated section to your working
notes (and mirror the key results into `docs/HSPACE_ENUMERATION.md` §5)
containing: what was attempted; compile fixes made (exact diffs); test
results per step above; the V2b run-id, config used, and trajectory verdict
(A or B) with hashes; caveats; and the recommended next action. The next
milestones after a clean V2b, per `docs/EVALUATOR_DERIVATION.md` §6, are:
V1 (the direct definition-faithful schema enumerator — first targets: the
Möbius two-stratum anchor from `LEMMA_L2_R_SQUARED.md` §5.3 and the
operation/coherence packing check from `HSPACE_ENUMERATION.md` §3), then V3
(git archaeology), then the Agda mechanization of L1's transpose-obstruction
and union-staging steps.

One last reminder, because it is the point of everything above: this exercise
exists to make the evaluator *answerable to its definitions*. If the numbers
and the derivation ever disagree, the derivation is the senior partner. ⚠
