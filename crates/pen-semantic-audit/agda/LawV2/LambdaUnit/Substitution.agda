{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.Substitution where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat)

infix 4 _≗_

cong :
  {A B : Set} {x y : A} ->
  (f : A -> B) ->
  x ≡ y ->
  f x ≡ f y
cong f refl = refl

cong₂ :
  {A B C : Set} {x x′ : A} {y y′ : B} ->
  (f : A -> B -> C) ->
  x ≡ x′ ->
  y ≡ y′ ->
  f x y ≡ f x′ y′
cong₂ f refl refl = refl

sym : {A : Set} {x y : A} -> x ≡ y -> y ≡ x
sym refl = refl

trans :
  {A : Set} {x y z : A} ->
  x ≡ y ->
  y ≡ z ->
  x ≡ z
trans refl proof = proof

_≗_ : {A B : Set} -> (A -> B) -> (A -> B) -> Set
f ≗ g = (x : _) -> f x ≡ g x

data Lift (V : Set) : Set where
  bound : Lift V
  free : V -> Lift V

lift-map : {V W : Set} -> (V -> W) -> Lift V -> Lift W
lift-map mapping bound = bound
lift-map mapping (free x) = free (mapping x)

data Tm (V : Set) : Set where
  var : V -> Tm V
  sort : Nat -> Tm V
  global : Nat -> Tm V
  pi : Tm V -> Tm (Lift V) -> Tm V
  lam : Tm V -> Tm (Lift V) -> Tm V
  app : Tm V -> Tm V -> Tm V
  unit-type : Tm V
  unit : Tm V

rename : {V W : Set} -> (V -> W) -> Tm V -> Tm W
rename ρ (var x) = var (ρ x)
rename ρ (sort level) = sort level
rename ρ (global identifier) = global identifier
rename ρ (pi parameter body) =
  pi (rename ρ parameter) (rename (lift-map ρ) body)
rename ρ (lam parameter-type body) =
  lam (rename ρ parameter-type) (rename (lift-map ρ) body)
rename ρ (app function argument) =
  app (rename ρ function) (rename ρ argument)
rename ρ unit-type = unit-type
rename ρ unit = unit

lift-substitution :
  {V W : Set} ->
  (V -> Tm W) ->
  Lift V ->
  Tm (Lift W)
lift-substitution σ bound = var bound
lift-substitution σ (free x) = rename free (σ x)

substitute : {V W : Set} -> (V -> Tm W) -> Tm V -> Tm W
substitute σ (var x) = σ x
substitute σ (sort level) = sort level
substitute σ (global identifier) = global identifier
substitute σ (pi parameter body) =
  pi
    (substitute σ parameter)
    (substitute (lift-substitution σ) body)
substitute σ (lam parameter-type body) =
  lam
    (substitute σ parameter-type)
    (substitute (lift-substitution σ) body)
substitute σ (app function argument) =
  app (substitute σ function) (substitute σ argument)
substitute σ unit-type = unit-type
substitute σ unit = unit

identity-substitution : {V : Set} -> V -> Tm V
identity-substitution = var

_then_ :
  {U V W : Set} ->
  (U -> Tm V) ->
  (V -> Tm W) ->
  U ->
  Tm W
(σ then τ) x = substitute τ (σ x)

rename-cong :
  {V W : Set} {ρ θ : V -> W} ->
  ρ ≗ θ ->
  (term : Tm V) ->
  rename ρ term ≡ rename θ term
rename-cong pointwise (var x) = cong var (pointwise x)
rename-cong pointwise (sort level) = refl
rename-cong pointwise (global identifier) = refl
rename-cong pointwise (pi parameter body) =
  cong₂ pi
    (rename-cong pointwise parameter)
    (rename-cong lift-pointwise body)
  where
    lift-pointwise : lift-map _ ≗ lift-map _
    lift-pointwise bound = refl
    lift-pointwise (free x) = cong free (pointwise x)
rename-cong pointwise (lam parameter-type body) =
  cong₂ lam
    (rename-cong pointwise parameter-type)
    (rename-cong lift-pointwise body)
  where
    lift-pointwise : lift-map _ ≗ lift-map _
    lift-pointwise bound = refl
    lift-pointwise (free x) = cong free (pointwise x)
rename-cong pointwise (app function argument) =
  cong₂ app
    (rename-cong pointwise function)
    (rename-cong pointwise argument)
rename-cong pointwise unit-type = refl
rename-cong pointwise unit = refl

