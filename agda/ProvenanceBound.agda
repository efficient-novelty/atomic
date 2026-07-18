{-# OPTIONS --safe --without-K #-}

module ProvenanceBound where

open import Agda.Builtin.Nat using (Nat; _*_)

open import CountingLemmas using
  (Code; El; Fin; fzero; fsuc; finC; sumC; prodC; size;
   Injective; AtMost; at-most-code; at-most-weaken;
   _≤_; z≤n; s≤s; ≤-refl; mul-fixed-right-mono)

------------------------------------------------------------------------
-- Extraction-guarded debt-credit provenance
------------------------------------------------------------------------

-- The left summand records intrinsic local credit.  Its first coordinate is
-- a finite local role and its second coordinate is a charged kernel clause.
-- The right summand is supplied by *previously live* semantic demand orbits.
-- Consequently an independent schema must either be anchored to a clause or
-- be assigned a live-demand witness; merely mentioning an old interface does
-- not create another tag.
provenanceCode : Code → Nat → Code → Code
provenanceCode roles kappa liveDemand =
  sumC (prodC roles (finC kappa)) liveDemand

-- This record is the semantic proof obligation.  Agda does not manufacture a
-- classifier from the shallow telescope syntax.  `faithful` proves that no
-- two schema classes collide.  `valid` proves, separately, that the assigned
-- tag is genuinely anchored to that schema; this is where normalization,
-- marginality, natural-family quotienting, and the rule that independently
-- exported instances require distinct demand-orbit evidence are enforced.
record ExtractionGuardedProvenance
       (Marginal : Set)
       (roles : Code)
       (kappa : Nat)
       (liveDemand : Code)
       (ValidAnchor :
          Marginal → El (provenanceCode roles kappa liveDemand) → Set) : Set where
  constructor provenance
  field
    classify : Marginal → El (provenanceCode roles kappa liveDemand)
    faithful : Injective classify
    valid    : (marginal : Marginal) → ValidAnchor marginal (classify marginal)

provenance-at-most :
  {Marginal : Set} {roles liveDemand : Code} {kappa : Nat} →
  {ValidAnchor :
    Marginal → El (provenanceCode roles kappa liveDemand) → Set} →
  ExtractionGuardedProvenance
    Marginal roles kappa liveDemand ValidAnchor →
  AtMost Marginal (size (provenanceCode roles kappa liveDemand))
provenance-at-most certificate =
  at-most-code
    (ExtractionGuardedProvenance.classify certificate)
    (ExtractionGuardedProvenance.faithful certificate)

------------------------------------------------------------------------
-- Debt exhaustion removes the amplification summand
------------------------------------------------------------------------

-- A debt-free certificate targets only the clause-anchored local summand.
-- It is separate from the full record so no inhabitant of an empty demand
-- code has to be postulated or eliminated by the semantic implementation.
record DebtFreeProvenance
       (Marginal : Set)
       (roles : Code)
       (kappa : Nat)
       (ValidAnchor :
          Marginal → El (prodC roles (finC kappa)) → Set) : Set where
  constructor debt-free-provenance
  field
    classify : Marginal → El (prodC roles (finC kappa))
    faithful : Injective classify
    valid    : (marginal : Marginal) → ValidAnchor marginal (classify marginal)

debt-free-at-most :
  {Marginal : Set} {roles : Code} {kappa : Nat} →
  {ValidAnchor : Marginal → El (prodC roles (finC kappa)) → Set} →
  DebtFreeProvenance Marginal roles kappa ValidAnchor →
  AtMost Marginal (size roles * kappa)
debt-free-at-most certificate =
  at-most-code
    (DebtFreeProvenance.classify certificate)
    (DebtFreeProvenance.faithful certificate)

record RoleCap (roles : Code) (ceiling : Nat) : Set where
  constructor role-cap
  field
    bounded : size roles ≤ ceiling

debt-free-under-role-cap :
  {Marginal : Set} {roles : Code} {kappa ceiling : Nat} →
  {ValidAnchor : Marginal → El (prodC roles (finC kappa)) → Set} →
  RoleCap roles ceiling →
  DebtFreeProvenance Marginal roles kappa ValidAnchor →
  AtMost Marginal (ceiling * kappa)
debt-free-under-role-cap {kappa = kappa} cap certificate =
  at-most-weaken
    (debt-free-at-most certificate)
    (mul-fixed-right-mono (RoleCap.bounded cap) kappa)

------------------------------------------------------------------------
-- A bar-blind four-role refinement and its conservative nine-role envelope
------------------------------------------------------------------------

-- These four values are slots, not a claim that the intended operational
-- schemas have already been classified.  A caller earns the 4*kappa bound
-- only by constructing `DebtFreeProvenance ... localRoleCode kappa
-- ValidAnchor`, including a proof that every assigned tag satisfies that
-- externally supplied semantic relation.
localRoleCode : Code
localRoleCode = finC 4

native-head-role : El localRoleCode
native-head-role = fzero

local-eliminator-role : El localRoleCode
local-eliminator-role = fsuc fzero

support-interaction-role : El localRoleCode
support-interaction-role = fsuc (fsuc fzero)

coherence-role : El localRoleCode
coherence-role = fsuc (fsuc (fsuc fzero))

local-role-cap-four : RoleCap localRoleCode 4
local-role-cap-four = role-cap ≤-refl

local-role-cap-nine : RoleCap localRoleCode 9
local-role-cap-nine =
  role-cap (s≤s (s≤s (s≤s (s≤s z≤n))))

debt-free-four-role-at-most :
  {Marginal : Set} {kappa : Nat} →
  {ValidAnchor :
    Marginal → El (prodC localRoleCode (finC kappa)) → Set} →
  DebtFreeProvenance Marginal localRoleCode kappa ValidAnchor →
  AtMost Marginal (4 * kappa)
debt-free-four-role-at-most =
  debt-free-under-role-cap local-role-cap-four

-- This theorem contains no Genesis bar constant.  It weakens the same
-- four-role certificate from 4*kappa to 9*kappa; it is not an independent
-- nine-role fallback.  Comparison with a historical bar is a later,
-- logically separate calculation.
debt-free-local-at-most-nine :
  {Marginal : Set} {kappa : Nat} →
  {ValidAnchor :
    Marginal → El (prodC localRoleCode (finC kappa)) → Set} →
  DebtFreeProvenance Marginal localRoleCode kappa ValidAnchor →
  AtMost Marginal (9 * kappa)
debt-free-local-at-most-nine =
  debt-free-under-role-cap local-role-cap-nine
