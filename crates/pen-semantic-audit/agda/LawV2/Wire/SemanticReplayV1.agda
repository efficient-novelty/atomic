{-# OPTIONS --safe --without-K #-}

-- Phase E, conversion side: semantic replay of decoded reduction traces.
--
-- The structural checker only verifies scoping, chaining, and census
-- recomputation; it never checks that a step labeled beta actually
-- beta-reduces, or that a transparent-delta step unfolds to the exact
-- signature body. This module closes that gap at the intrinsic level:
--
-- - `PStepV1` is the intrinsic base-Q0 step relation on `PTm` (beta via
--   `pinstantiate`, policy-authorized delta via the strict-prior
--   signature body, and the six congruence frames with binder-local
--   scope growth);
-- - decoded wire step trees are replayed by a Boolean checker whose
--   soundness theorem produces genuine `PStepV1` steps;
-- - accepted traces yield reflexive-transitive `PStepsV1` chains; and
-- - the accepted common normal form provably has no outgoing `PStepV1`
--   step, with a bridge from the structural census recomputation.
--
-- Nothing here mints authority; these are theorem packages for the
-- later private factory.

module LawV2.Wire.SemanticReplayV1 where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe
  using ()
  renaming (Maybe to WireMaybe; just to wire-just; nothing to wire-nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.Wire.Bytes
  using (length; nat-equal; _and_; _×_; _,_; _++_)
open import LawV2.Wire.ProductionBundleV1
open import LawV2.Wire.ContextChecker using (not; nat-less)
open import LawV2.Wire.BundleChecker
  using (nat-member; _or_; is-lambda-term; expected-census-from)
open import LawV2.LambdaUnit.ProductionSyntaxV1
open import LawV2.LambdaUnit.ProductionDecodingV1
open import LawV2.Wire.ContextCorrespondenceV1

-- Decidable equality on intrinsic terms, sound for propositional
-- equality.
nat-equal-sound :
  (left right : Nat) → nat-equal left right ≡ true → left ≡ right
nat-equal-sound zero zero proof = refl
nat-equal-sound zero (suc right) ()
nat-equal-sound (suc left) zero ()
nat-equal-sound (suc left) (suc right) proof =
  cong suc (nat-equal-sound left right proof)

suc-injective : {left right : Nat} → suc left ≡ suc right → left ≡ right
suc-injective refl = refl

fin-ordinal-injective :
  {count : Nat} (left right : Fin count) →
  fin-ordinal left ≡ fin-ordinal right → left ≡ right
fin-ordinal-injective fzero fzero proof = refl
fin-ordinal-injective fzero (fsuc right) ()
fin-ordinal-injective (fsuc left) fzero ()
fin-ordinal-injective (fsuc left) (fsuc right) proof =
  cong fsuc (fin-ordinal-injective left right (suc-injective proof))

fin-equal : {count : Nat} → Fin count → Fin count → Bool
fin-equal left right = nat-equal (fin-ordinal left) (fin-ordinal right)

fin-equal-sound :
  {count : Nat} (left right : Fin count) →
  fin-equal left right ≡ true → left ≡ right
fin-equal-sound left right proof =
  fin-ordinal-injective left right
    (nat-equal-sound (fin-ordinal left) (fin-ordinal right) proof)

ptm-equal :
  {globals locals : Nat} →
  PTm globals locals → PTm globals locals → Bool
ptm-equal (pvar left) (pvar right) = fin-equal left right
ptm-equal (psort left) (psort right) = nat-equal left right
ptm-equal (pglobal left) (pglobal right) = fin-equal left right
ptm-equal (ppi left-parameter left-body) (ppi right-parameter right-body) =
  ptm-equal left-parameter right-parameter and
  ptm-equal left-body right-body
ptm-equal (plam left-parameter left-body)
  (plam right-parameter right-body) =
  ptm-equal left-parameter right-parameter and
  ptm-equal left-body right-body
ptm-equal (papp left-function left-argument)
  (papp right-function right-argument) =
  ptm-equal left-function right-function and
  ptm-equal left-argument right-argument
ptm-equal punit-type punit-type = true
ptm-equal punit punit = true
ptm-equal _ _ = false

ptm-equal-sound :
  {globals locals : Nat} (left right : PTm globals locals) →
  ptm-equal left right ≡ true → left ≡ right
ptm-equal-sound (pvar left) (pvar right) proof =
  cong pvar (fin-equal-sound left right proof)
ptm-equal-sound (pvar left) (psort right) ()
ptm-equal-sound (pvar left) (pglobal right) ()
ptm-equal-sound (pvar left) (ppi _ _) ()
ptm-equal-sound (pvar left) (plam _ _) ()
ptm-equal-sound (pvar left) (papp _ _) ()
ptm-equal-sound (pvar left) punit-type ()
ptm-equal-sound (pvar left) punit ()
ptm-equal-sound (psort left) (pvar right) ()
ptm-equal-sound (psort left) (psort right) proof =
  cong psort (nat-equal-sound left right proof)
ptm-equal-sound (psort left) (pglobal right) ()
ptm-equal-sound (psort left) (ppi _ _) ()
ptm-equal-sound (psort left) (plam _ _) ()
ptm-equal-sound (psort left) (papp _ _) ()
ptm-equal-sound (psort left) punit-type ()
ptm-equal-sound (psort left) punit ()
ptm-equal-sound (pglobal left) (pvar right) ()
ptm-equal-sound (pglobal left) (psort right) ()
ptm-equal-sound (pglobal left) (pglobal right) proof =
  cong pglobal (fin-equal-sound left right proof)
ptm-equal-sound (pglobal left) (ppi _ _) ()
ptm-equal-sound (pglobal left) (plam _ _) ()
ptm-equal-sound (pglobal left) (papp _ _) ()
ptm-equal-sound (pglobal left) punit-type ()
ptm-equal-sound (pglobal left) punit ()
ptm-equal-sound (ppi _ _) (pvar _) ()
ptm-equal-sound (ppi _ _) (psort _) ()
ptm-equal-sound (ppi _ _) (pglobal _) ()
ptm-equal-sound (ppi left-parameter left-body)
  (ppi right-parameter right-body) proof =
  cong₂ ppi
    (ptm-equal-sound left-parameter right-parameter
      (and-first (ptm-equal left-parameter right-parameter)
        (ptm-equal left-body right-body) proof))
    (ptm-equal-sound left-body right-body
      (and-second (ptm-equal left-parameter right-parameter)
        (ptm-equal left-body right-body) proof))
ptm-equal-sound (ppi _ _) (plam _ _) ()
ptm-equal-sound (ppi _ _) (papp _ _) ()
ptm-equal-sound (ppi _ _) punit-type ()
ptm-equal-sound (ppi _ _) punit ()
ptm-equal-sound (plam _ _) (pvar _) ()
ptm-equal-sound (plam _ _) (psort _) ()
ptm-equal-sound (plam _ _) (pglobal _) ()
ptm-equal-sound (plam _ _) (ppi _ _) ()
ptm-equal-sound (plam left-parameter left-body)
  (plam right-parameter right-body) proof =
  cong₂ plam
    (ptm-equal-sound left-parameter right-parameter
      (and-first (ptm-equal left-parameter right-parameter)
        (ptm-equal left-body right-body) proof))
    (ptm-equal-sound left-body right-body
      (and-second (ptm-equal left-parameter right-parameter)
        (ptm-equal left-body right-body) proof))
ptm-equal-sound (plam _ _) (papp _ _) ()
ptm-equal-sound (plam _ _) punit-type ()
ptm-equal-sound (plam _ _) punit ()
ptm-equal-sound (papp _ _) (pvar _) ()
ptm-equal-sound (papp _ _) (psort _) ()
ptm-equal-sound (papp _ _) (pglobal _) ()
ptm-equal-sound (papp _ _) (ppi _ _) ()
ptm-equal-sound (papp _ _) (plam _ _) ()
ptm-equal-sound (papp left-function left-argument)
  (papp right-function right-argument) proof =
  cong₂ papp
    (ptm-equal-sound left-function right-function
      (and-first (ptm-equal left-function right-function)
        (ptm-equal left-argument right-argument) proof))
    (ptm-equal-sound left-argument right-argument
      (and-second (ptm-equal left-function right-function)
        (ptm-equal left-argument right-argument) proof))
ptm-equal-sound (papp _ _) punit-type ()
ptm-equal-sound (papp _ _) punit ()
ptm-equal-sound punit-type (pvar _) ()
ptm-equal-sound punit-type (psort _) ()
ptm-equal-sound punit-type (pglobal _) ()
ptm-equal-sound punit-type (ppi _ _) ()
ptm-equal-sound punit-type (plam _ _) ()
ptm-equal-sound punit-type (papp _ _) ()
ptm-equal-sound punit-type punit-type proof = refl
ptm-equal-sound punit-type punit ()
ptm-equal-sound punit (pvar _) ()
ptm-equal-sound punit (psort _) ()
ptm-equal-sound punit (pglobal _) ()
ptm-equal-sound punit (ppi _ _) ()
ptm-equal-sound punit (plam _ _) ()
ptm-equal-sound punit (papp _ _) ()
ptm-equal-sound punit punit-type ()
ptm-equal-sound punit punit proof = refl

-- Closed terms embed into any local scope.
fin-absurd : {A : Set} → Fin zero → A
fin-absurd ()

pembed-closed :
  {globals locals : Nat} → PTm globals zero → PTm globals locals
pembed-closed term = prename fin-absurd term

-- The policy-restricted transparent-delta body lookup: only slots on
-- the enabled list resolve, and only to their strict-prior signature
-- body.
policy-branch :
  {globals : Nat} → Bool → WireMaybe (PTm globals zero) →
  WireMaybe (PTm globals zero)
policy-branch false body = wire-nothing
policy-branch true body = body

policy-body :
  {globals : Nat} → List Nat → PSig globals →
  Fin globals → WireMaybe (PTm globals zero)
policy-body enabled sig slot =
  policy-branch (nat-member (fin-ordinal slot) enabled)
    (lookup-psig-body sig slot)

-- The intrinsic base-Q0 step relation: beta, authorized transparent
-- delta, and the six congruence frames. Body congruences descend under
-- the binder.
data PStepV1
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero)) :
  {locals : Nat} →
  PTm globals locals → PTm globals locals → Set where
  pstep-beta :
    {locals : Nat} {parameter : PTm globals locals}
    {body : PTm globals (suc locals)} {argument : PTm globals locals} →
    PStepV1 delta
      (papp (plam parameter body) argument)
      (pinstantiate body argument)
  pstep-delta :
    {locals : Nat} {slot : Fin globals} {body : PTm globals zero} →
    delta slot ≡ wire-just body →
    PStepV1 delta {locals} (pglobal slot) (pembed-closed body)
  pstep-pi-parameter :
    {locals : Nat} {left right : PTm globals locals}
    {body : PTm globals (suc locals)} →
    PStepV1 delta left right →
    PStepV1 delta (ppi left body) (ppi right body)
  pstep-pi-body :
    {locals : Nat} {parameter : PTm globals locals}
    {left right : PTm globals (suc locals)} →
    PStepV1 delta left right →
    PStepV1 delta (ppi parameter left) (ppi parameter right)
  pstep-lambda-parameter :
    {locals : Nat} {left right : PTm globals locals}
    {body : PTm globals (suc locals)} →
    PStepV1 delta left right →
    PStepV1 delta (plam left body) (plam right body)
  pstep-lambda-body :
    {locals : Nat} {parameter : PTm globals locals}
    {left right : PTm globals (suc locals)} →
    PStepV1 delta left right →
    PStepV1 delta (plam parameter left) (plam parameter right)
  pstep-apply-function :
    {locals : Nat} {left right argument : PTm globals locals} →
    PStepV1 delta left right →
    PStepV1 delta (papp left argument) (papp right argument)
  pstep-apply-argument :
    {locals : Nat} {function left right : PTm globals locals} →
    PStepV1 delta left right →
    PStepV1 delta (papp function left) (papp function right)

