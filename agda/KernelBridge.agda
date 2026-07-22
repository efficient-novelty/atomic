{-# OPTIONS --safe --without-K #-}

module KernelBridge where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat; zero; suc; _+_; _*_)

open import CountingLemmas using (_≤_; z≤n; s≤s; ≤-trans)

------------------------------------------------------------------------
-- The frozen candidate-clause grammar, intrinsically scoped
------------------------------------------------------------------------

-- This is the subgrammar used by the finite candidate-join catalog.  The
-- scope index makes an out-of-scope variable unrepresentable.  In
-- particular, `substitute` below is a well-scoping theorem by its type:
-- every simultaneous substitution from an m-scope into an n-scope maps an
-- `Expr m` to an `Expr n`.

data Fin : Nat → Set where
  fzero : {n : Nat} → Fin (suc n)
  fsuc  : {n : Nat} → Fin n → Fin (suc n)

data Expr (scope : Nat) : Set where
  univ       : Expr scope
  var        : Fin scope → Expr scope
  lib        : Nat → Expr scope
  pathCon    : Nat → Expr scope
  app        : Expr scope → Expr scope → Expr scope
  pi         : Expr scope → Expr (suc scope) → Expr scope
  sigma      : Expr scope → Expr (suc scope) → Expr scope
  lam        : Expr (suc scope) → Expr scope
  identity   : Expr scope → Expr scope → Expr scope → Expr scope
  reflexivity : Expr scope → Expr scope
  suspension : Expr scope → Expr scope
  truncation : Expr scope → Expr scope
  flat       : Expr scope → Expr scope
  sharp      : Expr scope → Expr scope
  disc       : Expr scope → Expr scope
  shape      : Expr scope → Expr scope
  next       : Expr scope → Expr scope
  eventually : Expr scope → Expr scope
  bang       : Expr scope → Expr scope
  whyNot     : Expr scope → Expr scope

Renaming : Nat → Nat → Set
Renaming source target = Fin source → Fin target

Substitution : Nat → Nat → Set
Substitution source target = Fin source → Expr target

mutual
  rename : {source target : Nat} → Renaming source target → Expr source → Expr target
  rename ρ univ = univ
  rename ρ (var x) = var (ρ x)
  rename ρ (lib step) = lib step
  rename ρ (pathCon dimension) = pathCon dimension
  rename ρ (app function argument) = app (rename ρ function) (rename ρ argument)
  rename ρ (pi domain codomain) =
    pi (rename ρ domain) (rename (liftRenaming ρ) codomain)
  rename ρ (sigma domain codomain) =
    sigma (rename ρ domain) (rename (liftRenaming ρ) codomain)
  rename ρ (lam body) = lam (rename (liftRenaming ρ) body)
  rename ρ (identity type left right) =
    identity (rename ρ type) (rename ρ left) (rename ρ right)
  rename ρ (reflexivity body) = reflexivity (rename ρ body)
  rename ρ (suspension body) = suspension (rename ρ body)
  rename ρ (truncation body) = truncation (rename ρ body)
  rename ρ (flat body) = flat (rename ρ body)
  rename ρ (sharp body) = sharp (rename ρ body)
  rename ρ (disc body) = disc (rename ρ body)
  rename ρ (shape body) = shape (rename ρ body)
  rename ρ (next body) = next (rename ρ body)
  rename ρ (eventually body) = eventually (rename ρ body)
  rename ρ (bang body) = bang (rename ρ body)
  rename ρ (whyNot body) = whyNot (rename ρ body)

  liftRenaming : {source target : Nat} →
                 Renaming source target → Renaming (suc source) (suc target)
  liftRenaming ρ fzero = fzero
  liftRenaming ρ (fsuc x) = fsuc (ρ x)

weaken : {scope : Nat} → Expr scope → Expr (suc scope)
weaken = rename fsuc

liftSubstitution : {source target : Nat} →
                   Substitution source target →
                   Substitution (suc source) (suc target)