rename-identity :
  {V : Set} ->
  (term : Tm V) ->
  rename (λ x -> x) term ≡ term
rename-identity (var x) = refl
rename-identity (sort level) = refl
rename-identity (global identifier) = refl
rename-identity (pi parameter body) =
  cong₂ pi
    (rename-identity parameter)
    (trans
      (rename-cong lift-identity body)
      (rename-identity body))
  where
    lift-identity : lift-map (λ x -> x) ≗ (λ x -> x)
    lift-identity bound = refl
    lift-identity (free x) = refl
rename-identity (lam parameter-type body) =
  cong₂ lam
    (rename-identity parameter-type)
    (trans
      (rename-cong lift-identity body)
      (rename-identity body))
  where
    lift-identity : lift-map (λ x -> x) ≗ (λ x -> x)
    lift-identity bound = refl
    lift-identity (free x) = refl
rename-identity (app function argument) =
  cong₂ app
    (rename-identity function)
    (rename-identity argument)
rename-identity unit-type = refl
rename-identity unit = refl

rename-composition :
  {U V W : Set} ->
  (ρ : U -> V) ->
  (θ : V -> W) ->
  (term : Tm U) ->
  rename θ (rename ρ term)
  ≡
  rename (λ x -> θ (ρ x)) term
rename-composition ρ θ (var x) = refl
rename-composition ρ θ (sort level) = refl
rename-composition ρ θ (global identifier) = refl
rename-composition ρ θ (pi parameter body) =
  cong₂ pi
    (rename-composition ρ θ parameter)
    (trans
      (rename-composition (lift-map ρ) (lift-map θ) body)
      (rename-cong lift-composition body))
  where
    lift-composition :
      (λ x -> lift-map θ (lift-map ρ x))
      ≗
      lift-map (λ x -> θ (ρ x))
    lift-composition bound = refl
    lift-composition (free x) = refl
rename-composition ρ θ (lam parameter-type body) =
  cong₂ lam
    (rename-composition ρ θ parameter-type)
    (trans
      (rename-composition (lift-map ρ) (lift-map θ) body)
      (rename-cong lift-composition body))
  where
    lift-composition :
      (λ x -> lift-map θ (lift-map ρ x))
      ≗
      lift-map (λ x -> θ (ρ x))
    lift-composition bound = refl
    lift-composition (free x) = refl
rename-composition ρ θ (app function argument) =
  cong₂ app
    (rename-composition ρ θ function)
    (rename-composition ρ θ argument)
rename-composition ρ θ unit-type = refl
rename-composition ρ θ unit = refl

substitute-cong :
  {V W : Set} {σ τ : V -> Tm W} ->
  σ ≗ τ ->
  (term : Tm V) ->
  substitute σ term ≡ substitute τ term
substitute-cong pointwise (var x) = pointwise x
substitute-cong pointwise (sort level) = refl
substitute-cong pointwise (global identifier) = refl
substitute-cong pointwise (pi parameter body) =
  cong₂ pi
    (substitute-cong pointwise parameter)
    (substitute-cong lift-pointwise body)
  where
    lift-pointwise : lift-substitution _ ≗ lift-substitution _
    lift-pointwise bound = refl
    lift-pointwise (free x) =
      cong (rename free) (pointwise x)
substitute-cong pointwise (lam parameter-type body) =
  cong₂ lam
    (substitute-cong pointwise parameter-type)
    (substitute-cong lift-pointwise body)
  where
    lift-pointwise : lift-substitution _ ≗ lift-substitution _
    lift-pointwise bound = refl
    lift-pointwise (free x) =
      cong (rename free) (pointwise x)
substitute-cong pointwise (app function argument) =
  cong₂ app
    (substitute-cong pointwise function)
    (substitute-cong pointwise argument)
substitute-cong pointwise unit-type = refl
substitute-cong pointwise unit = refl

rename-as-substitution :
  {V W : Set} ->
  (ρ : V -> W) ->
  (term : Tm V) ->
  rename ρ term
  ≡
  substitute (λ x -> var (ρ x)) term
rename-as-substitution ρ (var x) = refl
rename-as-substitution ρ (sort level) = refl
rename-as-substitution ρ (global identifier) = refl
rename-as-substitution ρ (pi parameter body) =
  cong₂ pi
    (rename-as-substitution ρ parameter)
    (trans
      (rename-as-substitution (lift-map ρ) body)
      (substitute-cong lift-pointwise body))
  where
    lift-pointwise :
      (λ x -> var (lift-map ρ x))
      ≗
      lift-substitution (λ x -> var (ρ x))
    lift-pointwise bound = refl
    lift-pointwise (free x) = refl