data PStepsV1
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero)) :
  {locals : Nat} →
  PTm globals locals → PTm globals locals → Set where
  psteps-refl :
    {locals : Nat} (term : PTm globals locals) →
    PStepsV1 delta term term
  psteps-next :
    {locals : Nat} {left middle right : PTm globals locals} →
    PStepV1 delta left middle →
    PStepsV1 delta middle right →
    PStepsV1 delta left right

-- Decoded step trees: the wire step payload with every term decoded at
-- its intrinsic scope. Body congruences carry premises one binder
-- deeper, mirroring the structural checker's discipline.
data PStepTreeV1 (globals : Nat) : Nat → Set where
  ptree-beta :
    {locals : Nat} → PTm globals locals → PTm globals locals →
    PStepTreeV1 globals locals
  ptree-delta :
    {locals : Nat} → PTm globals locals → PTm globals locals → Nat →
    PStepTreeV1 globals locals
  ptree-pi-parameter :
    {locals : Nat} → PTm globals locals → PTm globals locals →
    PStepTreeV1 globals locals → PStepTreeV1 globals locals
  ptree-pi-body :
    {locals : Nat} → PTm globals locals → PTm globals locals →
    PStepTreeV1 globals (suc locals) → PStepTreeV1 globals locals
  ptree-lambda-parameter :
    {locals : Nat} → PTm globals locals → PTm globals locals →
    PStepTreeV1 globals locals → PStepTreeV1 globals locals
  ptree-lambda-body :
    {locals : Nat} → PTm globals locals → PTm globals locals →
    PStepTreeV1 globals (suc locals) → PStepTreeV1 globals locals
  ptree-apply-function :
    {locals : Nat} → PTm globals locals → PTm globals locals →
    PStepTreeV1 globals locals → PStepTreeV1 globals locals
  ptree-apply-argument :
    {locals : Nat} → PTm globals locals → PTm globals locals →
    PStepTreeV1 globals locals → PStepTreeV1 globals locals

