# Law V2 cheating threat model

Status: The initial oracle-dependency, source-input, physical-isolation, and
certificate replay controls plus a finite-relative census are implemented for
the new production closure. The full cubical, authoritative-census, and
theorem-level adversarial suites remain open.

“Cheating” here includes accidental information leakage, incomplete search
reported as a theorem, and implementation choices that silently decide a
question on which the laws are neutral. It does not require malicious intent.

## 1. Assets

The controls protect:

- soundness of Constitutive-Law certificates;
- completeness of demand extraction and derivability decisions;
- completeness of the response cone;
- neutrality among genuinely distinct acceptable acts;
- separation of semantic-family and structural registers;
- autonomous, proof-based halt;
- equivariance under renaming and presentation; and
- reproducibility and provenance of both legacy and lawful artifacts.

## 2. Trust boundary

Trusted only after version binding and replay:

- the small normalization, typing, derivation, equivalence, and certificate
  checkers;
- canonical serialization used for evidence identity, not selection.

A build-provenance verifier is a planned future trust component, not an
implemented verifier in the current repository. Until it exists and replays,
the production lane cannot claim complete build provenance.

Untrusted as theorem sources:

- generators and search heuristics;
- schedulers, worker order, caches, and persistence;
- legacy bars, scores, and accepted histories;
- target fixtures and the oracle decoder;
- human labels;
- reports and archived claims;
- resource exhaustion; and
- a candidate's self-reported certificates.

The checker must validate evidence without trusting the generator that
produced it.

## 3. Threats and required controls

| ID | Threat | Attack or failure mode | Required test | Passing result |
|---|---|---|---|---|
| T01 | Hidden target dependency | Reference telescopes, hashes, ledgers, labels, or generated target data enter the lawful dependency closure. | Physically remove `pen-oracle` and reference files before building and running the lawful binary. | Build and non-oracle execution succeed. |
| T02 | Target-length stop | `until_step = 15`, a loop bound, fixture length, or equivalent stops the run. | Search lawful source/configuration and inject a live prospective demand after the observed final act. | The engine continues or reports `Blocked`/`Unknown`; it does not halt by length. |
| T03 | Stage-index guidance | A stage number selects a family, cap, clause surface, or admissibility policy. | Replace stages with random opaque event IDs. | Same cone up to identifier renaming. |
| T04 | Semantic-name leakage | Operator or package names reveal expected roles. | Permute anonymous operator IDs and withhold the inverse map during execution. | Equivariantly permuted output. |
| T05 | Bar legislation | Bar clearance, overshoot, or efficiency gates a guarded candidate. | Remove diagnostics and all bar code from the lawful build. | Accepted cone is byte-identical modulo build provenance. |
| T06 | Structural-register laundering | Legacy `nu`, including the historical Step-15 value 103, is consumed as semantic-family authority. | Enforce typed registers and attempt cross-register certificate construction. | Construction or verification fails. |
| T07 | Expected-future bias | A candidate is retained because it permits the known next act or final length. | Disable every future-viability routine and remove future fixtures. | Same lawful cone. |
| T08 | Presentation-order tie break | Enumeration order, hash, or canonical presentation chooses one acceptable class. | Shuffle grammar, catalogs, workers, and hash implementation. | Same complete class set; no preferred branch. |
| T09 | Premature quotient | Distinct acts are collapsed by syntax, family instances are counted as families, or equivalent presentations are counted as worlds. | Replay each quotient rung with explicit transport evidence. | Multiplicity changes only where a certified quotient applies. |
| T10 | Incomplete frontier as uniqueness | Search finds one result within a cap and calls it unique. | Vary and enlarge operational caps; verify the response-bound certificate independently. | Same complete cone or `Unknown`, never a new “winner” after claimed completion. |
| T11 | Unsound pruning | A bar bound, coarse DP key, or target-shaped rule removes a legal response. | Replay every prune from its proof object and inject known key collisions. | Unproved prunes are rejected; collisions cannot inherit a disposition. |
| T12 | Demand packing | A paying candidate adds unrelated structure, or hides public obligations in private syntax. | Add coherent demand-disconnected components and opaque packed bundles. | Unowed components are factored out or rejected; every public irreducible clause pays once. |
| T13 | Outcome-filtered registration | Demand motives or assignments are narrowed after observing which fillers work. | Replay the declared dependent context over every kernel-admissible assignment. | Total specialization holds without outcome filtering, or registration fails. |
| T14 | Family/instance inflation | Many substitutions of one natural family mint many semantic values. | Generate uniform specializations and aliases. | They remain one family after the adopted quotient. |
| T15 | Missing grammar capability | Removing a needed primitive is mistaken for debt freedom. | Ablate a later required primitive. | Live unpaid demand, grammar synthesis, or `Unknown`; never false halt. |
| T16 | Resource-dependent theorem | Low memory or worker variation changes the chosen branch or yields halt. | Sweep memory, worker count, schedule, and checkpoint/resume order. | Same certified cone or `Unknown`. |
| T17 | False debt-free halt | An incomplete census or failed search reports `O = empty`. | Inject a demanded-but-underdetermined F1 scheme and replay the census independently. | Halt fails and F1 is reported. |
| T18 | Step-16 incompleteness | Family-level emptiness hides undecided concrete instances. | Independently enumerate and replay the prospective instance census. | Every instance is decided and live orbit count is zero before halt. |
| T19 | Oracle-trained heuristic | Heuristic constants encode the accepted trace even though direct fixture reads are absent. | Run conservative extensions, renamings, grammar permutations, and target-withheld clean-room builds. | Mathematical result is invariant; heuristic changes affect only resources. |
| T20 | Decoder backflow | Post-run labels or equivalence matches feed a later acceptance or certificate. | Taint decoder outputs and assert no path to lawful inputs. | No dependency or runtime channel exists. |
| T21 | Certificate self-authentication | A generator sets booleans or hashes that the verifier trusts without re-derivation. | Mutate each field, recompute outer serialization hashes, and replay with the small checker. | Mutation is rejected from underlying proof checks. |
| T22 | Bootstrap smuggling | Three target telescopes are loaded while the run claims empty-context derivation. | Remove bootstrap fixtures and inspect the declared bootstrap contract. | The run either uses the disclosed registered rule or supplies a uniqueness proof. |
| T23 | Final-shell overlabeling | A target-shaped `Next`/`Eventually` shell is labeled DCT without a filtered realization. | Remove the semantic model witness before decoding. | Decoder withholds `DCT` and reports an open realization obligation. |
| T24 | Evidence drift concealment | Archived certificates fail deterministic replay but are silently treated as current proofs. | Reissue certificates from definitions and compare exact digests and logical projections. | Drift is disclosed; affected gates fail closed or are explicitly testimony-only. |