rename-as-substitution ρ (lam parameter-type body) =
  cong₂ lam
    (rename-as-substitution ρ parameter-type)
    (trans
      (rename-as-substitution (lift-map ρ) body)
      (substitute-cong lift-pointwise body))
  where
    lift-pointwise :
      (λ x -> var (lift-map ρ x))
      ≗
      lift-substitution (λ x -> var (ρ x))
    lift-pointwise bound = refl
    lift-pointwise (free x) = refl
rename-as-substitution ρ (app function argument) =
  cong₂ app
    (rename-as-substitution ρ function)
    (rename-as-substitution ρ argument)
rename-as-substitution ρ unit-type = refl
rename-as-substitution ρ unit = refl

substitution-after-renaming :
  {U V W : Set} ->
  (ρ : U -> V) ->
  (σ : V -> Tm W) ->
  (term : Tm U) ->
  substitute σ (rename ρ term)
  ≡
  substitute (λ x -> σ (ρ x)) term
substitution-after-renaming ρ σ (var x) = refl
substitution-after-renaming ρ σ (sort level) = refl
substitution-after-renaming ρ σ (global identifier) = refl
substitution-after-renaming ρ σ (pi parameter body) =
  cong₂ pi
    (substitution-after-renaming ρ σ parameter)
    (trans
      (substitution-after-renaming
        (lift-map ρ)
        (lift-substitution σ)
        body)
      (substitute-cong lift-pointwise body))
  where
    lift-pointwise :
      (λ x -> lift-substitution σ (lift-map ρ x))
      ≗
      lift-substitution (λ x -> σ (ρ x))
    lift-pointwise bound = refl
    lift-pointwise (free x) = refl
substitution-after-renaming ρ σ (lam parameter-type body) =
  cong₂ lam
    (substitution-after-renaming ρ σ parameter-type)
    (trans
      (substitution-after-renaming
        (lift-map ρ)
        (lift-substitution σ)
        body)
      (substitute-cong lift-pointwise body))
  where
    lift-pointwise :
      (λ x -> lift-substitution σ (lift-map ρ x))
      ≗
      lift-substitution (λ x -> σ (ρ x))
    lift-pointwise bound = refl
    lift-pointwise (free x) = refl
substitution-after-renaming ρ σ (app function argument) =
  cong₂ app
    (substitution-after-renaming ρ σ function)
    (substitution-after-renaming ρ σ argument)
substitution-after-renaming ρ σ unit-type = refl
substitution-after-renaming ρ σ unit = refl

renaming-after-substitution :
  {U V W : Set} ->
  (σ : U -> Tm V) ->
  (ρ : V -> W) ->
  (term : Tm U) ->
  rename ρ (substitute σ term)
  ≡
  substitute (λ x -> rename ρ (σ x)) term
renaming-after-substitution σ ρ (var x) = refl
renaming-after-substitution σ ρ (sort level) = refl
renaming-after-substitution σ ρ (global identifier) = refl
renaming-after-substitution σ ρ (pi parameter body) =
  cong₂ pi
    (renaming-after-substitution σ ρ parameter)
    (trans
      (renaming-after-substitution
        (lift-substitution σ)
        (lift-map ρ)
        body)
      (substitute-cong lift-pointwise body))
  where
    lift-pointwise :
      (λ x ->
        rename (lift-map ρ) (lift-substitution σ x))
      ≗
      lift-substitution (λ x -> rename ρ (σ x))
    lift-pointwise bound = refl
    lift-pointwise (free x) =
      trans
        (rename-composition free (lift-map ρ) (σ x))
        (sym (rename-composition ρ free (σ x)))
renaming-after-substitution σ ρ (lam parameter-type body) =
  cong₂ lam
    (renaming-after-substitution σ ρ parameter-type)
    (trans
      (renaming-after-substitution
        (lift-substitution σ)
        (lift-map ρ)
        body)
      (substitute-cong lift-pointwise body))
  where
    lift-pointwise :
      (λ x ->
        rename (lift-map ρ) (lift-substitution σ x))
      ≗
      lift-substitution (λ x -> rename ρ (σ x))
    lift-pointwise bound = refl
    lift-pointwise (free x) =
      trans
        (rename-composition free (lift-map ρ) (σ x))
        (sym (rename-composition ρ free (σ x)))