ptree-source :
  {globals locals : Nat} → PStepTreeV1 globals locals →
  PTm globals locals
ptree-source (ptree-beta source _) = source
ptree-source (ptree-delta source _ _) = source
ptree-source (ptree-pi-parameter source _ _) = source
ptree-source (ptree-pi-body source _ _) = source
ptree-source (ptree-lambda-parameter source _ _) = source
ptree-source (ptree-lambda-body source _ _) = source
ptree-source (ptree-apply-function source _ _) = source
ptree-source (ptree-apply-argument source _ _) = source

ptree-target :
  {globals locals : Nat} → PStepTreeV1 globals locals →
  PTm globals locals
ptree-target (ptree-beta _ target) = target
ptree-target (ptree-delta _ target _) = target
ptree-target (ptree-pi-parameter _ target _) = target
ptree-target (ptree-pi-body _ target _) = target
ptree-target (ptree-lambda-parameter _ target _) = target
ptree-target (ptree-lambda-body _ target _) = target
ptree-target (ptree-apply-function _ target _) = target
ptree-target (ptree-apply-argument _ target _) = target

-- Wire decoding with internal scope checks.
wire-maybe-bind :
  {A B : Set} → WireMaybe A → (A → WireMaybe B) → WireMaybe B
wire-maybe-bind wire-nothing continuation = wire-nothing
wire-maybe-bind (wire-just value) continuation = continuation value

decode-term-branch :
  (globals locals : Nat) (term : WireTermV1) (result : Bool) →
  wire-in-scope globals locals term ≡ result →
  WireMaybe (PTm globals locals)
decode-term-branch globals locals term false proof = wire-nothing
decode-term-branch globals locals term true proof =
  wire-just (wire-to-ptm globals locals term proof)

decode-term-checked :
  (globals locals : Nat) (term : WireTermV1) →
  WireMaybe (PTm globals locals)
decode-term-checked globals locals term =
  decode-term-branch globals locals term
    (wire-in-scope globals locals term) refl

decode-step-checked :
  (globals locals : Nat) → BaseQ0ReductionStepWireV1 →
  WireMaybe (PStepTreeV1 globals locals)
decode-step-checked globals locals (step-beta source target) =
  wire-maybe-bind (decode-term-checked globals locals source) (λ s →
  wire-maybe-bind (decode-term-checked globals locals target) (λ t →
  wire-just (ptree-beta s t)))
decode-step-checked globals locals
  (step-transparent-delta source target slot) =
  wire-maybe-bind (decode-term-checked globals locals source) (λ s →
  wire-maybe-bind (decode-term-checked globals locals target) (λ t →
  wire-just (ptree-delta s t slot)))
decode-step-checked globals locals
  (step-pi-parameter-congruence source target premise) =
  wire-maybe-bind (decode-term-checked globals locals source) (λ s →
  wire-maybe-bind (decode-term-checked globals locals target) (λ t →
  wire-maybe-bind (decode-step-checked globals locals premise) (λ p →
  wire-just (ptree-pi-parameter s t p))))
decode-step-checked globals locals
  (step-pi-body-congruence source target premise) =
  wire-maybe-bind (decode-term-checked globals locals source) (λ s →
  wire-maybe-bind (decode-term-checked globals locals target) (λ t →
  wire-maybe-bind (decode-step-checked globals (suc locals) premise)
    (λ p →
  wire-just (ptree-pi-body s t p))))
decode-step-checked globals locals
  (step-lambda-parameter-congruence source target premise) =
  wire-maybe-bind (decode-term-checked globals locals source) (λ s →
  wire-maybe-bind (decode-term-checked globals locals target) (λ t →
  wire-maybe-bind (decode-step-checked globals locals premise) (λ p →
  wire-just (ptree-lambda-parameter s t p))))
decode-step-checked globals locals
  (step-lambda-body-congruence source target premise) =
  wire-maybe-bind (decode-term-checked globals locals source) (λ s →
  wire-maybe-bind (decode-term-checked globals locals target) (λ t →
  wire-maybe-bind (decode-step-checked globals (suc locals) premise)
    (λ p →
  wire-just (ptree-lambda-body s t p))))
decode-step-checked globals locals
  (step-apply-function-congruence source target premise) =
  wire-maybe-bind (decode-term-checked globals locals source) (λ s →
  wire-maybe-bind (decode-term-checked globals locals target) (λ t →
  wire-maybe-bind (decode-step-checked globals locals premise) (λ p →
  wire-just (ptree-apply-function s t p))))
decode-step-checked globals locals
  (step-apply-argument-congruence source target premise) =
  wire-maybe-bind (decode-term-checked globals locals source) (λ s →
  wire-maybe-bind (decode-term-checked globals locals target) (λ t →
  wire-maybe-bind (decode-step-checked globals locals premise) (λ p →
  wire-just (ptree-apply-argument s t p))))

-- Semantic replay of one decoded step tree.
delta-target-matches :
  {globals locals : Nat} →
  WireMaybe (PTm globals zero) → PTm globals locals → Bool
delta-target-matches wire-nothing target = false
delta-target-matches (wire-just body) target =
  ptm-equal target (pembed-closed body)

beta-source-matches :
  {globals locals : Nat} →
  PTm globals locals → PTm globals locals → Bool
beta-source-matches (papp (plam parameter body) argument) target =
  ptm-equal target (pinstantiate body argument)
beta-source-matches _ _ = false

ptree-replays :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero)) →
  {locals : Nat} → PStepTreeV1 globals locals → Bool
ptree-replays delta (ptree-beta source target) =
  beta-source-matches source target
ptree-replays delta (ptree-delta (pglobal slot) target claimed-slot) =
  nat-equal claimed-slot (fin-ordinal slot) and
  delta-target-matches (delta slot) target
