{-# OPTIONS --safe --without-K #-}

-- Phase E bounded-normalization bridge.
--
-- The kernel records kernel-normalized terms (`normalize` in
-- `pen-kernel` `checker.rs`): globals with stored bodies unfold and
-- re-normalize, beta redexes contract after their components normalize,
-- and every other constructor normalizes componentwise. This module
-- mirrors that algorithm on the intrinsic production syntax as a
-- proof-carrying bounded normalizer:
--
-- - `pnormalize-cert` is fuel-indexed and total; when it returns a
--   result, the result carries a genuine `PStepsV1` reduction chain
--   from the input to the normal form and a `pdelta-normal` witness
--   that the normal form admits no delta unfolding, no beta redex, and
--   hence (by `pdelta-normal-no-step`) no outgoing intrinsic step;
-- - the delta map is a parameter, exactly as in `PStepV1`. The kernel
--   normalizer unfolds every bodyful declaration, so kernel-mirroring
--   callers pass `lookup-psig-body sig` (all stored bodies), while
--   policy-restricted callers may pass `policy-body enabled sig`;
-- - fuel is a per-call tree budget (each recursive call receives one
--   less), a documented deviation from the kernel's single shared
--   operation/rewrite budget: both sides fail closed on exhaustion,
--   but the exhaustion boundaries are not byte-identical. Exhaustion
--   returns `wire-nothing` and is never a negative theorem.
--
-- Nothing here mints authority.

module LawV2.Wire.NormalizationV1 where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe
  using ()
  renaming (Maybe to WireMaybe; just to wire-just; nothing to wire-nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.Wire.Bytes using (_and_)
open import LawV2.Wire.ContextChecker using (not)
open import LawV2.LambdaUnit.ProductionSyntaxV1
open import LawV2.LambdaUnit.ProductionDecodingV1
open import LawV2.Wire.ContextCorrespondenceV1
  using (and-first; and-second; and-intro; pinstantiate;
         lookup-psig-body; PSig)
open import LawV2.Wire.SemanticReplayV1

-- The kernel delta map: every declaration with a stored body unfolds to
-- exactly that strict-prior body, mirroring `normalize`'s `Global` case.
psig-delta :
  {globals : Nat} → PSig globals →
  Fin globals → WireMaybe (PTm globals zero)
psig-delta sig slot = lookup-psig-body sig slot

-- Delta-relative normality: no enabled global anywhere, no beta redex
-- anywhere. This is `pnormal` with the test taken from the delta map
-- itself, so it applies verbatim to both the kernel map and the
-- policy-restricted map.
pdelta-normal :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero)) →
  {locals : Nat} → PTm globals locals → Bool
pdelta-normal delta (pvar x) = true
pdelta-normal delta (psort level) = true
pdelta-normal delta (pglobal slot) = not (is-wire-just (delta slot))
pdelta-normal delta (ppi parameter body) =
  pdelta-normal delta parameter and pdelta-normal delta body
pdelta-normal delta (plam parameter body) =
  pdelta-normal delta parameter and pdelta-normal delta body
pdelta-normal delta (papp function argument) =
  not (is-plam function) and
  (pdelta-normal delta function and pdelta-normal delta argument)
pdelta-normal delta punit-type = true
pdelta-normal delta punit = true

-- A delta-normal term admits no outgoing intrinsic step at all.
pdelta-normal-no-step :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero)) →
  {locals : Nat} {term target : PTm globals locals} →
  PStepV1 delta term target →
  pdelta-normal delta term ≡ true →
  Empty
pdelta-normal-no-step delta pstep-beta ()
pdelta-normal-no-step delta (pstep-delta {slot = slot} body-equation)
  normal rewrite body-equation = false-true-eliminate normal
pdelta-normal-no-step delta
  (pstep-pi-parameter {left = left} {body = body} premise) normal =
  pdelta-normal-no-step delta premise
    (and-first (pdelta-normal delta left) (pdelta-normal delta body)
      normal)
pdelta-normal-no-step delta
  (pstep-pi-body {parameter = parameter} {left = left} premise) normal =
  pdelta-normal-no-step delta premise
    (and-second (pdelta-normal delta parameter)
      (pdelta-normal delta left) normal)
pdelta-normal-no-step delta
  (pstep-lambda-parameter {left = left} {body = body} premise) normal =
  pdelta-normal-no-step delta premise
    (and-first (pdelta-normal delta left) (pdelta-normal delta body)
      normal)
pdelta-normal-no-step delta
  (pstep-lambda-body {parameter = parameter} {left = left} premise)
  normal =
  pdelta-normal-no-step delta premise
    (and-second (pdelta-normal delta parameter)
      (pdelta-normal delta left) normal)