## 4. Stage-4 freedom

Discovery order of the Stage-4 candidates must be randomized and every
acceptable class must survive until the full quotient is certified. No
candidate may be privileged by the enacted branch, minimum value, package
name, or serialization key.

There is a current specification blocker: the normative appendix both calls
four Stage-4 classes genuinely distinct under the full quotient and leaves
the Pi/Sigma act equivalence open, potentially yielding two worlds. Until a
versioned adjudication closes that question, the test oracle must distinguish:

- four known candidate presentations;
- the currently certified quotient rungs; and
- unresolved post-act-equivalence cone cardinality.

A test that simply requires four final worlds would itself encode an
unproved target answer.

## 5. Halt falsification

Halt has the highest proof burden because it makes a completeness claim. The
halt suite must independently check:

1. active-window extraction completeness;
2. derivability completeness for every extracted family and required
   instance;
3. expiration and weakening of older demands;
4. zero live obligation orbits;
5. explicit F1 execution and exclusion;
6. absence of target length from the decision; and
7. the distinction between `Halted`, `Blocked`, and `Unknown`.

Failure of any item prevents a halt certificate.

## 6. Register abuse tests

The following mutations must be rejected:

- retagging a legacy structural total as semantic without a semantic audit;
- omitting a register;
- using a semantic total from a different candidate or prefix;
- attaching archived totals as provenance anchors;
- counting aliases, forced projections, or uniform instances as new
  families; and
- using a diagnostic bar result as a guarded certificate premise.

Historical discrepancies must remain visible. They may not be “fixed” by
changing semantic extraction to recover archived structural totals.

## 7. Evidence retention

Every campaign records:

- exact source and binary digests;
- dependency and feature graphs;
- clean/dirty workspace state;
- all input digests and resource budgets;
- randomization seeds;
- complete candidate and prune manifests;
- certificate-checker versions;
- outcome, including `Unknown`;
- negative-control results; and
- decoder invocation, if any, after the lawful artifacts are sealed.

Failed, falsifying, and resource-exhausted runs are retained. Keeping only
successful runs is outcome filtering.

## 8. Release gates

A result may be advertised as B2 only when:

- T01 through T24 have executable tests or a documented, fail-closed
  disposition;
- oracle deletion and diagnostics deletion tests pass;
- the response-bound verifier establishes enumeration completeness;
- every law-level numeric field is register-safe;
- the halt certificate replays independently on every branch; and
- known specification blockers are resolved or explicitly excluded from the
  claim.

Phase-0 documentation alone satisfies none of these runtime gates.

## 9. Recovered provenance input

The normative appendix states that
`docs/app_a_two_laws_formal_axioms_old.tex` preserves the earlier bar-law
formulation. The exact source has been recovered byte-for-byte from the sibling
`book` repository and is now SHA-256-bound in the partial legacy freeze
manifest. This authenticates that historical input; it does not complete the
remaining replay, artifact, dependency, or platform freeze requirements.
