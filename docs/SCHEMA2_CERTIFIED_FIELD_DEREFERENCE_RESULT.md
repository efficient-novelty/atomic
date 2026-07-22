# Schema2 certified-field dereference result

**Date:** 2026-07-21. **Status:** the adopted dereference rule is implemented
at term level and resolves the global E-4 v6 survivor with zero marginal
credit.

The create-new artifact is
`docs/schema2_certified_field_dereference_v1.json`, with result digest
`blake3:b702322fb0fa9501572650b03ad9f9990df239ce517981f7eb133453765e2585`.
Definition replay succeeds.

## Certified dereference

The successor token accepts a candidate-field reference only when its exact
referent clause has a nonempty `Internal` certificate hash in the strictly
prior ordered prefix. It recursively inlines that referent, transports bound
levels, and replays the unchanged ambient-former token on the resulting direct
candidate.

The raw kernel judgment at a field head can be deliberately coarse. For the
registered witness it is `Fun(Neutral, Neutral)`, while certified inlining
refines the direct judgment to `Fun(Neutral, Type)`. The certificate records
this refinement and requires both the dereferenced normal form and the direct
normal form to be identical. The complete predecessor clause record and its
derivation hash remain in the candidate-level certificate.

Uncertified, forward/cyclic, mismatched, and unused certificates fail closed.
Guarded candidates still require their separate inverse theorem. `PathCon`
still fails as charged. Dereference mints no family, anchor, orbit, or credit.

## Resolved witness

The exact candidate

```text
[Univ, Lam(App(Var(1), Lib(15)))]
```

dereferences through clause 0 to

```text
[Univ, Lam(App(Univ, Lib(15)))]
```

and earns `Internal` with `nu = 0`. This authorizes global E-4 v7 create-new,
but no later output by itself.
