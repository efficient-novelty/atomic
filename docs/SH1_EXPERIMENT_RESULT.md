# SH-1 experiment result

**Burn date:** 2026-07-18

**Outcome:** **burned as a supported continuation hypothesis**

**Surviving result:** the conditional staircase arithmetic is correct, but
neither its homotopy-capacity gate nor its claimed engine trajectory was
derived.

## Frozen inputs

- Hypothesis freeze:
  `7ab526f48eb976aaff83ea5c458b06c07a50e4ed`
- Blind T4 freeze:
  `4b408c83d4fd48502839953e1fd52b895b2a1634`
- T1--T3 harness freeze:
  `020b2dc0c33f8394eab23d54f5169fbdd803043d`
- Burned measurements: `docs/sh1_experiment_burned.json`
- Measurement bytes: `205539`
- Measurement SHA-256:
  `344C95B85D2F56B441F83F5FC3E9C15D9F65782C2EC04F86984269EA8A91633B`

The measurement runner uses create-new output semantics. A second execution
to a temporary path produced the same byte count and SHA-256.

## T1: seal-and-continue

The designated shipped Step-16 survivor was `hit_no_formation_d1`. The probe
sealed four steps and recomputed the exact ledger and bar before every round.
It is deliberately non-exhaustive: after Step 16 it selects from the registered
adversarial candidate table after rerunning identification, raw-surface,
guarded-admissibility, type, connectivity, semantic-minimality, valuation, and
acceptance-rank checks.

| Step | Selected candidate | nu | kappa | rho | Exact margin |
|---:|---|---:|---:|---:|---:|
| 16 | `hit_no_formation_d1` | 19 | 2 | `19/2` | `16547/39040` |
| 17 | `temporal_polymorphic_kappa2` | 34 | 2 | `17` | `3998/517` |
| 18 | `axiomatic_single_l17_kappa3` | 38 | 3 | `38/3` | `13718/4791` |
| 19 | `axiomatic_single_l18_kappa3` | 42 | 3 | `14` | `343523/91732` |

The exact Step-16 clause pair is in the canonical identification set after it
is sealed. At Steps 17--19 its re-proposal is therefore identified before
scoring, receives zero, and cannot be selected. The registered Branch-S claim
that the identical two-clause telescope is selected forever is false under its
own premise P1.

## T2: exact staircase arithmetic

The exact `Ratio<i128>` calculator reproduced every registered P5 plateau:

| Dimension | Steps | Length | nu | rho |
|---:|---|---:|---:|---:|
| 4 | 16--23 | 8 | 21 | `21/2` |
| 5 | 24--42 | 19 | 30 | `15` |
| 6 | 43--68 | 26 | 41 | `41/2` |
| 7 | 69--101 | 33 | 54 | `27` |

The Step-23 margin is exactly

```text
4867/1381458
```

which is approximately `0.0035231`. The ungated calculation moves to `d=8`
at Step 102. The declared-capacity calculation has no `d=8` entry and halts
there.

The raw fallback calculator also reproduced `nu_n = n + 3` and its increasing
margin. This is a formula diagnostic only: T1 proves that canonical
identification prevents the same telescope from earning that sequence after
its first seal.

## T3: capacity oracle v0

The oracle implements the declared table for dimensions 4--7 and records the
claimed multiplicities `Z, 2, 2, 12`. Its serialized trust boundary correctly
states:

- `epistemic_status = declared_not_derived`;
- no homotopy computation is verified;
- no candidate-to-host/class binding is verified;
- a typed kernel is required before the table can be earned.

Consequently T3 successfully tests the consequences of SH-1c but supplies no
evidence for SH-1c.

## T4: blind valuation

The blind derivation did **not** recover SH-1b. It found:

1. `1 + d^2` is the conditional raw cardinality of the single-constructor L1
   beta/Kan basis.
2. Law-level credit is the number of those normalized families proved to lie
   in `Marg_2` and carrying distinct EGP anchors.
3. An old/weakening family contributes zero. Missing normalization or EGP
   evidence makes novelty undefined and the candidate unrankable; it does not
   license the placeholder score.
4. L1 requires the generic sealed cubical filling apparatus, but it does not
   require attachment to a pre-existing nontrivial element of
   `pi_(d-1)` of a sealed sphere stratum.

Both permitted Genesis 1--15 numerical conservativity tests passed
(`sum nu = 359`, `sum kappa = 64`). The specification explicitly records that
this is not the missing semantic re-audit.

## Registered falsifiers

| Check | Result | Reason |
|---|---|---|
| Support condition (i) | **failed** | T4 did not license `d >= 4`; without typed marginality/EGP it is unrankable. |
| F-SH1 | not triggered | T4 did not return a capacity gate with `d_max = 1`; it rejected the proposed host-capacity precondition instead. |
| F-SH2 | **triggered** | The shipped continuation does not produce the identical Branch-S echo; identification kills it at Step 17. |
| F-SH3 | blocked as stated | T4 returned a partial certified law, so no honest T4 staircase exists to substitute. The P5 placeholder staircase nevertheless disagrees with T1 at the first sealed step. |
| F-SH4 | unresolved | Oracle v0 is declared and performs neither homotopy derivation nor host binding. |
| F-SH5 | unresolved | D2 remains a pending jurisdiction lemma; no completed local result was found. |

## Adjudication

SH-1 is not supported. Its exact arithmetic subclaim survives, but the central
inference

```text
nontrivial sealed pi_(d-1) class
  -> licensed L1 credit
  -> forced d=4 continuation
  -> theta-sector interpretation
```

does not. The first implication was not obtained by the blind law, the shipped
engine does not follow the claimed continuation trajectory, and the typed
homotopy and D2 obligations remain open. The theta and torsion readings are
therefore unearned rather than experimental outputs.

This burn does not prove the global halt by itself. It removes SH-1 as a
currently certified continuation mechanism. A future revival would require a
typed `d=4` marginal-family/EGP certificate and a derived candidate-to-homotopy
class binding before any further staircase iteration is probative.

## Verification

```text
cargo test -p pen-eval sh1_
  9 passed, 0 failed

cargo test -p pen-search halting_probe --lib
  5 passed, 0 failed, 1 intentionally ignored full-surface test

cargo check -p pen-search --example sh1_experiment
  passed (pre-existing unrelated warnings)

burned/replay artifact SHA-256 equality
  true
```
