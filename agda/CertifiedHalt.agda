{-# OPTIONS --safe --without-K #-}

module CertifiedHalt where

open import Agda.Builtin.Bool using (Bool; true; false)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat; zero; suc; _+_; _*_)

open import CountingLemmas using
  (Code; El; Injective; AtMost; finC; sumC; prodC; size; at-most-code)

open import ProvenanceBound using
  (DebtFreeProvenance; localRoleCode;
   debt-free-four-role-at-most; debt-free-local-at-most-nine)

------------------------------------------------------------------------
-- Frozen proof-carrying amplification boundary
------------------------------------------------------------------------

-- Evidence propositions are supplied by a typed semantic elaborator.  This
-- module intentionally declares no constructors for them: in particular,
-- there is no globally available inhabitant of dominant-import evidence.
record HitAmplification
       (FreshHeadEvidence FormationEvidence EliminatorEvidence : Set) : Set where
  constructor hit-amplification
  field
    fresh       : FreshHeadEvidence
    formation   : FormationEvidence
    eliminator  : EliminatorEvidence

record P5Amplification
       (FreshHeadEvidence IrreducibilityEvidence
        DominantImportEvidence EliminatorEvidence : Set) : Set where
  constructor p5-amplification
  field
    fresh         : FreshHeadEvidence
    irreducible   : IrreducibilityEvidence
    dominant      : DominantImportEvidence
    eliminator    : EliminatorEvidence

record TemporalAmplification
       (FreshHeadEvidence EliminatorEvidence NaturalityEvidence : Set) : Set where
  constructor temporal-amplification
  field
    fresh        : FreshHeadEvidence
    eliminator   : EliminatorEvidence
    naturality   : NaturalityEvidence

------------------------------------------------------------------------
-- Support-local normal forms
------------------------------------------------------------------------

-- After weakening-only schemas are removed, a normalized depth-two schema
-- with a first fresh head has one of three shapes:
--
--   * a fresh head itself                         : k
--   * an incoming/outgoing old-support probe      : 2*k*r
--   * an ordered interaction of two fresh heads   : k*k
--
-- Old/old composites contain no first fresh head.  They are inherited
-- weakening images, not marginal schemas.  Longer composites normalize to
-- these depth-two forms.
supportLocalCode : Nat → Nat → Code
supportLocalCode k r =
  sumC
    (finC k)
    (sumC
      (prodC (finC 2) (prodC (finC k) (finC r)))
      (prodC (finC k) (finC k)))

record SupportLocalSemantic (Schema : Set) (k r : Nat) : Set where
  constructor support-local
  field
    classify : Schema → El (supportLocalCode k r)
    faithful : Injective classify

-- This is an upper bound, not an exact presentation.  In particular it does
-- not require every ordered fresh/fresh pair or old-support probe to exist.
-- Constructing `SupportLocalSemantic` from the intended typed operational
-- semantics remains the explicit normalization/classification obligation.
support-local-at-most :
  {Schema : Set} {k r : Nat} →
  SupportLocalSemantic Schema k r →
  AtMost Schema (size (supportLocalCode k r))
support-local-at-most semantics =
  at-most-code
    (SupportLocalSemantic.classify semantics)
    (SupportLocalSemantic.faithful semantics)

------------------------------------------------------------------------
-- Exact Step-16 cap calculation, derived before consulting the bar
------------------------------------------------------------------------

data Kappa : Set where
  k2 k3 k4 : Kappa

data References : Set where
  r0 r1 r2 : References

kappaNat : Kappa → Nat
kappaNat k2 = 2
kappaNat k3 = 3
kappaNat k4 = 4

referenceNat : References → Nat
referenceNat r0 = 0
referenceNat r1 = 1
referenceNat r2 = 2

-- This is computed from the finite normal-form code, not chosen from Bar16.
opaqueBound : Kappa → References → Nat
opaqueBound k r = size (supportLocalCode (kappaNat k) (referenceNat r))

nineKappa : Kappa → Nat
nineKappa k = 9 * kappaNat k

fourKappa : Kappa → Nat
fourKappa k = 4 * kappaNat k

