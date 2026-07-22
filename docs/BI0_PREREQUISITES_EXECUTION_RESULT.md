# BI-0 prerequisites execution result

**Date:** 2026-07-22. **Status:** **NEGATIVE PREREQUISITE RESULT;
BI-0 AND ALL DOWNSTREAM BRANCH WORK REMAIN CLOSED.**

This is the create-new execution of
`docs/bi0_prerequisites_adjudication.md`. It preserves the sealed history and
the v1/v2 artifacts, applies F-AL1 and F-SM1 without using their desired
answers as construction inputs, and stops at the first lawful gate. It adopts
no semantic successor rule.

## Create-new artifacts

- `docs/t_bi_nu1_act_local_provenance_v3.json`, digest
  `blake3:79fbe85574843e795b6c19807f005da9ab7d5e3860cdb6cf57dff802d7fd54e4`;
- `docs/T_BI_NU1_ACT_LOCAL_PROVENANCE_V3_RESULT.md`;
- `docs/chronological_interface_slot_map_v3.json`, digest
  `blake3:20cc2a5797ab27d89ab2c2482211d1499503cb1d65da958516479c7db219697f`;
- `docs/CHRONOLOGICAL_INTERFACE_SLOT_MAP_V3_RESULT.md`.

Both directory replays are valid and have empty replay-error lists. A
replay-valid negative artifact certifies the falsifier result; it does not
authorize the next stage.

## T-BI-NU1 / F-AL1

The v3 issuer reads only the candidate, its sealed prefix, the count-blind
role grammar, the exact-prefix full-A3 inventory, and the adopted quotient and
provenance relations before sealing its intrinsic digest. The historical
certificate is opened only afterward as the F-AL1 comparator.

- All 15 candidates and all exact-prefix A3 windows issue.
- The A3 relative rule-inventory exhaustiveness theorem replays at every
  window.
- Candidate-minted demand-output positions: **0**.
- Hand-enumerated P5/P6/d-squared/combinatorial roles are not promoted from
  owner-clause typing to semantic-family typing. Each lacks its own constructed
  term, typing derivation, and naturality derivation, so it becomes an explicit
  `ROLE_SCHEMA_EXTRACTION_GAP`.
- Authoritative packages: **0/15**; named role-schema gaps: **250**.
- F-AL1: **false**. The first exact divergence is Stage 1,
  `THEOREM_GAP` versus archived `1`.
- The first capacity-level contradiction is Stage 10: the current lawful
  codomain has at most `4*kappa + exported = 17`, below the archived `19`.

Stage 15 is decisive independently of the incomplete role extractor:

| Quantity | Value |
|---|---:|
| `kappa` | 8 |
| blind local-role capacity `4*kappa` | 32 |
| independently exported A3 orbits | 1 |
| lawful upper bound | 33 |
| all 69 quotient A3 orbits exported (inadmissible relaxation) | 101 |
| archived comparator | 103 |
| all 71 raw A3 instances counted (inadmissible, pre-quotient relaxation) | 103 |

Thus `103 > 101 > 33`. The last equality is recorded only as a diagnostic
coincidence; it is not a claim about how the archive was causally produced.
Under the current four-role injection, full-A3 exhaustiveness, export flags,
and natural-family quotient, no implementation can make F-AL1 pass.

## Chronological slot map / F-SM1

The order-preserving identity declarations replay for **250/250** historical
chronological instances, with zero charge and no alternative permutation
trial. Membership now requires a replayed `VerifiedClosureDerivationV2` for
the source Internal judgment and for every substitution image. Typing,
well-scoping, or a nonempty hash is never promoted to Internal.

- Sealed chronological discharges: **72**.
- Derived: **0**; named gaps: **72**.
- Gap split: **54** missing genuine contextual-Formation Internal premises;
  **18** sources outside the current Formation/Type theorem and still needing
  a general motive-typed contextual eliminator.
- Former nine: **0/9** Derived.
- F-SM1: **false**; the slot-map side does not reopen BI-0.

The positive gate is nevertheless complete for a future theorem: it requires
declaration totality, unique instance IDs, exact order-preserving assignments,
72/72 exact specialized Internal derivations, zero named gaps, and exact 9/9
recovery of the former gap set.

## Sequencing and capability audit

The public crate boundary now enforces the same sequence as the artifact
ledger:

- low-level continuation and finale modules are crate-private;
- reissuance-based branch-certificate replay is crate-private;
- the cone assembler validates a replayed, passing BI-0 certificate before
  replaying any branch;
- the public post-BI-0 program remains the only branch-issuance surface.

The BI-0 create-new entrypoint was invoked as a gate check. It stopped before
enacted-branch issuance with replay-valid `F-SM1=false` and `F-AL1=false`.
No new `BI_REGRESSION_V2_*`, `BI_BRANCH_V2_*`, `BI_CONE_V2_*`, bridge, or
UC-1 artifact was created.

## Verification

- `cargo check -p pen-search --lib --examples`: pass;
- T-BI-NU1 v3 intrinsic tests: 2/2 pass;
- T-BI-NU1 v3 regression tests: 2/2 pass;
- chronological v3 tests: 3/3 pass;
- branch-program gate tests: 2/2 pass;
- branch-core tests: 5 pass, 1 intentionally ignored behind the BI gate;
- external capability compile-fail tests: 2/2 pass;
- both emitted-directory replays: valid;
- BI-0 gate check: expected prerequisite rejection, with no output files.

All other newly changed Rust surfaces pass `rustfmt --check` individually.
The already emitted T-BI-NU1 v3 regression source retains two formatting-only
differences so its create-new byte binding remains exact; changing those bytes
would correctly invalidate its artifact replay.

## Required successor decision

There is no lawful code-only continuation. A versioned adjudication must
resolve the now-explicit inconsistency between F-AL1's exact archived-total
requirement and the current semantic credit codomain. The conservative option
is to preserve the archive as structural/testimonial data while no longer
requiring those scalars to equal provenance-certified semantic family counts.
Any alternative that retains exact equality must independently justify a
larger pre-candidate output codomain or a different quotient/local-role law;
choosing exports because they recover 103 is forbidden.

Independently, F-SM1 needs a genuine contextual-Formation Internal theorem
and a general motive-typed open-specialization theorem. Completing that proof
alone cannot remove the T-BI-NU1 capacity contradiction.
