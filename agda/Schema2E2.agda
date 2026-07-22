{-# OPTIONS --safe --without-K #-}

module Schema2E2 where

open import Agda.Builtin.Bool using (Bool; true; false)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Schema2 public

------------------------------------------------------------------------
-- E-2: intrinsically typed Step-8 ordinary-signature fragment
------------------------------------------------------------------------

-- This is deliberately a typing fragment, not the E-4 generator-membership
-- decision. Constructing `step8-cell-action` proves term-level typing; it
-- does not say that the operation-on-cell is an independent natural family.

Path : {A : Set} → A → A → Set
Path x y = x ≡ y

cong : {A B : Set} {x y : A} →
       (f : A → B) → Path x y → Path (f x) (f y)
cong f refl = refl

infixr 4 _×_

_×_ : Set → Set → Set
A × B = Σ A (λ _ → B)

-- A registered constant-boundary 3-cell. The six face witnesses make the
-- base point part of the cube type rather than untyped metadata.
record Cube3At (A : Set) (at : A) : Set where
  constructor cube3-at
  field
    cube3-body : Bool → Bool → Bool → A
    cube3-i0   : (j k : Bool) → cube3-body false j k ≡ at
    cube3-i1   : (j k : Bool) → cube3-body true  j k ≡ at
    cube3-j0   : (i k : Bool) → cube3-body i false k ≡ at
    cube3-j1   : (i k : Bool) → cube3-body i true  k ≡ at
    cube3-k0   : (i j : Bool) → cube3-body i j false ≡ at
    cube3-k1   : (i j : Bool) → cube3-body i j true  ≡ at

open Cube3At public

map-cube3 : {A B : Set} {at : A} →
            (f : A → B) → Cube3At A at → Cube3At B (f at)
map-cube3 f cube =
  cube3-at
    (λ i j k → f (cube3-body cube i j k))
    (λ j k → cong f (cube3-i0 cube j k))
    (λ j k → cong f (cube3-i1 cube j k))
    (λ i k → cong f (cube3-j0 cube i k))
    (λ i k → cong f (cube3-j1 cube i k))
    (λ i j → cong f (cube3-k0 cube i j))
    (λ i j → cong f (cube3-k1 cube i j))

record Step8CellSignature : Set₁ where
  constructor s3-cell-signature
  field
    S3   : Set
    base : S3
    p    : Cube3At S3 base

open Step8CellSignature public

record Step8OperationSignature (cell : Step8CellSignature) : Set where
  constructor operation-signature
  field
    μ : S3 cell × S3 cell → S3 cell

open Step8OperationSignature public

record Step8LeftUnitSignature
       (cell : Step8CellSignature)
       (operation : Step8OperationSignature cell) : Set where
  constructor left-unit-signature
  field
    left-unit : (x : S3 cell) →
                Path (μ operation (base cell , x)) x

open Step8LeftUnitSignature public

record Step8OrdinarySignature : Set₁ where
  constructor step8-ordinary-signature
  field
    cell-signature : Step8CellSignature
    operation-signature-field :
      Step8OperationSignature cell-signature
    left-unit-signature-field :
      Step8LeftUnitSignature
        cell-signature operation-signature-field

open Step8OrdinarySignature public

step8-μ-typed : (signature : Step8OrdinarySignature) →
                S3 (cell-signature signature) ×
                S3 (cell-signature signature) →
                S3 (cell-signature signature)
step8-μ-typed signature = μ (operation-signature-field signature)

step8-left-unit-typed : (signature : Step8OrdinarySignature) →
                        (x : S3 (cell-signature signature)) →
                        Path
                          (step8-μ-typed signature
                            (base (cell-signature signature) , x))
                          x
step8-left-unit-typed signature =
  left-unit (left-unit-signature-field signature)

-- For each x, apply μ(-,x) to p. `map-cube3` checks that the body is
-- μ(p(i,j,k),x) and that every face lands at μ(base,x).
step8-cell-action : (signature : Step8OrdinarySignature) →
                    (x : S3 (cell-signature signature)) →
                    Cube3At
                      (S3 (cell-signature signature))
                      (step8-μ-typed signature
                        (base (cell-signature signature) , x))
step8-cell-action signature x =
  map-cube3
    (λ y → step8-μ-typed signature (y , x))
    (p (cell-signature signature))

step8-cell-action-body : (signature : Step8OrdinarySignature) →
                         (x : S3 (cell-signature signature)) →
                         (i j k : Bool) →
                         cube3-body (step8-cell-action signature x) i j k ≡
                         step8-μ-typed signature
                           (cube3-body
                              (p (cell-signature signature)) i j k , x)
step8-cell-action-body signature x i j k = refl

-- Scope boundary: this file defines no generator-independence verdict,
-- normalizer, classifier, score, acceptance comparison, or halt theorem.
