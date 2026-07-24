# BI-4 v2 cone report

**Date:** 2026-07-23. **Zone:** `Z-SPLIT`. **Refinement:** `Z-SPLIT-4`. **Certificate:** `blake3:75a9dcc023e8a7e7b00d9e20ced5516cb0d08f8b88ffd80f93d45e2f746191b1`.

The exact frozen BI-2 v2 aggregate `blake3:cfa5bbd27c6f2c9c788ae6eac68e2128f87d18ce54362e72850b8d3aa1fc03ff` replayed before comparison: **true**. All four branch certificates reissued, and the enacted root was retained only as indexical testimony, never as a comparison baseline or selector. The full frozen taxonomy was applied after certifying `Z-TREE`/`Z-STOP` preconditions as **false/false**.

| Measurement | Verdict | Exact basis |
|---|---|---|
| `G2_obligation_profile` | `Passed` | all six pairwise comparisons of the original Stage 5-15 required-package/constructor/live-count/total-discharge/debt-free rows plus terminal successor 16 |
| `G3a_stage5_to15_kappa_semantic_nu` | `Passed` | all six pairwise comparisons of replayed branch-local (kappa,semantic_nu) rows at stages 5-15; winner identity excluded |
| `G3b_complete_sigma_semantic_nu` | `Refuted` | all six pairwise comparisons of complete Stage 1-15 semantic Sigma-nu; the common Stage 5-15 suffix is not substituted |
| `G3c_diagnostic_bar_trajectory` | `Refuted` | all six pairwise comparisons of exact d=2 bars recomputed from each complete ordered Stage 1-15 semantic ledger |
| `G3_numeric_ledger_composite` | `Refuted` | the frozen composite passes iff G3a, G3b, and G3c all pass; sub-reporting does not weaken the registered standard |
| `cone_G4_debt_free_halt` | `Passed` | four replayed BI-2 v2 finales with halt=15, successor=16, exact D, semantic O16 empty, F1 executed/excluded, Theorem-12 full-instance proof, E5 complete, zero gaps, and one branch-index-free halt premise |

## All six branch pairs

### G2

| Left | Right | Equal | First stage | Exact context |
|---|---|---:|---:|---|
| `2016726758f3` | `43a0ed707770` | true | — | G2 exact obligation profiles agree at stages 5-15 and the debt-free successor 16 |
| `2016726758f3` | `4b2211ecae25` | true | — | G2 exact obligation profiles agree at stages 5-15 and the debt-free successor 16 |
| `2016726758f3` | `b4f821d9bb28` | true | — | G2 exact obligation profiles agree at stages 5-15 and the debt-free successor 16 |
| `43a0ed707770` | `4b2211ecae25` | true | — | G2 exact obligation profiles agree at stages 5-15 and the debt-free successor 16 |
| `43a0ed707770` | `b4f821d9bb28` | true | — | G2 exact obligation profiles agree at stages 5-15 and the debt-free successor 16 |
| `4b2211ecae25` | `b4f821d9bb28` | true | — | G2 exact obligation profiles agree at stages 5-15 and the debt-free successor 16 |

### G3a

| Left | Right | Equal | First stage | Exact context |
|---|---|---:|---:|---|
| `2016726758f3` | `43a0ed707770` | true | — | G3a exact-certified (kappa,semantic_nu) vectors agree at every stage 5-15; winner identities are excluded |
| `2016726758f3` | `4b2211ecae25` | true | — | G3a exact-certified (kappa,semantic_nu) vectors agree at every stage 5-15; winner identities are excluded |
| `2016726758f3` | `b4f821d9bb28` | true | — | G3a exact-certified (kappa,semantic_nu) vectors agree at every stage 5-15; winner identities are excluded |
| `43a0ed707770` | `4b2211ecae25` | true | — | G3a exact-certified (kappa,semantic_nu) vectors agree at every stage 5-15; winner identities are excluded |
| `43a0ed707770` | `b4f821d9bb28` | true | — | G3a exact-certified (kappa,semantic_nu) vectors agree at every stage 5-15; winner identities are excluded |
| `4b2211ecae25` | `b4f821d9bb28` | true | — | G3a exact-certified (kappa,semantic_nu) vectors agree at every stage 5-15; winner identities are excluded |

