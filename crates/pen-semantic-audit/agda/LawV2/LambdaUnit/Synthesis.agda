{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.Synthesis where

open import Agda.Builtin.Equality using (_≡_)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.LambdaUnit.TypingJudgment

infix 3 _⊢_∶_⇒_

data _⊢_∶_⇒_ {V : Set} (Σ : Signature) (Γ : Context V) :
  Tm V -> Tm V -> Set where

  synth-sort :
    (level : Nat) ->
    Σ ⊢ Γ ∶ sort level ⇒ sort (suc level)

  synth-unit-type :
    Σ ⊢ Γ ∶ unit-type ⇒ sort zero

  synth-unit :
    Σ ⊢ Γ ∶ unit ⇒ unit-type

  synth-variable :
    (x : V) ->
    Σ ⊢ Γ ∶ var x ⇒ Γ x

  synth-global :
    {identifier : Nat} {entry : GlobalDeclaration} ->
    lookup-global Σ identifier ≡ just entry ->
    Σ ⊢ empty-context
      ∶ closed (declared-type entry)
      ∶ sort (declared-universe entry) ->
    Σ ⊢ Γ
      ∶ global identifier
      ⇒ closed (declared-type entry)

  synth-pi :
    {parameter : Tm V} {body : Tm (Lift V)}
    {parameter-level body-level : Nat} ->
    Σ ⊢ Γ ∶ parameter ⇒ sort parameter-level ->
    Σ ⊢ extend-context Γ parameter ∶ body ⇒ sort body-level ->
    Σ ⊢ Γ
      ∶ pi parameter body
      ⇒ sort (parameter-level max body-level)

  synth-lambda :
    {parameter : Tm V} {body body-type : Tm (Lift V)}
    {parameter-level : Nat} ->
    Σ ⊢ Γ ∶ parameter ⇒ sort parameter-level ->
    Σ ⊢ extend-context Γ parameter ∶ body ⇒ body-type ->
    Σ ⊢ Γ
      ∶ lam parameter body
      ⇒ pi parameter body-type

  synth-application :
    {function argument parameter : Tm V}
    {body : Tm (Lift V)} ->
    Σ ⊢ Γ ∶ function ⇒ pi parameter body ->
    Σ ⊢ Γ ∶ argument ⇒ parameter ->
    Σ ⊢ Γ
      ∶ app function argument
      ⇒ instantiate body argument

synthesis-soundness :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {term type : Tm V} ->
  Σ ⊢ Γ ∶ term ⇒ type ->
  Σ ⊢ Γ ∶ term ∶ type
synthesis-soundness (synth-sort level) =
  type-sort level
synthesis-soundness synth-unit-type =
  type-unit-type
synthesis-soundness synth-unit =
  type-unit
synthesis-soundness (synth-variable x) =
  type-variable x
synthesis-soundness (synth-global lookup formation) =
  type-global lookup formation
synthesis-soundness (synth-pi parameter body) =
  type-pi
    (synthesis-soundness parameter)
    (synthesis-soundness body)
synthesis-soundness (synth-lambda parameter body) =
  type-lambda
    (synthesis-soundness parameter)
    (synthesis-soundness body)
synthesis-soundness (synth-application function argument) =
  type-application
    (synthesis-soundness function)
    (synthesis-soundness argument)

synthesis-completeness :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {term type : Tm V} ->
  Σ ⊢ Γ ∶ term ∶ type ->
  Σ ⊢ Γ ∶ term ⇒ type
synthesis-completeness (type-sort level) =
  synth-sort level
synthesis-completeness type-unit-type =
  synth-unit-type
synthesis-completeness type-unit =
  synth-unit
synthesis-completeness (type-variable x) =
  synth-variable x
synthesis-completeness (type-global lookup formation) =
  synth-global lookup formation
synthesis-completeness (type-pi parameter body) =
  synth-pi
    (synthesis-completeness parameter)
    (synthesis-completeness body)
synthesis-completeness (type-lambda parameter body) =
  synth-lambda
    (synthesis-completeness parameter)
    (synthesis-completeness body)
synthesis-completeness (type-application function argument) =
  synth-application
    (synthesis-completeness function)
    (synthesis-completeness argument)
