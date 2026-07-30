{-# OPTIONS --safe --without-K #-}

module LawV2.LambdaUnit.LambdaUnitProductionRefinementV1 where

open import LawV2.LambdaUnit.ProductionSyntaxV1 public
open import LawV2.LambdaUnit.ProductionDecodingV1 public
open import LawV2.LambdaUnit.ProductionConversionTyping public
open import LawV2.LambdaUnit.ProductionSynthesisCodeV2 public

-- Checked by this package:
--
-- * intrinsically scoped finite local variables and global slots;
-- * newest-first finite variables and ordinal shift;
-- * oldest-first snoc contexts;
-- * exact variable, finite-slot, and production-image term round trips;
-- * context-extension decoding;
-- * renaming/weakening decoding;
-- * simultaneous-substitution decoding; and
-- * context-entry lookup/shift decoding;
-- * exact SynthesisCodeV2 tags 0 through 7, total and unique;
-- * structural eight-constructor synthesis encode/decode round trips;
-- * decoded conversion-aware typing soundness for the local introduction
--   fragment (Sort, UnitType, Unit, VariableLookup, PiFormation, and
--   LambdaIntroduction);
-- * GlobalLookup soundness conditional on a finite slot typing bridge; and
-- * native ApplicationElimination soundness from conversion-typed function
--   and argument premises, including substitution preservation.
--
-- The remaining authority is deliberately stated rather than assumed.  In
-- particular this package does not relate its finite slots and structural
-- code erasure to the Rust GlobalId table and serialized payloads.
data ProductionRefinementFrontierV1 : Set where
  full-rust-eight-code-decoding-soundness-not-yet-refined :
    ProductionRefinementFrontierV1
  rust-term-code-and-finite-global-slot-agreement-not-yet-refined :
    ProductionRefinementFrontierV1

production-refinement-frontier-v1 :
  ProductionRefinementFrontierV1
production-refinement-frontier-v1 =
  full-rust-eight-code-decoding-soundness-not-yet-refined
