{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.ProductionConversionTyping where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat)
open import LawV2.LambdaUnit.TypingJudgment
open import LawV2.LambdaUnit.ReductionTyping
open import LawV2.LambdaUnit.SubstitutionTypingV3
open import LawV2.LambdaUnit.SubstitutionReduction
  using (beta-contractum-naturality)
open import LawV2.LambdaUnit.Substitution
  using (sym)

-- A typed multi-step trace retains the typing derivation at every edge.
-- Conversion certificates will decode to this relation; a bare equality of
-- endpoints is not enough to construct it.
data TypedSteps
  {V : Set}
  (Σ : Signature)
  (Γ : Context V)
  (type : Tm V) :
  Tm V -> Tm V -> Set₁ where

  typed-steps-refl :
    {term : Tm V} ->
    Σ ⊢ Γ ∶ term ∶ type ->
    TypedSteps Σ Γ type term term

  typed-steps-next :
    {left middle right : Tm V} ->
    TypedReduction Σ Γ left middle type ->
    TypedSteps Σ Γ type middle right ->
    TypedSteps Σ Γ type left right

typed-steps-left-typing :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {type left right : Tm V} ->
  TypedSteps Σ Γ type left right ->
  Σ ⊢ Γ ∶ left ∶ type
typed-steps-left-typing (typed-steps-refl typing) = typing
typed-steps-left-typing (typed-steps-next reduction rest) =
  left-typing reduction

typed-steps-right-typing :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {type left right : Tm V} ->
  TypedSteps Σ Γ type left right ->
  Σ ⊢ Γ ∶ right ∶ type
typed-steps-right-typing (typed-steps-refl typing) = typing
typed-steps-right-typing (typed-steps-next reduction rest) =
  typed-steps-right-typing rest

-- Production conversion is represented by two typed traces to one common
-- endpoint. Normality is checked by the finite production certificate
-- verifier; typing soundness needs only these proof-carrying traces.
record BaseQ0Equivalent
  {V : Set}
  (Σ : Signature)
  (Γ : Context V)
  (left right : Tm V) : Set₁ where

  constructor base-q0-equivalent

  field
    universe : Nat
    common : Tm V
    left-trace :
      TypedSteps Σ Γ (sort universe) left common
    right-trace :
      TypedSteps Σ Γ (sort universe) right common

open BaseQ0Equivalent public

infix 3 _⊢c_∶_∶_

data _⊢c_∶_∶_
  {V : Set}
  (Σ : Signature)
  (Γ : Context V) :
  Tm V -> Tm V -> Set₁ where

  conversion-exact :
    {term type : Tm V} ->
    Σ ⊢ Γ ∶ term ∶ type ->
    Σ ⊢c Γ ∶ term ∶ type

  conversion-convert :
    {term left right : Tm V} ->
    Σ ⊢c Γ ∶ term ∶ left ->
    BaseQ0Equivalent Σ Γ left right ->
    Σ ⊢c Γ ∶ term ∶ right

  conversion-application :
    {function argument parameter : Tm V}
    {body : Tm (Lift V)} ->
    Σ ⊢c Γ
      ∶ function
      ∶ pi parameter body ->
    Σ ⊢c Γ
      ∶ argument
      ∶ parameter ->
    Σ ⊢c Γ
      ∶ app function argument
      ∶ instantiate body argument

common-normal-form-induces-conversion :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {term left right : Tm V} ->
  Σ ⊢c Γ ∶ term ∶ left ->
  BaseQ0Equivalent Σ Γ left right ->
  Σ ⊢c Γ ∶ term ∶ right
common-normal-form-induces-conversion =
  conversion-convert

transport-conversion-type :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {term left right : Tm V} ->
  left ≡ right ->
  Σ ⊢c Γ ∶ term ∶ left ->
  Σ ⊢c Γ ∶ term ∶ right
transport-conversion-type refl derivation = derivation

substitution-preserves-typed-steps :
  {V W : Set} {Σ : Signature}
  {source : Context V} {target : Context W}
  {substitution : V -> Tm W}
  {type left right : Tm V} ->
  TypedSubstitution Σ source target substitution ->
  TypedSteps Σ source type left right ->
  TypedSteps
    Σ
    target
    (substitute substitution type)
    (substitute substitution left)
    (substitute substitution right)
substitution-preserves-typed-steps typed
  (typed-steps-refl typing) =
  typed-steps-refl
    (substitution-typing typed typing)
substitution-preserves-typed-steps typed
  (typed-steps-next reduction rest) =
  typed-steps-next
    (substitution-preserves-typed-reduction typed reduction)
    (substitution-preserves-typed-steps typed rest)

substitution-preserves-base-q0-equivalence :
  {V W : Set} {Σ : Signature}
  {source : Context V} {target : Context W}
  {substitution : V -> Tm W}
  {left right : Tm V} ->
  TypedSubstitution Σ source target substitution ->
  BaseQ0Equivalent Σ source left right ->
  BaseQ0Equivalent
    Σ
    target
    (substitute substitution left)
    (substitute substitution right)
substitution-preserves-base-q0-equivalence
  {substitution = substitution}
  typed
  equivalent =
  base-q0-equivalent
    (universe equivalent)
    (substitute substitution (common equivalent))
    (substitution-preserves-typed-steps
      typed
      (left-trace equivalent))
    (substitution-preserves-typed-steps
      typed
      (right-trace equivalent))

substitution-preserves-conversion-typing :
  {V W : Set} {Σ : Signature}
  {source : Context V} {target : Context W}
  {substitution : V -> Tm W}
  {term type : Tm V} ->
  TypedSubstitution Σ source target substitution ->
  Σ ⊢c source ∶ term ∶ type ->
  Σ ⊢c target
    ∶ substitute substitution term
    ∶ substitute substitution type
substitution-preserves-conversion-typing typed
  (conversion-exact typing) =
  conversion-exact
    (substitution-typing typed typing)
substitution-preserves-conversion-typing typed
  (conversion-convert derivation equivalent) =
  conversion-convert
    (substitution-preserves-conversion-typing typed derivation)
    (substitution-preserves-base-q0-equivalence typed equivalent)
substitution-preserves-conversion-typing
  {substitution = substitution}
  typed
  (conversion-application
    {body = body}
    function
    argument) =
  transport-conversion-type
    (sym
      (beta-contractum-naturality
        substitution
        body
        _))
    (conversion-application
      (substitution-preserves-conversion-typing
        typed
        function)
      (substitution-preserves-conversion-typing
        typed
        argument))
