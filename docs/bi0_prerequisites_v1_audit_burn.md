# BI-0 prerequisite v1 audit burn

**Date:** 2026-07-22. **Status:** **BURNED AS BI-0 AUTHORITY**.
This is an implementation audit, not a new semantic rule. It preserves the
two v1 artifacts as regression witnesses and applies the falsifiers already
adopted in `docs/bi0_prerequisites_adjudication.md`.

## Artifacts audited

- `docs/t_bi_nu1_act_local_provenance_v1.json`
  (`blake3:522d73cc46ac6dcc68fd8c363c6e6cabf98e13a37b5ea5903667185e7d841515`).
- `docs/chronological_interface_slot_map_v1.json`
  (`blake3:9079f6c3c7509d7d71ba581538dfb617af1f794b94eba6a417b24617e53201e5`).

Neither file is deleted or rewritten. Their successful self-replays certify
what the v1 programs computed; they do not certify that those programs met
the adopted theorem obligations.

## Burn 1: candidate-minted demand outputs

The v1 T-BI-NU1 issuer begins from `structural_nu`, reconstructs its scalar
with hand-authored role cells, and makes authority depend on equality with
that scalar. When a cell cannot receive a fresh local-role charge, the issuer
constructs a new `output_position_id` from the candidate's role, family, and
type and attaches it to the one pre-existing structural orbit. The output
position therefore did not exist before the candidate and is not an element
of the orbit's independently enumerated finite required-output set.

The emitted evidence makes the failure concrete. At Stage 15 one live orbit
funds 92 demand-anchored tokens, including 42 uniform-instance tokens; those
92 output-position hashes were minted from the candidate. This violates the
adopted requirements that demand outputs pre-exist the candidate and that
uniform specializations not multiply without independently exported demand
outputs. F-AL2/F-AL3 apply: the v1 decomposition is not authoritative and
cannot open BI-0, irrespective of its exact F-AL1 numerical replay.

## Burn 2: typing was promoted to Internal

The v1 chronological membership issuer correctly checked the declared
order-preserving slot map, constructed and replayed a typed structural
substitution, and checked its result. It then equated that typed substitution
with an `Internal` closure preimage without issuing and replaying the adopted
motive-parametric closure-under-substitution theorem. The missing step is
load-bearing for F-SM1: a well-typed specialization is not by itself a proof
that the source closure derivation specializes to the demanded family.

The existing proof-strength specialization theorem accepts closed,
motive-typed assignments. General chronological schemes contain open target
variables, so unsupported rows must remain named gaps unless an open
contextual specialization theorem is separately proved. F-SM1 therefore does
not pass in v1.

## Sequencing consequence

The v1 prerequisite artifacts are forbidden BI-0 inputs. Versioned v2
issuers must:

1. enumerate natural families first, quotient instances, and inject credit
   only into an actual `(clause, LocalRole)` slot or an actual pre-existing
   finite demand-output position;
2. replay the proof-strength closure specialization for chronological rows,
   returning a named gap wherever its premises cannot be formed; and
3. publish F-AL1 and F-SM1 outcomes without using the archived answers to
   repair either construction.

Only passing v2 prerequisite certificates may authorize a BI-0 create-new
rerun. BI-1, BI-4, the bridge, and UC-1 scoring remain closed meanwhile.