renaming-after-substitution σ ρ (app function argument) =
  cong₂ app
    (renaming-after-substitution σ ρ function)
    (renaming-after-substitution σ ρ argument)
renaming-after-substitution σ ρ unit-type = refl
renaming-after-substitution σ ρ unit = refl

substitution-identity :
  {V : Set} ->
  (term : Tm V) ->
  substitute identity-substitution term ≡ term
substitution-identity (var x) = refl
substitution-identity (sort level) = refl
substitution-identity (global identifier) = refl
substitution-identity (pi parameter body) =
  cong₂ pi
    (substitution-identity parameter)
    (trans
      (substitute-cong lift-identity body)
      (substitution-identity body))
  where
    lift-identity :
      lift-substitution identity-substitution
      ≗
      identity-substitution
    lift-identity bound = refl
    lift-identity (free x) = refl
substitution-identity (lam parameter-type body) =
  cong₂ lam
    (substitution-identity parameter-type)
    (trans
      (substitute-cong lift-identity body)
      (substitution-identity body))
  where
    lift-identity :
      lift-substitution identity-substitution
      ≗
      identity-substitution
    lift-identity bound = refl
    lift-identity (free x) = refl
substitution-identity (app function argument) =
  cong₂ app
    (substitution-identity function)
    (substitution-identity argument)
substitution-identity unit-type = refl
substitution-identity unit = refl

substitution-composition :
  {U V W : Set} ->
  (σ : U -> Tm V) ->
  (τ : V -> Tm W) ->
  (term : Tm U) ->
  substitute τ (substitute σ term)
  ≡
  substitute (σ then τ) term
substitution-composition σ τ (var x) = refl
substitution-composition σ τ (sort level) = refl
substitution-composition σ τ (global identifier) = refl
substitution-composition σ τ (pi parameter body) =
  cong₂ pi
    (substitution-composition σ τ parameter)
    (trans
      (substitution-composition
        (lift-substitution σ)
        (lift-substitution τ)
        body)
      (substitute-cong lift-composition body))
  where
    lift-composition :
      (lift-substitution σ then lift-substitution τ)
      ≗
      lift-substitution (σ then τ)
    lift-composition bound = refl
    lift-composition (free x) =
      trans
        (substitution-after-renaming
          free
          (lift-substitution τ)
          (σ x))
        (sym
          (renaming-after-substitution
            τ
            free
            (σ x)))
substitution-composition σ τ (lam parameter-type body) =
  cong₂ lam
    (substitution-composition σ τ parameter-type)
    (trans
      (substitution-composition
        (lift-substitution σ)
        (lift-substitution τ)
        body)
      (substitute-cong lift-composition body))
  where
    lift-composition :
      (lift-substitution σ then lift-substitution τ)
      ≗
      lift-substitution (σ then τ)
    lift-composition bound = refl
    lift-composition (free x) =
      trans
        (substitution-after-renaming
          free
          (lift-substitution τ)
          (σ x))
        (sym
          (renaming-after-substitution
            τ
            free
            (σ x)))
substitution-composition σ τ (app function argument) =
  cong₂ app
    (substitution-composition σ τ function)
    (substitution-composition σ τ argument)
substitution-composition σ τ unit-type = refl
substitution-composition σ τ unit = refl

substitution-associativity :
  {T U V W : Set} ->
  (ρ : T -> Tm U) ->
  (σ : U -> Tm V) ->
  (τ : V -> Tm W) ->
  ((ρ then σ) then τ)
  ≗
  (ρ then (σ then τ))
substitution-associativity ρ σ τ x =
  substitution-composition σ τ (ρ x)

lifting-composition :
  {U V W : Set} ->
  (σ : U -> Tm V) ->
  (τ : V -> Tm W) ->
  (lift-substitution σ then lift-substitution τ)
  ≗
  lift-substitution (σ then τ)
lifting-composition σ τ bound = refl
lifting-composition σ τ (free x) =
  trans
    (substitution-after-renaming
      free
      (lift-substitution τ)
      (σ x))
    (sym
      (renaming-after-substitution
        τ
        free
        (σ x)))

lifting-bound :
  {V W : Set} ->
  (σ : V -> Tm W) ->
  lift-substitution σ bound ≡ var bound
lifting-bound σ = refl

lifting-free :
  {V W : Set} ->
  (σ : V -> Tm W) ->
  (x : V) ->
  lift-substitution σ (free x)
  ≡
  rename free (σ x)
lifting-free σ x = refl
