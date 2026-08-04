# Preregistration: law-profile-2x4-v1

Status: SKELETON, draft-not-frozen. Created 2026-08-04. This document
freezes at K1 and may not be edited after any profile sees the registered
prefix. Until frozen it mints no authority.

Registration lineage: `docs/experimental_program_addition.md` (2x3 base
registration) as amended by its §17 (2026-08-04, 2x4 supersession);
operative phase ordering in `docs/260804_autonomous_plan_and_status.md` §8.

## 1. Design

Eight profiles from one verified common substrate:

```text
{C0, C1} x {S0, S_W, S+, S_up}
```

Only the constitutive and selective manifest fields may differ between
profiles. Shared components per plan §8.3. Outputs remain mutually sealed
until all eight runs terminate or return `Unknown`.

## 2. To be frozen at or before K1 (placeholders)

- [ ] Bootstrap digest and shared substrate digests (plan §8.3 identity
      tuple).
- [ ] Constitutive manifests `C0`, `C1` (K0 freeze record).
- [ ] Selective manifests `S0`, `S_W`, `S+`, `S_up` (K1 freeze records).
- [ ] KW0 task-language freeze record, including the vocabulary-selection
      procedure, the minimal-v1 vocabulary, the second sensitivity
      vocabulary, and the profile-level task carrier
      (`docs/260804_task_extension_language.md`).
- [ ] KW1 extension-authority freeze record
      (`docs/260804_extension_authority_protocol.md`).
- [ ] KW2 falsifier execution record (gamma-independent items 1-8, 11-14;
      items 9-10 deferred to post-JG9 with their deferral recorded here).
- [ ] KW3 `S_W` gate freeze record: `H -> Tasks(H)` derivation function,
      three-valued dominance semantics, per-act resource contract.
- [ ] Branch-aggregation rule for `TaskExt_E(P)` over a sealed cone
      (union-over-branches / per-branch / final-frontier), with rationale,
      registered BEFORE K2.
- [ ] "Independent evidence" rule for KW4's stronger-clause comparison
      (internal carrier at KW4; held-out facts only at KW5).
- [ ] Validity fact register hash (`docs/260804_validity_fact_register.md`).
- [ ] Holdout decoder registration hash and holdout inventory seal
      (`docs/260804_holdout_decoder_registration.md`).
- [ ] KP0 kappa contract hash; GQ0 quotient contract hash.
- [ ] Resource contracts for every gate, each with a declared `Unknown`
      outcome.
- [ ] Outcome taxonomy: `Halted`, `ConstitutivelyBlocked`,
      `SelectivelyBlocked`, `OutsideFragment`, `Unknown`, `Advanced(cone)`.

## 3. Pre-registered interpretation rules

Per plan §8.5, applied without post-hoc modification. Additional rules fixed
by this preregistration:

- A `WeaknessAdjudicationUnderdetermined`,
  `VocabularyAdjudicationUnderdetermined`, or aggregation-underdetermined
  outcome is a reportable result, never a license to re-adjudicate after
  unsealing.
- No profile is edited in place after execution; any revision is a new
  experiment with fresh held-out evidence (K3).
- `S_W` results are interpreted under binding decision 9 (epistemically
  motivated, vocabulary-relative, ontologically underdetermined).

## 4. Amendment discipline

Until the K1 freeze, edits to this skeleton are permitted and must be
dated. After the K1 freeze, this file is immutable; any change is a new
preregistration under a new experiment identity.