ptree-replays delta (ptree-delta _ _ _) = false
ptree-replays delta
  (ptree-pi-parameter (ppi sp sb) (ppi tp tb) premise) =
  ptm-equal sp (ptree-source premise) and
  (ptm-equal tp (ptree-target premise) and
  (ptm-equal sb tb and ptree-replays delta premise))
ptree-replays delta (ptree-pi-parameter _ _ _) = false
ptree-replays delta (ptree-pi-body (ppi sp sb) (ppi tp tb) premise) =
  ptm-equal sp tp and
  (ptm-equal sb (ptree-source premise) and
  (ptm-equal tb (ptree-target premise) and ptree-replays delta premise))
ptree-replays delta (ptree-pi-body _ _ _) = false
ptree-replays delta
  (ptree-lambda-parameter (plam sp sb) (plam tp tb) premise) =
  ptm-equal sp (ptree-source premise) and
  (ptm-equal tp (ptree-target premise) and
  (ptm-equal sb tb and ptree-replays delta premise))
ptree-replays delta (ptree-lambda-parameter _ _ _) = false
ptree-replays delta (ptree-lambda-body (plam sp sb) (plam tp tb) premise) =
  ptm-equal sp tp and
  (ptm-equal sb (ptree-source premise) and
  (ptm-equal tb (ptree-target premise) and ptree-replays delta premise))
ptree-replays delta (ptree-lambda-body _ _ _) = false
ptree-replays delta
  (ptree-apply-function (papp sf sa) (papp tf ta) premise) =
  ptm-equal sf (ptree-source premise) and
  (ptm-equal tf (ptree-target premise) and
  (ptm-equal sa ta and ptree-replays delta premise))
ptree-replays delta (ptree-apply-function _ _ _) = false
ptree-replays delta
  (ptree-apply-argument (papp sf sa) (papp tf ta) premise) =
  ptm-equal sf tf and
  (ptm-equal sa (ptree-source premise) and
  (ptm-equal ta (ptree-target premise) and ptree-replays delta premise))
ptree-replays delta (ptree-apply-argument _ _ _) = false

-- Soundness: an accepted replay is a genuine intrinsic step between the
-- recorded endpoints.
beta-source-sound :
  {globals locals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero))
  (source target : PTm globals locals) →
  beta-source-matches source target ≡ true →
  PStepV1 delta source target
beta-source-sound delta (papp (plam parameter body) argument) target
  proof
  rewrite ptm-equal-sound target (pinstantiate body argument) proof
  = pstep-beta
beta-source-sound delta (pvar _) target ()
beta-source-sound delta (psort _) target ()
beta-source-sound delta (pglobal _) target ()
beta-source-sound delta (ppi _ _) target ()
beta-source-sound delta (plam _ _) target ()
beta-source-sound delta (papp (pvar _) _) target ()
beta-source-sound delta (papp (psort _) _) target ()
beta-source-sound delta (papp (pglobal _) _) target ()
beta-source-sound delta (papp (ppi _ _) _) target ()
beta-source-sound delta (papp (papp _ _) _) target ()
beta-source-sound delta (papp punit-type _) target ()
beta-source-sound delta (papp punit _) target ()
beta-source-sound delta punit-type target ()
beta-source-sound delta punit target ()

delta-target-sound :
  {globals locals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero))
  (slot : Fin globals) (target : PTm globals locals) →
  delta-target-matches (delta slot) target ≡ true →
  PStepV1 delta (pglobal slot) target
delta-target-sound delta slot target proof
  with delta slot in delta-equation
... | wire-nothing = empty-eliminate (false-not-true proof)
  where
  false-not-true : false ≡ true → Empty
  false-not-true ()
... | wire-just body
  rewrite ptm-equal-sound target (pembed-closed body) proof
  = pstep-delta delta-equation

ptree-replays-sound :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero))
  {locals : Nat} (tree : PStepTreeV1 globals locals) →
  ptree-replays delta tree ≡ true →
  PStepV1 delta (ptree-source tree) (ptree-target tree)
ptree-replays-sound delta (ptree-beta source target) proof =
  beta-source-sound delta source target proof
ptree-replays-sound delta (ptree-delta (pglobal slot) target claimed)
  proof =
  delta-target-sound delta slot target
    (and-second (nat-equal claimed (fin-ordinal slot))
      (delta-target-matches (delta slot) target) proof)
ptree-replays-sound delta (ptree-delta (pvar _) _ _) ()
ptree-replays-sound delta (ptree-delta (psort _) _ _) ()
ptree-replays-sound delta (ptree-delta (ppi _ _) _ _) ()
ptree-replays-sound delta (ptree-delta (plam _ _) _ _) ()
ptree-replays-sound delta (ptree-delta (papp _ _) _ _) ()
ptree-replays-sound delta (ptree-delta punit-type _ _) ()
ptree-replays-sound delta (ptree-delta punit _ _) ()
ptree-replays-sound delta
  (ptree-pi-parameter (ppi sp sb) (ppi tp tb) premise) proof
  rewrite ptm-equal-sound sp (ptree-source premise)
      (and-first (ptm-equal sp (ptree-source premise)) _ proof)
    | ptm-equal-sound tp (ptree-target premise)
        (and-first (ptm-equal tp (ptree-target premise)) _
          (and-second (ptm-equal sp (ptree-source premise)) _ proof))
    | ptm-equal-sound sb tb
        (and-first (ptm-equal sb tb) _
          (and-second (ptm-equal tp (ptree-target premise)) _
            (and-second (ptm-equal sp (ptree-source premise)) _ proof)))
  = pstep-pi-parameter
      (ptree-replays-sound delta premise
        (and-second (ptm-equal sb tb) _
          (and-second (ptm-equal tp (ptree-target premise)) _
            (and-second (ptm-equal sp (ptree-source premise)) _
              proof))))
ptree-replays-sound delta (ptree-pi-parameter (pvar _) _ _) ()
ptree-replays-sound delta (ptree-pi-parameter (psort _) _ _) ()
ptree-replays-sound delta (ptree-pi-parameter (pglobal _) _ _) ()
ptree-replays-sound delta
  (ptree-pi-parameter (ppi _ _) (pvar _) _) ()
ptree-replays-sound delta
  (ptree-pi-parameter (ppi _ _) (psort _) _) ()
ptree-replays-sound delta
  (ptree-pi-parameter (ppi _ _) (pglobal _) _) ()
ptree-replays-sound delta
  (ptree-pi-parameter (ppi _ _) (plam _ _) _) ()
ptree-replays-sound delta
  (ptree-pi-parameter (ppi _ _) (papp _ _) _) ()
