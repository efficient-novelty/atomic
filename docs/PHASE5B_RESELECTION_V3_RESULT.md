# Phase-5b reselection v3: two-register result

Date: 2026-07-21

## Outcome

The preregistered create-new v3 burn completed through Stage 15 and its
independent replay is valid.  It imports the immutable v2 prefix through
Stage 7 and recomputes Stages 8--15 under the adopted two-register rule.

The complete revised semantic vector is

```text
[1, 1, 2, 5, 7, 8, 10, 17, 17, 19, 26, 34, 46, 62, 103]
```

There are no winner divergences and no score divergences from the certified
branch-(ii) reference sequence.

## Guarded discharger census

| Stage | Live demand | Canonical admitted cone | Typed total dischargers | Diagnostic bar | Winner rho | Would clear? |
| ---: | --- | ---: | ---: | ---: | ---: | :---: |
| 8 | `sphere_lift` | 1 | 1 | 357/104 | 17/5 | no |
| 9 | `axiomatic_bundle` | 1 | 1 | 578/147 | 17/4 | yes |
| 10 | `modal_shell` | 1 | 1 | 22/5 | 19/4 | yes |
| 11 | `connection_shell` | 1 | 1 | 267/55 | 26/5 | yes |
| 12 | `curvature_shell` | 1 | 1 | 8136/1513 | 17/3 | yes |
| 13 | `operator_bundle` | 1 | 1 | 11417/1920 | 46/7 | yes |
| 14 | `hilbert_functional` | 1 | 1 | 72761/10951 | 62/9 | yes |
| 15 | `temporal_shell` | 1 | 1 | 77775/10556 | 103/8 | yes |

At every recomputed stage the bar was recorded but neither gated nor selected
the winner.  Stage 8 is the operative witness: its unique discharger does not
clear, yet advances by the guarded rule.  At Stages 9--15 the unique discharger
also clears, so pricing is extensionally redundant there.

F-S8-1 did not fire: every reached guarded stage had exactly one discharger.
F-S8-3 is honored and vacuous on this burn because no Stage 9--15 winner
diverged.

## Revised diagnostic bar and successor gate

The completed revised ledger gives

```text
Bar16 = 176673/19520
      = 353346/39040 before reduction.
```

This remains diagnostic under the two-register account; v3 does not adopt a
bar-removal theorem.  The burn establishes that pricing did no selection work
on its recomputed guarded domain, Stages 8--15.  A global claim covering the
imported bootstrap/prefix and the debt-free open-band envelope remains a
separate proof obligation.

The v3 gate authorizes E-5/F1 over the completed revised Step-15/14 window,
followed by the bridge.  F1 remains the relocated falsifiability frontier and
is not discharged by this census alone.

## Immutable artifacts

- Program: `docs/phase5b_reselection_program_v3.json`
  (`blake3:1b3bc90b7ab61fec7cefbe964d6441fb16894f317912f8286da9adaeef2a0da8`)
- Burn: `docs/phase5b_reselection_burn_v3.json`
  (`blake3:65ac7f569968666035bf88026e8f9034d51b23eaa15d793b9ca6aaf08e256870`)

The program was minted before the first full run.  Its source binding, imported
v2 prefix, two-register jurisdiction split, and fail-closed outcomes are frozen.