### G3b

| Left | Right | Equal | First stage | Exact context |
|---|---|---:|---:|---|
| `2016726758f3` | `43a0ed707770` | false | 4 | G3b complete Stage 1-15 semantic Sigma-nu differs: left=32, right=31; the first source-row difference is Stage 4 with left semantic_nu=3 and right semantic_nu=2 |
| `2016726758f3` | `4b2211ecae25` | false | 4 | G3b complete Stage 1-15 semantic Sigma-nu differs: left=32, right=31; the first source-row difference is Stage 4 with left semantic_nu=3 and right semantic_nu=2 |
| `2016726758f3` | `b4f821d9bb28` | true | — | G3b complete Stage 1-15 semantic Sigma-nu agrees at 32 |
| `43a0ed707770` | `4b2211ecae25` | true | — | G3b complete Stage 1-15 semantic Sigma-nu agrees at 31 |
| `43a0ed707770` | `b4f821d9bb28` | false | 4 | G3b complete Stage 1-15 semantic Sigma-nu differs: left=31, right=32; the first source-row difference is Stage 4 with left semantic_nu=2 and right semantic_nu=3 |
| `4b2211ecae25` | `b4f821d9bb28` | false | 4 | G3b complete Stage 1-15 semantic Sigma-nu differs: left=31, right=32; the first source-row difference is Stage 4 with left semantic_nu=2 and right semantic_nu=3 |

### G3c

| Left | Right | Equal | First stage | Exact context |
|---|---|---:|---:|---|
| `2016726758f3` | `43a0ed707770` | false | 5 | G3c diagnostic bar first differs at Stage 5: left=25/21 (phi=5/3,omega=5/7); right=20/21 (phi=5/3,omega=4/7) |
| `2016726758f3` | `4b2211ecae25` | false | 5 | G3c diagnostic bar first differs at Stage 5: left=25/21 (phi=5/3,omega=5/7); right=20/21 (phi=5/3,omega=4/7) |
| `2016726758f3` | `b4f821d9bb28` | true | — | G3c exact recomputed descriptive bar trajectories agree at stages 5-15 |
| `43a0ed707770` | `4b2211ecae25` | true | — | G3c exact recomputed descriptive bar trajectories agree at stages 5-15 |
| `43a0ed707770` | `b4f821d9bb28` | false | 5 | G3c diagnostic bar first differs at Stage 5: left=20/21 (phi=5/3,omega=4/7); right=25/21 (phi=5/3,omega=5/7) |
| `4b2211ecae25` | `b4f821d9bb28` | false | 5 | G3c diagnostic bar first differs at Stage 5: left=20/21 (phi=5/3,omega=4/7); right=25/21 (phi=5/3,omega=5/7) |


## Branch measurements

| Branch | Enacted | Original G2 discharge predicate | sum kappa | complete sum semantic nu | suffix sum nu 5-15 | halt/O16/F1/E5 |
|---|---:|---:|---:|---:|---:|---|
| `2016726758f3` | true | true | 64 | 32 | 27 | `true/true/true/true` |
| `43a0ed707770` | false | true | 64 | 31 | 27 | `true/true/true/true` |
| `4b2211ecae25` | false | true | 64 | 31 | 27 | `true/true/true/true` |
| `b4f821d9bb28` | false | true | 64 | 32 | 27 | `true/true/true/true` |

## Diagnostic bar trajectories

- `2016726758f3`: 5=25/21, 6=44/25, 7=7/4, 8=315/208, 9=544/441, 10=88/85, 11=356/319, 12=1512/1513, 13=1631/1920, 14=8671/10951, 15=305/406
- `43a0ed707770`: 5=20/21, 6=8/5, 7=13/8, 8=147/104, 9=170/147, 10=33/34, 11=1691/1595, 12=1440/1513, 13=233/288, 14=8294/10951, 15=7625/10556
- `4b2211ecae25`: 5=20/21, 6=8/5, 7=13/8, 8=147/104, 9=170/147, 10=33/34, 11=1691/1595, 12=1440/1513, 13=233/288, 14=8294/10951, 15=7625/10556
- `b4f821d9bb28`: 5=25/21, 6=44/25, 7=7/4, 8=315/208, 9=544/441, 10=88/85, 11=356/319, 12=1512/1513, 13=1631/1920, 14=8671/10951, 15=305/406

