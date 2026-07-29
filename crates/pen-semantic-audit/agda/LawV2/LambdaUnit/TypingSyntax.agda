{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.TypingSyntax where

open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.LambdaUnit.Substitution public
open import LawV2.LambdaUnit.SubstitutionReduction
  using (Empty; closed; instantiate; weaken)
  public

data Maybe (A : Set) : Set where
  nothing : Maybe A
  just : A -> Maybe A

record GlobalDeclaration : Set where
  constructor declaration
  field
    declared-type : Tm Empty
    declared-universe : Nat

open GlobalDeclaration public

record Signature : Set where
  constructor signature
  field
    lookup-global : Nat -> Maybe GlobalDeclaration

open Signature public

Context : Set -> Set
Context V = V -> Tm V

empty-context : Context Empty
empty-context ()

extend-context :
  {V : Set} ->
  Context V ->
  Tm V ->
  Context (Lift V)
extend-context context parameter bound = weaken parameter
extend-context context parameter (free x) = weaken (context x)

substitute-context-entry :
  {V W : Set} ->
  (V -> Tm W) ->
  Context V ->
  V ->
  Tm W
substitute-context-entry substitution context x =
  substitute substitution (context x)

infixl 6 _max_

_max_ : Nat -> Nat -> Nat
zero max right = right
suc left max zero = suc left
suc left max suc right = suc (left max right)
