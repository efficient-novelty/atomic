{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.TypingJudgment where

open import Agda.Builtin.Equality using (_≡_)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.LambdaUnit.TypingSyntax public

infix 3 _⊢_∶_∶_

data _⊢_∶_∶_ {V : Set} (Σ : Signature) (Γ : Context V) :
  Tm V -> Tm V -> Set where

  type-sort :
    (level : Nat) ->
    Σ ⊢ Γ ∶ sort level ∶ sort (suc level)

  type-unit-type :
    Σ ⊢ Γ ∶ unit-type ∶ sort zero

  type-unit :
    Σ ⊢ Γ ∶ unit ∶ unit-type

  type-variable :
    (x : V) ->
    Σ ⊢ Γ ∶ var x ∶ Γ x

  type-global :
    {identifier : Nat} {entry : GlobalDeclaration} ->
    lookup-global Σ identifier ≡ just entry ->
    Σ ⊢ empty-context
      ∶ closed (declared-type entry)
      ∶ sort (declared-universe entry) ->
    Σ ⊢ Γ
      ∶ global identifier
      ∶ closed (declared-type entry)

  type-pi :
    {parameter : Tm V} {body : Tm (Lift V)}
    {parameter-level body-level : Nat} ->
    Σ ⊢ Γ ∶ parameter ∶ sort parameter-level ->
    Σ ⊢ extend-context Γ parameter ∶ body ∶ sort body-level ->
    Σ ⊢ Γ
      ∶ pi parameter body
      ∶ sort (parameter-level max body-level)

  type-lambda :
    {parameter : Tm V} {body body-type : Tm (Lift V)}
    {parameter-level : Nat} ->
    Σ ⊢ Γ ∶ parameter ∶ sort parameter-level ->
    Σ ⊢ extend-context Γ parameter ∶ body ∶ body-type ->
    Σ ⊢ Γ
      ∶ lam parameter body
      ∶ pi parameter body-type

  type-application :
    {function argument parameter : Tm V}
    {body : Tm (Lift V)} ->
    Σ ⊢ Γ ∶ function ∶ pi parameter body ->
    Σ ⊢ Γ ∶ argument ∶ parameter ->
    Σ ⊢ Γ
      ∶ app function argument
      ∶ instantiate body argument

record TypeFormation
  {V : Set}
  (Σ : Signature)
  (Γ : Context V)
  (type : Tm V) : Set where
  constructor type-formation
  field
    universe : Nat
    formation : Σ ⊢ Γ ∶ type ∶ sort universe

open TypeFormation public

WellFormedSignature : Signature -> Set
WellFormedSignature Σ =
  (identifier : Nat) ->
  (entry : GlobalDeclaration) ->
  lookup-global Σ identifier ≡ just entry ->
  Σ ⊢ empty-context
    ∶ closed (declared-type entry)
    ∶ sort (declared-universe entry)

WellFormedContext :
  {V : Set} ->
  Signature ->
  Context V ->
  Set
WellFormedContext {V} Σ Γ =
  (x : V) -> TypeFormation Σ Γ (Γ x)
