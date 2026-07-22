# EGP-v2 bootstrap burned result

**Executed:** 2026-07-18  
**Disposition:** burned experiment completed; bootstrap proof gate **failed**.  
**Artifact:** `docs/egp_v2_bootstrap_burned.json`

```text
bytes  = 40813
sha256 = 42B6F662E3AF3604A9503E4C88F304C3AF71F71E36F211DF2BD16B674B6A0D5F
raw    = blake3:5d8604df32cc999fb4035fd16d4292a358e8d8d0acdd9fda91c303b88175d8c5
run    = blake3:9dd34cc5167034d805ac00e94f7f60d5f5352c0bc43c70b18e9c2f9dc0443158
```

The immutable EGP-v1 artifact remains byte-identical:

```text
path   = docs/semantic_reselection.json
bytes  = 4107
sha256 = 4F6669F8A11554E8A1B7982CDCFF64C82B280EA5A60B348A4736407D1F60AFC4
```

This document reports the first complete execution of the program
preregistered in `docs/EGP_V2_BOOTSTRAP_PROGRAM.md`. It does not modify that
preregistration and does not repair EGP-v2 after observing its result.

## 1. Numerical result

| Stage | Bar before | Raw / admitted / deduped | Semantic variants | Result |
|---:|---:|---:|---:|---|
| 1 | `1/2` | `288 / 1 / 1` | `1` | Universe package accepted with `nu=1`, `kappa=2`, `rho=1/2` |
| 2 | `1/2` | `33 / 1 / 1` | `2` | one-nullary fresh formation accepted with `nu=1`, `kappa=1`, `rho=1` |
| 3 | `4/3` | `56 / 1 / 1` | `2` | no candidate clears; plain and exact-completion variants both have `nu=1`, `kappa=1`, `rho=1` |

The raw outcome is therefore

```text
HaltedNoClearingCandidate { stage: 3, bar: 4/3 }
```

Stage 4 was not run, as required by the preregistered stop-on-first-halt rule.
The first post-hoc divergence from the sealed history is Stage 3, field
`no_clearing_candidate`. Stages 1 and 2 agree with the legacy winner and score.

## 2. What this says about Unit

The revised interpretation fixes the specific EGP-v1 mistake.

- Stage 2 is a fresh, open, one-constructor inductive formation. It is not the
  old `App(Univ,m)` natural family counted again. Acceptance creates exactly
  one live constructor demand.
- The Stage-3 completion candidate is a single fresh constructor aimed at
  that exact live slot. It is charged once, through its demand provenance;
  it is not also charged as a local kernel head.
- Under the EGP-v2 basis-only score, generated induction and beta data do not
  create a second independent atom. Hence the completion candidate has
  density `1`, while the revised bar inherited from `(1,1)` is `4/3`.

So “Unit” at the end of the accepted EGP-v2 history still means **proto-Unit**:
the formation is present and its constructor orbit is live. The constructor
candidate is checked but is not accepted, the orbit is not consumed, and the
signature is not sealed. It would be incorrect to call the accepted prefix a
completed terminal type.

The central mathematical diagnosis is clean: after repairing the meaning of
Unit, the next disagreement is no longer semantic identification. It is the
interaction between the basis-only novelty law and the inherited bar. A
one-atom mandatory debt answer cannot clear `4/3`.

## 3. Proof-gate audit

The JSON correctly records

```text
stage3_star_answers_that_orbit = false
stage3_signature_sealed        = false
```

Therefore the bootstrap gate fails even on its own run-level invariants and
does not authorize Stages 5--15.

An adversarial code review also found four proof obligations that the current
certificate surface does not discharge:

1. **Declaration/presentation bridge.** The experimental checker permits a
   one-nullary declaration overlay to cover the marginal family of any
   one-clause legacy surface. It needs a typed bridge proving that the surface
   actually presents that formation or constructor; otherwise an unrelated
   path, Pi, or modal family can be hidden behind the declaration atom.
2. **Declaration-class marginality.** Fresh binder identity is not semantic
   novelty. The checker needs a predecessor closure of normalized/univalent
   declaration signatures so that a repeated equivalent one-constructor
   structure does not receive another atom.
3. **Demand-anchor internality.** The sequential runner checks that the slot
   is live and unused, but the emitted basis anchor is not itself bound to a
   replayable live-ledger witness. Absence, staleness, and reuse need negative
   certificate tests.
4. **Free-completion typing.** The eliminator and beta conclusions are
   currently generated target identifiers with registered edges, not typed
   schema objects reconstructed by the declaration kernel. A real replayable
   one-constructor free-completion token is still required.

These defects do not rescue the Stage-3 candidate: rejecting its completion
certificate would still leave no candidate above `4/3`. They do mean that the
artifact is a reproducible diagnostic experiment, not a completed semantic
proof.

Secondary audit issues are that the artifact's frozen rule fields hash version
labels rather than all source bytes, minimum-basis ordering is keyed by atom
hash rather than directly by semantic-family order, post-hoc winner
equivalence is raw telescope equality, and the writer used for the first burn
did not refuse an existing EGP-v2 output path. The reproduction entrypoint has
since been hardened to refuse every existing output path; the remaining
certificate issues belong in a successor program.

## 4. Methodological consequence

EGP-v2 is burned. The preregistration expressly forbids changing declaration,
closure, demand, or tie-break rules after observing a Stage 1--4 result. The
items above therefore cannot be described as EGP-v2 fixes. Any repaired run is
**EGP-v3** and needs a new preregistration, law identifier, schema version, and
artifact path.

Recommended EGP-v3 sequence:

1. Add typed formation-presentation and constructor-presentation bridge
   tokens, with negative tests over unrelated one-clause surfaces.
2. Canonicalize declaration signatures independently of fresh binder IDs and
   compare them against a predecessor declaration closure.
3. Bind demand provenance to a replayable complete ledger witness and add
   absent, stale, reused, and post-sealing rejection tests.
4. Construct and type the one-constructor eliminator and beta schemas in the
   declaration kernel; make free-completion coverage replay those tokens.
5. Freeze actual source-file hashes and semantic-family tie-break ordering,
   then preregister the score question explicitly:
   - retain `nu=|G|`, in which case the principled expectation is a Stage-3
     halt; or
   - introduce a generic, separately justified debt-discharge or universal-
     property atom. This is a new Selective-Law axiom, not a reinterpretation
     of the same atom and not a historical score exception.

Only after that Stage 1--4 gate passes should a new program be registered for
Stages 5--15. Nothing in this run proves the canonical fifteen-step history or
the global halt at fifteen.