ptree-replays-sound delta
  (ptree-pi-parameter (ppi _ _) punit-type _) ()
ptree-replays-sound delta
  (ptree-pi-parameter (ppi _ _) punit _) ()
ptree-replays-sound delta (ptree-pi-parameter (plam _ _) _ _) ()
ptree-replays-sound delta (ptree-pi-parameter (papp _ _) _ _) ()
ptree-replays-sound delta (ptree-pi-parameter punit-type _ _) ()
ptree-replays-sound delta (ptree-pi-parameter punit _ _) ()
ptree-replays-sound delta
  (ptree-pi-body (ppi sp sb) (ppi tp tb) premise) proof
  rewrite ptm-equal-sound sp tp
      (and-first (ptm-equal sp tp) _ proof)
    | ptm-equal-sound sb (ptree-source premise)
        (and-first (ptm-equal sb (ptree-source premise)) _
          (and-second (ptm-equal sp tp) _ proof))
    | ptm-equal-sound tb (ptree-target premise)
        (and-first (ptm-equal tb (ptree-target premise)) _
          (and-second (ptm-equal sb (ptree-source premise)) _
            (and-second (ptm-equal sp tp) _ proof)))
  = pstep-pi-body
      (ptree-replays-sound delta premise
        (and-second (ptm-equal tb (ptree-target premise)) _
          (and-second (ptm-equal sb (ptree-source premise)) _
            (and-second (ptm-equal sp tp) _ proof))))
ptree-replays-sound delta (ptree-pi-body (pvar _) _ _) ()
ptree-replays-sound delta (ptree-pi-body (psort _) _ _) ()
ptree-replays-sound delta (ptree-pi-body (pglobal _) _ _) ()
ptree-replays-sound delta (ptree-pi-body (ppi _ _) (pvar _) _) ()
ptree-replays-sound delta (ptree-pi-body (ppi _ _) (psort _) _) ()
ptree-replays-sound delta (ptree-pi-body (ppi _ _) (pglobal _) _) ()
ptree-replays-sound delta (ptree-pi-body (ppi _ _) (plam _ _) _) ()
ptree-replays-sound delta (ptree-pi-body (ppi _ _) (papp _ _) _) ()
ptree-replays-sound delta (ptree-pi-body (ppi _ _) punit-type _) ()
ptree-replays-sound delta (ptree-pi-body (ppi _ _) punit _) ()
ptree-replays-sound delta (ptree-pi-body (plam _ _) _ _) ()
ptree-replays-sound delta (ptree-pi-body (papp _ _) _ _) ()
ptree-replays-sound delta (ptree-pi-body punit-type _ _) ()
ptree-replays-sound delta (ptree-pi-body punit _ _) ()
ptree-replays-sound delta
  (ptree-lambda-parameter (plam sp sb) (plam tp tb) premise) proof
  rewrite ptm-equal-sound sp (ptree-source premise)
      (and-first (ptm-equal sp (ptree-source premise)) _ proof)
    | ptm-equal-sound tp (ptree-target premise)
        (and-first (ptm-equal tp (ptree-target premise)) _
          (and-second (ptm-equal sp (ptree-source premise)) _ proof))
    | ptm-equal-sound sb tb
        (and-first (ptm-equal sb tb) _
          (and-second (ptm-equal tp (ptree-target premise)) _
            (and-second (ptm-equal sp (ptree-source premise)) _ proof)))
  = pstep-lambda-parameter
      (ptree-replays-sound delta premise
        (and-second (ptm-equal sb tb) _
          (and-second (ptm-equal tp (ptree-target premise)) _
            (and-second (ptm-equal sp (ptree-source premise)) _
              proof))))
ptree-replays-sound delta (ptree-lambda-parameter (pvar _) _ _) ()
ptree-replays-sound delta (ptree-lambda-parameter (psort _) _ _) ()
ptree-replays-sound delta (ptree-lambda-parameter (pglobal _) _ _) ()
ptree-replays-sound delta (ptree-lambda-parameter (ppi _ _) _ _) ()
ptree-replays-sound delta
  (ptree-lambda-parameter (plam _ _) (pvar _) _) ()
ptree-replays-sound delta
  (ptree-lambda-parameter (plam _ _) (psort _) _) ()
ptree-replays-sound delta
  (ptree-lambda-parameter (plam _ _) (pglobal _) _) ()
ptree-replays-sound delta
  (ptree-lambda-parameter (plam _ _) (ppi _ _) _) ()
ptree-replays-sound delta
  (ptree-lambda-parameter (plam _ _) (papp _ _) _) ()
ptree-replays-sound delta
  (ptree-lambda-parameter (plam _ _) punit-type _) ()
ptree-replays-sound delta
  (ptree-lambda-parameter (plam _ _) punit _) ()
ptree-replays-sound delta (ptree-lambda-parameter (papp _ _) _ _) ()
ptree-replays-sound delta (ptree-lambda-parameter punit-type _ _) ()
ptree-replays-sound delta (ptree-lambda-parameter punit _ _) ()
ptree-replays-sound delta
  (ptree-lambda-body (plam sp sb) (plam tp tb) premise) proof
  rewrite ptm-equal-sound sp tp
      (and-first (ptm-equal sp tp) _ proof)
    | ptm-equal-sound sb (ptree-source premise)
        (and-first (ptm-equal sb (ptree-source premise)) _
          (and-second (ptm-equal sp tp) _ proof))
    | ptm-equal-sound tb (ptree-target premise)
        (and-first (ptm-equal tb (ptree-target premise)) _
          (and-second (ptm-equal sb (ptree-source premise)) _
            (and-second (ptm-equal sp tp) _ proof)))
  = pstep-lambda-body
      (ptree-replays-sound delta premise
        (and-second (ptm-equal tb (ptree-target premise)) _
          (and-second (ptm-equal sb (ptree-source premise)) _
            (and-second (ptm-equal sp tp) _ proof))))
ptree-replays-sound delta (ptree-lambda-body (pvar _) _ _) ()
ptree-replays-sound delta (ptree-lambda-body (psort _) _ _) ()
ptree-replays-sound delta (ptree-lambda-body (pglobal _) _ _) ()
ptree-replays-sound delta (ptree-lambda-body (ppi _ _) _ _) ()
ptree-replays-sound delta
  (ptree-lambda-body (plam _ _) (pvar _) _) ()
ptree-replays-sound delta
  (ptree-lambda-body (plam _ _) (psort _) _) ()
ptree-replays-sound delta
  (ptree-lambda-body (plam _ _) (pglobal _) _) ()
ptree-replays-sound delta
  (ptree-lambda-body (plam _ _) (ppi _ _) _) ()