liftSubstitution σ fzero = var fzero
liftSubstitution σ (fsuc x) = weaken (σ x)

substitute : {source target : Nat} →
             Substitution source target → Expr source → Expr target
substitute σ univ = univ
substitute σ (var x) = σ x
substitute σ (lib step) = lib step
substitute σ (pathCon dimension) = pathCon dimension
substitute σ (app function argument) =
  app (substitute σ function) (substitute σ argument)
substitute σ (pi domain codomain) =
  pi (substitute σ domain) (substitute (liftSubstitution σ) codomain)
substitute σ (sigma domain codomain) =
  sigma (substitute σ domain) (substitute (liftSubstitution σ) codomain)
substitute σ (lam body) = lam (substitute (liftSubstitution σ) body)
substitute σ (identity type left right) =
  identity (substitute σ type) (substitute σ left) (substitute σ right)
substitute σ (reflexivity body) = reflexivity (substitute σ body)
substitute σ (suspension body) = suspension (substitute σ body)
substitute σ (truncation body) = truncation (substitute σ body)
substitute σ (flat body) = flat (substitute σ body)
substitute σ (sharp body) = sharp (substitute σ body)
substitute σ (disc body) = disc (substitute σ body)
substitute σ (shape body) = shape (substitute σ body)
substitute σ (next body) = next (substitute σ body)
substitute σ (eventually body) = eventually (substitute σ body)
substitute σ (bang body) = bang (substitute σ body)
substitute σ (whyNot body) = whyNot (substitute σ body)

-- An explicit witness exposing the intrinsic well-scoping result to callers.
-- No semantic typing or naturality claim is hidden in this record.
record ScopedSubstitutionWitness (source target : Nat) : Set where
  constructor scoped-substitution
  field
    substitution : Substitution source target
    apply        : Expr source → Expr target
    agrees       : (expression : Expr source) →
                   apply expression ≡ substitute substitution expression

scopedSubstitutionWitness :
  {source target : Nat} →
  (σ : Substitution source target) →
  ScopedSubstitutionWitness source target
scopedSubstitutionWitness σ = scoped-substitution σ (substitute σ) (λ _ → refl)

------------------------------------------------------------------------
-- Restricted parameter-sort preservation
------------------------------------------------------------------------

-- The operational kernel currently distinguishes only type parameters from
-- opaque parameters.  This mirror therefore admits exactly the two cases the
-- Rust bridge can justify without inventing a type for an opaque term:
--
--   * a Type parameter is renamed to another Type variable;
--   * an Opaque parameter is renamed to another Opaque variable.
--
-- There is deliberately no constructor for an arbitrary expression image.

data ParameterSort : Set where
  type-sort   : ParameterSort
  opaque-sort : ParameterSort

data RestrictedVariableImage : ParameterSort → ParameterSort → Set where
  type-variable-renaming   : RestrictedVariableImage type-sort type-sort
  opaque-variable-renaming : RestrictedVariableImage opaque-sort opaque-sort

restricted-variable-sort-preservation :
  {source target : ParameterSort} →
  RestrictedVariableImage source target → source ≡ target
restricted-variable-sort-preservation type-variable-renaming = refl
restricted-variable-sort-preservation opaque-variable-renaming = refl

------------------------------------------------------------------------
-- Clause-local fuel composes to the whole-telescope bound
------------------------------------------------------------------------

data List (A : Set) : Set where
  []  : List A
  _∷_ : A → List A → List A

infixr 5 _∷_

-- A local witness is supplied by the executable checker.  Its only semantic
-- content here is the independently checkable inequality
--
--     spent ≤ nodeCount * kappa.

record ClauseFuelEvidence (kappa : Nat) : Set where
  constructor clause-fuel
  field
    nodeCount : Nat
    spent     : Nat
    within    : spent ≤ nodeCount * kappa

sumNodes : {kappa : Nat} → List (ClauseFuelEvidence kappa) → Nat
sumNodes [] = zero
sumNodes (clause ∷ clauses) =
  ClauseFuelEvidence.nodeCount clause + sumNodes clauses

