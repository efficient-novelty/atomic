# Schema2 global E-4 assembly result

**Date:** 2026-07-20. **Status:** assembly executed; F-G4 triggered; global
E-4 completeness not proved.

The create-new artifact is `docs/schema2_global_e4_assembly_v1.json`, with
result digest
`blake3:edb3260ba046361b80f00d248a0fa8d5f27625a3041fcfd6532fba998e076a0e`.
Definition replay succeeds.

## Assembly disposition

The run replayed and joined:

- the G-2--G-7 grammar-completion certificate
  (`blake3:26ccc358c25ace6501e81125363064730c1642cfe775b0a931e8298bc5d60215`);
- the G-8 total disjoint-sum classifier
  (`blake3:d19d97137f6ebb8fcda38117603189beda62d70eebe5692ecb4acff8e7f308da`);
- the current E-4 legal-morphism generator-basis audit
  (`blake3:aa37c38051d2ae09eca50fc9557a41b06e1920bfa9a32785adf98f40659e3446`).

The joined inputs are sufficient to execute global assembly. They are not
sufficient to prove class exhaustion, because the run finds a concrete live
F-G4 witness.

## Exact F-G4 witness

In the frozen Step-16 raw context (library size 15, base scope 2, path
dimension at most 1, modal and temporal constructors enabled, expression
size at most 6), the two-clause telescope

```text
[ Formation: Univ,
  Introduction: Lam(Var(1)) ]
```

is an exact raw-catalog member. Its typed elaboration succeeds, but the G-8
classifier returns `TelescopeClass::Unknown` with the required named
obstruction
`F_G4_TYPED_CANDIDATE_OUTSIDE_EVERY_ADOPTED_SCHEMA2_CLASS`.

- Candidate digest:
  `blake3:09e281cf55d61d66ec1b64dcac943fcf0ef8b138a915da508eaf7677be345f75`
- Elaboration derivation:
  `blake3:ca690e741ea0351818a3b71ffb588881eddab6e5864f44724aa33461ad1991a9`
- Witness derivation:
  `blake3:d0d50f27c29378848d3aaa9dc3d0485225377a8c9fa07383b6a23cacf5cd363c`

One exact typed-Unknown member is enough to refute the class-exhaustion
premise, so the assembly terminates fail-closed at F-G4. This is consistent
with G-8: G-8 proved a total *disjoint-sum decision procedure*, not the
absence of its named-obstruction branch.

## Downstream firewall

The certificate records all of the following as false:

- global E-4 completeness;
- authorization of the five pending membership verdicts;
- issuance of independent-family tokens;
- E-2b execution, historical score recomputation, or F-Q2 evaluation;
- Agent A handoff; and
- any halt or continuation conclusion.

No downstream verdict or count was run.

## Required successor

Adjudicate a versioned grammar/classifier successor for mixed
formation/introduction shapes beginning with `[Univ, Lam(Var(1))]`, or revise
the frozen raw domain through an explicit adopted rule that excludes the
witness for semantic reasons. Then emit a create-new successor artifact and
rerun global E-4 assembly. The witness must not be silently omitted or
relabelled to force exhaustion.
