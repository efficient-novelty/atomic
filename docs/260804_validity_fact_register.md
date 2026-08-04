# Validity Fact Register: the `Valid_E` Evidence Enumeration

Date: 2026-08-04

Status: DRAFT-NOT-FROZEN (formulation lane, plan binding decision 7)

This document mints no authority. It is formulation-lane text under binding
decision 7 of `docs/260804_autonomous_plan_and_status.md` (the plan). Its
registered freeze gate is the KW1 task-extension-authority gate: the register
freezes together with KW1, after adversarial review, no later than K1 (plan
§6.4), and in every case before any profile sees the registered prefix (plan
§5.6). Until that gate, every enumeration below may change freely and nothing
here may be cited as a frozen fact.

## 1. Purpose

Plan §3.2 defines `Valid_E(P)` over a registered body of development evidence
`E` and states that this document enumerates `E`. The register exists to make
two commitments checkable before exposure:

1. `E` is one fixed, finite, artifact-backed list. It is identical for all
   eight profiles of the `{C0,C1} x {S0,S_W,S+,S_up}` experiment (plan §8.3);
   "required to explain" never varies by profile. No profile may be handed a
   smaller or larger explanandum set, and no post-run edit may add or remove
   a member.
2. Every `Valid_E` bullet is bound to a named deciding suite or authority
   with fail-closed `Unknown` semantics (§4 below). A `Valid_E` verdict that
   cannot be decided is `Unknown`, not exclusion (plan §§3.2, 5.2).

The facts in `E` bind the common substrate that all eight profiles share
(plan §8.3), not per-profile run outcomes. A profile reproduces a registered
fact when the substrate it runs on continues to validate that fact's frozen
artifact; `E` never requires a profile's own run to re-enact any historical
outcome.

## 2. Explicit exclusion: the historical Genesis continuation

Reproduction of the historical Genesis continuation is **not** a member of
`E`. Its absence is never a `Valid_E` failure. Plan §5.1 is binding here: the
archived Genesis sequence is regression evidence only, and divergence from
the historical sequence is a mathematical result, not an automatic software
defect. Consequently:

- no entry below names, hashes, counts, or otherwise encodes the historical
  continuation, expected stages, or decoded physical interpretations;
- no future amendment may admit such an entry without violating plan §§2 and
  5.1, which place these outside generation, acceptance, quotienting,
  selection, and halting; and
- a profile whose sealed output diverges from the archive is evaluated by
  exactly the same `E` as one that does not.

## 3. DRAFT enumeration of `E`

The following list is gathered from the plan §6.1 discharged-foundations
table and the architecture record `docs/260730_architecture.md` §§4-5. Every
entry, and the list as a whole, REQUIRES CONFIRMATION AT FREEZE: at the KW1
gate each member must be re-enumerated against the immutable commit tree hash
backing its frozen or discharged status (binding decision 5), and any member
whose backing artifact has been re-versioned must be re-cited by its new
version.

- **E1. Registered Law V2A bootstrap validation.** The exact registered Law
  V2A bootstrap and export, with its fail-closed certificate contracts.
  Source: `pen-law` (architecture §4.3), validated through `pen-kernel`
  replay (architecture §4.1).
- **E2. Frozen H3/H4 profile-relative results.** The frozen narrow
  inductive-completion slice: H3's exhaustion of the narrow response carrier
  with its unique direct eliminator class, and H4's sealing, width-two
  inventory recomputation, and profile-relative debt-free halt. These facts
  are relative to the frozen H3/H4 profile and grant no generic advance or
  halt constructor. Source: `pen-demand` `gsc` subtree with pinned safe-Agda
  reference agreement (architecture §4.2), `pen-law` H3/H4 registry entries
  (§4.3), `pen-engine`/`pen-law-v2` specialized execution (§4.5).
- **E3. Pinned Rust/safe-Agda correspondence transcripts.** The
  byte-compared canonical transcripts of the production bridge: independent
  Rust replay, pinned-Agda generated-package acceptance, transcript
  agreement, and the private Phase H correspondence factory. Source:
  `pen-semantic-audit` production-refinement modules
  (`production_refinement_wire_authority`, `production_wire_replay`,
  `production_transcript`, `production_correspondence_factory`;
  architecture §5.2), plus the pinned Agda reference agreement of E2.
- **E4. Frozen semantic-audit negative results.** The completed
  semantic-audit artifacts held as frozen reference evidence (plan §6.1),
  including: the exact SR2 noninjectivity theorem (neither registered
  marginal vector admits a typed, role-preserving injection into the frozen
  SR2 codomain), retained strictly as negative evidence — `nu` is undefined,
  never reported as zero (plan §§1.1, 11.1); the verified-empty demand-orbit
  and realization censuses; and the restricted kernel-cost basis theorem.
  Source: `pen-semantic-audit` V3 modules (`sr2_noninjectivity_v3`,
  `sr2_dependency_support_v3`, `demand_orbit_census_v3`,
  `demand_realization_census_v3`, `restricted_kernel_cost_basis_v3`;
  architecture §5.2).
