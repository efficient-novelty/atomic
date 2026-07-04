# run_clause_computation.md — Engine Target: Internal Derivation of n* = 16

**Handoff document for the agent operating in `pen/atomic` (Rust/Agda environment).**
Written 2026-07-04. Companions: `pen/book/note_lambda_magnitude_derivation.md`, `note_lambda_trigger_lemma.md`, `note_lambda_coefficient_lemma.md`.

---

## 0. Context and goal

The Λ program has one remaining computational open item that lives in the engine: the **internal derivation of n\* = 16**, the step count from matter–radiation equality to the saturation trigger. Currently n* = 16 is selected by one bit of observation (late acceleration exists — see trigger note §5); closing L-Λ1 means deriving it from the clause calculus.

**What is already done — do not redo:** the coefficient ε = F₁₄/F₁₆ ≈ 1/φ² (bridge derivation, coefficient note), all cosmological numerics, the closed form Ω_Λ/Ω_m = (1+z_eq)³/φ⁴⁹, and the empirical adjudications. None of that involved a clause computation. This task is the first engine-side item.

**Correction superseding trigger-note §8:** the earlier statement "n* = 16 ⟺ ρ̄ ∈ (Bar₁₅, Bar₁₆] ≈ (7.40, 9.08]" conflated two readings. Bar_n = Φ_n·Ω_{n−1} is asymptotically ≈ φ·Ω (roughly **constant** once Ω stabilizes); the geometric ×φ growth lives in the **debt** Δ_n = F_n, not in the bar ratio. The two crossing formulations, each valid under its own ledger reading, are specified in §4. Run both.

## 1. Repo anchors (verify before starting)

- Ground truth trajectory: `tests/fixtures/trajectory/reference_steps_until_15.json` (exact fractions for ρ and bar per step). **Never overwrite this fixture.**
- Reference telescopes: `Telescope::reference()` in `crates/pen-core/src/telescope.rs`.
- ν split rules: `crates/pen-eval/src/nu.rs` (hit class: ν_G = pre-path clauses + 3 + parametric, parametric = Trunc(Var) only; ν_H = path_count + dim²; ν_C = post + (post+1)/2; map class (Hopf): ν_C = 2κ + |lib_refs|²).
- Obligation/band machinery: `crates/pen-type/src/obligations.rs` (`requires_sphere_lift_package`, exact κ-band export) and `crates/pen-type/src/admissibility.rs` (`clause_band_for_mode`).
- Exact constants after Step 15: Ω₁₅ = 359/64; Φ₁₆ = 987/610; Bar₁₆ = 354333/39040 ≈ 9.0762. Bar₁₅ = 7.40 (ρ₁₅ = 103/8 = 12.875 cleared it).

## 2. Task A — bar/debt extension module (mechanical)

Add a module (suggested: `crates/pen-eval/src/runtime_bar.rs`) that emits, in **exact rational arithmetic**, for n = 16…24:

1. Δ_n = F_n and Φ_n = F_n/F_{n−1} (exact fractions, F₁ = F₂ = 1 indexing — same convention as Δ₁₆ = 987).
2. Inherited-ledger bar: Bar_n = Φ_n · Ω, with Ω frozen at 359/64 in the absence of accepted runtime steps. (Expected: Bar_n ≈ φ·359/64 ≈ 9.076, asymptotically constant — confirming that the ratio bar plateaus while the debt grows ×φ.)
3. A fresh-stratum trajectory hook: given a stratum's own per-step (ν, κ) sequence (from Task B), accumulate Ω^(s) and emit Bar_n^(s) from stratum step 1 with Δ_n = F_n restarted.

Skeleton (adapt to local API; exact-fraction type as used by the trajectory code):

```rust
// crates/pen-eval/src/runtime_bar.rs
// TODO(local-API): use the crate's exact rational type (the one backing
// reference_steps_until_15.json), not floats.

pub struct LedgerState { pub omega_num: u64, pub omega_den: u64 } // 359/64 post-15

pub fn fib(n: usize) -> u64 { /* F1 = F2 = 1 */ }

/// Inherited reading: Bar_n = (F_n / F_{n-1}) * Omega, Omega frozen.
pub fn inherited_bar(n: usize, omega: &LedgerState) -> (u64, u64) { /* exact */ }

/// Fresh-stratum reading: fold the stratum's own (nu, kappa) steps into
/// Omega^(s), restart Delta at F_1, emit Bar_n^(s).
pub fn fresh_stratum_bars(steps: &[(u64, u64)]) -> Vec<(u64, u64)> { /* exact */ }
```

## 3. Task B — the structure schema (the creative core) + freeze protocol