ptree-replays-sound delta
  (ptree-lambda-body (plam _ _) (papp _ _) _) ()
ptree-replays-sound delta
  (ptree-lambda-body (plam _ _) punit-type _) ()
ptree-replays-sound delta
  (ptree-lambda-body (plam _ _) punit _) ()
ptree-replays-sound delta (ptree-lambda-body (papp _ _) _ _) ()
ptree-replays-sound delta (ptree-lambda-body punit-type _ _) ()
ptree-replays-sound delta (ptree-lambda-body punit _ _) ()
ptree-replays-sound delta
  (ptree-apply-function (papp sf sa) (papp tf ta) premise) proof
  rewrite ptm-equal-sound sf (ptree-source premise)
      (and-first (ptm-equal sf (ptree-source premise)) _ proof)
    | ptm-equal-sound tf (ptree-target premise)
        (and-first (ptm-equal tf (ptree-target premise)) _
          (and-second (ptm-equal sf (ptree-source premise)) _ proof))
    | ptm-equal-sound sa ta
        (and-first (ptm-equal sa ta) _
          (and-second (ptm-equal tf (ptree-target premise)) _
            (and-second (ptm-equal sf (ptree-source premise)) _ proof)))
  = pstep-apply-function
      (ptree-replays-sound delta premise
        (and-second (ptm-equal sa ta) _
          (and-second (ptm-equal tf (ptree-target premise)) _
            (and-second (ptm-equal sf (ptree-source premise)) _
              proof))))
ptree-replays-sound delta (ptree-apply-function (pvar _) _ _) ()
ptree-replays-sound delta (ptree-apply-function (psort _) _ _) ()
ptree-replays-sound delta (ptree-apply-function (pglobal _) _ _) ()
ptree-replays-sound delta (ptree-apply-function (ppi _ _) _ _) ()
ptree-replays-sound delta (ptree-apply-function (plam _ _) _ _) ()
ptree-replays-sound delta
  (ptree-apply-function (papp _ _) (pvar _) _) ()
ptree-replays-sound delta
  (ptree-apply-function (papp _ _) (psort _) _) ()
ptree-replays-sound delta
  (ptree-apply-function (papp _ _) (pglobal _) _) ()
ptree-replays-sound delta
  (ptree-apply-function (papp _ _) (ppi _ _) _) ()
ptree-replays-sound delta
  (ptree-apply-function (papp _ _) (plam _ _) _) ()
ptree-replays-sound delta
  (ptree-apply-function (papp _ _) punit-type _) ()
ptree-replays-sound delta
  (ptree-apply-function (papp _ _) punit _) ()
ptree-replays-sound delta (ptree-apply-function punit-type _ _) ()
ptree-replays-sound delta (ptree-apply-function punit _ _) ()
ptree-replays-sound delta
  (ptree-apply-argument (papp sf sa) (papp tf ta) premise) proof
  rewrite ptm-equal-sound sf tf
      (and-first (ptm-equal sf tf) _ proof)
    | ptm-equal-sound sa (ptree-source premise)
        (and-first (ptm-equal sa (ptree-source premise)) _
          (and-second (ptm-equal sf tf) _ proof))
    | ptm-equal-sound ta (ptree-target premise)
        (and-first (ptm-equal ta (ptree-target premise)) _
          (and-second (ptm-equal sa (ptree-source premise)) _
            (and-second (ptm-equal sf tf) _ proof)))
  = pstep-apply-argument
      (ptree-replays-sound delta premise
        (and-second (ptm-equal ta (ptree-target premise)) _
          (and-second (ptm-equal sa (ptree-source premise)) _
            (and-second (ptm-equal sf tf) _ proof))))
ptree-replays-sound delta (ptree-apply-argument (pvar _) _ _) ()
ptree-replays-sound delta (ptree-apply-argument (psort _) _ _) ()
ptree-replays-sound delta (ptree-apply-argument (pglobal _) _ _) ()
ptree-replays-sound delta (ptree-apply-argument (ppi _ _) _ _) ()
ptree-replays-sound delta (ptree-apply-argument (plam _ _) _ _) ()
ptree-replays-sound delta
  (ptree-apply-argument (papp _ _) (pvar _) _) ()
ptree-replays-sound delta
  (ptree-apply-argument (papp _ _) (psort _) _) ()
ptree-replays-sound delta
  (ptree-apply-argument (papp _ _) (pglobal _) _) ()
ptree-replays-sound delta
  (ptree-apply-argument (papp _ _) (ppi _ _) _) ()
ptree-replays-sound delta
  (ptree-apply-argument (papp _ _) (plam _ _) _) ()
ptree-replays-sound delta
  (ptree-apply-argument (papp _ _) punit-type _) ()
ptree-replays-sound delta
  (ptree-apply-argument (papp _ _) punit _) ()
ptree-replays-sound delta (ptree-apply-argument punit-type _ _) ()
ptree-replays-sound delta (ptree-apply-argument punit _ _) ()

-- Trace chains: consecutive decoded steps replay from the recorded
-- start to the recorded end.
chain-replays :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero)) →
  {locals : Nat} → PTm globals locals →
  List (PStepTreeV1 globals locals) → PTm globals locals → Bool
chain-replays delta expected [] end = ptm-equal expected end
chain-replays delta expected (tree ∷ trees) end =
  ptm-equal (ptree-source tree) expected and
  (ptree-replays delta tree and
   chain-replays delta (ptree-target tree) trees end)

chain-replays-sound :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero)) →
  {locals : Nat} (start : PTm globals locals)
  (trees : List (PStepTreeV1 globals locals))
  (end : PTm globals locals) →
  chain-replays delta start trees end ≡ true →
  PStepsV1 delta start end
chain-replays-sound delta start [] end proof
  rewrite ptm-equal-sound start end proof = psteps-refl end
chain-replays-sound delta start (tree ∷ trees) end proof
  rewrite sym
    (ptm-equal-sound (ptree-source tree) start
      (and-first (ptm-equal (ptree-source tree) start) _ proof))
  = psteps-next
      (ptree-replays-sound delta tree
        (and-first (ptree-replays delta tree) _
          (and-second (ptm-equal (ptree-source tree) start) _ proof)))
      (chain-replays-sound delta (ptree-target tree) trees end
        (and-second (ptree-replays delta tree) _
          (and-second (ptm-equal (ptree-source tree) start) _ proof)))

-- Semantic normality: no beta redex anywhere and no enabled global
-- anywhere. Accepted normal forms provably admit no outgoing intrinsic
-- step under the policy-restricted delta.
is-plam : {globals locals : Nat} → PTm globals locals → Bool
is-plam (plam _ _) = true
is-plam _ = false

