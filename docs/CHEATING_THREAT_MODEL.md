# Law V2 cheating threat model

Status: The initial oracle-dependency, source-input, physical-isolation, and
certificate replay controls, finite-relative census, structural GF2
boundary, pinned checker-readiness probe, and registered-bootstrap replay are
implemented for the new production closure. The authoritative GF2 theorem
backend, authoritative census, and theorem-level adversarial suites remain
open; unrestricted ambient-cubical completeness is not a first-slice gate.

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

The current Agda integration is a readiness verifier, not a theorem verifier.
It checks an exact toolchain, pinned checkout HEAD, reviewed canonical source
tree, and internally fixed smoke source. The
readiness contract now binds a repository-reviewed canonical Cubical tree and
an independently reviewed, trusted-configuration primitive-runtime tree for
the exact Agda distribution, then checks private canonical snapshots. Imported
text must be UTF-8; CRLF is canonicalized to LF and bare carriage returns are
rejected, while primitive `.agdai` members remain raw. The local primitive
reference digest is a reproducibility aid rather than a trust anchor. There is
still no reviewed GF2-to-Agda translator or generated theorem module.

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
| T05 | Diagnostic legislation | Bar clearance, overshoot, `rho`, efficiency, or novelty authorizes a candidate, debt-free continuation, or halt. | Remove diagnostics and all bar/value code from the lawful build. | Admissibility, continuation, quotient classes, and halt evidence are unchanged. |
| T06 | Structural-register laundering | Legacy `nu`, including the historical Step-15 value 103, is consumed as semantic-family authority. | Enforce typed registers and attempt cross-register certificate construction. | Construction or verification fails. |
| T07 | Expected-future bias | A candidate is retained because it permits the known next act or final length. | Disable every future-viability routine and remove future fixtures. | Same lawful cone. |
| T08 | Presentation-order tie break | Enumeration order, hash, or canonical presentation chooses one acceptable class. | Shuffle grammar, catalogs, workers, and hash implementation. | Same GF2-complete class set; no preferred branch. |
| T09 | Premature quotient | Distinct acts are collapsed by syntax, family instances are counted as families, or equivalent presentations are counted as worlds. | Replay each quotient rung with explicit transport evidence. | Multiplicity changes only where a certified quotient applies. |
| T10 | Incomplete frontier as uniqueness | Search finds one result within a cap and calls it unique. | Vary and enlarge operational caps; verify the response-bound certificate independently. | Same GF2-complete cone or `Unknown`, never a new “winner” after claimed completion. |
| T11 | Unsound pruning | A bar bound, coarse DP key, or target-shaped rule removes a legal response. | Replay every prune from its proof object and inject known key collisions. | Unproved prunes are rejected; collisions cannot inherit a disposition. |
| T12 | Demand packing | A paying candidate adds unrelated structure, or hides public obligations in private syntax. | Add coherent demand-disconnected components and opaque packed bundles, including at an empty obligation profile. | Unowed components are rejected; every positive-cost clause lies in a live discharge dependency, and no positive-cost continuation survives when no demand is live. |
| T13 | Outcome-filtered registration | Demand motives or assignments are narrowed after observing which fillers work. | Replay the declared dependent context over every kernel-admissible assignment. | Total specialization holds without outcome filtering, or registration fails. |
| T14 | Family/instance inflation | Many substitutions of one natural family mint many semantic values. | Generate uniform specializations and aliases. | They remain one family after the adopted quotient. |
| T15 | Missing grammar capability | Removing a needed primitive is mistaken for debt freedom. | Ablate a later required primitive. | Live unpaid demand, grammar synthesis, or `Unknown`; never false halt. |
| T16 | Resource-dependent theorem | Low memory or worker variation changes the chosen branch or yields halt. | Sweep memory, worker count, schedule, and checkpoint/resume order. | Same certified cone or `Unknown(ResourceExhausted)`. |
| T17 | False debt-free halt | An incomplete, outside-fragment, or failed census reports `O = empty`. | Inject a demanded-but-underdetermined F1 scheme and replay the census independently. | F1 is reported, or the run returns the applicable `Unknown`; it never halts. |
| T18 | Step-16 incompleteness | Family-level emptiness hides undecided concrete instances. | Independently enumerate and replay the prospective instance census. | Every instance is decided and live orbit count is zero before halt. |
| T19 | Oracle-trained heuristic | Heuristic constants encode the accepted trace even though direct fixture reads are absent. | Run conservative extensions, renamings, grammar permutations, and target-withheld clean-room builds. | Mathematical result is invariant; heuristic changes affect only resources. |
| T20 | Decoder backflow | Post-run labels or equivalence matches feed a later acceptance or certificate. | Taint decoder outputs and assert no path to lawful inputs. | No dependency or runtime channel exists. |
| T21 | Certificate self-authentication | A generator sets booleans or hashes that the verifier trusts without re-derivation. | Mutate each field, recompute outer serialization hashes, and replay with the small checker. | Mutation is rejected from underlying proof checks. |
| T22 | Bootstrap smuggling | A Law V2A run loads three registered acts while claiming Law V2B empty-context derivation. | Inspect the manifest and remove the registered prefix. | V2A discloses its registered three-act input; V2B supplies a least-arena derivation and uniqueness proof. |
| T23 | Final-shell overlabeling | A target-shaped `Next`/`Eventually` shell is labeled DCT without a filtered realization. | Remove the semantic model witness before decoding. | Decoder withholds `DCT` and reports an open realization obligation. |
| T24 | Evidence drift concealment | Archived certificates fail deterministic replay but are silently treated as current proofs. | Reissue certificates from definitions and compare exact digests and logical projections. | Drift is disclosed; affected gates fail closed or are explicitly testimony-only. |
| T25 | Fragment escape | Work outside GF2 is treated as complete, blocked, unique, or halted. | Inject a well-formed out-of-fragment judgment and exhaust a certified budget independently. | `Unknown(OutsideFragment)` or `Unknown(ResourceExhausted)`; no negative theorem or halt. |
| T26 | Checker-readiness laundering | A successful smoke test, caller-selected fake executable, mutable imported source, hostile Git configuration, escaped child process, arbitrary Agda source, or unpinned checkout is presented as a GF2 theorem. | Mutate trusted executable-digest pins, version, Cubical/primitive tree, line endings, snapshot, checkout, library flags, source, invocation, output, and manifest digest; substitute a self-derived primitive pin; inject Git hooks/fsmonitor or content filters; leave a background descendant; attempt to submit caller source. | Executable bytes must match independently trusted pins before invocation; the Cubical canonical tree must match its repository pin; the exact Agda distribution must match an independently reviewed primitive-runtime pin; UTF-8 text is canonicalized to LF with bare carriage returns rejected and `.agdai` retained raw; canonical members are materialized and checked from private snapshots; local reference pins confer no authenticity; Git is restricted to sanitized non-content `rev-parse` probes with no `status`/attribute-filter path; process groups are contained; readiness replay fails on mutation; no arbitrary-source API exists; and no theorem capability is minted. |
| T27 | Bootstrap artifact contamination | Registered input carries downstream labels, candidates, totals, or a broken predecessor chain. | Inject unknown top-level and per-act fields and mutate every source/binding/chain digest. | Strict decoding or kernel replay rejects the artifact; V2A remains registration-only. |

