{-# OPTIONS --safe --without-K #-}

module P5RecordBoundary where

open import Agda.Builtin.Bool using (Bool; true; false)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat; zero)

open import CountingLemmas using
  (El; finC; Injective; AtMost; at-most-code)

------------------------------------------------------------------------
-- The conditional internality theorem
------------------------------------------------------------------------

-- Base and Extended stand for depth-2 schema classes modulo the intended
-- operational definitional equality.  The current MBTT syntax does not
-- construct values of this record: its erasure map and both inverse laws are
-- exactly the transparent-elaboration / conservativity evidence still owed.
record TransparentExtension
       (Base Extended : Set)
       (weaken : Base → Extended) : Set where
  constructor transparent
  field
    erase        : Extended → Base
    erase-weaken : (base : Base) → erase (weaken base) ≡ base
    weaken-erase : (extended : Extended) → weaken (erase extended) ≡ extended

record Iso (A B : Set) : Set where
  constructor iso
  field
    to      : A → B
    from    : B → A
    to-from : (b : B) → to (from b) ≡ b
    from-to : (a : A) → from (to a) ≡ a

transparent-conservativity :
  {Base Extended : Set} {weaken : Base → Extended} →
  TransparentExtension Base Extended weaken → Iso Base Extended
transparent-conservativity {weaken = weaken} extension =
  iso
    weaken
    (TransparentExtension.erase extension)
    (TransparentExtension.weaken-erase extension)
    (TransparentExtension.erase-weaken extension)

record MarginalValuation
       (Base Extended : Set)
       (weaken : Base → Extended) : Set where
  field
    nu         : Nat
    internal-0 : Iso Base Extended → nu ≡ zero

transparent-internality :
  {Base Extended : Set} {weaken : Base → Extended} →
  (extension : TransparentExtension Base Extended weaken) →
  (valuation : MarginalValuation Base Extended weaken) →
  MarginalValuation.nu valuation ≡ zero
transparent-internality extension valuation =
  MarginalValuation.internal-0 valuation
    (transparent-conservativity extension)

------------------------------------------------------------------------
-- Empty elimination for marginality and graph counterexamples
------------------------------------------------------------------------

data Empty : Set where

empty-elim : {A : Set} → Empty → A
empty-elim ()

false-not-true : false ≡ true → Empty
false-not-true ()

------------------------------------------------------------------------
-- Operational marginality from weakening and erasure
------------------------------------------------------------------------

-- Equality on Base and Extended is assumed to be the intended typed,
-- univalent equality of normalized natural schema families.  A marginal
-- witness is an extended schema together with evidence that it is not in the
-- image of weakening.  This definition counts families, not their individual
-- type-indexed instances.
record MarginalSchema
       (Base Extended : Set)
       (weaken : Base → Extended) : Set where
  constructor marginal-schema
  field
    schema       : Extended
    not-weakened : (base : Base) → weaken base ≡ schema → Empty

-- The right inverse law alone says that every normalized extended schema is
-- a weakening image.  Hence a transparent extension has no marginal schema.
-- `erase-weaken` is retained in TransparentExtension because it is also
-- needed for full conservativity (old schemas neither collapse nor drift).
transparent-no-marginal :
  {Base Extended : Set} {weaken : Base → Extended} →
  TransparentExtension Base Extended weaken →
  MarginalSchema Base Extended weaken → Empty
transparent-no-marginal extension marginal =
  MarginalSchema.not-weakened marginal
    (TransparentExtension.erase extension
      (MarginalSchema.schema marginal))
    (TransparentExtension.weaken-erase extension
      (MarginalSchema.schema marginal))

transparent-marginal-classify-zero :
  {Base Extended : Set} {weaken : Base → Extended} →
  TransparentExtension Base Extended weaken →
  MarginalSchema Base Extended weaken → El (finC zero)
transparent-marginal-classify-zero extension marginal =
  empty-elim (transparent-no-marginal extension marginal)

transparent-marginal-classify-zero-injective :
  {Base Extended : Set} {weaken : Base → Extended} →
  (extension : TransparentExtension Base Extended weaken) →
  Injective (transparent-marginal-classify-zero extension)
transparent-marginal-classify-zero-injective extension {x} equality =
  empty-elim (transparent-no-marginal extension x)

