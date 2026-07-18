{-# OPTIONS --safe --without-K #-}

-- Phase 4 of docs/SEMANTIC_NORMALIZATION_PROGRAM.md: the typed
-- ValidAnchor and debt-free inputs that discharge ProvenanceBound's
-- conditional AtMost boundary for the four certified Step-16 falsifier
-- witnesses, plus the identity TransparentExtension instantiating the
-- guarded Step-15 internality theorem.
--
-- The Marginal data types below mirror, constructor for constructor,
-- the canonical marginal families certified by the Rust kernel
-- (crates/pen-eval/src/typed_families.rs; dispositions pinned in
-- crates/pen-search/src/halting_probe.rs's t1 grader and serialized in
-- the adversarial probe artifact). The quotient happened Rust-side;
-- Agda equality is constructor equality. Each ValidAnchor constructor
-- corresponds to one certified Anchors(f, pi(f)) judgment; the BLAKE3
-- derivation references live in the Rust extraction certificates and
-- travel here as comments. Debt freedom (the empty live-demand summand
-- at stage 16) is encoded by choosing DebtFreeProvenance and is
-- justified by the kernel orbit extraction
-- (crates/pen-eval/src/demand_orbits.rs: semantic O(16) empty with
-- kernel-verified J2/J3/locality).

module ProvenanceWitness16 where

open import Agda.Builtin.Nat using (Nat; zero; suc; _*_)
open import Agda.Builtin.Equality using (_≡_; refl)

open import CountingLemmas using
  (Code; El; Fin; fzero; fsuc; finC; prodC; size;
   Injective; AtMost; _×_; _,_)

open import ProvenanceBound using
  (DebtFreeProvenance; debt-free-provenance;
   debt-free-four-role-at-most; localRoleCode;
   native-head-role; support-interaction-role)

open import P5RecordBoundary using
  (TransparentExtension; transparent; MarginalSchema;
   guarded-step15-flow-internality)

private
  cong : {A B : Set} (f : A → B) {x y : A} → x ≡ y → f x ≡ f y
  cong f refl = refl

  snd-of : {a b : Code} → El (prodC a b) → El b
  snd-of (x , y) = y

------------------------------------------------------------------------
-- hit_no_formation_d1 (kappa 2): exactly one marginal family — the
-- point of the path-attach field (Opaque parameter), generator clause 1,
-- mechanism IntrinsicKernel, local role native-head.
------------------------------------------------------------------------

data MarginalHit : Set where
  -- family: Var(1) over [Opaque] (point of the PathCon field)
  hit-point-of-path-field : MarginalHit

classify-hit : MarginalHit → El (prodC localRoleCode (finC 2))
classify-hit hit-point-of-path-field = native-head-role , fsuc fzero

faithful-hit : Injective classify-hit
faithful-hit {hit-point-of-path-field} {hit-point-of-path-field} _ = refl

data ValidAnchorHit :
       MarginalHit → El (prodC localRoleCode (finC 2)) → Set where
  -- Anchors(f, pi(f)): the family is the kernel-head content of its
  -- generator clause 1 (kernel-v1 mechanism table; faithfulness gated
  -- by the Rust bridge, crates/pen-eval/src/egp.rs).
  anchor-hit-point :
    ValidAnchorHit hit-point-of-path-field (native-head-role , fsuc fzero)

valid-hit :
  (marginal : MarginalHit) → ValidAnchorHit marginal (classify-hit marginal)
valid-hit hit-point-of-path-field = anchor-hit-point

hit-provenance :
  DebtFreeProvenance MarginalHit localRoleCode 2 ValidAnchorHit
hit-provenance = debt-free-provenance classify-hit faithful-hit valid-hit

hit-at-most-four-kappa : AtMost MarginalHit (4 * 2)
hit-at-most-four-kappa = debt-free-four-role-at-most hit-provenance

------------------------------------------------------------------------
-- temporal_polymorphic_kappa2: ZERO marginal families (its one family
-- is internal-identical to the DCT bridge clause). The Marginal type is
-- empty and the certificate is the canonical vacuous one.
------------------------------------------------------------------------

data MarginalTemporal : Set where

classify-temporal : MarginalTemporal → El (prodC localRoleCode (finC 2))
classify-temporal ()

faithful-temporal : Injective classify-temporal
faithful-temporal {()}

data ValidAnchorTemporal :
       MarginalTemporal → El (prodC localRoleCode (finC 2)) → Set where

valid-temporal :
  (marginal : MarginalTemporal) →
  ValidAnchorTemporal marginal (classify-temporal marginal)
valid-temporal ()

temporal-provenance :
  DebtFreeProvenance MarginalTemporal localRoleCode 2 ValidAnchorTemporal
temporal-provenance =
  debt-free-provenance classify-temporal faithful-temporal valid-temporal

temporal-at-most-four-kappa : AtMost MarginalTemporal (4 * 2)
temporal-at-most-four-kappa = debt-free-four-role-at-most temporal-provenance

------------------------------------------------------------------------
-- axiomatic_single_l15_kappa3: three marginal families, one per clause.
-- Clause 0: Pi(Lib 15, ·) — P5InheritedSurface / support-interaction.
-- Clause 1: Sigma(A, A)   — IntrinsicKernel / native-head.
-- Clause 2: App(Lib 15, A) — P5InheritedSurface / support-interaction.
-- Injectivity is by the charged-clause coordinate.
------------------------------------------------------------------------

data MarginalSingleL15 : Set where
  single-pi-lib15    : MarginalSingleL15
  single-sigma-diag  : MarginalSingleL15
  single-app-lib15   : MarginalSingleL15

classify-single : MarginalSingleL15 → El (prodC localRoleCode (finC 3))
classify-single single-pi-lib15   = support-interaction-role , fzero
classify-single single-sigma-diag = native-head-role , fsuc fzero
classify-single single-app-lib15  = support-interaction-role , fsuc (fsuc fzero)

faithful-single : Injective classify-single
faithful-single {single-pi-lib15} {single-pi-lib15} _ = refl
faithful-single {single-sigma-diag} {single-sigma-diag} _ = refl
faithful-single {single-app-lib15} {single-app-lib15} _ = refl
faithful-single {single-pi-lib15} {single-sigma-diag} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()
faithful-single {single-pi-lib15} {single-app-lib15} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()
faithful-single {single-sigma-diag} {single-pi-lib15} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()
faithful-single {single-sigma-diag} {single-app-lib15} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()
faithful-single {single-app-lib15} {single-pi-lib15} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()
faithful-single {single-app-lib15} {single-sigma-diag} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()

data ValidAnchorSingleL15 :
       MarginalSingleL15 → El (prodC localRoleCode (finC 3)) → Set where
  anchor-single-pi :
    ValidAnchorSingleL15 single-pi-lib15 (support-interaction-role , fzero)
  anchor-single-sigma :
    ValidAnchorSingleL15 single-sigma-diag (native-head-role , fsuc fzero)
  anchor-single-app :
    ValidAnchorSingleL15
      single-app-lib15 (support-interaction-role , fsuc (fsuc fzero))

valid-single :
  (marginal : MarginalSingleL15) →
  ValidAnchorSingleL15 marginal (classify-single marginal)
valid-single single-pi-lib15   = anchor-single-pi
valid-single single-sigma-diag = anchor-single-sigma
valid-single single-app-lib15  = anchor-single-app

single-l15-provenance :
  DebtFreeProvenance MarginalSingleL15 localRoleCode 3 ValidAnchorSingleL15
single-l15-provenance =
  debt-free-provenance classify-single faithful-single valid-single

single-l15-at-most-four-kappa : AtMost MarginalSingleL15 (4 * 3)
single-l15-at-most-four-kappa =
  debt-free-four-role-at-most single-l15-provenance

------------------------------------------------------------------------
-- axiomatic_inheritance_kappa3: the same three-slot shape (its terminal
-- clause applies Lib 14 instead of Lib 15; the marginal family
-- inventory and anchors are isomorphic).
------------------------------------------------------------------------

data MarginalInheritance : Set where
  inheritance-pi-lib15   : MarginalInheritance
  inheritance-sigma-diag : MarginalInheritance
  inheritance-app-lib14  : MarginalInheritance

classify-inheritance :
  MarginalInheritance → El (prodC localRoleCode (finC 3))
classify-inheritance inheritance-pi-lib15 = support-interaction-role , fzero
classify-inheritance inheritance-sigma-diag = native-head-role , fsuc fzero
classify-inheritance inheritance-app-lib14 =
  support-interaction-role , fsuc (fsuc fzero)

faithful-inheritance : Injective classify-inheritance
faithful-inheritance {inheritance-pi-lib15} {inheritance-pi-lib15} _ = refl
faithful-inheritance {inheritance-sigma-diag} {inheritance-sigma-diag} _ = refl
faithful-inheritance {inheritance-app-lib14} {inheritance-app-lib14} _ = refl
faithful-inheritance {inheritance-pi-lib15} {inheritance-sigma-diag} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()
faithful-inheritance {inheritance-pi-lib15} {inheritance-app-lib14} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()
faithful-inheritance {inheritance-sigma-diag} {inheritance-pi-lib15} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()
faithful-inheritance {inheritance-sigma-diag} {inheritance-app-lib14} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()
faithful-inheritance {inheritance-app-lib14} {inheritance-pi-lib15} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()
faithful-inheritance {inheritance-app-lib14} {inheritance-sigma-diag} eq
  with cong (snd-of {localRoleCode} {finC 3}) eq
... | ()

data ValidAnchorInheritance :
       MarginalInheritance → El (prodC localRoleCode (finC 3)) → Set where
  anchor-inheritance-pi :
    ValidAnchorInheritance
      inheritance-pi-lib15 (support-interaction-role , fzero)
  anchor-inheritance-sigma :
    ValidAnchorInheritance
      inheritance-sigma-diag (native-head-role , fsuc fzero)
  anchor-inheritance-app :
    ValidAnchorInheritance
      inheritance-app-lib14 (support-interaction-role , fsuc (fsuc fzero))

valid-inheritance :
  (marginal : MarginalInheritance) →
  ValidAnchorInheritance marginal (classify-inheritance marginal)
valid-inheritance inheritance-pi-lib15   = anchor-inheritance-pi
valid-inheritance inheritance-sigma-diag = anchor-inheritance-sigma
valid-inheritance inheritance-app-lib14  = anchor-inheritance-app

inheritance-provenance :
  DebtFreeProvenance
    MarginalInheritance localRoleCode 3 ValidAnchorInheritance
inheritance-provenance =
  debt-free-provenance
    classify-inheritance faithful-inheritance valid-inheritance

inheritance-at-most-four-kappa : AtMost MarginalInheritance (4 * 3)
inheritance-at-most-four-kappa =
  debt-free-four-role-at-most inheritance-provenance

------------------------------------------------------------------------
-- Guarded Step-15 internality: the strict guarded subspace's weakening
-- and erasure maps are pointwise identities (kernel certificate:
-- crates/pen-eval/src/internality.rs — every family of the five guarded
-- flows is a checked weakening image of a closure family with a typed
-- equality witness). The identity TransparentExtension instantiates
-- the operational internality theorem: AtMost Marginal zero.
------------------------------------------------------------------------

data GuardedStep15Flow : Set where
  reproposal-step13 : GuardedStep15Flow
  reproposal-step14 : GuardedStep15Flow
  reproposal-step15 : GuardedStep15Flow
  bare-variable     : GuardedStep15Flow
  trunc-hybrid      : GuardedStep15Flow

guarded-weaken : GuardedStep15Flow → GuardedStep15Flow
guarded-weaken flow = flow

guarded-transparent :
  TransparentExtension GuardedStep15Flow GuardedStep15Flow guarded-weaken
guarded-transparent =
  transparent (λ flow → flow) (λ _ → refl) (λ _ → refl)

guarded-step15-marginal-zero :
  AtMost
    (MarginalSchema GuardedStep15Flow GuardedStep15Flow guarded-weaken)
    zero
guarded-step15-marginal-zero =
  guarded-step15-flow-internality guarded-transparent