-- The 4*kappa refinement is conditional on a faithful four-role provenance
-- classifier.  No such classifier is inferred from a candidate class label.
debt-free-step16-at-most-four :
  {Schema : Set} →
  (k : Kappa) →
  {ValidAnchor :
    Schema →
    El (prodC localRoleCode (finC (kappaNat k))) → Set} →
  DebtFreeProvenance
    Schema localRoleCode (kappaNat k) ValidAnchor →
  AtMost Schema (fourKappa k)
debt-free-step16-at-most-four k = debt-free-four-role-at-most

-- The weaker 9*kappa envelope is a weakening of the very same valid four-role
-- certificate, not an independent fallback.  Its arithmetic is independent
-- of (and defined before) the Step-16 bar.
debt-free-step16-at-most-nine :
  {Schema : Set} →
  (k : Kappa) →
  {ValidAnchor :
    Schema →
    El (prodC localRoleCode (finC (kappaNat k))) → Set} →
  DebtFreeProvenance
    Schema localRoleCode (kappaNat k) ValidAnchor →
  AtMost Schema (nineKappa k)
debt-free-step16-at-most-nine k = debt-free-local-at-most-nine

infix 4 _≤ᵇ_

_≤ᵇ_ : Nat → Nat → Bool
zero  ≤ᵇ n     = true
suc m ≤ᵇ zero  = false
suc m ≤ᵇ suc n = m ≤ᵇ n

-- All nine surface-cap cases are closed computation.  The tight case is
-- kappa=4, r=2: 4 + 2*4*2 + 4*4 = 36 = 9*4.
support-local-below-nine :
  (k : Kappa) (r : References) →
  (opaqueBound k r ≤ᵇ nineKappa k) ≡ true
support-local-below-nine k2 r0 = refl
support-local-below-nine k2 r1 = refl
support-local-below-nine k2 r2 = refl
support-local-below-nine k3 r0 = refl
support-local-below-nine k3 r1 = refl
support-local-below-nine k3 r2 = refl
support-local-below-nine k4 r0 = refl
support-local-below-nine k4 r1 = refl
support-local-below-nine k4 r2 = refl

-- Frozen Bar16 arithmetic: Bar16 = 354333/39040 = 9 + 2973/39040.
bar16-above-nine : 354333 ≡ 9 * 39040 + 2973
bar16-above-nine = refl

-- Directly check the rational comparison as integer cross multiplication.
-- This avoids importing any division or rational-number implementation:
--
--   opaqueBound / kappa < 354333 / 39040
--
-- iff
--
--   39040 * opaqueBound < 354333 * kappa.
ltb : Nat → Nat → Bool
ltb zero    zero    = false
ltb zero    (suc n) = true
ltb (suc m) zero    = false
ltb (suc m) (suc n) = ltb m n

support-local-below-bar16 :
  (k : Kappa) (r : References) →
  ltb
    (39040 * opaqueBound k r)
    (354333 * kappaNat k)
  ≡ true
support-local-below-bar16 k2 r0 = refl
support-local-below-bar16 k2 r1 = refl
support-local-below-bar16 k2 r2 = refl
support-local-below-bar16 k3 r0 = refl
support-local-below-bar16 k3 r1 = refl
support-local-below-bar16 k3 r2 = refl
support-local-below-bar16 k4 r0 = refl
support-local-below-bar16 k4 r1 = refl
support-local-below-bar16 k4 r2 = refl

-- Once a semantic provenance certificate supplies the preceding AtMost
-- theorem, this independent arithmetic closes the strict Step-16 comparison.
nine-kappa-below-bar16 :
  (k : Kappa) →
  ltb
    (39040 * nineKappa k)
    (354333 * kappaNat k)
  ≡ true
nine-kappa-below-bar16 k2 = refl
nine-kappa-below-bar16 k3 = refl
nine-kappa-below-bar16 k4 = refl

four-kappa-below-nine :
  (k : Kappa) →
  (fourKappa k ≤ᵇ nineKappa k) ≡ true
four-kappa-below-nine k2 = refl
four-kappa-below-nine k3 = refl
four-kappa-below-nine k4 = refl

