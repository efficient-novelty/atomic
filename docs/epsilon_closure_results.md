# ε-Closure Frozen Run — B1–B5 Adjudication

**Date:** 2026-07-16. **Specification:** `d2_engine_instructions.md` v2,
SHA-256 `D263B0E298B3008CE8272A6B80C76C5A4EABF2D8B13D733188B6EE8DBE42FD84`.
**Freeze commit:** `b7f89751e257e6c8eb30edb6f8253706249d9005`. **Run:** one
artifact-producing execution with Cargo's lockfile frozen. **Artifact:**
`docs/epsilon_closure_run.json`, SHA-256
`22254893AFED0393AC6C500F5A2B1FDC0C1B71739771E6F41C4A56A2482E3301`.

The specification hash was identical before freeze, at execution, and after
execution. The theorem/scoring firewall grep had no forbidden-identifier hits.
The engine used `num_rational::Ratio<i128>` through checked `i128` arithmetic;
the artifact records zero floating-point values. The emitter used create-new
semantics and refused overwrite.

## Adjudication

| Registration | Frozen result | Verdict |
| --- | --- | --- |
| B1 — Stage-A uniqueness | Linear rank 5 on 6 variables, solution dimension 1. `Agrav` reduces to `y_H - 3*y_Q`, hence `y_H = 3*y_Q`; `A111` reduces identically to zero. Normalized ray: `(1/6, -2/3, 1/3, -1/2, 1, 1/2)`. | **PASS** |
| B2 — simultaneous family and quotient | Linear rank 5 on 7 variables, solution dimension 2, with exact basis `Y` and `B-L`. The canonicalizer confirms `ε -> -1-ε`, `-1 ≡ 0`, `1 ≡ -2`, and fixed point `-1/2`. The finite κ=6 stratum has two classes and the finite κ=5 stratum one. | **PASS** |
| B3 — sealed extension | The extension Yukawa line, `Agrav`, and `A111` reduce independently to `y_ν`, `y_ν`, and `y_ν^3`; each uniquely forces `y_ν = 0`. No sealed value is rewritten. This remains conditional on the registered sealing-order premise. | **PASS** |
| B4 — diagnostic trap | Accounting (a) has minimum κ=5 at `{−1/2}`. Accounting (b) has lexicographic minimum `(1,5)` at `{−1/2}`. Accounting (c) retains `{0 ≡ −1}` only as the sealed baseline and marks every nonzero shift inadmissible (`ν=0`, marginal `κ>=1`). The module declares no winner. | **PASS — registered trap reproduced** |
| B5 — post-run characterization | Winner check values are `(q_ν,q_e,q_u,q_d)=(0,-1,2/3,-1/3)`; traces are `2`, `10/3`, `16/3`; normalization is `3/8`. The winner stabilizer is cyclic of order 6 with witness `(1,-1,0,0,1,0,1)`. | **PASS** |

There were no misses and no surprise findings.

## Simultaneous record

The engine derived the normalized family rather than copying the incomplete
line from the specification:

```text
y_Q   = 1/6 + epsilon/3       y_u   = -2/3 - epsilon/3
y_d   = 1/3 - epsilon/3       y_L   = -1/2 - epsilon
y_e   = 1 + epsilon           y_ν^c = epsilon
y_H   = 1/2
```

Here the `ν^c` slot value is `epsilon`; the post-run physical-particle check
uses the conjugate convention `q_ν = -epsilon`, as registered. Both vanish at
the sealed winner.

The exact finite branch strata are:

| ε | zero slots | active slots |
| ---: | --- | ---: |
| `-2` | `u^c` | 6 |
| `-1` | `e^c` | 6 |
| `-1/2` | `Q`, `L` | 5 |
| `0` | `ν^c` | 6 |
| `1` | `d^c` | 6 |

`H` has no finite zero. The pure `B-L` branch at `ε = infinity` was retained,
normalized by `y_Q=1/3`, included in all three diagnostic tables, and has six
active slots.

## Post-run group record

For the winner, `y_unit=1/6`; the six elements are represented by
`(a,b,theta) = (0,0,0), (0,1,1/2), (1,0,2/3), (1,1,1/6), (2,0,1/3),
(2,1,5/6)`. The element `(1,1,1/6)` generates the cyclic group and produces
the registered per-slot integrality witness. For the diagnostic `ε=-1/2`
class, `y_unit=1/2` and the computed stabilizer is the trivial group of order
1. This latter value was characterization only and did not enter scoring.

## One-line handback for `pen/book/note_weinberg_matching.md` §8

> **Engine outcome 2026-07-16 (ε-closure, frozen PASS):** B1–B5 all confirmed — Stage A is unique up to rescaling, sealing forces `y_ν=0` independently three ways, the simultaneous quotient is `ε~−1−ε` and reproduces the `ε=−1/2` minimality trap, while the winner has the registered traces and cyclic order-6 stabilizer; artifact `docs/epsilon_closure_run.json`, SHA-256 `22254893AFED0393AC6C500F5A2B1FDC0C1B71739771E6F41C4A56A2482E3301`.
