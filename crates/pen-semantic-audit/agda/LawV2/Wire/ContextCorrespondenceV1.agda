{-# OPTIONS --safe --without-K #-}

-- Phase D bridge: the finite context/global correspondence between the
-- decoded wire surface and the intrinsically scoped production syntax.
--
-- This module proves that scoped wire terms, oldest-first wire contexts,
-- and the chronological global-slot table decode faithfully onto
-- `PTm`/`PCtx` and a strict-prior signature mirror, with:
--
-- - structural round trips in both directions;
-- - the exact variable-lookup equations used by the synthesis checker
--   (oldest-first ordinal `locals - suc index`, shift distance
--   `suc index` realized as iterated weakening);
-- - context-extension transport; and
-- - erasure commutation for weakening (de Bruijn shift on wire terms).
--
-- Nothing here mints correspondence authority: these are theorem
-- packages consumed later by the private factory after replay and
-- transcript agreement.

module LawV2.Wire.ContextCorrespondenceV1 where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe
  using ()
  renaming (Maybe to WireMaybe; just to wire-just; nothing to wire-nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc; _-_)
open import LawV2.Wire.Bytes using (_++_; length; nat-equal; _and_)
open import LawV2.Wire.ProductionBundleV1
open import LawV2.Wire.ContextChecker
open import LawV2.Wire.BundleChecker using (checker-term-scoped)
open import LawV2.LambdaUnit.ProductionSyntaxV1
open import LawV2.LambdaUnit.ProductionDecodingV1
open import LawV2.LambdaUnit.SubstitutionReduction using (single)

-- Boolean-conjunction projections with explicit sides.
and-first : (left right : Bool) → (left and right) ≡ true → left ≡ true
and-first true right proof = refl
and-first false right ()

and-second : (left right : Bool) → (left and right) ≡ true → right ≡ true
and-second true right proof = proof
and-second false right ()

and-intro :
  (left right : Bool) → left ≡ true → right ≡ true →
  (left and right) ≡ true
and-intro true true left-true right-true = refl
and-intro true false left-true ()
and-intro false right () right-true

-- List utilities local to the correspondence.
wire-++-empty :
  {A : Set} (values : List A) → values ++ [] ≡ values
wire-++-empty [] = refl
wire-++-empty (x ∷ values) rewrite wire-++-empty values = refl

wire-++-assoc :
  {A : Set} (first second third : List A) →
  (first ++ second) ++ third ≡ first ++ (second ++ third)
wire-++-assoc [] second third = refl
wire-++-assoc (x ∷ first) second third
  rewrite wire-++-assoc first second third = refl

wire-map-maybe :
  {A B : Set} → (A → B) → WireMaybe A → WireMaybe B
wire-map-maybe function wire-nothing = wire-nothing
wire-map-maybe function (wire-just value) = wire-just (function value)

-- Finite indices from Boolean bounds, and their ordinal equations.
fin-from-less :
  (ordinal count : Nat) → nat-less ordinal count ≡ true → Fin count
fin-from-less zero zero ()
fin-from-less (suc ordinal) zero ()
fin-from-less zero (suc count) proof = fzero
fin-from-less (suc ordinal) (suc count) proof =
  fsuc (fin-from-less ordinal count proof)

fin-from-less-ordinal :
  (ordinal count : Nat) (proof : nat-less ordinal count ≡ true) →
  fin-ordinal (fin-from-less ordinal count proof) ≡ ordinal
fin-from-less-ordinal zero zero ()
fin-from-less-ordinal (suc ordinal) zero ()
fin-from-less-ordinal zero (suc count) proof = refl
fin-from-less-ordinal (suc ordinal) (suc count) proof =
  cong suc (fin-from-less-ordinal ordinal count proof)

fin-from-less-irrelevant :
  (ordinal count : Nat)
  (first-proof second-proof : nat-less ordinal count ≡ true) →
  fin-from-less ordinal count first-proof ≡
  fin-from-less ordinal count second-proof
fin-from-less-irrelevant zero zero () second-proof
fin-from-less-irrelevant (suc ordinal) zero () second-proof
fin-from-less-irrelevant zero (suc count) first-proof second-proof = refl
fin-from-less-irrelevant (suc ordinal) (suc count)
  first-proof second-proof =
  cong fsuc
    (fin-from-less-irrelevant ordinal count first-proof second-proof)

fin-from-less-complete :
  {count : Nat} (x : Fin count)
  (proof : nat-less (fin-ordinal x) count ≡ true) →
  fin-from-less (fin-ordinal x) count proof ≡ x
fin-from-less-complete fzero proof = refl
fin-from-less-complete (fsuc x) proof =
  cong fsuc (fin-from-less-complete x proof)

fin-ordinal-less :
  {count : Nat} (x : Fin count) →
  nat-less (fin-ordinal x) count ≡ true
fin-ordinal-less fzero = refl
fin-ordinal-less (fsuc x) = fin-ordinal-less x

-- Minimal scope predicate: exactly the variable and global bounds needed
-- for intrinsic scoping. Universe bounds are checker concerns, so both
-- checker roles imply this predicate.
wire-in-scope : Nat → Nat → WireTermV1 → Bool
wire-in-scope globals locals (wire-sort level) = true
wire-in-scope globals locals (wire-variable index) = nat-less index locals
wire-in-scope globals locals (wire-global-slot slot) =
  nat-less slot globals
wire-in-scope globals locals (wire-pi parameter body) =
  wire-in-scope globals locals parameter and
  wire-in-scope globals (suc locals) body
wire-in-scope globals locals (wire-lambda parameter body) =
  wire-in-scope globals locals parameter and
  wire-in-scope globals (suc locals) body
wire-in-scope globals locals (wire-apply function argument) =
  wire-in-scope globals locals function and
  wire-in-scope globals locals argument
wire-in-scope globals locals wire-unit-type = true
wire-in-scope globals locals wire-unit = true

term-scoped-in-scope :
  (globals locals : Nat) (term : WireTermV1) →
  term-scoped globals locals term ≡ true →
  wire-in-scope globals locals term ≡ true
term-scoped-in-scope globals locals (wire-sort level) proof = refl
term-scoped-in-scope globals locals (wire-variable index) proof = proof
term-scoped-in-scope globals locals (wire-global-slot slot) proof = proof
term-scoped-in-scope globals locals (wire-pi parameter body) proof =
  and-intro
    (wire-in-scope globals locals parameter)
    (wire-in-scope globals (suc locals) body)
    (term-scoped-in-scope globals locals parameter
      (and-first (term-scoped globals locals parameter)
        (term-scoped globals (suc locals) body) proof))
    (term-scoped-in-scope globals (suc locals) body
      (and-second (term-scoped globals locals parameter)
        (term-scoped globals (suc locals) body) proof))
term-scoped-in-scope globals locals (wire-lambda parameter body) proof =
  and-intro
    (wire-in-scope globals locals parameter)
    (wire-in-scope globals (suc locals) body)
    (term-scoped-in-scope globals locals parameter
      (and-first (term-scoped globals locals parameter)
        (term-scoped globals (suc locals) body) proof))
    (term-scoped-in-scope globals (suc locals) body
      (and-second (term-scoped globals locals parameter)
        (term-scoped globals (suc locals) body) proof))
term-scoped-in-scope globals locals (wire-apply function argument) proof =
  and-intro
    (wire-in-scope globals locals function)
    (wire-in-scope globals locals argument)
    (term-scoped-in-scope globals locals function
      (and-first (term-scoped globals locals function)
        (term-scoped globals locals argument) proof))
    (term-scoped-in-scope globals locals argument
      (and-second (term-scoped globals locals function)
        (term-scoped globals locals argument) proof))
term-scoped-in-scope globals locals wire-unit-type proof = refl
term-scoped-in-scope globals locals wire-unit proof = refl

checker-term-scoped-in-scope :
  (globals locals : Nat) (term : WireTermV1) →
  checker-term-scoped globals locals term ≡ true →
  wire-in-scope globals locals term ≡ true
checker-term-scoped-in-scope globals locals (wire-sort level) proof = refl
checker-term-scoped-in-scope globals locals (wire-variable index) proof =
  proof
checker-term-scoped-in-scope globals locals (wire-global-slot slot)
  proof = proof
checker-term-scoped-in-scope globals locals (wire-pi parameter body)
  proof =
  and-intro
    (wire-in-scope globals locals parameter)
    (wire-in-scope globals (suc locals) body)
    (checker-term-scoped-in-scope globals locals parameter
      (and-first (checker-term-scoped globals locals parameter)
        (checker-term-scoped globals (suc locals) body) proof))
    (checker-term-scoped-in-scope globals (suc locals) body
      (and-second (checker-term-scoped globals locals parameter)
        (checker-term-scoped globals (suc locals) body) proof))
checker-term-scoped-in-scope globals locals (wire-lambda parameter body)
  proof =
  and-intro
    (wire-in-scope globals locals parameter)
    (wire-in-scope globals (suc locals) body)
    (checker-term-scoped-in-scope globals locals parameter
      (and-first (checker-term-scoped globals locals parameter)
        (checker-term-scoped globals (suc locals) body) proof))
    (checker-term-scoped-in-scope globals (suc locals) body
      (and-second (checker-term-scoped globals locals parameter)
        (checker-term-scoped globals (suc locals) body) proof))
checker-term-scoped-in-scope globals locals
  (wire-apply function argument) proof =
  and-intro
    (wire-in-scope globals locals function)
    (wire-in-scope globals locals argument)
    (checker-term-scoped-in-scope globals locals function
      (and-first (checker-term-scoped globals locals function)
        (checker-term-scoped globals locals argument) proof))
    (checker-term-scoped-in-scope globals locals argument
      (and-second (checker-term-scoped globals locals function)
        (checker-term-scoped globals locals argument) proof))
checker-term-scoped-in-scope globals locals wire-unit-type proof = refl
checker-term-scoped-in-scope globals locals wire-unit proof = refl

-- Scoped wire terms decode totally onto intrinsically scoped terms.
wire-to-ptm :
  (globals locals : Nat) (term : WireTermV1) →
  wire-in-scope globals locals term ≡ true →
  PTm globals locals
wire-to-ptm globals locals (wire-sort level) proof = psort level
wire-to-ptm globals locals (wire-variable index) proof =
  pvar (fin-from-less index locals proof)
wire-to-ptm globals locals (wire-global-slot slot) proof =
  pglobal (fin-from-less slot globals proof)
wire-to-ptm globals locals (wire-pi parameter body) proof =
  ppi
    (wire-to-ptm globals locals parameter
      (and-first (wire-in-scope globals locals parameter)
        (wire-in-scope globals (suc locals) body) proof))
    (wire-to-ptm globals (suc locals) body
      (and-second (wire-in-scope globals locals parameter)
        (wire-in-scope globals (suc locals) body) proof))
wire-to-ptm globals locals (wire-lambda parameter body) proof =
  plam
    (wire-to-ptm globals locals parameter
      (and-first (wire-in-scope globals locals parameter)
        (wire-in-scope globals (suc locals) body) proof))
    (wire-to-ptm globals (suc locals) body
      (and-second (wire-in-scope globals locals parameter)
        (wire-in-scope globals (suc locals) body) proof))
wire-to-ptm globals locals (wire-apply function argument) proof =
  papp
    (wire-to-ptm globals locals function
      (and-first (wire-in-scope globals locals function)
        (wire-in-scope globals locals argument) proof))
    (wire-to-ptm globals locals argument
      (and-second (wire-in-scope globals locals function)
        (wire-in-scope globals locals argument) proof))
wire-to-ptm globals locals wire-unit-type proof = punit-type
wire-to-ptm globals locals wire-unit proof = punit

-- Erasure back to authority-free wire terms.
ptm-to-wire : {globals locals : Nat} → PTm globals locals → WireTermV1
ptm-to-wire (pvar x) = wire-variable (fin-ordinal x)
ptm-to-wire (psort level) = wire-sort level
ptm-to-wire (pglobal slot) = wire-global-slot (fin-ordinal slot)
ptm-to-wire (ppi parameter body) =
  wire-pi (ptm-to-wire parameter) (ptm-to-wire body)
ptm-to-wire (plam parameter body) =
  wire-lambda (ptm-to-wire parameter) (ptm-to-wire body)
ptm-to-wire (papp function argument) =
  wire-apply (ptm-to-wire function) (ptm-to-wire argument)
ptm-to-wire punit-type = wire-unit-type
ptm-to-wire punit = wire-unit

wire-to-ptm-irrelevant :
  (globals locals : Nat) (term : WireTermV1)
  (first-proof second-proof : wire-in-scope globals locals term ≡ true) →
  wire-to-ptm globals locals term first-proof ≡
  wire-to-ptm globals locals term second-proof
wire-to-ptm-irrelevant globals locals (wire-sort level)
  first-proof second-proof = refl
wire-to-ptm-irrelevant globals locals (wire-variable index)
  first-proof second-proof =
  cong pvar (fin-from-less-irrelevant index locals first-proof second-proof)
wire-to-ptm-irrelevant globals locals (wire-global-slot slot)
  first-proof second-proof =
  cong pglobal
    (fin-from-less-irrelevant slot globals first-proof second-proof)
wire-to-ptm-irrelevant globals locals (wire-pi parameter body)
  first-proof second-proof =
  cong₂ ppi
    (wire-to-ptm-irrelevant globals locals parameter _ _)
    (wire-to-ptm-irrelevant globals (suc locals) body _ _)
wire-to-ptm-irrelevant globals locals (wire-lambda parameter body)
  first-proof second-proof =
  cong₂ plam
    (wire-to-ptm-irrelevant globals locals parameter _ _)
    (wire-to-ptm-irrelevant globals (suc locals) body _ _)
wire-to-ptm-irrelevant globals locals (wire-apply function argument)
  first-proof second-proof =
  cong₂ papp
    (wire-to-ptm-irrelevant globals locals function _ _)
    (wire-to-ptm-irrelevant globals locals argument _ _)
wire-to-ptm-irrelevant globals locals wire-unit-type
  first-proof second-proof = refl
wire-to-ptm-irrelevant globals locals wire-unit
  first-proof second-proof = refl

-- Structural round trip, direction one: decoding then erasing restores
-- the exact wire term.
wire-round-trip :
  (globals locals : Nat) (term : WireTermV1)
  (proof : wire-in-scope globals locals term ≡ true) →
  ptm-to-wire (wire-to-ptm globals locals term proof) ≡ term
wire-round-trip globals locals (wire-sort level) proof = refl
wire-round-trip globals locals (wire-variable index) proof =
  cong wire-variable (fin-from-less-ordinal index locals proof)
wire-round-trip globals locals (wire-global-slot slot) proof =
  cong wire-global-slot (fin-from-less-ordinal slot globals proof)
wire-round-trip globals locals (wire-pi parameter body) proof =
  cong₂ wire-pi
    (wire-round-trip globals locals parameter _)
    (wire-round-trip globals (suc locals) body _)
wire-round-trip globals locals (wire-lambda parameter body) proof =
  cong₂ wire-lambda
    (wire-round-trip globals locals parameter _)
    (wire-round-trip globals (suc locals) body _)
wire-round-trip globals locals (wire-apply function argument) proof =
  cong₂ wire-apply
    (wire-round-trip globals locals function _)
    (wire-round-trip globals locals argument _)
wire-round-trip globals locals wire-unit-type proof = refl
wire-round-trip globals locals wire-unit proof = refl

-- Erasure always lands inside the scope predicate.
ptm-to-wire-in-scope :
  {globals locals : Nat} (term : PTm globals locals) →
  wire-in-scope globals locals (ptm-to-wire term) ≡ true
ptm-to-wire-in-scope (pvar x) = fin-ordinal-less x
ptm-to-wire-in-scope (psort level) = refl
ptm-to-wire-in-scope (pglobal slot) = fin-ordinal-less slot
ptm-to-wire-in-scope {globals} {locals} (ppi parameter body) =
  and-intro
    (wire-in-scope globals locals (ptm-to-wire parameter))
    (wire-in-scope globals (suc locals) (ptm-to-wire body))
    (ptm-to-wire-in-scope parameter)
    (ptm-to-wire-in-scope body)
ptm-to-wire-in-scope {globals} {locals} (plam parameter body) =
  and-intro
    (wire-in-scope globals locals (ptm-to-wire parameter))
    (wire-in-scope globals (suc locals) (ptm-to-wire body))
    (ptm-to-wire-in-scope parameter)
    (ptm-to-wire-in-scope body)
ptm-to-wire-in-scope {globals} {locals} (papp function argument) =
  and-intro
    (wire-in-scope globals locals (ptm-to-wire function))
    (wire-in-scope globals locals (ptm-to-wire argument))
    (ptm-to-wire-in-scope function)
    (ptm-to-wire-in-scope argument)
ptm-to-wire-in-scope punit-type = refl
ptm-to-wire-in-scope punit = refl

-- Structural round trip, direction two: erasing then decoding restores
-- the exact intrinsic term, for every scope proof.
ptm-round-trip :
  {globals locals : Nat} (term : PTm globals locals)
  (proof : wire-in-scope globals locals (ptm-to-wire term) ≡ true) →
  wire-to-ptm globals locals (ptm-to-wire term) proof ≡ term
ptm-round-trip (pvar x) proof = cong pvar (fin-from-less-complete x proof)
ptm-round-trip (psort level) proof = refl
ptm-round-trip (pglobal slot) proof =
  cong pglobal (fin-from-less-complete slot proof)
ptm-round-trip (ppi parameter body) proof =
  cong₂ ppi (ptm-round-trip parameter _) (ptm-round-trip body _)
ptm-round-trip (plam parameter body) proof =
  cong₂ plam (ptm-round-trip parameter _) (ptm-round-trip body _)
ptm-round-trip (papp function argument) proof =
  cong₂ papp (ptm-round-trip function _) (ptm-round-trip argument _)
ptm-round-trip punit-type proof = refl
ptm-round-trip punit proof = refl

-- Oldest-first context building onto `PCtx`.
total-from : {A : Set} → Nat → List A → Nat
total-from count [] = count
total-from count (value ∷ values) = total-from (suc count) values

total-from-suc :
  {A : Set} (count : Nat) (values : List A) →
  total-from (suc count) values ≡ suc (total-from count values)
total-from-suc count [] = refl
total-from-suc count (value ∷ values) = total-from-suc (suc count) values

total-from-length :
  {A : Set} (values : List A) →
  total-from zero values ≡ length values
total-from-length [] = refl
total-from-length (value ∷ values) =
  trans (total-from-suc zero values) (cong suc (total-from-length values))

wire-context-extend :
  {globals : Nat} (locals : Nat) → PCtx globals locals →
  (entries : List WireTermV1) →
  check-context-entries globals locals entries ≡ true →
  PCtx globals (total-from locals entries)
wire-context-extend locals context [] proof = context
wire-context-extend {globals} locals context (entry ∷ entries) proof =
  wire-context-extend (suc locals)
    (context psnoc
      (wire-to-ptm globals locals entry
        (term-scoped-in-scope globals locals entry
          (and-first (term-scoped globals locals entry)
            (check-context-entries globals (suc locals) entries) proof))))
    entries
    (and-second (term-scoped globals locals entry)
      (check-context-entries globals (suc locals) entries) proof)

wire-context-to-pctx :
  (globals : Nat) (entries : List WireTermV1) →
  check-context-entries globals zero entries ≡ true →
  PCtx globals (total-from zero entries)
wire-context-to-pctx globals entries proof =
  wire-context-extend zero pempty entries proof

pctx-to-wire :
  {globals locals : Nat} → PCtx globals locals → List WireTermV1
pctx-to-wire pempty = []
pctx-to-wire (context psnoc parameter) =
  pctx-to-wire context ++ (ptm-to-wire parameter ∷ [])

pctx-to-wire-length :
  {globals locals : Nat} (context : PCtx globals locals) →
  length (pctx-to-wire context) ≡ locals
pctx-to-wire-length pempty = refl
pctx-to-wire-length (context psnoc parameter) =
  trans
    (length-snoc (pctx-to-wire context) (ptm-to-wire parameter))
    (cong suc (pctx-to-wire-length context))
  where
  length-snoc :
    {A : Set} (values : List A) (value : A) →
    length (values ++ (value ∷ [])) ≡ suc (length values)
  length-snoc [] value = refl
  length-snoc (x ∷ values) value = cong suc (length-snoc values value)

wire-context-round-trip :
  {globals : Nat} (locals : Nat) (context : PCtx globals locals)
  (entries : List WireTermV1)
  (proof : check-context-entries globals locals entries ≡ true) →
  pctx-to-wire (wire-context-extend locals context entries proof)
  ≡ pctx-to-wire context ++ entries
wire-context-round-trip locals context [] proof =
  sym (wire-++-empty (pctx-to-wire context))
wire-context-round-trip {globals} locals context (entry ∷ entries) proof =
  trans
    (wire-context-round-trip (suc locals)
      (context psnoc
        (wire-to-ptm globals locals entry
          (term-scoped-in-scope globals locals entry
            (and-first (term-scoped globals locals entry)
              (check-context-entries globals (suc locals) entries)
              proof))))
      entries
      (and-second (term-scoped globals locals entry)
        (check-context-entries globals (suc locals) entries) proof))
    (trans
      (wire-++-assoc (pctx-to-wire context)
        (ptm-to-wire
          (wire-to-ptm globals locals entry
            (term-scoped-in-scope globals locals entry
              (and-first (term-scoped globals locals entry)
                (check-context-entries globals (suc locals) entries)
                proof)))
          ∷ [])
        entries)
      (cong (λ head → pctx-to-wire context ++ (head ∷ entries))
        (wire-round-trip globals locals entry _)))

wire-context-to-pctx-round-trip :
  (globals : Nat) (entries : List WireTermV1)
  (proof : check-context-entries globals zero entries ≡ true) →
  pctx-to-wire (wire-context-to-pctx globals entries proof) ≡ entries
wire-context-to-pctx-round-trip globals entries proof =
  wire-context-round-trip zero pempty entries proof

-- Building is independent of the particular scoping proof.
wire-context-extend-irrelevant :
  {globals : Nat} (locals : Nat) (context : PCtx globals locals)
  (entries : List WireTermV1)
  (first-proof second-proof :
    check-context-entries globals locals entries ≡ true) →
  wire-context-extend locals context entries first-proof
  ≡ wire-context-extend locals context entries second-proof
wire-context-extend-irrelevant locals context []
  first-proof second-proof = refl
wire-context-extend-irrelevant {globals} locals context (entry ∷ entries)
  first-proof second-proof
  rewrite wire-to-ptm-irrelevant globals locals entry
    (term-scoped-in-scope globals locals entry
      (and-first (term-scoped globals locals entry)
        (check-context-entries globals (suc locals) entries)
        first-proof))
    (term-scoped-in-scope globals locals entry
      (and-first (term-scoped globals locals entry)
        (check-context-entries globals (suc locals) entries)
        second-proof))
  = wire-context-extend-irrelevant (suc locals) _ entries _ _

-- Context extension transport: building one appended entry equals a
-- `psnoc` of the built context.
cast-pctx :
  {globals from to : Nat} → from ≡ to →
  PCtx globals from → PCtx globals to
cast-pctx refl context = context

total-from-snoc :
  {A : Set} (count : Nat) (values : List A) (value : A) →
  total-from count (values ++ (value ∷ [])) ≡ suc (total-from count values)
total-from-snoc count [] value = refl
total-from-snoc count (x ∷ values) value =
  total-from-snoc (suc count) values value

wire-context-extension-correspondence :
  {globals : Nat} (locals : Nat) (context : PCtx globals locals)
  (entries : List WireTermV1) (entry : WireTermV1)
  (whole-proof :
    check-context-entries globals locals (entries ++ (entry ∷ []))
    ≡ true)
  (prefix-proof : check-context-entries globals locals entries ≡ true)
  (entry-proof :
    wire-in-scope globals (total-from locals entries) entry ≡ true) →
  cast-pctx (total-from-snoc locals entries entry)
    (wire-context-extend locals context (entries ++ (entry ∷ []))
      whole-proof)
  ≡
  (wire-context-extend locals context entries prefix-proof) psnoc
  (wire-to-ptm globals (total-from locals entries) entry entry-proof)
wire-context-extension-correspondence {globals} locals context []
  entry whole-proof prefix-proof entry-proof =
  cong (λ term → context psnoc term)
    (wire-to-ptm-irrelevant globals locals entry _ entry-proof)
wire-context-extension-correspondence {globals} locals context
  (first ∷ entries) entry whole-proof prefix-proof entry-proof
  rewrite wire-to-ptm-irrelevant globals locals first
    (term-scoped-in-scope globals locals first
      (and-first (term-scoped globals locals first)
        (check-context-entries globals (suc locals)
          (entries ++ (entry ∷ [])))
        whole-proof))
    (term-scoped-in-scope globals locals first
      (and-first (term-scoped globals locals first)
        (check-context-entries globals (suc locals) entries)
        prefix-proof))
  = wire-context-extension-correspondence (suc locals) _ entries entry
      (and-second (term-scoped globals locals first)
        (check-context-entries globals (suc locals)
          (entries ++ (entry ∷ [])))
        whole-proof)
      (and-second (term-scoped globals locals first)
        (check-context-entries globals (suc locals) entries)
        prefix-proof)
      entry-proof

-- Oldest-first list indexing.
list-entry : {A : Set} → List A → Nat → WireMaybe A
list-entry [] index = wire-nothing
list-entry (value ∷ values) zero = wire-just value
list-entry (value ∷ values) (suc index) = list-entry values index

list-entry-at-length :
  {A : Set} (values : List A) (value : A) (index : Nat) →
  length values ≡ index →
  list-entry (values ++ (value ∷ [])) index ≡ wire-just value
list-entry-at-length values value index refl = at-end values value
  where
  at-end :
    {A : Set} (values : List A) (value : A) →
    list-entry (values ++ (value ∷ [])) (length values) ≡ wire-just value
  at-end [] value = refl
  at-end (x ∷ values) value = at-end values value

list-entry-prefix-at :
  {A : Set} (values suffix : List A) (index boundary : Nat) →
  length values ≡ boundary →
  nat-less index boundary ≡ true →
  list-entry (values ++ suffix) index ≡ list-entry values index
list-entry-prefix-at values suffix index boundary refl below =
  in-prefix values suffix index below
  where
  in-prefix :
    {A : Set} (values suffix : List A) (index : Nat) →
    nat-less index (length values) ≡ true →
    list-entry (values ++ suffix) index ≡ list-entry values index
  in-prefix [] suffix zero ()
  in-prefix [] suffix (suc index) ()
  in-prefix (x ∷ values) suffix zero below = refl
  in-prefix (x ∷ values) suffix (suc index) below =
    in-prefix values suffix index below

-- Arithmetic support for the oldest-first position.
nat-less-suc-self : (count : Nat) → nat-less count (suc count) ≡ true
nat-less-suc-self zero = refl
nat-less-suc-self (suc count) = nat-less-suc-self count

nat-less-widen :
  (left right : Nat) → nat-less left right ≡ true →
  nat-less left (suc right) ≡ true
nat-less-widen zero zero ()
nat-less-widen zero (suc right) proof = refl
nat-less-widen (suc left) zero ()
nat-less-widen (suc left) (suc right) proof =
  nat-less-widen left right proof

monus-less :
  (index count : Nat) → nat-less index count ≡ true →
  nat-less (count - suc index) count ≡ true
monus-less zero zero ()
monus-less zero (suc count) proof = nat-less-suc-self count
monus-less (suc index) zero ()
monus-less (suc index) (suc count) proof =
  nat-less-widen (count - suc index) count (monus-less index count proof)

-- De Bruijn shift on wire terms: the erasure image of intrinsic
-- weakening.
shift-branch : Bool → Nat → Nat
shift-branch true index = index
shift-branch false index = suc index

shift-index : Nat → Nat → Nat
shift-index cutoff index = shift-branch (nat-less index cutoff) index

wire-shift : Nat → WireTermV1 → WireTermV1
wire-shift cutoff (wire-sort level) = wire-sort level
wire-shift cutoff (wire-variable index) =
  wire-variable (shift-index cutoff index)
wire-shift cutoff (wire-global-slot slot) = wire-global-slot slot
wire-shift cutoff (wire-pi parameter body) =
  wire-pi (wire-shift cutoff parameter) (wire-shift (suc cutoff) body)
wire-shift cutoff (wire-lambda parameter body) =
  wire-lambda (wire-shift cutoff parameter) (wire-shift (suc cutoff) body)
wire-shift cutoff (wire-apply function argument) =
  wire-apply (wire-shift cutoff function) (wire-shift cutoff argument)
wire-shift cutoff wire-unit-type = wire-unit-type
wire-shift cutoff wire-unit = wire-unit

shift-branch-suc :
  (branch : Bool) (index : Nat) →
  shift-branch branch (suc index) ≡ suc (shift-branch branch index)
shift-branch-suc true index = refl
shift-branch-suc false index = refl

shift-index-suc :
  (cutoff index : Nat) →
  shift-index (suc cutoff) (suc index) ≡ suc (shift-index cutoff index)
shift-index-suc cutoff index =
  shift-branch-suc (nat-less index cutoff) index

nat-less-zero : (index : Nat) → nat-less index zero ≡ false
nat-less-zero zero = refl
nat-less-zero (suc index) = refl

lift-pren-spec :
  {source target : Nat} (cutoff : Nat) (rho : PRen source target) →
  ((x : Fin source) →
    fin-ordinal (rho x) ≡ shift-index cutoff (fin-ordinal x)) →
  (x : Fin (suc source)) →
  fin-ordinal (lift-pren rho x) ≡ shift-index (suc cutoff) (fin-ordinal x)
lift-pren-spec cutoff rho spec fzero = refl
lift-pren-spec cutoff rho spec (fsuc x) =
  trans
    (cong suc (spec x))
    (sym (shift-index-suc cutoff (fin-ordinal x)))

erasure-prename-shift :
  {globals source target : Nat} (cutoff : Nat) (rho : PRen source target)
  (spec : (x : Fin source) →
    fin-ordinal (rho x) ≡ shift-index cutoff (fin-ordinal x))
  (term : PTm globals source) →
  ptm-to-wire (prename rho term) ≡ wire-shift cutoff (ptm-to-wire term)
erasure-prename-shift cutoff rho spec (pvar x) =
  cong wire-variable (spec x)
erasure-prename-shift cutoff rho spec (psort level) = refl
erasure-prename-shift cutoff rho spec (pglobal slot) = refl
erasure-prename-shift cutoff rho spec (ppi parameter body) =
  cong₂ wire-pi
    (erasure-prename-shift cutoff rho spec parameter)
    (erasure-prename-shift (suc cutoff) (lift-pren rho)
      (lift-pren-spec cutoff rho spec) body)
erasure-prename-shift cutoff rho spec (plam parameter body) =
  cong₂ wire-lambda
    (erasure-prename-shift cutoff rho spec parameter)
    (erasure-prename-shift (suc cutoff) (lift-pren rho)
      (lift-pren-spec cutoff rho spec) body)
erasure-prename-shift cutoff rho spec (papp function argument) =
  cong₂ wire-apply
    (erasure-prename-shift cutoff rho spec function)
    (erasure-prename-shift cutoff rho spec argument)
erasure-prename-shift cutoff rho spec punit-type = refl
erasure-prename-shift cutoff rho spec punit = refl

fsuc-shift-spec :
  {count : Nat} (x : Fin count) →
  fin-ordinal (fsuc x) ≡ shift-index zero (fin-ordinal x)
fsuc-shift-spec x =
  sym
    (cong (λ branch → shift-branch branch (fin-ordinal x))
      (nat-less-zero (fin-ordinal x)))

-- The shift correspondence: intrinsic weakening erases to the exact
-- de Bruijn shift the Rust side computes.
erasure-pweaken :
  {globals locals : Nat} (term : PTm globals locals) →
  ptm-to-wire (pweaken term) ≡ wire-shift zero (ptm-to-wire term)
erasure-pweaken term =
  erasure-prename-shift zero fsuc (λ x → fsuc-shift-spec x) term

shift-iter : Nat → WireTermV1 → WireTermV1
shift-iter zero term = term
shift-iter (suc count) term = wire-shift zero (shift-iter count term)

wire-map-maybe-compose :
  {A B C : Set} (outer : B → C) (inner : A → B) (value : WireMaybe A) →
  wire-map-maybe (λ item → outer (inner item)) value
  ≡ wire-map-maybe outer (wire-map-maybe inner value)
wire-map-maybe-compose outer inner wire-nothing = refl
wire-map-maybe-compose outer inner (wire-just value) = refl

-- Variable-lookup correspondence: the intrinsic in-context type of a
-- variable is exactly the raw wire entry at oldest-first position
-- `locals - suc index`, shifted `suc index` times. This is the intrinsic
-- content of the synthesis checker's `context_ordinal` and
-- `shift_distance` equations.
variable-lookup-correspondence :
  {globals locals : Nat} (context : PCtx globals locals)
  (x : Fin locals) →
  wire-map-maybe (shift-iter (suc (fin-ordinal x)))
    (list-entry (pctx-to-wire context) (locals - suc (fin-ordinal x)))
  ≡ wire-just (ptm-to-wire (lookup-pctx context x))
variable-lookup-correspondence pempty ()
variable-lookup-correspondence {globals} {suc locals}
  (context psnoc parameter) fzero
  rewrite list-entry-at-length (pctx-to-wire context)
    (ptm-to-wire parameter) locals (pctx-to-wire-length context)
  = cong wire-just (sym (erasure-pweaken parameter))
variable-lookup-correspondence {globals} {suc locals}
  (context psnoc parameter) (fsuc x)
  rewrite list-entry-prefix-at (pctx-to-wire context)
    (ptm-to-wire parameter ∷ [])
    (locals - suc (fin-ordinal x)) locals
    (pctx-to-wire-length context)
    (monus-less (fin-ordinal x) locals (fin-ordinal-less x))
  = trans
      (wire-map-maybe-compose (wire-shift zero)
        (shift-iter (suc (fin-ordinal x)))
        (list-entry (pctx-to-wire context)
          (locals - suc (fin-ordinal x))))
      (trans
        (cong (wire-map-maybe (wire-shift zero))
          (variable-lookup-correspondence context x))
        (cong wire-just
          (sym (erasure-pweaken (lookup-pctx context x)))))

-- Chronological global signatures: slot `k` is declared under exactly
-- the `k` strictly prior globals, mirroring `check-slots-from`.
data PSig : Nat → Set where
  psig-empty : PSig zero
  psig-snoc :
    {count : Nat} → PSig count → PTm count zero →
    WireMaybe (PTm count zero) → PSig (suc count)

decode-global-body :
  (globals : Nat) (body : WireMaybe WireTermV1) →
  body-scoped globals body ≡ true →
  WireMaybe (PTm globals zero)
decode-global-body globals wire-nothing proof = wire-nothing
decode-global-body globals (wire-just body) proof =
  wire-just
    (wire-to-ptm globals zero body
      (term-scoped-in-scope globals zero body proof))

wire-sig-extend :
  (count : Nat) → PSig count →
  (entries : List GlobalSlotEntryWireV1) →
  check-slots-from count entries ≡ true →
  PSig (total-from count entries)
wire-sig-extend count sig [] proof = sig
wire-sig-extend count sig (entry ∷ entries) proof =
  wire-sig-extend (suc count)
    (psig-snoc sig
      (wire-to-ptm count zero (declaration-type entry)
        (term-scoped-in-scope count zero (declaration-type entry)
          (and-first
            (term-scoped count zero (declaration-type entry))
            (body-scoped count (declaration-body entry) and
             check-slots-from (suc count) entries)
            (and-second (id-is-32-bytes (global-id entry))
              (term-scoped count zero (declaration-type entry) and
               (body-scoped count (declaration-body entry) and
                check-slots-from (suc count) entries))
              (and-second (nat-equal (global-slot entry) count)
                (id-is-32-bytes (global-id entry) and
                 (term-scoped count zero (declaration-type entry) and
                  (body-scoped count (declaration-body entry) and
                   check-slots-from (suc count) entries)))
                proof)))))
      (decode-global-body count (declaration-body entry)
        (and-first
          (body-scoped count (declaration-body entry))
          (check-slots-from (suc count) entries)
          (and-second
            (term-scoped count zero (declaration-type entry))
            (body-scoped count (declaration-body entry) and
             check-slots-from (suc count) entries)
            (and-second (id-is-32-bytes (global-id entry))
              (term-scoped count zero (declaration-type entry) and
               (body-scoped count (declaration-body entry) and
                check-slots-from (suc count) entries))
              (and-second (nat-equal (global-slot entry) count)
                (id-is-32-bytes (global-id entry) and
                 (term-scoped count zero (declaration-type entry) and
                  (body-scoped count (declaration-body entry) and
                   check-slots-from (suc count) entries)))
                proof))))))
    entries
    (and-second
      (body-scoped count (declaration-body entry))
      (check-slots-from (suc count) entries)
      (and-second
        (term-scoped count zero (declaration-type entry))
        (body-scoped count (declaration-body entry) and
         check-slots-from (suc count) entries)
        (and-second (id-is-32-bytes (global-id entry))
          (term-scoped count zero (declaration-type entry) and
           (body-scoped count (declaration-body entry) and
            check-slots-from (suc count) entries))
          (and-second (nat-equal (global-slot entry) count)
            (id-is-32-bytes (global-id entry) and
             (term-scoped count zero (declaration-type entry) and
              (body-scoped count (declaration-body entry) and
               check-slots-from (suc count) entries)))
            proof))))

wire-slots-to-psig :
  (entries : List GlobalSlotEntryWireV1) →
  check-slots-from zero entries ≡ true →
  PSig (total-from zero entries)
wire-slots-to-psig entries proof =
  wire-sig-extend zero psig-empty entries proof

psig-to-wire-types : {count : Nat} → PSig count → List WireTermV1
psig-to-wire-types psig-empty = []
psig-to-wire-types (psig-snoc sig ty body) =
  psig-to-wire-types sig ++ (ptm-to-wire ty ∷ [])

psig-to-wire-bodies :
  {count : Nat} → PSig count → List (WireMaybe WireTermV1)
psig-to-wire-bodies psig-empty = []
psig-to-wire-bodies (psig-snoc sig ty body) =
  psig-to-wire-bodies sig ++ (wire-map-maybe ptm-to-wire body ∷ [])

psig-types-length :
  {count : Nat} (sig : PSig count) →
  length (psig-to-wire-types sig) ≡ count
psig-types-length psig-empty = refl
psig-types-length (psig-snoc sig ty body) =
  trans
    (length-snoc-general (psig-to-wire-types sig) (ptm-to-wire ty))
    (cong suc (psig-types-length sig))
  where
  length-snoc-general :
    {A : Set} (values : List A) (value : A) →
    length (values ++ (value ∷ [])) ≡ suc (length values)
  length-snoc-general [] value = refl
  length-snoc-general (x ∷ values) value =
    cong suc (length-snoc-general values value)

psig-bodies-length :
  {count : Nat} (sig : PSig count) →
  length (psig-to-wire-bodies sig) ≡ count
psig-bodies-length psig-empty = refl
psig-bodies-length (psig-snoc sig ty body) =
  trans
    (length-snoc-general (psig-to-wire-bodies sig)
      (wire-map-maybe ptm-to-wire body))
    (cong suc (psig-bodies-length sig))
  where
  length-snoc-general :
    {A : Set} (values : List A) (value : A) →
    length (values ++ (value ∷ [])) ≡ suc (length values)
  length-snoc-general [] value = refl
  length-snoc-general (x ∷ values) value =
    cong suc (length-snoc-general values value)

map-declaration-types : List GlobalSlotEntryWireV1 → List WireTermV1
map-declaration-types [] = []
map-declaration-types (entry ∷ entries) =
  declaration-type entry ∷ map-declaration-types entries

map-declaration-bodies :
  List GlobalSlotEntryWireV1 → List (WireMaybe WireTermV1)
map-declaration-bodies [] = []
map-declaration-bodies (entry ∷ entries) =
  declaration-body entry ∷ map-declaration-bodies entries

decode-global-body-round-trip :
  (globals : Nat) (body : WireMaybe WireTermV1)
  (proof : body-scoped globals body ≡ true) →
  wire-map-maybe ptm-to-wire (decode-global-body globals body proof)
  ≡ body
decode-global-body-round-trip globals wire-nothing proof = refl
decode-global-body-round-trip globals (wire-just body) proof =
  cong wire-just (wire-round-trip globals zero body _)

wire-sig-types-round-trip :
  (count : Nat) (sig : PSig count)
  (entries : List GlobalSlotEntryWireV1)
  (proof : check-slots-from count entries ≡ true) →
  psig-to-wire-types (wire-sig-extend count sig entries proof)
  ≡ psig-to-wire-types sig ++ map-declaration-types entries
wire-sig-types-round-trip count sig [] proof =
  sym (wire-++-empty (psig-to-wire-types sig))
wire-sig-types-round-trip count sig (entry ∷ entries) proof =
  trans
    (wire-sig-types-round-trip (suc count) _ entries _)
    (trans
      (wire-++-assoc (psig-to-wire-types sig) _
        (map-declaration-types entries))
      (cong
        (λ head →
          psig-to-wire-types sig ++
          (head ∷ map-declaration-types entries))
        (wire-round-trip count zero (declaration-type entry) _)))

wire-sig-bodies-round-trip :
  (count : Nat) (sig : PSig count)
  (entries : List GlobalSlotEntryWireV1)
  (proof : check-slots-from count entries ≡ true) →
  psig-to-wire-bodies (wire-sig-extend count sig entries proof)
  ≡ psig-to-wire-bodies sig ++ map-declaration-bodies entries
wire-sig-bodies-round-trip count sig [] proof =
  sym (wire-++-empty (psig-to-wire-bodies sig))
wire-sig-bodies-round-trip count sig (entry ∷ entries) proof =
  trans
    (wire-sig-bodies-round-trip (suc count) _ entries _)
    (trans
      (wire-++-assoc (psig-to-wire-bodies sig) _
        (map-declaration-bodies entries))
      (cong
        (λ head →
          psig-to-wire-bodies sig ++
          (head ∷ map-declaration-bodies entries))
        (decode-global-body-round-trip count (declaration-body entry) _)))

-- Global weakening: adding a newer global preserves every slot ordinal,
-- so it erases to the identity on wire terms.
fin-inject : {count : Nat} → Fin count → Fin (suc count)
fin-inject fzero = fzero
fin-inject (fsuc x) = fsuc (fin-inject x)

fin-inject-ordinal :
  {count : Nat} (x : Fin count) →
  fin-ordinal (fin-inject x) ≡ fin-ordinal x
fin-inject-ordinal fzero = refl
fin-inject-ordinal (fsuc x) = cong suc (fin-inject-ordinal x)

gweaken :
  {globals locals : Nat} → PTm globals locals → PTm (suc globals) locals
gweaken (pvar x) = pvar x
gweaken (psort level) = psort level
gweaken (pglobal slot) = pglobal (fin-inject slot)
gweaken (ppi parameter body) = ppi (gweaken parameter) (gweaken body)
gweaken (plam parameter body) = plam (gweaken parameter) (gweaken body)
gweaken (papp function argument) =
  papp (gweaken function) (gweaken argument)
gweaken punit-type = punit-type
gweaken punit = punit

gweaken-erasure :
  {globals locals : Nat} (term : PTm globals locals) →
  ptm-to-wire (gweaken term) ≡ ptm-to-wire term
gweaken-erasure (pvar x) = refl
gweaken-erasure (psort level) = refl
gweaken-erasure (pglobal slot) =
  cong wire-global-slot (fin-inject-ordinal slot)
gweaken-erasure (ppi parameter body) =
  cong₂ wire-pi (gweaken-erasure parameter) (gweaken-erasure body)
gweaken-erasure (plam parameter body) =
  cong₂ wire-lambda (gweaken-erasure parameter) (gweaken-erasure body)
gweaken-erasure (papp function argument) =
  cong₂ wire-apply (gweaken-erasure function) (gweaken-erasure argument)
gweaken-erasure punit-type = refl
gweaken-erasure punit = refl

-- Splitting a finite index of `suc count` into the strictly prior
-- prefix (`wire-just`, same ordinal) or the newest slot (`wire-nothing`,
-- ordinal `count`).
fin-split : {count : Nat} → Fin (suc count) → WireMaybe (Fin count)
fin-split {zero} fzero = wire-nothing
fin-split {zero} (fsuc ())
fin-split {suc count} fzero = wire-just fzero
fin-split {suc count} (fsuc x) = wire-map-maybe fsuc (fin-split x)

wire-just-injective :
  {A : Set} {left right : A} →
  wire-just left ≡ wire-just right → left ≡ right
wire-just-injective refl = refl

wire-map-nothing :
  {A B : Set} (function : A → B) (value : WireMaybe A) →
  wire-map-maybe function value ≡ wire-nothing → value ≡ wire-nothing
wire-map-nothing function wire-nothing proof = refl
wire-map-nothing function (wire-just value) ()

fin-split-top :
  {count : Nat} (x : Fin (suc count)) →
  fin-split x ≡ wire-nothing → fin-ordinal x ≡ count
fin-split-top {zero} fzero proof = refl
fin-split-top {zero} (fsuc ()) proof
fin-split-top {suc count} fzero ()
fin-split-top {suc count} (fsuc x) proof =
  cong suc
    (fin-split-top x (wire-map-nothing fsuc (fin-split x) proof))

fin-split-prefix :
  {count : Nat} (x : Fin (suc count)) (y : Fin count) →
  fin-split x ≡ wire-just y → fin-ordinal x ≡ fin-ordinal y
fin-split-prefix {zero} fzero y ()
fin-split-prefix {zero} (fsuc ()) y proof
fin-split-prefix {suc count} fzero y proof =
  cong fin-ordinal (wire-just-injective proof)
fin-split-prefix {suc count} (fsuc x) y proof
  with fin-split x in split-equation
... | wire-nothing = empty-eliminate (nothing-not-just-wire proof)
  where
  nothing-not-just-wire :
    {A : Set} {value : A} →
    wire-nothing ≡ wire-just value → Empty
  nothing-not-just-wire ()
... | wire-just y-prefix =
  trans
    (cong suc (fin-split-prefix x y-prefix split-equation))
    (cong fin-ordinal (wire-just-injective proof))

-- Strict-prior lookup: every slot's declaration, weakened to the full
-- signature, erases to exactly the stored wire declaration at the
-- chronological list position `fin-ordinal`.
lookup-psig-branch :
  {count : Nat} → PSig count → PTm count zero →
  WireMaybe (Fin count) → PTm (suc count) zero
lookup-psig-type :
  {count : Nat} → PSig count → Fin count → PTm count zero

lookup-psig-branch sig ty wire-nothing = gweaken ty
lookup-psig-branch sig ty (wire-just y) =
  gweaken (lookup-psig-type sig y)

lookup-psig-type (psig-snoc sig ty body) x =
  lookup-psig-branch sig ty (fin-split x)

lookup-psig-body-branch :
  {count : Nat} → PSig count → WireMaybe (PTm count zero) →
  WireMaybe (Fin count) → WireMaybe (PTm (suc count) zero)
lookup-psig-body :
  {count : Nat} → PSig count → Fin count →
  WireMaybe (PTm count zero)

lookup-psig-body-branch sig body wire-nothing =
  wire-map-maybe gweaken body
lookup-psig-body-branch sig body (wire-just y) =
  wire-map-maybe gweaken (lookup-psig-body sig y)

lookup-psig-body (psig-snoc sig ty body) x =
  lookup-psig-body-branch sig body (fin-split x)

global-slot-lookup-correspondence :
  {count : Nat} (sig : PSig count) (x : Fin count) →
  list-entry (psig-to-wire-types sig) (fin-ordinal x)
  ≡ wire-just (ptm-to-wire (lookup-psig-type sig x))
global-slot-lookup-correspondence (psig-snoc sig ty body) x
  with fin-split x in split-equation
... | wire-nothing
  rewrite fin-split-top x split-equation
        | list-entry-at-length (psig-to-wire-types sig)
            (ptm-to-wire ty) _ (psig-types-length sig)
  = cong wire-just (sym (gweaken-erasure ty))
... | wire-just y
  rewrite fin-split-prefix x y split-equation
        | list-entry-prefix-at (psig-to-wire-types sig)
            (ptm-to-wire ty ∷ []) (fin-ordinal y) _
            (psig-types-length sig) (fin-ordinal-less y)
  = trans
      (global-slot-lookup-correspondence sig y)
      (cong wire-just (sym (gweaken-erasure (lookup-psig-type sig y))))

global-body-lookup-correspondence :
  {count : Nat} (sig : PSig count) (x : Fin count) →
  list-entry (psig-to-wire-bodies sig) (fin-ordinal x)
  ≡ wire-just (wire-map-maybe ptm-to-wire (lookup-psig-body sig x))
global-body-lookup-correspondence (psig-snoc sig ty body) x
  with fin-split x in split-equation
... | wire-nothing
  rewrite fin-split-top x split-equation
        | list-entry-at-length (psig-to-wire-bodies sig)
            (wire-map-maybe ptm-to-wire body) _ (psig-bodies-length sig)
  = cong wire-just (body-erasure body)
  where
  body-erasure :
    {count : Nat} (body : WireMaybe (PTm count zero)) →
    wire-map-maybe ptm-to-wire body
    ≡ wire-map-maybe ptm-to-wire (wire-map-maybe gweaken body)
  body-erasure wire-nothing = refl
  body-erasure (wire-just value) =
    cong wire-just (sym (gweaken-erasure value))
... | wire-just y
  rewrite fin-split-prefix x y split-equation
        | list-entry-prefix-at (psig-to-wire-bodies sig)
            (wire-map-maybe ptm-to-wire body ∷ []) (fin-ordinal y) _
            (psig-bodies-length sig) (fin-ordinal-less y)
  = trans
      (global-body-lookup-correspondence sig y)
      (cong wire-just (body-erasure (lookup-psig-body sig y)))
  where
  body-erasure :
    {count : Nat} (body : WireMaybe (PTm count zero)) →
    wire-map-maybe ptm-to-wire body
    ≡ wire-map-maybe ptm-to-wire (wire-map-maybe gweaken body)
  body-erasure wire-nothing = refl
  body-erasure (wire-just value) =
    cong wire-just (sym (gweaken-erasure value))

-- Erasure is injective: together with the erasure round trips this
-- makes the intrinsic layer a faithful embedding — the wire direction
-- (`wire-context-round-trip`, `wire-sig-*-round-trip`) pins the
-- oldest-first/chronological order, and injectivity guarantees that an
-- intrinsic context or signature is uniquely determined by its wire
-- erasure. (A cast-free intrinsic-direction round trip is recovered
-- entry-wise from `ptm-round-trip` plus these theorems.)
wire-to-ptm-cong :
  (globals locals : Nat) {left right : WireTermV1} → left ≡ right →
  (left-proof : wire-in-scope globals locals left ≡ true)
  (right-proof : wire-in-scope globals locals right ≡ true) →
  wire-to-ptm globals locals left left-proof
  ≡ wire-to-ptm globals locals right right-proof
wire-to-ptm-cong globals locals refl left-proof right-proof =
  wire-to-ptm-irrelevant globals locals _ left-proof right-proof

ptm-to-wire-injective :
  {globals locals : Nat} (left right : PTm globals locals) →
  ptm-to-wire left ≡ ptm-to-wire right → left ≡ right
ptm-to-wire-injective left right proof =
  trans
    (sym (ptm-round-trip left (ptm-to-wire-in-scope left)))
    (trans
      (wire-to-ptm-cong _ _ proof
        (ptm-to-wire-in-scope left) (ptm-to-wire-in-scope right))
      (ptm-round-trip right (ptm-to-wire-in-scope right)))

snoc-nonempty :
  {A : Set} (values : List A) (value : A) →
  values ++ (value ∷ []) ≡ [] → Empty
snoc-nonempty [] value ()
snoc-nonempty (x ∷ values) value ()

cons-head-equal :
  {A : Set} {left right : A} {lefts rights : List A} →
  left ∷ lefts ≡ right ∷ rights → left ≡ right
cons-head-equal refl = refl

cons-tail-equal :
  {A : Set} {left right : A} {lefts rights : List A} →
  left ∷ lefts ≡ right ∷ rights → lefts ≡ rights
cons-tail-equal refl = refl

snoc-injective-head :
  {A : Set} (lefts rights : List A) (left right : A) →
  lefts ++ (left ∷ []) ≡ rights ++ (right ∷ []) → lefts ≡ rights
snoc-injective-head [] [] left right proof = refl
snoc-injective-head [] (y ∷ rights) left right proof =
  empty-eliminate
    (snoc-nonempty rights right (sym (cons-tail-equal proof)))
snoc-injective-head (x ∷ lefts) [] left right proof =
  empty-eliminate (snoc-nonempty lefts left (cons-tail-equal proof))
snoc-injective-head (x ∷ lefts) (y ∷ rights) left right proof
  rewrite cons-head-equal proof
        | snoc-injective-head lefts rights left right
            (cons-tail-equal proof)
  = refl

snoc-injective-last :
  {A : Set} (lefts rights : List A) (left right : A) →
  lefts ++ (left ∷ []) ≡ rights ++ (right ∷ []) → left ≡ right
snoc-injective-last [] [] left right proof = cons-head-equal proof
snoc-injective-last [] (y ∷ rights) left right proof =
  empty-eliminate
    (snoc-nonempty rights right (sym (cons-tail-equal proof)))
snoc-injective-last (x ∷ lefts) [] left right proof =
  empty-eliminate (snoc-nonempty lefts left (cons-tail-equal proof))
snoc-injective-last (x ∷ lefts) (y ∷ rights) left right proof =
  snoc-injective-last lefts rights left right (cons-tail-equal proof)

pctx-to-wire-injective :
  {globals locals : Nat} (left right : PCtx globals locals) →
  pctx-to-wire left ≡ pctx-to-wire right → left ≡ right
pctx-to-wire-injective pempty pempty proof = refl
pctx-to-wire-injective (left psnoc left-parameter)
  (right psnoc right-parameter) proof
  rewrite pctx-to-wire-injective left right
      (snoc-injective-head (pctx-to-wire left) (pctx-to-wire right)
        (ptm-to-wire left-parameter) (ptm-to-wire right-parameter) proof)
    | ptm-to-wire-injective left-parameter right-parameter
        (snoc-injective-last (pctx-to-wire left) (pctx-to-wire right)
          (ptm-to-wire left-parameter) (ptm-to-wire right-parameter)
          proof)
  = refl

wire-maybe-term-injective :
  {globals locals : Nat}
  (left right : WireMaybe (PTm globals locals)) →
  wire-map-maybe ptm-to-wire left ≡ wire-map-maybe ptm-to-wire right →
  left ≡ right
wire-maybe-term-injective wire-nothing wire-nothing proof = refl
wire-maybe-term-injective wire-nothing (wire-just right) ()
wire-maybe-term-injective (wire-just left) wire-nothing ()
wire-maybe-term-injective (wire-just left) (wire-just right) proof =
  cong wire-just
    (ptm-to-wire-injective left right (wire-just-injective proof))

psig-to-wire-injective :
  {count : Nat} (left right : PSig count) →
  psig-to-wire-types left ≡ psig-to-wire-types right →
  psig-to-wire-bodies left ≡ psig-to-wire-bodies right →
  left ≡ right
psig-to-wire-injective psig-empty psig-empty types-proof bodies-proof =
  refl
psig-to-wire-injective (psig-snoc left left-type left-body)
  (psig-snoc right right-type right-body) types-proof bodies-proof
  rewrite psig-to-wire-injective left right
      (snoc-injective-head (psig-to-wire-types left)
        (psig-to-wire-types right)
        (ptm-to-wire left-type) (ptm-to-wire right-type) types-proof)
      (snoc-injective-head (psig-to-wire-bodies left)
        (psig-to-wire-bodies right)
        (wire-map-maybe ptm-to-wire left-body)
        (wire-map-maybe ptm-to-wire right-body) bodies-proof)
    | ptm-to-wire-injective left-type right-type
        (snoc-injective-last (psig-to-wire-types left)
          (psig-to-wire-types right)
          (ptm-to-wire left-type) (ptm-to-wire right-type) types-proof)
    | wire-maybe-term-injective left-body right-body
        (snoc-injective-last (psig-to-wire-bodies left)
          (psig-to-wire-bodies right)
          (wire-map-maybe ptm-to-wire left-body)
          (wire-map-maybe ptm-to-wire right-body) bodies-proof)
  = refl

-- Single substitution on intrinsic terms, and its transport through the
-- abstract decoder: dependent application results computed on the wire
-- side correspond to `instantiate` in the abstract calculus.
psub-single :
  {globals locals : Nat} → PTm globals locals →
  PSub globals (suc locals) locals
psub-single argument fzero = argument
psub-single argument (fsuc x) = pvar x

pinstantiate :
  {globals locals : Nat} → PTm globals (suc locals) →
  PTm globals locals → PTm globals locals
pinstantiate body argument = psubstitute (psub-single argument) body

decode-psub-single :
  {globals locals : Nat} (argument : PTm globals locals) →
  decode-psubstitution (psub-single argument)
  ≗ single (decode-ptm argument)
decode-psub-single argument bound = refl
decode-psub-single argument (free x) =
  cong var (decode-fin-encode-var x)

decode-pinstantiate :
  {globals locals : Nat}
  (body : PTm globals (suc locals)) (argument : PTm globals locals) →
  decode-ptm (pinstantiate body argument)
  ≡ instantiate (decode-ptm body) (decode-ptm argument)
decode-pinstantiate body argument =
  trans
    (decode-psubstitute (psub-single argument) body)
    (substitute-cong (decode-psub-single argument) (decode-ptm body))

-- Composite decode from scoped wire terms to the abstract calculus, and
-- the context-extension transport at the abstract level.
wire-to-tm :
  (globals locals : Nat) (term : WireTermV1) →
  wire-in-scope globals locals term ≡ true →
  Tm (PVar locals)
wire-to-tm globals locals term proof =
  decode-ptm (wire-to-ptm globals locals term proof)

wire-context-extension-to-abstract :
  {globals locals : Nat} (context : PCtx globals locals)
  (parameter : PTm globals locals) →
  decode-pctx (context psnoc parameter)
  ≗ extend-context (decode-pctx context) (decode-ptm parameter)
wire-context-extension-to-abstract context parameter =
  decode-pctx-snoc context parameter
