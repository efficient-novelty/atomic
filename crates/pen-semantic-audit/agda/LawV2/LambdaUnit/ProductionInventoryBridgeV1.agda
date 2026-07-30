{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.ProductionInventoryBridgeV1 where

open import Agda.Builtin.Equality using (_≡_; refl)
open import LawV2.LambdaUnit.FamilyNaturality
  using
    ( Family
    ; seed
    ; generic-public-application
    ; generic-equation-action
    ; substitute-family
    )
open import LawV2.LambdaUnit.SubstitutionReduction
  using
    ( Step
    ; beta-step
    ; public-delta-step
    ; fresh-equation-step
    ; pi-parameter-congruence-step
    ; pi-body-congruence-step
    ; lambda-parameter-congruence-step
    ; lambda-body-congruence-step
    ; apply-function-congruence-step
    ; apply-argument-congruence-step
    )
open import LawV2.LambdaUnit.Substitution using (Tm)

-- This is an internal, payload-free code inventory. The Rust-enum erasure is
-- deliberately left in ProductionInventoryBridgeFrontier below.
data AbstractQ0TagV1 : Set where
  q0-de-bruijn : AbstractQ0TagV1
  q0-sequential-substitution : AbstractQ0TagV1
  q0-beta : AbstractQ0TagV1
  q0-provenance-preserving-delta : AbstractQ0TagV1
  q0-unit : AbstractQ0TagV1
  q0-telescope-flattening : AbstractQ0TagV1
  q0-fresh-nonrecursive-constructor-computation : AbstractQ0TagV1

data ProductionQ0CategoryV1 : Set where
  representation : ProductionQ0CategoryV1
  base-semantic : ProductionQ0CategoryV1
  runtime-public : ProductionQ0CategoryV1

q0-category :
  AbstractQ0TagV1 ->
  ProductionQ0CategoryV1
q0-category q0-de-bruijn =
  representation
q0-category q0-sequential-substitution =
  representation
q0-category q0-beta =
  base-semantic
q0-category q0-provenance-preserving-delta =
  base-semantic
q0-category q0-unit =
  base-semantic
q0-category q0-telescope-flattening =
  representation
q0-category q0-fresh-nonrecursive-constructor-computation =
  runtime-public

data Q0ClassificationV1 :
  AbstractQ0TagV1 ->
  ProductionQ0CategoryV1 ->
  Set where

  classify-de-bruijn :
    Q0ClassificationV1
      q0-de-bruijn
      representation

  classify-sequential-substitution :
    Q0ClassificationV1
      q0-sequential-substitution
      representation

  classify-beta :
    Q0ClassificationV1
      q0-beta
      base-semantic

  classify-provenance-preserving-delta :
    Q0ClassificationV1
      q0-provenance-preserving-delta
      base-semantic

  classify-unit :
    Q0ClassificationV1
      q0-unit
      base-semantic

  classify-telescope-flattening :
    Q0ClassificationV1
      q0-telescope-flattening
      representation

  classify-fresh-nonrecursive-constructor-computation :
    Q0ClassificationV1
      q0-fresh-nonrecursive-constructor-computation
      runtime-public

q0-classification-total :
  (tag : AbstractQ0TagV1) ->
  Q0ClassificationV1 tag (q0-category tag)
q0-classification-total q0-de-bruijn =
  classify-de-bruijn
q0-classification-total q0-sequential-substitution =
  classify-sequential-substitution
q0-classification-total q0-beta =
  classify-beta
q0-classification-total q0-provenance-preserving-delta =
  classify-provenance-preserving-delta
q0-classification-total q0-unit =
  classify-unit
q0-classification-total q0-telescope-flattening =
  classify-telescope-flattening
q0-classification-total
  q0-fresh-nonrecursive-constructor-computation =
  classify-fresh-nonrecursive-constructor-computation

q0-classification-unique :
  {tag : AbstractQ0TagV1}
  {category : ProductionQ0CategoryV1} ->
  Q0ClassificationV1 tag category ->
  category ≡ q0-category tag
q0-classification-unique classify-de-bruijn =
  refl
q0-classification-unique classify-sequential-substitution =
  refl
q0-classification-unique classify-beta =
  refl
q0-classification-unique
  classify-provenance-preserving-delta =
  refl
q0-classification-unique classify-unit =
  refl
q0-classification-unique classify-telescope-flattening =
  refl
q0-classification-unique
  classify-fresh-nonrecursive-constructor-computation =
  refl

data ProductionFamilyCodeV1 : Set where
  family-seed : ProductionFamilyCodeV1
  family-generic-public-application : ProductionFamilyCodeV1
  family-generic-equation-action : ProductionFamilyCodeV1

family-code :
  {V : Set} ->
  Family V ->
  ProductionFamilyCodeV1
family-code (seed term) =
  family-seed
family-code (generic-public-application function argument) =
  family-generic-public-application
family-code
  (generic-equation-action equation one-hole-context) =
  family-generic-equation-action

data FamilyClassificationV1 {V : Set} :
  Family V ->
  ProductionFamilyCodeV1 ->
  Set where

  classify-seed :
    (term : Tm V) ->
    FamilyClassificationV1
      (seed term)
      family-seed

  classify-generic-public-application :
    (function argument : Family V) ->
    FamilyClassificationV1
      (generic-public-application function argument)
      family-generic-public-application

  classify-generic-equation-action :
    (equation one-hole-context : Family V) ->
    FamilyClassificationV1
      (generic-equation-action equation one-hole-context)
      family-generic-equation-action

family-classification-total :
  {V : Set} ->
  (family : Family V) ->
  FamilyClassificationV1 family (family-code family)
family-classification-total (seed term) =
  classify-seed term
family-classification-total
  (generic-public-application function argument) =
  classify-generic-public-application function argument
family-classification-total
  (generic-equation-action equation one-hole-context) =
  classify-generic-equation-action equation one-hole-context

family-classification-unique :
  {V : Set}
  {family : Family V}
  {code : ProductionFamilyCodeV1} ->
  FamilyClassificationV1 family code ->
  code ≡ family-code family
family-classification-unique (classify-seed term) =
  refl
family-classification-unique
  (classify-generic-public-application function argument) =
  refl
family-classification-unique
  (classify-generic-equation-action equation one-hole-context) =
  refl

family-code-substitution :
  {V W : Set} ->
  (substitution : V -> Tm W) ->
  (family : Family V) ->
  family-code (substitute-family substitution family)
  ≡
  family-code family
family-code-substitution substitution (seed term) =
  refl
family-code-substitution substitution
  (generic-public-application function argument) =
  refl
family-code-substitution substitution
  (generic-equation-action equation one-hole-context) =
  refl

-- The abstract Step relation supplies the three rewrite classifications
-- plus positioned congruence framing. A congruence step performs exactly
-- the rewrite its premise performs, so its Q0 tag is the premise's tag.
-- This does not make fresh equations part of the V2 base conversion
-- trace, and an abstract delta step contains no predecessor-policy
-- authorization.
abstract-step-q0-tag :
  {V : Set}
  {left right : Tm V} ->
  Step {V} left right ->
  AbstractQ0TagV1
abstract-step-q0-tag
  (beta-step parameter-type body argument) =
  q0-beta
abstract-step-q0-tag
  (public-delta-step identifier body) =
  q0-provenance-preserving-delta
abstract-step-q0-tag
  (fresh-equation-step left right match) =
  q0-fresh-nonrecursive-constructor-computation
abstract-step-q0-tag
  (pi-parameter-congruence-step body premise) =
  abstract-step-q0-tag premise
abstract-step-q0-tag
  (pi-body-congruence-step parameter premise) =
  abstract-step-q0-tag premise
abstract-step-q0-tag
  (lambda-parameter-congruence-step body premise) =
  abstract-step-q0-tag premise
abstract-step-q0-tag
  (lambda-body-congruence-step parameter premise) =
  abstract-step-q0-tag premise
abstract-step-q0-tag
  (apply-function-congruence-step argument premise) =
  abstract-step-q0-tag premise
abstract-step-q0-tag
  (apply-argument-congruence-step function premise) =
  abstract-step-q0-tag premise

abstract-step-classification :
  {V : Set}
  {left right : Tm V} ->
  (step : Step {V} left right) ->
  Q0ClassificationV1
    (abstract-step-q0-tag step)
    (q0-category (abstract-step-q0-tag step))
abstract-step-classification step =
  q0-classification-total (abstract-step-q0-tag step)

-- These constructors name missing correspondence theorems. They are frontier
-- markers, not axioms and not production capabilities.
data ProductionInventoryBridgeFrontierV1 : Set where
  rust-q0-tag-erasure-not-mechanized :
    ProductionInventoryBridgeFrontierV1
  rust-family-payload-erasure-not-mechanized :
    ProductionInventoryBridgeFrontierV1
  declaration-order-global-slot-erasure-not-mechanized :
    ProductionInventoryBridgeFrontierV1
  base-conversion-certificate-correspondence-not-mechanized :
    ProductionInventoryBridgeFrontierV1
  carrier-derived-subject-completeness-not-mechanized :
    ProductionInventoryBridgeFrontierV1