sumSpent : {kappa : Nat} → List (ClauseFuelEvidence kappa) → Nat
sumSpent [] = zero
sumSpent (clause ∷ clauses) =
  ClauseFuelEvidence.spent clause + sumSpent clauses

≤-step : {m n : Nat} → m ≤ n → m ≤ suc n
≤-step z≤n = z≤n
≤-step (s≤s proof) = s≤s (≤-step proof)

≤-refl : (n : Nat) → n ≤ n
≤-refl zero = z≤n
≤-refl (suc n) = s≤s (≤-refl n)

right-extension : (prefix suffix : Nat) → suffix ≤ prefix + suffix
right-extension zero suffix = ≤-refl suffix
right-extension (suc prefix) suffix =
  ≤-step (right-extension prefix suffix)

add-fixed-right-mono : {m n : Nat} → m ≤ n → (suffix : Nat) →
                       m + suffix ≤ n + suffix
add-fixed-right-mono (z≤n {n}) suffix = right-extension n suffix
add-fixed-right-mono (s≤s proof) suffix =
  s≤s (add-fixed-right-mono proof suffix)

add-fixed-left-mono : (prefix : Nat) → {m n : Nat} → m ≤ n →
                      prefix + m ≤ prefix + n
add-fixed-left-mono zero proof = proof
add-fixed-left-mono (suc prefix) proof =
  s≤s (add-fixed-left-mono prefix proof)

add-mono : {a b c d : Nat} → a ≤ b → c ≤ d → a + c ≤ b + d
add-mono {b = b} {c = c} first second =
  ≤-trans
    (add-fixed-right-mono first c)
    (add-fixed-left-mono b second)

+-assoc : (a b c : Nat) → (a + b) + c ≡ a + (b + c)
+-assoc zero b c = refl
+-assoc (suc a) b c rewrite +-assoc a b c = refl

*-distrib-+ : (a b kappa : Nat) →
              (a + b) * kappa ≡ a * kappa + b * kappa
*-distrib-+ zero b kappa = refl
*-distrib-+ (suc a) b kappa
  rewrite *-distrib-+ a b kappa
        | +-assoc kappa (a * kappa) (b * kappa) = refl

-- The explicit distributivity lemma connects the sum of the clause-local
-- allocations to the whole telescope's `totalNodes * kappa` allocation.
sum-local-fuel-within :
  {kappa : Nat} →
  (clauses : List (ClauseFuelEvidence kappa)) →
  sumSpent clauses ≤ sumNodes clauses * kappa
sum-local-fuel-within [] = z≤n
sum-local-fuel-within {kappa} (clause ∷ clauses)
  rewrite *-distrib-+
            (ClauseFuelEvidence.nodeCount clause)
            (sumNodes clauses)
            kappa =
  add-mono
    (ClauseFuelEvidence.within clause)
    (sum-local-fuel-within clauses)

record WholeTelescopeFuelEvidence
       (kappa : Nat)
       (clauses : List (ClauseFuelEvidence kappa)) : Set where
  constructor whole-telescope-fuel
  field
    within-global-bound :
      sumSpent clauses ≤ sumNodes clauses * kappa

compose-clause-fuel :
  {kappa : Nat} →
  (clauses : List (ClauseFuelEvidence kappa)) →
  WholeTelescopeFuelEvidence kappa clauses
compose-clause-fuel clauses =
  whole-telescope-fuel (sum-local-fuel-within clauses)

------------------------------------------------------------------------
-- Scope boundary
------------------------------------------------------------------------

-- This module proves only:
--
--   1. simultaneous substitution preserves the finite grammar's scope;
--   2. the two explicitly supported parameter-image cases preserve sorts;
--   3. checked clause-local fuel inequalities compose arithmetically.
--
-- It does not define or prove complete semantic naturality, a depth-two
-- schema classifier, a score, a ranking judgement, or an acceptance result.
