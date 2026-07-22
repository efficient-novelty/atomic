# Closure adjudication: certified-field dereference

**Date:** 2026-07-21. **Status:** **ADOPTED** 2026-07-21 (see ADOPTION
block). Prepared from `docs/SCHEMA2_GLOBAL_E4_V6_RESULT.md`
(surviving witness `[Univ, Lam(App(Var(1), Lib(15)))]`, refused by the
adopted F-A3 firewall — correctly, under its current width).

## The question

F-A3 excludes candidate-fresh application heads from the transparent
closure. The v6 witness's head is a candidate field — but one carrying a
replayed Internal certificate in the ordered prefix. Is a
*certified-Internal* field reference "fresh"? This adjudication draws the
line at certification rather than at field-hood, keeping F-A3 fully
intact for everything uncertified.

## Rule: `certified-field-dereference-rule-v1`

1. **Dereference.** A candidate-field reference whose referent clause
   carries a replayed Internal certificate strictly prior in the ordered
   prefix may serve as a premise of the transparent ambient-former
   closure — in any position, including application head. The closure
   judgment *dereferences*: it replays the referent's complete Internal
   derivation into the premise tree, and the judgment must be identical
   to the judgment on the direct form (the reference is semantically
   invisible; only provenance records the indirection).
2. **F-A3 sharpened, not weakened.** Candidate-field references without
   an Internal certificate, forward references, cyclic references, and
   candidate-declared fresh heads remain excluded exactly as before.
   F-A3's exclusion becomes precisely: *uncertified* heads.
3. **Zero credit; standing exceptions.** Dereference mints nothing; the
   orbit exception and the guarded inverse-law requirement are
   unchanged.

**Grounds (non-numeric, non-verdict).** (i) **A2 idempotence:**
D(D(B)) = D(B). A certified field is proven content of D(B₁₅); building
from it by ambient formers cannot exit the closure — derivable-from-
derivable is derivable. This is the one clause of the Guard-Rail's
standing axioms not yet cashed at constructor level. (ii) **Adopted
precedent:** the inductive-projection rule already admits certified
prior-field references in body position (`Lam(Var(1))` earned Internal
exactly this way); this rule extends the same ordered-prefix discipline
to head position, with the same fail-closed dependency checks.
(iii) **The dereferenced witness is a known family shape:**
`App(Univ, Lib(15))` is R1's completion form — independently derivable
on the adopted Stage-1 grounds. (iv) No count, verdict, pending case, or
bar appears in any clause.

## Falsifiers

- **F-D1.** Dereference attempted on an uncertified, forward, or cyclic
  field → fail closed (ordered-prefix discipline); classification
  unchanged.
- **F-D2.** A dereferenced judgment differing from the judgment on the
  direct form → dereference is not semantically invisible; soundness bug
  event; report verbatim; the rule's implementation, not the taxonomy,
  is at fault.
- **F-D3.** Any credit, anchor, or family minted through a dereference
  chain → invalid on its face.
- **F-A5 (standing).** The next assembly rerun finding a further Unknown
  → the ladder continues honestly; no exhaustion license.

## ADOPTION

**Adopted.** Recorded verbatim from the user's adoption message of
2026-07-21 (replay pins this text):

> I adopt `certified-field-dereference-rule-v1`, admitting
> certified-Internal field references as closure premises in all
> positions by semantically invisible dereference, with F-A3 retained
> for all uncertified references and the zero-credit clause unchanged.
>
> — Halvor Lande, 21 July 2026

Amendment goes through a versioned successor; no silent modification.