Construct the **gravitational-collapse / bound-structure schema** as a telescope candidate in the same clause grammar as the Genesis steps. Structural content it must express (from the notes, stated here as the schema's job description, not as target numbers): a formation rule (overdensity → bound interface), a closure/balance clause (virialization), and a hierarchical composition rule (bound structures compose into bound structures). Let `pen-eval` score ν (standard split rules) and κ. **No hand-set numbers anywhere.**

**Freeze protocol (firewall — mandatory):**

1. Write the schema specification (telescope entries, clause lists) and commit it **before** running the evaluator on it. Commit message: `L-Lambda1: freeze structure schema variant(s), pre-evaluation`.
2. Up to three a-priori variants are permitted (e.g., minimal collapse-only; collapse + virial; collapse + virial + merger closure), all declared in the same freeze commit with a one-line rationale each.
3. Evaluate. Report **all** frozen variants' results, including misses.
4. No post-freeze edits that move a variant toward the target band. A miss is a result (see §6), not an invitation to retune. If you discover a genuine formalization *error* (not a tuning opportunity), document it, re-freeze in a new commit, and report both the erroneous and corrected runs.

## 4. Task C — the two crossing tests

**C1 (fresh-stratum, ratio reading).** Run the stratum trajectory: the schema's own steps accumulate Ω^(s); find the first n with Bar_n^(s) > ρ̄ (the schema's per-step ratio at plateau). **Success criterion: first crossing at n = 16.** If the stratum's Ω dynamics mirror Genesis, the crossing band is (Bar₁₅^(s), Bar₁₆^(s)] ≈ (7.40, 9.08] — but compute the stratum's own bars; do not assume the Genesis values.

**C2 (inherited, debt reading).** Compute the schema's per-step discharge capacity C in clause units — κ-clauses dischargeable per cadence step at plateau (g = 1: capacity equals the previous step's discharged demand). **Success criterion: C ∈ [F₁₅, F₁₆) = [610, 987)** — equivalently, first n with Δ_n = F_n > C is n = 16.

**C3 (export band — bonus, checks Premise A of the coefficient note).** At the crossing step, extract which obligation band goes unserviced. **Prediction: the exported component is exactly the depth-2 band** (the chronological-memory echo), with exported share Δ_{n−2}/Δ_n = 377/987 at n = 16. The mechanism is analogous to the exact κ-band export of `requires_sphere_lift_package` — reuse that machinery pattern if applicable.

## 5. Output format

Write `docs/lambda_trigger_computation.json`:

```json
{
  "date": "...", "freeze_commit": "...", "eval_commit": "...",
  "task_a": { "inherited_bars_16_24": [["num","den"], "..."], "omega_frozen": "359/64" },
  "variants": [
    {
      "name": "...", "rationale": "...",
      "nu": {"G": 0, "C": 0, "H": 0}, "kappa": 0, "rho": "num/den",
      "c1_crossing_step": 0, "c1_stratum_bars": ["..."],
      "c2_capacity_clause_units": "num/den", "c2_crossing_step": 0,
      "c3_exported_band": "depth-2 | depth-1 | mixed", "c3_exported_share": "num/den"
    }
  ],
  "verdict": "close | partial-C1 | partial-C2 | miss"
}
```

## 6. Acceptance and failure semantics (pre-registered)

- **Close:** C1 and C2 both yield 16 for at least one frozen variant, and C3 exports depth-2. H16 becomes a theorem-level claim; L-Λ1 closes; update the notes, `docs/CONSTANTS_PROGRAM.md`, and the book Scorecard row status ("Exponent derived" → "Exponent and trigger derived; coefficient bridged").
- **Partial:** exactly one of C1/C2 yields 16. Report which; the ledger-reading question (fresh vs inherited) becomes the primary open item. H16 remains observation-selected but gains or loses standing accordingly.
- **Miss:** no frozen variant yields 16 in either reading. Report plainly. H16 stays observation-selected; the pre-registered falsifier discipline applies — no successor schema may be constructed *in order to* land in the band. A principled later reformalization must re-enter through a new freeze commit with its own rationale.
- In all cases: append the verdict to `pen/book/note_lambda_trigger_lemma.md` §8 and to the memory file's constants-program entry.

## 7. Run instructions

- Branch: `lambda-trigger-computation`.
- Locate the exact-fraction type and trajectory runner used by `reference_steps_until_15.json` tests; extend, don't fork.
- `cargo test -p pen-eval` (and workspace `cargo test`) must stay green; add new tests under the new module, not by editing reference fixtures.
- Agda is **not** required for this task; it is pure engine-side clause computation. (Premises A/B formalization is a separate, later task.)
- Sanity checks before evaluating variants: Task A must reproduce Bar₁₆ = 354333/39040 exactly, and fib indexing must give Δ₁₆ = 987 (else the F-index convention is off by one — check against Φ₁₆ = 987/610 from the engine state).

## Appendix: exact constants

F₁…F₁₇ = 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597. Ω₁₅ = 359/64. Φ₁₆ = 987/610. Bar₁₆ = 354333/39040 ≈ 9.0762. Bar₁₅ = 7.40. ρ₁₅ = 103/8. Depth-2 share at 16: 377/987 = 0.3819656 (1/φ² − 4.6×10⁻⁷). Debt-capacity target: C ∈ [610, 987). Cadence: one step = ln φ in ln a (see trigger note, Lemma A).