- **E5. Generic test and falsifier suites of each frozen layer.** For every
  row of the plan §6.1 discharged-foundations table — the
  Rust/safe-Agda bridge and minting factory; native carrier, root inventory,
  subject bundle, and typed-occurrence census; typed rewrite authority; JG1
  grammar and admission obligations; JG2a constructor/stage surface; the
  JG2b1 sealed-history chain; the JG2b2 substitution, occurrence, and
  ontology substrate; and the frozen A2-O/A3-O/A4 schema layers — the
  layer's canonical fixtures, mutation suites, and independent replays as
  committed with that layer. Source: `pen-kernel`, `pen-kernel-synthesis`,
  `pen-semantic-audit`, `pen-generative-audit`, `pen-sealed-history`
  (architecture §§4-5), each cited at freeze by its backing tree hash.

Membership rule pending freeze: an artifact enters `E` only if it is
recorded `Discharged` or `Frozen` in the plan ledger at the KW1 gate and is
committed (binding decision 5). Layers that freeze between this draft and
the gate (per the plan §6.4 implementation lane) enter E5 automatically
under that rule; nothing enters by narrative citation alone.

## 4. Per-bullet decidable checks

Plan §3.2 lists six `Valid_E` bullets. Each is mapped to a deciding suite or
authority. Every check is three-valued: pass, verified fail, or `Unknown`.
An undecidable, unexecuted, or resource-exhausted verdict is `Unknown`, and
`Unknown` never excludes a profile (plan §§3.2, 3.4, 5.2).

- **V1. Satisfies every constitutive and typing requirement.** Decided by
  `pen-kernel` bounded replay through the `pen-gf2` four-way boundary
  (architecture §§4.1, 4.4). Proven satisfies; refuted fails; outside
  fragment and resource-exhausted are `Unknown`.
- **V2. Reproduces all registered development facts it was required to
  explain.** Decided by replaying each `E` member (§3) on the profile's
  substrate against its committed baseline, byte-for-byte wherever the
  frozen artifact binds bytes or digests (plan §5.2: exact bytes before
  digest). Any member whose replay does not complete is `Unknown` for that
  member and hence for the bullet.
- **V3. Passes the complete generic test and falsifier suite.** Decided by
  the union of the E5 layer suites and the gamma-independent KW2 falsifiers
  (plan §8.4 KW2; falsifiers 9-10 execute only after JG9 and are not gate
  inputs before then). A suite row that cannot execute is `Unknown`; a
  caller-supplied pass vector mints nothing (plan §5.6).
- **V4. Returns `Unknown` rather than inventing answers outside its
  fragment.** Decided by the fail-closed outcome-taxonomy audit against the
  K2 outcome set (plan §8.4 K2), including mutant probes that present
  outside-fragment and resource-starved inputs and require
  `OutsideFragment`/`Unknown` rather than a fabricated disposition. An
  unauditable outcome path is `Unknown`.
- **V5. Exposes no oracle labels, expected trace, future survivor, or
  held-out result.** Decided by the dependency and isolation audit: the
  oracle firewall (`pen-oracle` forbidden as a production dependency,
  architecture §4.6), workspace isolation checks in the style of
  `scripts/check_generative_audit_isolation.py`, and the evidence-bound
  manifest/lockfile discipline (plan §5.5). A dependency that cannot be
  audited is `Unknown`, not presumed clean.
- **V6. Has complete theorem authority for every result it claims.** Decided
  by the authority-ordering audit (architecture §8.2): every claimed result
  must trace to an opaque verifier-minted capability with a single private
  constructor; a digest, Boolean, tag, count, or typed proposition
  descriptor is not authority (plan §5.2; lesson 15). A claim whose
  authority chain cannot be completed is `Unknown`.

A profile is excluded from extension comparison only on a verified fail of
some bullet; any `Unknown` bullet leaves `Valid_E(P)` itself `Unknown`, and
the plan §3.4 three-valued semantics then govern every comparison that would
consume it.

## 5. Registered open items

- **OPEN-1. Sense of "reproduces" in V2.** Byte-identical replay versus
  verified re-derivation up to a registered presentation quotient is
  underdetermined for entries whose frozen artifact binds meaning but not a
  canonical byte encoding. Adjudication: decided per `E` member at the KW1
  freeze during adversarial review; default is byte-identical wherever the
  backing artifact commits bytes or digests, and the chosen sense is
  recorded per member in the frozen register.
- **OPEN-2. Time-indexed E5 boundary.** Which layer suites are members
  depends on which layers are frozen at the gate. Adjudication: the §3
  membership rule is applied mechanically at the KW1 freeze against the
  plan ledger and commit tree hashes; no hand additions or omissions.
- **OPEN-3. Failure semantics of a substrate-side E2/E3 replay mismatch.**
  Whether a mismatch is a verified `Valid_E` fail for the profile or a
  substrate defect that invalidates the experiment run for all eight
  profiles is underdetermined (a shared-substrate fault cannot
  discriminate among profiles). Adjudication: resolved at the KW1 freeze;
  the draft position is that a mismatch reproducible on the bare common
  substrate aborts the run fail-closed for all profiles rather than
  scoring any one of them.

No open item may be resolved silently; each resolution is a recorded freeze
decision under adversarial review.

## 6. Freeze gate

This register freezes with KW1, after adversarial review, no later than K1,
and strictly before any profile sees the registered prefix (plan §§5.6, 6.4,
8.4). At the freeze: confirm every §3 entry against its committed tree hash
(binding decision 5), resolve every §5 open item, bind the §4 checks to
their concrete suite identities, and record the frozen register's own
digest. After the freeze, any change to `E` is a new version under plan
§5.5 and a new experiment under the K3 fresh-holdout rule; no in-place
amendment is permitted.
