{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.SubstitutionReduction where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat)
open import LawV2.LambdaUnit.Substitution

data Empty : Set where

empty-elimination : {A : Set} -> Empty -> A
empty-elimination ()

single :
  {V : Set} ->
  Tm V ->
  Lift V ->
  Tm V
single argument bound = argument
single argument (free x) = var x

instantiate :
  {V : Set} ->
  Tm (Lift V) ->
  Tm V ->
  Tm V
instantiate body argument =
  substitute (single argument) body

weaken :
  {V : Set} ->
  Tm V ->
  Tm (Lift V)
weaken = rename free

instantiate-weaken :
  {V : Set} ->
  (term argument : Tm V) ->
  instantiate (weaken term) argument ≡ term
instantiate-weaken {V = V} term argument =
  trans
    (substitution-after-renaming free (single argument) term)
    (trans
      (substitute-cong pointwise term)
      (substitution-identity term))
  where
    pointwise :
      (λ x -> single argument (free x))
      ≗
      identity-substitution
    pointwise x = refl

beta-naturality :
  {V W : Set} ->
  (σ : V -> Tm W) ->
  (parameter-type : Tm V) ->
  (body : Tm (Lift V)) ->
  (argument : Tm V) ->
  substitute σ
    (app (lam parameter-type body) argument)
  ≡
  app
    (lam
      (substitute σ parameter-type)
      (substitute (lift-substitution σ) body))
    (substitute σ argument)
beta-naturality σ parameter-type body argument = refl

beta-contractum-naturality :
  {V W : Set} ->
  (σ : V -> Tm W) ->
  (body : Tm (Lift V)) ->
  (argument : Tm V) ->
  substitute σ (instantiate body argument)
  ≡
  instantiate
    (substitute (lift-substitution σ) body)
    (substitute σ argument)
beta-contractum-naturality {V = V} {W = W} σ body argument =
  trans
    (substitution-composition (single argument) σ body)
    (trans
      (substitute-cong pointwise body)
      (sym
        (substitution-composition
          (lift-substitution σ)
          (single (substitute σ argument))
          body)))
  where
    pointwise :
      (single argument then σ)
      ≗
      (lift-substitution σ then single (substitute σ argument))
    pointwise bound = refl
    pointwise (free x) =
      sym (instantiate-weaken (σ x) (substitute σ argument))

closed :
  {V : Set} ->
  Tm Empty ->
  Tm V
closed term = substitute empty-elimination term

public-delta-naturality :
  {V W : Set} ->
  (σ : V -> Tm W) ->
  (body : Tm Empty) ->
  substitute σ (closed body)
  ≡
  closed body
public-delta-naturality {V = V} {W = W} σ body =
  trans
    (substitution-composition empty-elimination σ body)
    (substitute-cong pointwise body)
  where
    pointwise :
      (empty-elimination then σ)
      ≗
      empty-elimination
    pointwise ()

unit-type-naturality :
  {V W : Set} ->
  (σ : V -> Tm W) ->
  substitute σ unit-type ≡ unit-type
unit-type-naturality σ = refl

unit-naturality :
  {V W : Set} ->
  (σ : V -> Tm W) ->
  substitute σ unit ≡ unit
unit-naturality σ = refl

-- A fresh equation is represented by its two open pattern terms.  Matching
-- supplies `match`; an arbitrary later substitution supplies `σ`.  Stability
-- is the generic composition theorem, not an enumeration of either map.
fresh-left-naturality :
  {P V W : Set} ->
  (left : Tm P) ->
  (match : P -> Tm V) ->
  (σ : V -> Tm W) ->
  substitute σ (substitute match left)
  ≡
  substitute (match then σ) left
fresh-left-naturality left match σ =
  substitution-composition match σ left

fresh-right-naturality :
  {P V W : Set} ->
  (right : Tm P) ->
  (match : P -> Tm V) ->
  (σ : V -> Tm W) ->
  substitute σ (substitute match right)
  ≡
  substitute (match then σ) right
fresh-right-naturality right match σ =
  substitution-composition match σ right

-- Untyped rule-schema stability.  The full production theorem additionally
-- needs the dependent typing substitution lemma from SubstitutionTyping.
data Step {V : Set} : Tm V -> Tm V -> Set₁ where
  beta-step :
    (parameter-type : Tm V) ->
    (body : Tm (Lift V)) ->
    (argument : Tm V) ->
    Step
      (app (lam parameter-type body) argument)
      (instantiate body argument)

  public-delta-step :
    (identifier : Nat) ->
    (body : Tm Empty) ->
    Step (global identifier) (closed body)

  fresh-equation-step :
    {P : Set} ->
    (left right : Tm P) ->
    (match : P -> Tm V) ->
    Step
      (substitute match left)
      (substitute match right)

transport-step :
  {V : Set} {left left′ right right′ : Tm V} ->
  left ≡ left′ ->
  right ≡ right′ ->
  Step left right ->
  Step left′ right′
transport-step refl refl step = step

substitution-step :
  {V W : Set} {left right : Tm V} ->
  (σ : V -> Tm W) ->
  Step left right ->
  Step (substitute σ left) (substitute σ right)
substitution-step σ (beta-step parameter-type body argument) =
  transport-step
    refl
    (sym (beta-contractum-naturality σ body argument))
    (beta-step
      (substitute σ parameter-type)
      (substitute (lift-substitution σ) body)
      (substitute σ argument))
substitution-step σ (public-delta-step identifier body) =
  transport-step
    refl
    (sym (public-delta-naturality σ body))
    (public-delta-step identifier body)
substitution-step σ (fresh-equation-step left right match) =
  transport-step
    (sym (fresh-left-naturality left match σ))
    (sym (fresh-right-naturality right match σ))
    (fresh-equation-step left right (match then σ))

data Steps {V : Set} : Tm V -> Tm V -> Set₁ where
  steps-refl : (term : Tm V) -> Steps term term
  steps-next :
    {left middle right : Tm V} ->
    Step left middle ->
    Steps middle right ->
    Steps left right

substitution-steps :
  {V W : Set} {left right : Tm V} ->
  (σ : V -> Tm W) ->
  Steps left right ->
  Steps (substitute σ left) (substitute σ right)
substitution-steps σ (steps-refl term) =
  steps-refl (substitute σ term)
substitution-steps σ (steps-next step rest) =
  steps-next
    (substitution-step σ step)
    (substitution-steps σ rest)