pdelta-normal-no-step delta
  (pstep-apply-function {left = left} {argument = argument} premise)
  normal =
  pdelta-normal-no-step delta premise
    (and-first (pdelta-normal delta left)
      (pdelta-normal delta argument)
      (and-second (not (is-plam left)) _ normal))
pdelta-normal-no-step delta
  (pstep-apply-argument {function = function} {left = left} premise)
  normal =
  pdelta-normal-no-step delta premise
    (and-second (pdelta-normal delta function)
      (pdelta-normal delta left)
      (and-second (not (is-plam function)) _ normal))

-- Reflexive-transitive chains compose and lift through every congruence
-- frame.
psteps-append :
  {globals : Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {left middle right : PTm globals locals} →
  PStepsV1 delta left middle → PStepsV1 delta middle right →
  PStepsV1 delta left right
psteps-append (psteps-refl _) rest = rest
psteps-append (psteps-next step chain) rest =
  psteps-next step (psteps-append chain rest)

psteps-pi-parameter :
  {globals : Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {left right : PTm globals locals}
  (body : PTm globals (suc locals)) →
  PStepsV1 delta left right →
  PStepsV1 delta (ppi left body) (ppi right body)
psteps-pi-parameter body (psteps-refl term) = psteps-refl (ppi term body)
psteps-pi-parameter body (psteps-next step chain) =
  psteps-next (pstep-pi-parameter step) (psteps-pi-parameter body chain)

psteps-pi-body :
  {globals : Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} (parameter : PTm globals locals)
  {left right : PTm globals (suc locals)} →
  PStepsV1 delta left right →
  PStepsV1 delta (ppi parameter left) (ppi parameter right)
psteps-pi-body parameter (psteps-refl term) =
  psteps-refl (ppi parameter term)
psteps-pi-body parameter (psteps-next step chain) =
  psteps-next (pstep-pi-body step) (psteps-pi-body parameter chain)

psteps-lambda-parameter :
  {globals : Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {left right : PTm globals locals}
  (body : PTm globals (suc locals)) →
  PStepsV1 delta left right →
  PStepsV1 delta (plam left body) (plam right body)
psteps-lambda-parameter body (psteps-refl term) =
  psteps-refl (plam term body)
psteps-lambda-parameter body (psteps-next step chain) =
  psteps-next (pstep-lambda-parameter step)
    (psteps-lambda-parameter body chain)

psteps-lambda-body :
  {globals : Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} (parameter : PTm globals locals)
  {left right : PTm globals (suc locals)} →
  PStepsV1 delta left right →
  PStepsV1 delta (plam parameter left) (plam parameter right)
psteps-lambda-body parameter (psteps-refl term) =
  psteps-refl (plam parameter term)
psteps-lambda-body parameter (psteps-next step chain) =
  psteps-next (pstep-lambda-body step)
    (psteps-lambda-body parameter chain)

psteps-apply-function :
  {globals : Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {left right : PTm globals locals}
  (argument : PTm globals locals) →
  PStepsV1 delta left right →
  PStepsV1 delta (papp left argument) (papp right argument)
psteps-apply-function argument (psteps-refl term) =
  psteps-refl (papp term argument)
psteps-apply-function argument (psteps-next step chain) =
  psteps-next (pstep-apply-function step)
    (psteps-apply-function argument chain)

psteps-apply-argument :
  {globals : Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} (function : PTm globals locals)
  {left right : PTm globals locals} →
  PStepsV1 delta left right →
  PStepsV1 delta (papp function left) (papp function right)
psteps-apply-argument function (psteps-refl term) =
  psteps-refl (papp function term)
psteps-apply-argument function (psteps-next step chain) =
  psteps-next (pstep-apply-argument step)
    (psteps-apply-argument function chain)

-- Proof-carrying normalization result: the normal form, a genuine
-- intrinsic reduction chain reaching it, and its delta-normality.
record PNormalizeResultV1
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero))
  {locals : Nat}
  (term : PTm globals locals) : Set where
  constructor pnormalize-result
  field
    normal-form : PTm globals locals
    reduction : PStepsV1 delta term normal-form
    normality : pdelta-normal delta normal-form ≡ true

open PNormalizeResultV1 public

-- Default tree fuel for kernel-mirroring callers. The kernel bounds
-- work by one shared operation/depth/rewrite budget; this constant
-- bounds the recursion tree depth instead. Both exhaust fail-closed.
normalization-fuel-v1 : Nat
normalization-fuel-v1 = 4096

private
  global-normality-from :
    {globals locals : Nat}
    (delta : Fin globals → WireMaybe (PTm globals zero))
    (slot : Fin globals) →
    delta slot ≡ wire-nothing →
    pdelta-normal delta {locals} (pglobal slot) ≡ true
  global-normality-from delta slot equation rewrite equation = refl

  global-branch :
    {globals locals : Nat}
    (delta : Fin globals → WireMaybe (PTm globals zero))
    (fuel : Nat) (slot : Fin globals)
    (looked : WireMaybe (PTm globals zero)) →
    delta slot ≡ looked →
    ((body : PTm globals zero) → delta slot ≡ wire-just body →
      WireMaybe (PNormalizeResultV1 delta {locals} (pglobal slot))) →
    WireMaybe (PNormalizeResultV1 delta {locals} (pglobal slot))
  global-branch {globals} {locals} delta fuel slot wire-nothing
    looked-equation continue =
    wire-just
      (pnormalize-result (pglobal slot) (psteps-refl (pglobal slot))
        (global-normality-from {globals} {locals} delta slot
          looked-equation))
  global-branch delta fuel slot (wire-just body) looked-equation
    continue = continue body looked-equation

-- The certified normalizer. The traversal order mirrors the kernel
-- exactly: components first, then the top-level beta or delta redex,
-- then re-normalization of the contractum or unfolded body.
pnormalize-cert :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero)) →
  Nat →
  {locals : Nat} (term : PTm globals locals) →
  WireMaybe (PNormalizeResultV1 delta term)