## 4. Stage-4 freedom

Discovery order of the Stage-4 candidates must be randomized and every
acceptable class must survive until the full quotient is certified. No
candidate may be privileged by the enacted branch, minimum value, package
name, or serialization key.

The contract distinguishes four normalized representative slots from the
classes that survive the full quotient. The current repository has not yet
issued the four Law V2 representative certificates. Once those and the
order-axis obstruction replay, the only permitted class-count assertion is
`2 <= count <= 4`. The exact count remains open until the Pi/Sigma former-axis
equivalence is constructed or obstructed. A test requiring four final worlds
would encode an unproved target answer. Failure to settle the quotient returns
`Unknown(UnknownQuotient)`.

## 5. Halt falsification

Halt has the highest proof burden because it makes a completeness claim
relative to GF2. The halt suite must independently check:

1. active-window extraction completeness;
2. derivability completeness for every extracted family and required
   instance;
3. expiration and weakening of older demands;
4. zero live obligation orbits;
5. explicit F1 execution and exclusion;
6. absence of target length from the decision;
7. positive-cost exclusion from demand-connectedness, with zero-cost theorem
   readouts excluded from history extension; and
8. the distinction between `Halted`, `Blocked`,
   `Unknown(OutsideFragment)`, and `Unknown(ResourceExhausted)`.

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

Failed, falsifying, outside-fragment, unknown-quotient, and
resource-exhausted runs are retained. Keeping only successful runs is outcome
filtering.

## 8. Release gates

A result may be advertised as B2 only when:

- T01 through T27 have executable tests or a documented, fail-closed
  disposition;
- oracle deletion and diagnostics deletion tests pass;
- the response-bound verifier establishes GF2-relative enumeration
  completeness;
- every law-level numeric field is register-safe;
- the halt certificate replays independently on every surviving certified
  quotient class; and
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