pnormal :
  (test : Nat → Bool) →
  {globals locals : Nat} → PTm globals locals → Bool
pnormal test (pvar x) = true
pnormal test (psort level) = true
pnormal test (pglobal slot) = not (test (fin-ordinal slot))
pnormal test (ppi parameter body) =
  pnormal test parameter and pnormal test body
pnormal test (plam parameter body) =
  pnormal test parameter and pnormal test body
pnormal test (papp function argument) =
  not (is-plam function) and
  (pnormal test function and pnormal test argument)
pnormal test punit-type = true
pnormal test punit = true

false-true-eliminate : {A : Set} → false ≡ true → A
false-true-eliminate ()

policy-body-member :
  {globals : Nat} (enabled : List Nat) (sig : PSig globals)
  (slot : Fin globals) {body : PTm globals zero} →
  policy-body enabled sig slot ≡ wire-just body →
  nat-member (fin-ordinal slot) enabled ≡ true
policy-body-member enabled sig slot proof
  with nat-member (fin-ordinal slot) enabled
... | true = refl
... | false = wire-nothing-not-just proof
  where
  wire-nothing-not-just :
    {A B : Set} {value : A} →
    wire-nothing ≡ wire-just value → B
  wire-nothing-not-just ()

pnormal-no-step :
  {globals : Nat} (enabled : List Nat) (sig : PSig globals)
  {locals : Nat} {term target : PTm globals locals} →
  PStepV1 (policy-body enabled sig) term target →
  pnormal (λ slot → nat-member slot enabled) term ≡ true →
  Empty
pnormal-no-step enabled sig pstep-beta ()
pnormal-no-step enabled sig (pstep-delta {slot = slot} body-equation)
  normal
  rewrite policy-body-member enabled sig slot body-equation =
  false-true-eliminate normal
pnormal-no-step enabled sig
  (pstep-pi-parameter {left = left} {body = body} premise) normal =
  pnormal-no-step enabled sig premise
    (and-first (pnormal _ left) (pnormal _ body) normal)
pnormal-no-step enabled sig
  (pstep-pi-body {parameter = parameter} {left = left} premise) normal =
  pnormal-no-step enabled sig premise
    (and-second (pnormal _ parameter) (pnormal _ left) normal)
pnormal-no-step enabled sig
  (pstep-lambda-parameter {left = left} {body = body} premise) normal =
  pnormal-no-step enabled sig premise
    (and-first (pnormal _ left) (pnormal _ body) normal)
pnormal-no-step enabled sig
  (pstep-lambda-body {parameter = parameter} {left = left} premise)
  normal =
  pnormal-no-step enabled sig premise
    (and-second (pnormal _ parameter) (pnormal _ left) normal)
pnormal-no-step enabled sig
  (pstep-apply-function {left = left} {argument = argument} premise)
  normal =
  pnormal-no-step enabled sig premise
    (and-first (pnormal _ left) (pnormal _ argument)
      (and-second (not (is-plam left)) _ normal))
pnormal-no-step enabled sig
  (pstep-apply-argument {function = function} {left = left} premise)
  normal =
  pnormal-no-step enabled sig premise
    (and-second (pnormal _ function) (pnormal _ left)
      (and-second (not (is-plam function)) _ normal))

-- Bridge from the structural census recomputation: a term whose erasure
-- admits a complete no-redex census is semantically normal.
is-wire-just : {A : Set} → WireMaybe A → Bool
is-wire-just wire-nothing = false
is-wire-just (wire-just _) = true

erasure-is-lambda :
  {globals locals : Nat} (term : PTm globals locals) →
  is-lambda-term (ptm-to-wire term) ≡ is-plam term
erasure-is-lambda (pvar _) = refl
erasure-is-lambda (psort _) = refl
erasure-is-lambda (pglobal _) = refl
erasure-is-lambda (ppi _ _) = refl
erasure-is-lambda (plam _ _) = refl
erasure-is-lambda (papp _ _) = refl
erasure-is-lambda punit-type = refl
erasure-is-lambda punit = refl

census-normal :
  (enabled : List Nat)
  (path : List ConversionPathComponentWireV1)
  {globals locals : Nat} (term : PTm globals locals) →
  is-wire-just (expected-census-from enabled path (ptm-to-wire term))
  ≡ true →
  pnormal (λ slot → nat-member slot enabled) term ≡ true
census-normal enabled path (pvar x) proof = refl
census-normal enabled path (psort level) proof = refl
census-normal enabled path (pglobal slot) proof
  with nat-member (fin-ordinal slot) enabled
... | false = refl
... | true = false-true-eliminate proof
census-normal enabled path {globals} {locals} (ppi parameter body) proof
  with expected-census-from enabled (path ++ (path-pi-parameter ∷ []))
         (ptm-to-wire parameter)
    in parameter-census
... | wire-nothing = false-true-eliminate proof
... | wire-just parameter-entries
  with expected-census-from enabled (path ++ (path-pi-body ∷ []))
         (ptm-to-wire body)
    in body-census
... | wire-nothing = false-true-eliminate proof
... | wire-just body-entries =
  and-intro (pnormal _ parameter) (pnormal _ body)
    (census-normal enabled (path ++ (path-pi-parameter ∷ []))
      parameter (transport-is-just parameter-census))
    (census-normal enabled (path ++ (path-pi-body ∷ []))
      body (transport-is-just body-census))
  where
  transport-is-just :
    {A : Set} {value : WireMaybe A} {result : A} →
    value ≡ wire-just result → is-wire-just value ≡ true
  transport-is-just refl = refl
census-normal enabled path {globals} {locals} (plam parameter body)
  proof
  with expected-census-from enabled
         (path ++ (path-lambda-parameter ∷ []))
         (ptm-to-wire parameter)
    in parameter-census
... | wire-nothing = false-true-eliminate proof
... | wire-just parameter-entries
  with expected-census-from enabled (path ++ (path-lambda-body ∷ []))
         (ptm-to-wire body)
    in body-census
... | wire-nothing = false-true-eliminate proof
... | wire-just body-entries =
  and-intro (pnormal _ parameter) (pnormal _ body)
    (census-normal enabled (path ++ (path-lambda-parameter ∷ []))
      parameter (transport-is-just parameter-census))
    (census-normal enabled (path ++ (path-lambda-body ∷ []))
      body (transport-is-just body-census))
  where
  transport-is-just :
    {A : Set} {value : WireMaybe A} {result : A} →
    value ≡ wire-just result → is-wire-just value ≡ true
  transport-is-just refl = refl