-- This is the cardinal form of conservative internality.  It follows from
-- weakening/erasure, rather than assuming a valuation-specific `nu = 0` law.
transparent-marginal-at-most-zero :
  {Base Extended : Set} {weaken : Base → Extended} →
  (extension : TransparentExtension Base Extended weaken) →
  AtMost (MarginalSchema Base Extended weaken) zero
transparent-marginal-at-most-zero extension =
  at-most-code
    (transparent-marginal-classify-zero extension)
    (transparent-marginal-classify-zero-injective extension)

-- Guarded Step-15 flows use this theorem once their operational elaboration
-- supplies the two inverse laws.  The theorem deliberately does not claim
-- that the current shallow AST can construct that semantic certificate.
guarded-step15-flow-internality :
  {Base Extended : Set} {weaken : Base → Extended} →
  (extension : TransparentExtension Base Extended weaken) →
  AtMost (MarginalSchema Base Extended weaken) zero
guarded-step15-flow-internality = transparent-marginal-at-most-zero

------------------------------------------------------------------------
-- The fixed Genesis import-DAG premise of P5-record
------------------------------------------------------------------------

data Interface : Set where
  l10 l11 l12 l13 l14 l15 : Interface

-- Reflexive transitive closure of the direct-reference graph
--
--   11 -> 10
--   12 -> 11
--   13 -> 11,12
--   14 -> 11,12,13
--   15 -> 10
--
-- `reaches source target` is true exactly when target is in source's prior
-- dependency closure.  The Rust checker computes this closure from the
-- accepted telescopes; this table is its small, independently checked image.
reaches : Interface → Interface → Bool
reaches l10 l10 = true
reaches l10 _   = false
reaches l11 l10 = true
reaches l11 l11 = true
reaches l11 _   = false
reaches l12 l10 = true
reaches l12 l11 = true
reaches l12 l12 = true
reaches l12 _   = false
reaches l13 l10 = true
reaches l13 l11 = true
reaches l13 l12 = true
reaches l13 l13 = true
reaches l13 _   = false
reaches l14 l10 = true
reaches l14 l11 = true
reaches l14 l12 = true
reaches l14 l13 = true
reaches l14 l14 = true
reaches l14 _   = false
reaches l15 l10 = true
reaches l15 l15 = true
reaches l15 _   = false

data Step13Import : Interface → Set where
  s13-l11 : Step13Import l11
  s13-l12 : Step13Import l12

data Step14Import : Interface → Set where
  s14-l11 : Step14Import l11
  s14-l12 : Step14Import l12
  s14-l13 : Step14Import l13

data SurvivorImport : Interface → Set where
  survivor-l14 : SurvivorImport l14
  survivor-l15 : SurvivorImport l15

record Dominates (Import : Interface → Set) (source : Interface) : Set where
  constructor dominates
  field
    source-is-import : Import source
    reaches-import   : (target : Interface) → Import target →
                       reaches source target ≡ true

step13-dominant : Dominates Step13Import l12
step13-dominant = dominates s13-l12 proof
  where
  proof : (target : Interface) → Step13Import target →
          reaches l12 target ≡ true
  proof .l11 s13-l11 = refl
  proof .l12 s13-l12 = refl

step14-dominant : Dominates Step14Import l13
step14-dominant = dominates s14-l13 proof
  where
  proof : (target : Interface) → Step14Import target →
          reaches l13 target ≡ true
  proof .l11 s14-l11 = refl
  proof .l12 s14-l12 = refl
  proof .l13 s14-l13 = refl

-- P5 asks for a dominant member of the direct imports.  Packaging the source
-- membership inside the record excludes an unrelated interface such as L10.
record SurvivorDominant : Set where
  constructor survivor-dominant
  field
    source        : Interface
    is-import     : SurvivorImport source
    reaches-both  : (target : Interface) → SurvivorImport target →
                    reaches source target ≡ true

-- The surviving Step-16 syntax imports L14 and L15.  L14 cannot reach L15,
-- and L15 cannot reach L14, so the unique/dominant-import premise of the
-- stated P5 theorem is not merely unproved: it is false in the reference DAG.
no-survivor-dominant : SurvivorDominant → Empty
no-survivor-dominant witness with SurvivorDominant.is-import witness
... | survivor-l14 =
  false-not-true
    (SurvivorDominant.reaches-both witness l15 survivor-l15)
... | survivor-l15 =
  false-not-true
    (SurvivorDominant.reaches-both witness l14 survivor-l14)
