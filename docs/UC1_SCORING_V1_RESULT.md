# UC-1 scoring v1

**Date:** 2026-07-23. **Zone:** `Z-LEDGER`. **Certificate:** `blake3:74ede5271122f8f7998f46350ce95dd2f6ae7fa5d019bde6156ceaf5188b86c6`.

The sealed inputs replay and join: M-1 transport `true`, BI-2 `true`, and BI-4 `true`. Input surface: 22 [artifact_metadata]. Prerequisite reissuance is authentication only; it is never a scoring observation.

**R-T2 drift disclosure:** live current-source reissue valid = `false`; sealed fallback used = `true`; sealed fallback authenticated = `true`. Exact live errors: `live R-T2 v2 issuance failed`. Exact serialized blockers: `prerequisite failed: create-new A3 inventory artifact replay failed: certificate differs from definition replay`. The fallback requires, and this certificate separately records, the certificate self-digest, archived Stage-4 fork projection replay, and full P3 pair-surface gates.

Register notation: dates are `[artifact_metadata]`; P/M/F/U labels, Stage labels, hash suffixes, and the numeral in `Expr::Lib(4)` are `[syntax_identifier]`. Every quantitative payload below is rendered as `value [register]`.

## Mechanical score

| prediction | verdict | quantified evidence |
|---|---|---|
| P1 ledger identity | `Passed` | 4 [proof_inventory] branches, 6 [proof_inventory] unordered pairs, 11 [proof_inventory] stages |
| P2 winner identity modulo fork shadow | `Passed` | 18 [proof_inventory] normalized pair-stage comparisons plus 48 [proof_inventory] enacted-root-transitive pair-stage proofs |
| P3 margin confinement | `Refuted` | 6 [proof_inventory] counterexamples over 6 [proof_inventory] unordered pairs |

Every displayed quantity is followed by its numeric register. Structural testimony is not consumed as semantic authority.

## P2 fork-shadow normalization

The relabeler recursively replaces every `Expr::Lib(4)` with a branch-neutral Stage-4-reference marker before comparing telescopes. A zero `[proof_inventory]` count means the registered relabeling was run but no such reference occurred.

| stage | branch pair | relabelled references (left / right) | normalized telescope equal |
|---|---|---:|---|
| 5 [syntax_identifier] | `2016726758f3` / `43a0ed707770` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 6 [syntax_identifier] | `2016726758f3` / `43a0ed707770` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 7 [syntax_identifier] | `2016726758f3` / `43a0ed707770` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 5 [syntax_identifier] | `2016726758f3` / `4b2211ecae25` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 6 [syntax_identifier] | `2016726758f3` / `4b2211ecae25` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 7 [syntax_identifier] | `2016726758f3` / `4b2211ecae25` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 5 [syntax_identifier] | `2016726758f3` / `b4f821d9bb28` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 6 [syntax_identifier] | `2016726758f3` / `b4f821d9bb28` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 7 [syntax_identifier] | `2016726758f3` / `b4f821d9bb28` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 5 [syntax_identifier] | `43a0ed707770` / `4b2211ecae25` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 6 [syntax_identifier] | `43a0ed707770` / `4b2211ecae25` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 7 [syntax_identifier] | `43a0ed707770` / `4b2211ecae25` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 5 [syntax_identifier] | `43a0ed707770` / `b4f821d9bb28` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 6 [syntax_identifier] | `43a0ed707770` / `b4f821d9bb28` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 7 [syntax_identifier] | `43a0ed707770` / `b4f821d9bb28` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 5 [syntax_identifier] | `4b2211ecae25` / `b4f821d9bb28` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 6 [syntax_identifier] | `4b2211ecae25` / `b4f821d9bb28` | 0 [proof_inventory] / 0 [proof_inventory] | true |
| 7 [syntax_identifier] | `4b2211ecae25` / `b4f821d9bb28` | 0 [proof_inventory] / 0 [proof_inventory] | true |

The later-band proof uses 24 [proof_inventory] sealed alternate-to-enacted rows to construct 48 [proof_inventory] pair-stage identities through enacted-root transitivity; no pair identity is assumed.

## P3 / F-UC2 counterexamples

The live demand is read from each retired BI-1 Stage-5 demand record and tested as the literal ordered pair required by its sealed R-T2 perfect matching.

| branch pair | live structural scheme IDs | matching edges | live pair matched |
|---|---|---:|---|
| `2016726758f3` / `43a0ed707770` | `effc6b16907f` / `eb17d204816b` | 3 [proof_inventory] | false |
| `2016726758f3` / `4b2211ecae25` | `effc6b16907f` / `ea777de7a4c3` | 2 [proof_inventory] | false |
| `2016726758f3` / `b4f821d9bb28` | `effc6b16907f` / `4ca45f603ddf` | 3 [proof_inventory] | false |
| `43a0ed707770` / `4b2211ecae25` | `eb17d204816b` / `ea777de7a4c3` | 3 [proof_inventory] | false |
| `43a0ed707770` / `b4f821d9bb28` | `eb17d204816b` / `4ca45f603ddf` | 2 [proof_inventory] | false |
| `4b2211ecae25` / `b4f821d9bb28` | `ea777de7a4c3` / `4ca45f603ddf` | 3 [proof_inventory] | false |

P3 is `Refuted`; F-UC2 burns UC-1c and therefore UC-1d.

## F-UC4

M-1 contributes 5 [proof_inventory] typed maps and 2 [proof_inventory] order-axis obstructions in its exact narrow proof-bearing transport class.

| fixed former | order candidates | semantic-family nu | accepted typed bijection absent |
|---|---|---:|---|
| `former=pi` | `2016726758f3` / `43a0ed707770` | 3 [semantic_family_nu] / 2 [semantic_family_nu] | true |
| `former=sigma` | `b4f821d9bb28` / `4b2211ecae25` | 3 [semantic_family_nu] / 2 [semantic_family_nu] | true |

The two within-former semantic-family 3-versus-2 `[semantic_family_nu]` obstructions burn UC-1a and UC-1d in the adopted class. They do not assess UC-1b's Pi/Sigma act-level former semantics.

## Disposition

`UC-1a = Burned`; `UC-1b = Untested`; `UC-1c = Burned`; `UC-1d = Burned`. The scored zone is `Z-LEDGER`. The sole former-axis residual is `U_T2_UNATTEMPTED`; it is nonblocking here because P3 is already refuted. This artifact does not guess Z-COLLAPSE or Z-HEAL.

Permitted conclusion: P1 Passed and P2 Passed, but P3 Refuted under F-UC2; F-UC4 independently burns UC-1a in the adopted narrow transport class. Therefore UC-1c and UC-1d are burned, the scored zone is Z-LEDGER, and UC-1b remains only as U_T2_UNATTEMPTED.

Forbidden conclusion: This artifact does not infer Pi/Sigma former equivalence or inequivalence, does not guess Z-COLLAPSE or Z-HEAL, does not broaden M-1's transport class, and does not use semantic nu, structural testimony, a bar, a hash, or enumeration order as a selector.
