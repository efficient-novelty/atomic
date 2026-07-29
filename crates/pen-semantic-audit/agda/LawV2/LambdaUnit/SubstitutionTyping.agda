{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.SubstitutionTyping where

open import Agda.Builtin.Nat using (Nat; suc)
open import LawV2.LambdaUnit.Substitution

data Fin : Nat -> Set where
  fzero : {count : Nat} -> Fin (suc count)
  fsuc : {count : Nat} -> Fin count -> Fin (suc count)

-- The scoped representation makes capture avoidance a construction:
-- substituting a scoped image for every source variable returns a term scoped
-- in the target.  This is deliberately *not* the dependent typing lemma.
ScopedTerm : Nat -> Set
ScopedTerm count = Tm (Fin count)

ScopedSubstitution : Nat -> Nat -> Set
ScopedSubstitution source target =
  Fin source -> ScopedTerm target

substitution-preserves-scope :
  {source target : Nat} ->
  ScopedSubstitution source target ->
  ScopedTerm source ->
  ScopedTerm target
substitution-preserves-scope = substitute

-- Exact fail-closed frontier of this package.  The production capability must
-- not be minted from scope preservation: it still needs an inductive
-- dependent typing judgment (including globals and universe formation) and a
-- proof that arbitrary typed simultaneous substitution preserves it.
data MissingDependentTypingTheorem : Set where
  dependent-typing-lemma-not-yet-mechanized :
    MissingDependentTypingTheorem

typing-frontier : MissingDependentTypingTheorem
typing-frontier = dependent-typing-lemma-not-yet-mechanized