census-normal enabled path {globals} {locals} (papp function argument)
  proof
  with expected-census-from enabled
         (path ++ (path-apply-function ∷ []))
         (ptm-to-wire function)
    in function-census
... | wire-nothing = false-true-eliminate proof
... | wire-just function-entries
  with expected-census-from enabled (path ++ (path-apply-argument ∷ []))
         (ptm-to-wire argument)
    in argument-census
... | wire-nothing = false-true-eliminate proof
... | wire-just argument-entries
  with is-lambda-term (ptm-to-wire function) in lambda-equation
... | true = false-true-eliminate proof
... | false =
  and-intro (not (is-plam function))
    (pnormal _ function and pnormal _ argument)
    (cong not
      (trans (sym (erasure-is-lambda function)) lambda-equation))
    (and-intro (pnormal _ function) (pnormal _ argument)
      (census-normal enabled (path ++ (path-apply-function ∷ []))
        function (transport-is-just function-census))
      (census-normal enabled (path ++ (path-apply-argument ∷ []))
        argument (transport-is-just argument-census)))
  where
  transport-is-just :
    {A : Set} {value : WireMaybe A} {result : A} →
    value ≡ wire-just result → is-wire-just value ≡ true
  transport-is-just refl = refl
census-normal enabled path punit-type proof = refl
census-normal enabled path punit proof = refl

-- Conversion-certificate semantic verdict.
decode-steps-checked :
  (globals locals : Nat) → List BaseQ0ReductionStepWireV1 →
  WireMaybe (List (PStepTreeV1 globals locals))
decode-steps-checked globals locals [] = wire-just []
decode-steps-checked globals locals (step ∷ steps) =
  wire-maybe-bind (decode-step-checked globals locals step) (λ tree →
  wire-maybe-bind (decode-steps-checked globals locals steps) (λ trees →
  wire-just (tree ∷ trees)))

replay-trace-core :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero)) →
  {locals : Nat} → PTm globals locals →
  WireMaybe (List (PStepTreeV1 globals locals)) →
  PTm globals locals → Bool
replay-trace-core delta start wire-nothing end = false
replay-trace-core delta start (wire-just trees) end =
  chain-replays delta start trees end

replay-conversion-endpoints :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero))
  (test : Nat → Bool) →
  {locals : Nat} →
  PTm globals locals → PTm globals locals → PTm globals locals →
  ConversionCertificateWireV1 → Bool
replay-conversion-endpoints delta test {locals} left right common
  certificate =
  replay-trace-core delta left
    (decode-steps-checked _ locals
      (trace-steps (conversion-left-trace certificate)))
    common and
  (replay-trace-core delta right
    (decode-steps-checked _ locals
      (trace-steps (conversion-right-trace certificate)))
    common and
   pnormal test common)

replay-conversion-decoded :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero))
  (test : Nat → Bool)
  (locals : Nat) →
  WireMaybe (PTm globals locals) → WireMaybe (PTm globals locals) →
  WireMaybe (PTm globals locals) →
  ConversionCertificateWireV1 → Bool
replay-conversion-decoded delta test locals
  (wire-just left) (wire-just right) (wire-just common) certificate =
  replay-conversion-endpoints delta test left right common certificate
replay-conversion-decoded delta test locals _ _ _ certificate = false

replay-conversion :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals) →
  ConversionCertificateWireV1 → Bool
replay-conversion globals enabled sig certificate =
  replay-conversion-decoded
    (policy-body enabled sig)
    (λ slot → nat-member slot enabled)
    (length (entries-oldest-first (conversion-context certificate)))
    (decode-term-checked globals _ (conversion-left certificate))
    (decode-term-checked globals _ (conversion-right certificate))
    (decode-term-checked globals _
      (conversion-common-normal-form certificate))
    certificate

-- Soundness: an accepted conversion yields genuine intrinsic reduction
-- chains from both decoded endpoints to the decoded common form, which
-- admits no outgoing step under the policy-restricted delta.
record ConversionReplayEvidenceV1
  {globals : Nat} (enabled : List Nat) (sig : PSig globals)
  {locals : Nat}
  (left right common : PTm globals locals) : Set where
  constructor conversion-replay-evidence
  field
    left-chain : PStepsV1 (policy-body enabled sig) left common
    right-chain : PStepsV1 (policy-body enabled sig) right common
    common-has-no-step :
      {target : PTm globals locals} →
      PStepV1 (policy-body enabled sig) common target → Empty

replay-conversion-trees-sound :
  {globals : Nat} (enabled : List Nat) (sig : PSig globals)
  {locals : Nat}
  (left right common : PTm globals locals)
  (left-trees right-trees :
    WireMaybe (List (PStepTreeV1 globals locals))) →
  (replay-trace-core (policy-body enabled sig) left left-trees common
   and
   (replay-trace-core (policy-body enabled sig) right right-trees
     common
    and
    pnormal (λ slot → nat-member slot enabled) common)) ≡ true →
  ConversionReplayEvidenceV1 enabled sig left right common
replay-conversion-trees-sound enabled sig left right common
  wire-nothing right-trees proof = false-true-eliminate proof
replay-conversion-trees-sound enabled sig left right common
  (wire-just left-trees) wire-nothing proof =
  false-true-eliminate
    (and-second
      (chain-replays (policy-body enabled sig) left left-trees common)
      false proof)
replay-conversion-trees-sound enabled sig left right common
  (wire-just left-trees) (wire-just right-trees) proof =
  conversion-replay-evidence
    (chain-replays-sound _ left left-trees common
      (and-first (chain-replays _ left left-trees common) _ proof))
    (chain-replays-sound _ right right-trees common
      (and-first (chain-replays _ right right-trees common) _
        (and-second (chain-replays _ left left-trees common) _ proof)))
    (λ step →
      pnormal-no-step enabled sig step
        (and-second (chain-replays _ right right-trees common) _
          (and-second (chain-replays _ left left-trees common) _
            proof)))

replay-conversion-endpoints-sound :
  {globals : Nat} (enabled : List Nat) (sig : PSig globals)
  {locals : Nat}
  (left right common : PTm globals locals)
  (certificate : ConversionCertificateWireV1) →
  replay-conversion-endpoints
    (policy-body enabled sig)
    (λ slot → nat-member slot enabled)
    left right common certificate ≡ true →
  ConversionReplayEvidenceV1 enabled sig left right common
replay-conversion-endpoints-sound enabled sig {locals}
  left right common certificate proof =
  replay-conversion-trees-sound enabled sig left right common
    (decode-steps-checked _ locals
      (trace-steps (conversion-left-trace certificate)))
    (decode-steps-checked _ locals
      (trace-steps (conversion-right-trace certificate)))
    proof