private
  apply-branch :
    {globals : Nat}
    (delta : Fin globals → WireMaybe (PTm globals zero))
    (fuel : Nat)
    {locals : Nat} (function argument : PTm globals locals) →
    PNormalizeResultV1 delta function →
    PNormalizeResultV1 delta argument →
    WireMaybe (PNormalizeResultV1 delta (papp function argument))

pnormalize-cert delta zero term = wire-nothing
pnormalize-cert delta (suc fuel) (pvar x) =
  wire-just (pnormalize-result (pvar x) (psteps-refl (pvar x)) refl)
pnormalize-cert delta (suc fuel) (psort level) =
  wire-just
    (pnormalize-result (psort level) (psteps-refl (psort level)) refl)
pnormalize-cert delta (suc fuel) (pglobal slot) =
  global-branch delta fuel slot (delta slot) refl
    (λ body looked-equation →
      wire-maybe-bind (pnormalize-cert delta fuel (pembed-closed body))
        (λ body-result →
          wire-just
            (pnormalize-result
              (normal-form body-result)
              (psteps-next (pstep-delta looked-equation)
                (reduction body-result))
              (normality body-result))))
pnormalize-cert delta (suc fuel) (ppi parameter body) =
  wire-maybe-bind (pnormalize-cert delta fuel parameter)
    (λ parameter-result →
  wire-maybe-bind (pnormalize-cert delta fuel body) (λ body-result →
  wire-just
    (pnormalize-result
      (ppi (normal-form parameter-result) (normal-form body-result))
      (psteps-append
        (psteps-pi-parameter body (reduction parameter-result))
        (psteps-pi-body (normal-form parameter-result)
          (reduction body-result)))
      (and-intro
        (pdelta-normal delta (normal-form parameter-result))
        (pdelta-normal delta (normal-form body-result))
        (normality parameter-result)
        (normality body-result)))))
pnormalize-cert delta (suc fuel) (plam parameter body) =
  wire-maybe-bind (pnormalize-cert delta fuel parameter)
    (λ parameter-result →
  wire-maybe-bind (pnormalize-cert delta fuel body) (λ body-result →
  wire-just
    (pnormalize-result
      (plam (normal-form parameter-result) (normal-form body-result))
      (psteps-append
        (psteps-lambda-parameter body (reduction parameter-result))
        (psteps-lambda-body (normal-form parameter-result)
          (reduction body-result)))
      (and-intro
        (pdelta-normal delta (normal-form parameter-result))
        (pdelta-normal delta (normal-form body-result))
        (normality parameter-result)
        (normality body-result)))))
pnormalize-cert delta (suc fuel) (papp function argument) =
  wire-maybe-bind (pnormalize-cert delta fuel function)
    (λ function-result →
  wire-maybe-bind (pnormalize-cert delta fuel argument)
    (λ argument-result →
  apply-branch delta fuel function argument
    function-result argument-result))
pnormalize-cert delta (suc fuel) punit-type =
  wire-just (pnormalize-result punit-type (psteps-refl punit-type) refl)
pnormalize-cert delta (suc fuel) punit =
  wire-just (pnormalize-result punit (psteps-refl punit) refl)

