# Terminal DCT

This file explains the step-15 terminal shell, the DCT completion story, and the current executable novelty account now frozen in the repo.

## The Current Executable Step 15

The frozen live artifact and structural evaluator now agree on the step-15 row:

- selected shell:
  temporal-cohesive shell (semantic DCT completion)
- `Delta = 610`
- `nu = 103`
- `kappa = 8`
- `rho = 103/8`
- `Bar = 19520/2639`
- absolute overshoot:
  `115657/21112`

Rounded, this is `rho ~= 12.88`, `Bar ~= 7.40`, and overshoot `~= 5.48`.
This is the largest overshoot in the current executable sequence.

## What The Shell Contains

The late signatures in `pen_lmcs.tex` and `synthetic_framework_abstraction_mscs_draft.tex` present a common shell:

- temporal operators:
  `Next A`, `Eventually A`,
- a bridge:
  `Next A -> Eventually A`,
- spatial-temporal exchange:
  `flat(Next A) ~= Next(flat A)`,
  `sharp(Eventually A) ~= Eventually(sharp A)`,
- guarded self-interaction:
  `Next Next A -> Next A`,
- inherited temporal action on the exported interface.

The important negative fact is also repeated often:

- no primitive infinitesimal object is selected in the strict AST.

The infinitesimal reading is a semantic consequence, not a hot-path primitive.

## Why 103 Is Now The Canonical Repo Value

The current repo no longer has an executable late-step split for DCT.

- `crates/pen-eval/src/nu.rs` freezes the reference sequence at `...103` and asserts it in tests.
- `tests/fixtures/trajectory/reference_steps_until_15.json` freezes the live step-15 artifact with `nu = 103` and `rho = 103/8`.
- `overall_plan.md` records step 15 as structurally live-discoverable at `nu = 103`.

The older mismatched DCT claim now survives only in superseded theory and manuscript commentary.
For the repo's current truth surface, `103` is the DCT total to cite.

## Current Structural Decomposition

The repo's step-15 total is the structural account:

- `nu = 103`
- `kappa = 8`
- `rho = 12.875`

with the usual split:

- `nu_G = 2`
- `nu_H = 15`
- `nu_C = 86`

## Theorem Package Around Step 15

The late papers attach four important ideas to the temporal shell.

### 1. Temporal-tangent transfer

In semantic completion, `Next X` is identified with a tangent-style endofunctor:

```text
Next X ~= X^D
```

The key point for the rewrite is not the differential semantics itself.
It is the boundary:

- the AST selects temporal structure,
- the tangent or infinitesimal reading comes afterward.

### 2. Internalization of meta-evolution

Once guarded temporal structure is present, the library can express its own evolution internally.

This is why step 15 is treated as qualitatively different from earlier steps.

### 3. Combinatorial squeeze and halting

After step 15:

- internal continuations contribute zero marginal novelty up to equivalence,
- disconnected jumps face the full next interface debt.

The papers repeatedly quote:

- `Delta_16 = 987`

as the first post-step-15 debt barrier.

### 4. Path-style novelty shortcut

The strict paper uses a theorem-backed shortcut for surviving path payload:

```text
nu_H = m + d^2
```

on explicit path data.

This matters because the geometric branch dies early if the quadratic term is removed.

## What `pen-atomic` Should Do With Step 15

- Preserve the temporal atoms and shell structure in the hot path.
- Keep DCT as a post-hoc semantic completion, not a target label.
- Default to the executable `nu = 103` canon for replay, checkpoint, and report fixtures.
- Treat older mismatched donor notes as superseded provenance, not as the repo's current claim boundary.

## Cross-Links

- Read [genesis.md](genesis.md) for the exact strict row.
- Read [late-framework-abstraction.md](late-framework-abstraction.md) for the steps that prepare the temporal shell.
- Read [downstream-interpretations.md](downstream-interpretations.md) for the cosmology, physics, and RH papers that build on this step.
