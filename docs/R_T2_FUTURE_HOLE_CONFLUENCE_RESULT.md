# R-T2 future-hole confluence result

**Date:** 2026-07-21  
**Schema:** `r-t2-future-hole-confluence-v1`  
**Status:** **V1 SCOPED COUNTEREXAMPLE; LAW-LEVEL R-T2 STILL OPEN**

> **Post-audit correction.** The four-branch comparison is mechanically sound
> over the frozen five-scheme surface: all six pairs are genuinely
> inequivalent there. However, the A3 generator supplying those surfaces
> still explicitly marks its intended rule-constructor inventory
> non-exhaustive. Missing schemes could in principle change set equivalence,
> so v1 cannot yet refute confluence of the *full intended* successors or
> lawfully open R-T3. The artifact remains an immutable scoped
> counterexample; a versioned rerun must first consume the full-A3
> exhaustiveness theorem.

## Result

The create-new run instantiated the Stage-5 next-window A3 scheme set
independently over all four singleton R-T1 classes.  Each branch generated
five typed schemes, and all six pairwise frozen-set comparisons were well
formed.  No pair admitted a perfect matching under the frozen certified
family quotient.  Consequently:

- `all_stage5_scheme_sets_equivalent = false`;
- `r_t2_confluence_proved = false`;
- `r_t2_confluence_refuted = true`;
- `r_t3_user_adjudication_opened = true`.

The certificate selected no candidate. It did not authorize T-BF1, T-BF3,
bar-free adoption, the bridge, or a halt claim. Its post-audit status also
means R-T3 remains closed pending the complete-grammar rerun.

## What was compared

For each Stage-4 minimizer, the runner constructed a fresh sealed prefix
containing exactly Steps 1--3 plus that minimizer at Step 4.  It then invoked
the count-blind arbitrary-prefix A3 generator at Stage 5.  Raw signature,
scheme, instance, family, and derivation identifiers were retained only for
the evidence join; none entered a semantic family key or selected a branch.

Each branch also issued exactly one structural future-hole judgment through
`issue_structural_future_holes_for_window`.  The four judgments agree on the
branch-independent content:

- constructor `initial_hit`;
- body `Var(1)`;
- declared motive `Type`;
- source and output kernel type `Type`;
- typed open elaboration, transparent-closure replay, generic substitution,
  and hypothetical D-membership;
- zero marginal kappa and nu, zero anchors and demand orbits, and no credit;
- no named gap.

Their shared semantic key is
`blake3:790ffcd61dd4844492f9125f0df70ff756bd3b51ae03f69ce516b21d0f1cf121`.
This proves equality of the structural constructor predicate, but the runner
records explicitly that this equality was **not** used as semantic
confluence.

The actual R-T2 comparison covered the entire five-scheme formation set:
rule constructor, origin, support depth, ordered source-family list, output
shape, parameter sorts, kernel types, and beta-normal/univalent equality of
the canonical family presentations.  A bipartite perfect matching was then
computed from those semantic equality edges.

## Pairwise result

The four branches are the two independent Stage-4 axes: `Pi` versus `Sigma`
and application argument order `(Var(2), Var(3))` versus
`(Var(3), Var(2))`.

| Left branch | Right branch | Matched of 5 | Verdict |
|---|---|---:|---|
| `201672...` (`Pi`, 2-3) | `43a0ed...` (`Pi`, 3-2) | 3 | inequivalent |
| `201672...` (`Pi`, 2-3) | `4b2211...` (`Sigma`, 3-2) | 2 | inequivalent |
| `201672...` (`Pi`, 2-3) | `b4f821...` (`Sigma`, 2-3) | 3 | inequivalent |
| `43a0ed...` (`Pi`, 3-2) | `4b2211...` (`Sigma`, 3-2) | 3 | inequivalent |
| `43a0ed...` (`Pi`, 3-2) | `b4f821...` (`Sigma`, 2-3) | 2 | inequivalent |
| `4b2211...` (`Sigma`, 3-2) | `b4f821...` (`Sigma`, 2-3) | 3 | inequivalent |

Thus the common `InitialHit` label does not erase the semantic dependence of
the next window on the selected former and ordered application family.  This
is an immediate, typed confluence counterexample, so no additional
future-isomorphism premise is needed to refute R-T2.

## F-RT discipline

The artifact records:

- all four Stage-4 candidates answer the same certified live
  `former_eliminator` demand and are strict total dischargers;
- the four minimizer hashes join extensionally across T-BF1, the predecessor
  tie rerun, and the R-T1 transport artifact;
- every branch was generated independently over its exact prefix;
- all six comparisons are well formed;
- enumeration/hash order was not a selector;
- desired history, counts, scores, and the bar were not premises;
- `selected_candidate_hash = null`.

R-T3 opens only because `immediate_inequivalent_successor_counterexample` is
true.  It is the next lawful action.  The bridge and final certificate remain
blocked.

## Artifacts and verification

- Certificate: `docs/r_t2_future_hole_confluence_v1.json`
- Issuer/replay: `crates/pen-search/src/r_t2_future_hole_confluence.rs`
- Create-new driver: `crates/pen-search/examples/r_t2_future_hole_confluence.rs`
- Result digest:
  `blake3:dc08580127f3176c6875ba042b2e788b3d0849da69dd0a12305798db08bb71f2`

Focused verification:

```text
cargo test -p pen-search --example r_t2_future_hole_confluence -- --nocapture --test-threads=1
test result: ok. 3 passed; 0 failed
```

The tests cover the live full-scheme run, the rule that constructor equality
alone cannot prove confluence, and rejection of a redigested forged
confluence/selection/downstream promotion.