private
  neutral-apply-result :
    {globals : Nat}
    (delta : Fin globals → WireMaybe (PTm globals zero))
    {locals : Nat} {function argument : PTm globals locals}
    (function-result : PNormalizeResultV1 delta function)
    (argument-result : PNormalizeResultV1 delta argument) →
    not (is-plam (normal-form function-result)) ≡ true →
    WireMaybe (PNormalizeResultV1 delta (papp function argument))
  neutral-apply-result delta {function = function} {argument = argument}
    function-result argument-result head-not-lambda =
    wire-just
      (pnormalize-result
        (papp (normal-form function-result)
          (normal-form argument-result))
        (psteps-append
          (psteps-apply-function argument (reduction function-result))
          (psteps-apply-argument (normal-form function-result)
            (reduction argument-result)))
        (and-intro
          (not (is-plam (normal-form function-result)))
          (pdelta-normal delta (normal-form function-result) and
           pdelta-normal delta (normal-form argument-result))
          head-not-lambda
          (and-intro
            (pdelta-normal delta (normal-form function-result))
            (pdelta-normal delta (normal-form argument-result))
            (normality function-result)
            (normality argument-result))))

  apply-branch delta fuel function argument
    (pnormalize-result (plam parameter body) function-steps
      function-normal)
    argument-result =
    wire-maybe-bind
      (pnormalize-cert delta fuel
        (pinstantiate body (normal-form argument-result)))
      (λ contractum-result →
        wire-just
          (pnormalize-result
            (normal-form contractum-result)
            (psteps-append
              (psteps-apply-function argument function-steps)
              (psteps-append
                (psteps-apply-argument (plam parameter body)
                  (reduction argument-result))
                (psteps-next pstep-beta (reduction contractum-result))))
            (normality contractum-result)))
  apply-branch delta fuel function argument
    (pnormalize-result (pvar x) function-steps function-normal)
    argument-result =
    neutral-apply-result delta
      (pnormalize-result (pvar x) function-steps function-normal)
      argument-result refl
  apply-branch delta fuel function argument
    (pnormalize-result (psort level) function-steps function-normal)
    argument-result =
    neutral-apply-result delta
      (pnormalize-result (psort level) function-steps function-normal)
      argument-result refl
  apply-branch delta fuel function argument
    (pnormalize-result (pglobal slot) function-steps function-normal)
    argument-result =
    neutral-apply-result delta
      (pnormalize-result (pglobal slot) function-steps function-normal)
      argument-result refl
  apply-branch delta fuel function argument
    (pnormalize-result (ppi pi-parameter pi-body) function-steps
      function-normal)
    argument-result =
    neutral-apply-result delta
      (pnormalize-result (ppi pi-parameter pi-body) function-steps
        function-normal)
      argument-result refl
  apply-branch delta fuel function argument
    (pnormalize-result (papp inner-function inner-argument)
      function-steps function-normal)
    argument-result =
    neutral-apply-result delta
      (pnormalize-result (papp inner-function inner-argument)
        function-steps function-normal)
      argument-result refl
  apply-branch delta fuel function argument
    (pnormalize-result punit-type function-steps function-normal)
    argument-result =
    neutral-apply-result delta
      (pnormalize-result punit-type function-steps function-normal)
      argument-result refl
  apply-branch delta fuel function argument
    (pnormalize-result punit function-steps function-normal)
    argument-result =
    neutral-apply-result delta
      (pnormalize-result punit function-steps function-normal)
      argument-result refl

-- Proof-erased normal form, for Boolean comparisons.
pnormalize :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero)) →
  Nat →
  {locals : Nat} → PTm globals locals →
  WireMaybe (PTm globals locals)
pnormalize delta fuel term =
  wire-maybe-bind (pnormalize-cert delta fuel term)
    (λ result → wire-just (normal-form result))

-- The two evidence fields, restated as standalone theorems.
pnormalize-sound :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero))
  (fuel : Nat)
  {locals : Nat} (term : PTm globals locals)
  (result : PNormalizeResultV1 delta term) →
  pnormalize-cert delta fuel term ≡ wire-just result →
  PStepsV1 delta term (normal-form result)
pnormalize-sound delta fuel term result equation = reduction result

pnormalize-normal :
  {globals : Nat}
  (delta : Fin globals → WireMaybe (PTm globals zero))
  (fuel : Nat)
  {locals : Nat} (term : PTm globals locals)
  (result : PNormalizeResultV1 delta term) →
  pnormalize-cert delta fuel term ≡ wire-just result →
  pdelta-normal delta (normal-form result) ≡ true
pnormalize-normal delta fuel term result equation = normality result
