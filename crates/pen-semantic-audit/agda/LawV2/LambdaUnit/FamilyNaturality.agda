{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.FamilyNaturality where

open import Agda.Builtin.Equality using (_≡_; refl)
open import LawV2.LambdaUnit.Substitution

data Family (V : Set) : Set where
  seed : Tm V -> Family V
  generic-public-application :
    Family V ->
    Family V ->
    Family V
  generic-equation-action :
    Family V ->
    Family V ->
    Family V

substitute-family :
  {V W : Set} ->
  (V -> Tm W) ->
  Family V ->
  Family W
substitute-family σ (seed term) =
  seed (substitute σ term)
substitute-family σ (generic-public-application function argument) =
  generic-public-application
    (substitute-family σ function)
    (substitute-family σ argument)
substitute-family σ (generic-equation-action equation one-hole-context) =
  generic-equation-action
    (substitute-family σ equation)
    (substitute-family σ one-hole-context)

generic-public-application-naturality :
  {V W : Set} ->
  (σ : V -> Tm W) ->
  (function argument : Family V) ->
  substitute-family σ
    (generic-public-application function argument)
  ≡
  generic-public-application
    (substitute-family σ function)
    (substitute-family σ argument)
generic-public-application-naturality σ function argument = refl

generic-equation-action-naturality :
  {V W : Set} ->
  (σ : V -> Tm W) ->
  (equation one-hole-context : Family V) ->
  substitute-family σ
    (generic-equation-action equation one-hole-context)
  ≡
  generic-equation-action
    (substitute-family σ equation)
    (substitute-family σ one-hole-context)
generic-equation-action-naturality σ equation one-hole-context = refl

family-identity :
  {V : Set} ->
  (family : Family V) ->
  substitute-family identity-substitution family ≡ family
family-identity (seed term) =
  cong seed (substitution-identity term)
family-identity (generic-public-application function argument) =
  cong₂ generic-public-application
    (family-identity function)
    (family-identity argument)
family-identity (generic-equation-action equation one-hole-context) =
  cong₂ generic-equation-action
    (family-identity equation)
    (family-identity one-hole-context)

family-composition :
  {U V W : Set} ->
  (σ : U -> Tm V) ->
  (τ : V -> Tm W) ->
  (family : Family U) ->
  substitute-family τ (substitute-family σ family)
  ≡
  substitute-family (σ then τ) family
family-composition σ τ (seed term) =
  cong seed (substitution-composition σ τ term)
family-composition σ τ (generic-public-application function argument) =
  cong₂ generic-public-application
    (family-composition σ τ function)
    (family-composition σ τ argument)
family-composition σ τ
  (generic-equation-action equation one-hole-context) =
  cong₂ generic-equation-action
    (family-composition σ τ equation)
    (family-composition σ τ one-hole-context)
