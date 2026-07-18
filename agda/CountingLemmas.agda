{-# OPTIONS --safe --without-K #-}

module CountingLemmas where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Nat using (Nat; zero; suc; _+_; _*_)

------------------------------------------------------------------------
-- Semantic boundary
------------------------------------------------------------------------

-- This module machine-checks the finite cardinality part of the two
-- counting lemmas.  It deliberately does not postulate a cubical syntax,
-- a definitional equality, or a fibration semantics that the current MBTT
-- AST does not contain.
--
-- A caller must supply L1Semantic or L2Semantic for a type Schema whose
-- equality already is the intended equality of schema classes (normally:
-- raw schemas modulo definitional interderivability).  The four maps/laws
-- in either record are precisely the outstanding semantic burden:
--
--   * realize proves availability of every displayed basis schema;
--   * classify proves exhaustiveness by assigning every schema a basis tag;
--   * classify-realize proves independence of the tags (including the L1
--     transpose obstruction and L2 face/transport disjointness);
--   * realize-classify proves completeness of normalization (including
--     union/transport staging).
--
-- Thus successful type checking proves the generic equations conditional
-- on those semantic equivalences.  It does not prove that CCHM union-staging
-- is definitional, that a particular weave is strongly connected and
-- nontrivial, that the engine AST satisfies either lemma's hypotheses, or
-- that the Genesis sequence halts after Step 15.

------------------------------------------------------------------------
-- A tiny finite-type universe (no Agda standard-library dependency)
------------------------------------------------------------------------

data Fin : Nat → Set where
  fzero : {n : Nat} → Fin (suc n)
  fsuc  : {n : Nat} → Fin n → Fin (suc n)

data Unit : Set where
  unit : Unit

infixr 4 _⊎_

data _⊎_ (A B : Set) : Set where
  inl : A → A ⊎ B
  inr : B → A ⊎ B

infixr 5 _×_

record _×_ (A B : Set) : Set where
  constructor _,_
  field
    fst : A
    snd : B

data Code : Set where
  unitC : Code
  finC  : Nat → Code
  sumC  : Code → Code → Code
  prodC : Code → Code → Code

El : Code → Set
El unitC       = Unit
El (finC n)    = Fin n
El (sumC a b)  = El a ⊎ El b
El (prodC a b) = El a × El b

size : Code → Nat
size unitC       = 1
size (finC n)    = n
size (sumC a b)  = size a + size b
size (prodC a b) = size a * size b

record Iso (A B : Set) : Set where
  constructor iso
  field
    to      : A → B
    from    : B → A
    to-from : (b : B) → to (from b) ≡ b
    from-to : (a : A) → from (to a) ≡ a

-- A finite presentation is an explicit finite code, an exact equivalence
-- with its interpretation, and the checked size of that code.
record Presented (A : Set) (n : Nat) : Set where
  constructor present
  field
    code       : Code
    exact      : Iso A (El code)
    size-exact : size code ≡ n

------------------------------------------------------------------------
-- Finite upper bounds do not require a saturated presentation
------------------------------------------------------------------------

-- `Presented A n` is deliberately exact: every code point must be realized.
-- Counting marginal schemas only needs the weaker statement that distinct
-- schemas receive distinct finite tags.  Keeping that distinction explicit
-- prevents a sparse semantic family from being forced to realize interactions
-- that do not exist.

infix 4 _≤_

data _≤_ : Nat → Nat → Set where
  z≤n : {n : Nat} → zero ≤ n
  s≤s : {m n : Nat} → m ≤ n → suc m ≤ suc n

≤-refl : {n : Nat} → n ≤ n
≤-refl {zero}  = z≤n
≤-refl {suc n} = s≤s ≤-refl

≤-trans : {m n p : Nat} → m ≤ n → n ≤ p → m ≤ p
≤-trans z≤n       _          = z≤n
≤-trans (s≤s m≤n) (s≤s n≤p) = s≤s (≤-trans m≤n n≤p)

add-fixed-left-mono : (k : Nat) {m n : Nat} → m ≤ n → k + m ≤ k + n
add-fixed-left-mono zero    m≤n = m≤n
add-fixed-left-mono (suc k) m≤n = s≤s (add-fixed-left-mono k m≤n)

-- The varying quantity is the left factor; the fixed right factor is the
-- per-clause capacity.  This is the arithmetic used by the role-cap theorem.
mul-fixed-right-mono : {m n : Nat} → m ≤ n → (k : Nat) → m * k ≤ n * k
mul-fixed-right-mono z≤n       k = z≤n
mul-fixed-right-mono (s≤s m≤n) k =
  add-fixed-left-mono k (mul-fixed-right-mono m≤n k)

Injective : {A B : Set} → (A → B) → Set
Injective f = ∀ {x y} → f x ≡ f y → x ≡ y

record Embedding (A B : Set) : Set where
  constructor embedding
  field
    map       : A → B
    injective : Injective map

-- `AtMost A n` carries an explicit finite code, an injection into that code,
-- and a proof that the code has no more than `n` points.  It says nothing
-- about the unused tags, which is exactly what an upper-bound proof needs.
record AtMost (A : Set) (n : Nat) : Set where
  constructor at-most
  field
    code          : Code
    embeds        : Embedding A (El code)
    size-at-most  : size code ≤ n

at-most-code :
  {A : Set} {code : Code} →
  (classify : A → El code) →
  Injective classify →
  AtMost A (size code)
at-most-code {code = code} classify classify-injective =
  at-most code (embedding classify classify-injective) ≤-refl

at-most-weaken : {A : Set} {m n : Nat} → AtMost A m → m ≤ n → AtMost A n
at-most-weaken bound m≤n =
  at-most
    (AtMost.code bound)
    (AtMost.embeds bound)
    (≤-trans (AtMost.size-at-most bound) m≤n)

------------------------------------------------------------------------
-- L1: one beta schema plus the ordered d by d Kan matrix
------------------------------------------------------------------------

data Positive : Nat → Set where
  positive : {n : Nat} → Positive (suc n)

l1Code : Nat → Code
l1Code d = sumC unitC (prodC (finC d) (finC d))

-- In El (l1Code d), inl unit is beta.  A value inr (a , b) is
-- monodromy when a = b and directed variation when a differs from b.
-- Keeping the coordinates ordered is what preserves the transpose
-- obstruction in the finite basis.
l1-basis-size : (d : Nat) → size (l1Code d) ≡ 1 + d * d
l1-basis-size d = refl

record L1Semantic (Schema : Set) (d : Nat) : Set where
  field
    positive-d       : Positive d
    classify         : Schema → El (l1Code d)
    realize          : El (l1Code d) → Schema
    classify-realize : (tag : El (l1Code d)) →
                       classify (realize tag) ≡ tag
    realize-classify : (schema : Schema) →
                       realize (classify schema) ≡ schema

l1-conditional-cardinality :
  {Schema : Set} {d : Nat} →
  L1Semantic Schema d →
  Presented Schema (1 + d * d)
l1-conditional-cardinality {d = d} semantics =
  present
    (l1Code d)
    (iso
      (L1Semantic.classify semantics)
      (L1Semantic.realize semantics)
      (L1Semantic.classify-realize semantics)
      (L1Semantic.realize-classify semantics))
    (l1-basis-size d)

------------------------------------------------------------------------
-- L2: two faces per clause plus the ordered r by r transport matrix
------------------------------------------------------------------------

data AtLeastTwo : Nat → Set where
  at-least-two : {n : Nat} → AtLeastTwo (suc (suc n))

l2Code : Nat → Nat → Code
l2Code kappa r =
  sumC
    (prodC (finC 2) (finC kappa))
    (prodC (finC r) (finC r))

-- In El (l2Code kappa r), the left summand is (face, clause), with
-- fzero denoting the forward face and fsuc fzero the lifting face.  The
-- right summand is the ordered (source, target) transport tag; its diagonal
-- contains monodromies and its off-diagonal contains directed couplings.
l2-basis-size : (kappa r : Nat) →
                size (l2Code kappa r) ≡ 2 * kappa + r * r
l2-basis-size kappa r = refl

record L2Semantic (Schema : Set) (kappa r : Nat) : Set where
  field
    at-least-two-clauses : AtLeastTwo kappa
    positive-references  : Positive r
    classify             : Schema → El (l2Code kappa r)
    realize              : El (l2Code kappa r) → Schema
    classify-realize     : (tag : El (l2Code kappa r)) →
                           classify (realize tag) ≡ tag
    realize-classify     : (schema : Schema) →
                           realize (classify schema) ≡ schema

l2-conditional-cardinality :
  {Schema : Set} {kappa r : Nat} →
  L2Semantic Schema kappa r →
  Presented Schema (2 * kappa + r * r)
l2-conditional-cardinality {kappa = kappa} {r = r} semantics =
  present
    (l2Code kappa r)
    (iso
      (L2Semantic.classify semantics)
      (L2Semantic.realize semantics)
      (L2Semantic.classify-realize semantics)
      (L2Semantic.realize-classify semantics))
    (l2-basis-size kappa r)
