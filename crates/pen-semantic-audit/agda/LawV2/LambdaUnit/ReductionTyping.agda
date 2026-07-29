{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.ReductionTyping where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat)
open import LawV2.LambdaUnit.TypingJudgment
open import LawV2.LambdaUnit.SubstitutionTypingV3
open import LawV2.LambdaUnit.SubstitutionReduction
  using
    ( Step
    ; beta-step
    ; public-delta-step
    ; fresh-equation-step
    ; single
    ; instantiate-weaken
    ; empty-elimination
    ; substitution-step
    )

record TypedReduction
  {V : Set}
  (Σ : Signature)
  (Γ : Context V)
  (left right type : Tm V) : Set₁ where
  constructor typed-reduction
  field
    raw-reduction : Step left right
    left-typing : Σ ⊢ Γ ∶ left ∶ type
    right-typing : Σ ⊢ Γ ∶ right ∶ type

open TypedReduction public

single-typed-substitution :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {parameter argument : Tm V} ->
  Σ ⊢ Γ ∶ argument ∶ parameter ->
  TypedSubstitution
    Σ
    (extend-context Γ parameter)
    Γ
    (single argument)
single-typed-substitution
  {Σ = Σ}
  {Γ = Γ}
  {parameter = parameter}
  {argument = argument}
  argument-typing =
  typed-substitution image
  where
    image :
      (x : _) ->
      Σ ⊢ Γ
        ∶ single argument x
        ∶ substitute
            (single argument)
            (extend-context Γ parameter x)
    image bound =
      transport-type
        (sym (instantiate-weaken parameter argument))
        argument-typing
    image (free x) =
      transport-type
        (sym (instantiate-weaken (Γ x) argument))
        (type-variable x)

typed-beta-reduction :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {parameter argument : Tm V}
  {body body-type : Tm (Lift V)}
  {parameter-level : Nat} ->
  Σ ⊢ Γ ∶ parameter ∶ sort parameter-level ->
  Σ ⊢ extend-context Γ parameter ∶ body ∶ body-type ->
  Σ ⊢ Γ ∶ argument ∶ parameter ->
  TypedReduction
    Σ
    Γ
    (app (lam parameter body) argument)
    (instantiate body argument)
    (instantiate body-type argument)
typed-beta-reduction parameter-typing body-typing argument-typing =
  typed-reduction
    (beta-step _ _ _)
    (type-application
      (type-lambda parameter-typing body-typing)
      argument-typing)
    (substitution-typing
      (single-typed-substitution argument-typing)
      body-typing)

empty-typed-substitution :
  {V : Set} {Σ : Signature} ->
  (Γ : Context V) ->
  TypedSubstitution
    Σ
    empty-context
    Γ
    empty-elimination
empty-typed-substitution Γ =
  typed-substitution (λ ())

typed-public-delta-reduction :
  {V : Set} {Σ : Signature} {Γ : Context V}
  {identifier : Nat}
  {entry : GlobalDeclaration}
  {body : Tm Empty} ->
  lookup-global Σ identifier ≡ just entry ->
  Σ ⊢ empty-context
    ∶ closed (declared-type entry)
    ∶ sort (declared-universe entry) ->
  Σ ⊢ empty-context
    ∶ body
    ∶ declared-type entry ->
  TypedReduction
    Σ
    Γ
    (global identifier)
    (closed body)
    (closed (declared-type entry))
typed-public-delta-reduction
  {Γ = Γ}
  {identifier = identifier}
  {body = body}
  lookup
  declaration-formation
  body-typing =
  typed-reduction
    (public-delta-step identifier body)
    (type-global lookup declaration-formation)
    (substitution-typing
      (empty-typed-substitution Γ)
      body-typing)

record TypedFreshRuleSchema
  {P : Set}
  (Σ : Signature)
  (pattern-context : Context P)
  (left right type : Tm P) : Set where
  constructor typed-fresh-rule-schema
  field
    fresh-left-typing :
      Σ ⊢ pattern-context ∶ left ∶ type
    fresh-right-typing :
      Σ ⊢ pattern-context ∶ right ∶ type

open TypedFreshRuleSchema public

instantiate-typed-fresh-rule :
  {P V : Set} {Σ : Signature}
  {pattern-context : Context P}
  {target-context : Context V}
  {left right type : Tm P}
  {match : P -> Tm V} ->
  TypedFreshRuleSchema
    Σ pattern-context left right type ->
  TypedSubstitution
    Σ pattern-context target-context match ->
  TypedReduction
    Σ
    target-context
    (substitute match left)
    (substitute match right)
    (substitute match type)
instantiate-typed-fresh-rule
  {left = left}
  {right = right}
  {match = match}
  schema
  typed-match =
  typed-reduction
    (fresh-equation-step left right match)
    (substitution-typing
      typed-match
      (fresh-left-typing schema))
    (substitution-typing
      typed-match
      (fresh-right-typing schema))

substitution-preserves-typed-reduction :
  {V W : Set} {Σ : Signature}
  {source : Context V} {target : Context W}
  {substitution : V -> Tm W}
  {left right type : Tm V} ->
  TypedSubstitution Σ source target substitution ->
  TypedReduction Σ source left right type ->
  TypedReduction
    Σ
    target
    (substitute substitution left)
    (substitute substitution right)
    (substitute substitution type)
substitution-preserves-typed-reduction typed
  (typed-reduction raw left-derivation right-derivation) =
  typed-reduction
    (substitution-step _ raw)
    (substitution-typing typed left-derivation)
    (substitution-typing typed right-derivation)

unit-type-typing-stability :
  {V W : Set} {Σ : Signature}
  {source : Context V} {target : Context W}
  {substitution : V -> Tm W} ->
  TypedSubstitution Σ source target substitution ->
  Σ ⊢ target
    ∶ substitute substitution unit-type
    ∶ substitute substitution (sort 0)
unit-type-typing-stability typed =
  substitution-typing typed type-unit-type

unit-typing-stability :
  {V W : Set} {Σ : Signature}
  {source : Context V} {target : Context W}
  {substitution : V -> Tm W} ->
  TypedSubstitution Σ source target substitution ->
  Σ ⊢ target
    ∶ substitute substitution unit
    ∶ substitute substitution unit-type
unit-typing-stability typed =
  substitution-typing typed type-unit