## First new divergence

First new divergence is G3b for branch pair blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407 versus blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308 at source stage 4: G3b: G3b complete Stage 1-15 semantic Sigma-nu differs: left=32, right=31; the first source-row difference is Stage 4 with left semantic_nu=3 and right semantic_nu=2; G3c: G3c diagnostic bar first differs at Stage 5: left=25/21 (phi=5/3,omega=5/7); right=20/21 (phi=5/3,omega=4/7). Composite G3=refuted; no divergence was averaged, suppressed, or repaired.

The inherited G1 R-T2 refutation remains testimony and was not re-litigated. Its self-digest, exact inequivalent outcome, order-reversal audit, selector absence, and six inequivalent five-scheme pairs replayed: **true/true/true/true/true**.

## F-R3-B1 branch-index disposition

| Claim | Disposition | Exact premise |
|---|---|---|
| `G2_obligation_profile_and_one-demand-per-stage_O_ladder_from_Stage5_through_halt` | `PromotedToConeLevel` | `G2_pass` |
| `G3a_per_stage_kappa_and_semantic_nu_vector_Stage5_through15` | `PromotedToConeLevel` | `G3a_pass` |
| `complete_Stage1_through15_Sigma_semantic_nu` | `RemainsBranchIndexed` | `G3b_pass` |
| `diagnostic_bar_trajectory_and_WB1_cumulative_tables` | `RemainsBranchIndexed` | `G3c_pass` |
| `G4_debt_free_halt_at_15` | `PromotedToConeLevel` | `cone_G4_pass` |
| `semantic_O16_empty_at_full_instance_granularity` | `PromotedToConeLevel` | `cone_G4_pass` |
| `Guard_Rail_F1_excluded` | `PromotedToConeLevel` | `cone_G4_pass` |
| `Theorem12_full_instance_granularity_relative_to_adopted_A3` | `PromotedToConeLevel` | `cone_G4_pass` |
| `Stage4_root_act_identity_and_semantic_nu` | `RemainsBranchIndexed` | `R-T3_free_choice_and_G3b_G3c_refutation` |
| `G1_Stage5_successor_scheme_sets_and_exact_derivations` | `RemainsBranchIndexed` | `inherited_R_T2_refutation` |
| `branch_specific_candidate_winner_and_act_local_provenance_identities` | `RemainsBranchIndexed` | `G3a_excludes_identity_and_compares_only_kappa_semantic_nu` |
| `complete_Stage1_through15_Sigma_kappa` | `RemainsBranchIndexed` | `not_a_registered_G3_sub_verdict` |
| `complete_cumulative_Delta_Phi_Omega_interface_trajectories` | `RemainsBranchIndexed` | `G3b_and_G3c_pass` |

## Frozen drift and correspondence hygiene

BI-1b public deterministic replay passed: **false**; frozen-testimony fallback used: **true**; exact disclosed errors: `["BI-1b sweep differs from deterministic reissuance"]`. The drift was repaired inside assembly: **false**; concealed: **false**.

The correspondence ledger was opened after every comparison verdict: **true**. It corroborates 24/24 Stage-8–15 rows and was used by a verdict, selector, or provenance issuer: **false/false/false**.

No UC-1 content, bridge execution, or final certificate is present: **true/true/true**.

BI-4 v2 assigns Z-SPLIT with refinement Z-SPLIT-4 from the computed comparisons: G2=passed, G3a=passed, G3b=refuted, G3c=refuted, composite-G3=refuted, and cone-G4=passed. Cone-level promotion and retained branch indices are exactly those in the disposition table. No UC-1, bridge, or final-certificate conclusion is issued.