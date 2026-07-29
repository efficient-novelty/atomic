{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.LambdaUnitTypingMetatheoryV3 where

open import LawV2.LambdaUnit.TypingSyntax public
open import LawV2.LambdaUnit.TypingJudgment public
open import LawV2.LambdaUnit.Synthesis public
open import LawV2.LambdaUnit.SubstitutionTypingV3 public
open import LawV2.LambdaUnit.ReductionTyping public
open import LawV2.LambdaUnit.FamilyNaturality public

-- This package mechanizes the conversion-free declarative lambda/unit core,
-- relational synthesis soundness and completeness for that exact core,
-- dependent simultaneous substitution (including binder lifting), and typed
-- beta/closed-delta/fresh-schema stability.
--
-- It deliberately does not identify these Agda codes with the production
-- Rust syntax, kernel conversion checker, Q0 inventory, or semantic-family
-- inventory.  A checker transcript for this package therefore establishes a
-- theorem foundation, not the production metatheory capability.
data ProductionCorrespondenceFrontier : Set where
  rust-kernel-conversion-q0-family-correspondence-not-mechanized :
    ProductionCorrespondenceFrontier

production-correspondence-frontier :
  ProductionCorrespondenceFrontier
production-correspondence-frontier =
  rust-kernel-conversion-q0-family-correspondence-not-mechanized