-- Exact maximum support-local cardinalities at the widest reference window.
k2-r2-bound : opaqueBound k2 r2 ≡ 14
k2-r2-bound = refl

k3-r2-bound : opaqueBound k3 r2 ≡ 24
k3-r2-bound = refl

k4-r2-bound : opaqueBound k4 r2 ≡ 36
k4-r2-bound = refl

------------------------------------------------------------------------
-- Exact frozen proof-carrying class calculus
------------------------------------------------------------------------

-- The Rust certificate uses these nine support-local class ceilings.  They
-- are recorded here independently so that drift in any one class formula is
-- visible to Agda.  Step 16 has r <= 2 and d <= 1.  No Bar16 constant occurs
-- in the definitions below.
data CandidateClass : Set where
  foundation former hit suspension map modal axiomatic synthesis unknown :
    CandidateClass

modalPairs : Kappa → Nat
modalPairs k2 = 1
modalPairs k3 = 3
modalPairs k4 = 6

classCeiling : Kappa → CandidateClass → Nat
classCeiling k foundation = kappaNat k
classCeiling k former     = 2 * kappaNat k
classCeiling k hit        = 3 * kappaNat k + 2
classCeiling k suspension = 5
classCeiling k map        = 2 * kappaNat k + 4
classCeiling k modal      = kappaNat k + 2 + modalPairs k
classCeiling k axiomatic  = 3 * kappaNat k + 1
classCeiling k synthesis  = 3 * kappaNat k
classCeiling k unknown    = 2 * kappaNat k

certifiedMaximum : Kappa → Nat
certifiedMaximum k2 = 8
certifiedMaximum k3 = 11
certifiedMaximum k4 = 14

-- Exhaust the complete 3 x 9 finite table.
class-ceiling-below-maximum :
  (k : Kappa) (candidateClass : CandidateClass) →
  (classCeiling k candidateClass ≤ᵇ certifiedMaximum k) ≡ true
class-ceiling-below-maximum k2 foundation = refl
class-ceiling-below-maximum k2 former = refl
class-ceiling-below-maximum k2 hit = refl
class-ceiling-below-maximum k2 suspension = refl
class-ceiling-below-maximum k2 map = refl
class-ceiling-below-maximum k2 modal = refl
class-ceiling-below-maximum k2 axiomatic = refl
class-ceiling-below-maximum k2 synthesis = refl
class-ceiling-below-maximum k2 unknown = refl
class-ceiling-below-maximum k3 foundation = refl
class-ceiling-below-maximum k3 former = refl
class-ceiling-below-maximum k3 hit = refl
class-ceiling-below-maximum k3 suspension = refl
class-ceiling-below-maximum k3 map = refl
class-ceiling-below-maximum k3 modal = refl
class-ceiling-below-maximum k3 axiomatic = refl
class-ceiling-below-maximum k3 synthesis = refl
class-ceiling-below-maximum k3 unknown = refl
class-ceiling-below-maximum k4 foundation = refl
class-ceiling-below-maximum k4 former = refl
class-ceiling-below-maximum k4 hit = refl
class-ceiling-below-maximum k4 suspension = refl
class-ceiling-below-maximum k4 map = refl
class-ceiling-below-maximum k4 modal = refl
class-ceiling-below-maximum k4 axiomatic = refl
class-ceiling-below-maximum k4 synthesis = refl
class-ceiling-below-maximum k4 unknown = refl

-- Direct rational comparison of the exact maxima.  The three computations
-- respectively check 39040*8 < 354333*2,
-- 39040*11 < 354333*3, and 39040*14 < 354333*4.
certified-maximum-below-bar16 :
  (k : Kappa) →
  ltb
    (39040 * certifiedMaximum k)
    (354333 * kappaNat k)
  ≡ true
certified-maximum-below-bar16 k2 = refl
certified-maximum-below-bar16 k3 = refl
certified-maximum-below-bar16 k4 = refl

certified-k2-maximum : certifiedMaximum k2 ≡ 8
certified-k2-maximum = refl

certified-k3-maximum : certifiedMaximum k3 ≡ 11
certified-k3-maximum = refl

certified-k4-maximum : certifiedMaximum k4 ≡ 14
certified-k4-maximum = refl
