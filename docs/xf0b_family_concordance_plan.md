# XF-0b — The Family Concordance Study

**Date:** 2026-07-24. **Status:** brief frozen. Campaign artifact of
XF-1 (not a prediction; ledger event on freeze). Successor to the XF-0
census, whose published mismatch pattern it exists to explain:
everywhere certified worth is rich (\(\ge 2\)), blind
independent-structure counting undercounts --- the semantic-family
quotient is strictly finer than standard-presentation counting. This
study asks the question that localization poses: \emph{finer how?}
For every certified family at the rich strata, either mathematics has
a name for it, or it does not --- and both answers are results.

**Scope.** The 21 certified families of the rich strata --- stage 4
(3, enacted branch), stage 5 (6), stage 6 (3), stage 10 (4), stage 13
(2), stage 14 (3) --- plus stage 15 (6), total 27; optionally extended
to all 32 for completeness. Inputs: the families' certified content at
claim granularity, from the sealed provenance artifacts bound in MS-1.
(The DNF-4 caveat is inherited honestly: family content is certified
at claim granularity, not universal-naturality granularity; the
concordance names what is certified, no more.)

**Information-flow declaration.** This study is \emph{not} blind and
does not pretend to be: it reads the certified register directly. It
is an interpretive audit, governed by the translation standard, and
its output contaminates any future blind work --- recorded now: any
XF-0 v2 census must disclose this study among the reasons its
blindness is weaker than v1's.

## The translation standard (the only admissible naming)

A family is \emph{named} only by a lawful translation in the book's
Layer-1 sense: a mapping under which the family's typed content maps
clause by clause onto a classical structure's defining data, and the
laws among clauses become theorems on the target side --- a functor,
not a resemblance. Every naming carries the translation and its
citations (HoTT Book, Kobayashi--Nomizu, Steenrod, Reed--Simon,
Schreiber, or equivalent standard sources). \textbf{NO-NAME is a
first-class verdict}, rendered when three independent naming attempts
fail the standard, with the attempts recorded. Partial matches
(``resembles X but violates law Y'') are recorded as NO-NAME with the
obstruction named.

## Tasks

- **C-1 (extraction).** Per family: the certified typed content,
  pulled by digest from the sealed artifacts; one dossier per family,
  register-tagged.
- **C-2 (naming attempts).** Per family: candidate classical
  identifications with full translations per the standard; sourced,
  never asserted. Multiple names are permitted where mathematics has
  synonyms (one structure, several registry entries --- the
  univalence point).
- **C-3 (adversarial verification).** Every proposed translation
  checked clause-by-clause by verifier contexts distinct from the
  proposing analyst; disputes adjudicated; the XF-0 verification
  pattern reused.
- **C-4 (bidirectional join).** Join against the XF-0 blind column:
  (i) families \(\to\) names explains the undercounts exactly ---
  which certified families did the blind unit merge, and into what;
  (ii) blind class-N data \(\to\) families checks the inverse ---
  every blindly-counted independent structure should land on some
  certified family (a blind structure with no family is a finding
  against the register and is published as such).
- **C-5 (report).** The concordance table; the compression
  explanation per rich stratum (e.g., precisely which six named or
  unnamed structures the circle's single blind ``loop constructor''
  merged); and one of the two registered recommendations below.

## Registered outcomes

- **Outcome N (all named).** Every rich-stratum family has a lawful
  classical name. Then the v1 census unit was merely coarser than
  nomenclature, and a principled v2 unit exists: \emph{named classical
  structures at family granularity}. A v2 census may then be
  preregistered --- with disclosed weaker blindness --- and a v2
  systematic match would retroactively upgrade XF-0's diagnostic into
  first contact at the right granularity.
- **Outcome U (some unnamed).** One or more certified families fail
  every naming attempt. Then the register distinguishes structures
  mathematics has not named --- each unnamed family is a candidate
  novel mathematical object in the DCT pattern (specification first,
  existence obligations named), registered individually as potential
  exports. This outcome is \emph{more} interesting than N and must
  not be engineered against (F-C2).
- **Outcome X (inverse failure).** Some blind class-N structure maps
  to no certified family. A finding against the register's
  completeness at that stratum, published verbatim; the row is
  referred back to the internal program as a named question.

## Falsifiers

- **F-C1.** Any name assigned without a clause-preserving translation
  and citations → the row is void; resemblance is not naming.
- **F-C2.** Any NO-NAME suppressed, padded with a forced match, or
  any translation weakened to secure Outcome N → invalid; the
  standard does not bend toward tidiness.
- **F-C3.** Fix the name, never the family: no certified family may
  be reinterpreted, split, or merged to fit a classical target;
  mismatches between the audit's granularity and nomenclature are
  results, not defects.
- **F-C4.** Contamination honesty: this study's outputs are recorded
  as inputs to any future blind exercise's disclosure statement.
- **F-C5.** All 27 (or 32) rows report; a partial table is not a
  concordance.

## Deliverables

`docs/XF0B_CONCORDANCE_RESULT.md` + `docs/xf0b_concordance_v1.json`
(per-family dossiers, translations, verifier reports, verdicts,
bidirectional join), ledger event on freeze and on result. Mutation
falsifiers: flipping any verdict, translation clause, or citation
must invalidate replay.

## Relation to the campaign

XF-0b gates nothing: XF-A's derivation obligation and XF-B2's premise
closure proceed in parallel as the campaign front. Its payoffs: under
N, the path to an honest v2 census; under U, the program's next
mathematical exports; under X, an internal correction; under all
three, the exact explanation of the first pass's most informative
failure.
