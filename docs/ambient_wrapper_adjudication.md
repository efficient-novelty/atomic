# Domain adjudication: the ambient-telescope candidate wrapper

**Date:** 2026-07-21. **Status:** **ADOPTED** 2026-07-21 (see ADOPTION
block). Prepared from `docs/SCHEMA2_GLOBAL_E4_V8_RESULT.md`
(named gap `RAW_TELESCOPE_AMBIENT_MOTIVE_DECLARATION_MISSING`) and
`docs/SCHEMA2_CONTEXTUAL_INTERNALITY_RESULT.md` (term-level rule proven
against declared telescopes; the raw payload cannot carry the
declaration). This is a *representational* adjudication, kin to P1: it
changes what a raw candidate is as a serialized object, and no adopted
semantic rule.

## Rule: `ambient-telescope-candidate-wrapper-v1`

1. **The wrapper.** A versioned raw candidate is
   `{ clauses: exact frozen Telescope, ambient: ordered motive list }`,
   where every motive is B₁₅-formable within the frozen caps. The
   forgetful projection π discards motives and yields the frozen
   clause-level candidate and its ambient arity.
2. **Coverage and finiteness.** π must be surjective onto the frozen raw
   catalog, and the fiber over each catalog member is exactly the finite
   set of admissible motive vectors for its arity (finite because
   motives live within the frozen six-node caps). The wrapped domain is
   the total space of this fibration; closed candidates are the
   empty-ambient fiber, embedded unchanged.
3. **Declaration independence (the binding clause).** Motives are
   *enumeration data*: the enumerator lists all admissible motive
   vectors per catalog member, before and independently of any
   classification. The classifier consumes declarations; it never
   infers, selects, filters, or adjusts one. A wrapped candidate whose
   clauses are ill-typed against its own declaration receives a *named
   typed exclusion* — never a repaired motive, never Unknown-by-default.
4. **Binding.** Enumeration, hashing, certificate identity, and
   classifier replay all bind to the wrapper. Exhaustion is redefined
   over the wrapped domain. Closed-fiber classifications must agree
   exactly with the existing frozen-pipeline results.
5. **Archival conservativity.** All prior artifacts remain byte-stable
   and true as statements about the clause-level projection; the wrapped
   certificate carries the π-coverage proof connecting the two domains.
   The operational-halt record (checker-relative, A5-conditional) is
   unchanged in scope and is not retroactively reinterpreted.

**Grounds (non-numeric, non-verdict).** (i) The adopted contextual rule
already made declared Γ-motives the evidence form (its certificate
"records Γ's motives"); a domain whose members cannot carry that
declaration cannot host the adopted rule — the wrapper supplies the
carrier, nothing more. (ii) The declaration discipline is the program's
oldest pattern: boundary diagrams (v2), element overlays (v3), Step-8
signatures, endpoint ledgers — interpretation data is always registered
explicitly and versioned, never inferred at judgment time. This extends
that pattern to the last object that lacked it: the candidate itself.
(iii) Clause 3 is the user's own formulation, adopted verbatim: motives
declared independently of verdicts. No count, verdict, pending case, or
bar appears in any clause.

## Falsifiers

- **F-W1.** Any motive traced to a classification outcome — selected,
  filtered, repaired, or preferred by verdict — invalid on its face.
- **F-W2.** π-coverage failure in either direction (a frozen catalog
  member without wrapped preimages, or a wrapped candidate projecting
  outside the catalog) → domain broken; certificate invalid.
- **F-W3.** Any closed-fiber candidate classified differently by the
  wrapped and frozen pipelines → regression break; the wrapper, not the
  archive, is at fault.
- **F-W4.** The admissible motive space provably non-finite within the
  frozen caps → report verbatim; a cap adjudication is required, and
  silent truncation is forbidden.
- **F-A5 (standing).** A surviving Unknown in the wrapped rerun → the
  ladder continues honestly; no exhaustion license.

## Consequences upon adoption

The enumeration is rebuilt over the wrapped domain (create-new, v9);
the contextual rule becomes consumable at classifier level; the
genuinely-live-context fiber either closes under the adopted rules or
yields the next exact witness. Exhaustion, if proved, is proved over the
typed domain the adopted semantics actually speaks about — the first
time the enumerated object and the judged object coincide exactly.

## ADOPTION

**Adopted.** Recorded verbatim from the user's adoption message of
2026-07-21 (replay pins this text):

> I adopt `ambient-telescope-candidate-wrapper-v1`, binding enumeration,
> hashing, and classification to the wrapped candidate with its ordered
> B₁₅-formable ambient motives, with declaration independence, π-coverage,
> closed-fiber agreement, and archival conservativity as stated.
>
> — Halvor Lande, 21 July 2026

Amendment goes through a versioned successor; no silent modification.
