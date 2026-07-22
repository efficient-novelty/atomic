{-# OPTIONS --safe --without-K #-}

module Schema2E3E4 where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Unit using (⊤; tt)
open import Schema2 public
import Schema2E2 as E2

------------------------------------------------------------------------
-- Constructive equality and decision helpers
------------------------------------------------------------------------

data Empty : Set where

Not : Set → Set
Not A = A → Empty

data Decision (A : Set) : Set where
  yes : A → Decision A
  no  : Not A → Decision A

sym : {A : Set} {x y : A} → x ≡ y → y ≡ x
sym refl = refl

trans : {A : Set} {x y z : A} → x ≡ y → y ≡ z → x ≡ z
trans refl q = q

cong : {A B : Set} {x y : A} →
       (f : A → B) → x ≡ y → f x ≡ f y
cong f refl = refl

------------------------------------------------------------------------
-- E-3: a typed normalization fragment
------------------------------------------------------------------------

-- This syntax is intentionally smaller than arbitrary Schema2(W).  It is an
-- intrinsically typed fragment over the constructive Code universe already
-- checked by Schema2.  The two redex constructors make normalization a real
-- reduction rather than merely a relabelling pass.

data E3Var : List Code → Code → Set where
  top : {Γ : List Code} {A : Code} → E3Var (A ∷ Γ) A
  pop : {Γ : List Code} {A B : Code} →
        E3Var Γ A → E3Var (B ∷ Γ) A

data E3Term (Γ : List Code) : Code → Set where
  term-var       : {A : Code} → E3Var Γ A → E3Term Γ A
  term-unit      : E3Term Γ unit-code
  term-zero      : E3Term Γ nat-code
  term-suc       : E3Term Γ nat-code → E3Term Γ nat-code
  term-point     : {A : Code} → E3Term Γ A → E3Term Γ (trunc-code A)
  identity-redex : {A : Code} → E3Term Γ A → E3Term Γ A
  trunc-id-redex : {A : Code} →
                   E3Term Γ (trunc-code A) → E3Term Γ (trunc-code A)

data E3Normal (Γ : List Code) : Code → Set where
  normal-var   : {A : Code} → E3Var Γ A → E3Normal Γ A
  normal-unit  : E3Normal Γ unit-code
  normal-zero  : E3Normal Γ nat-code
  normal-suc   : E3Normal Γ nat-code → E3Normal Γ nat-code
  normal-point : {A : Code} →
                 E3Normal Γ A → E3Normal Γ (trunc-code A)

normalize : {Γ : List Code} {A : Code} → E3Term Γ A → E3Normal Γ A
normalize (term-var x) = normal-var x
normalize term-unit = normal-unit
normalize term-zero = normal-zero
normalize (term-suc t) = normal-suc (normalize t)
normalize (term-point t) = normal-point (normalize t)
normalize (identity-redex t) = normalize t
normalize (trunc-id-redex t) = normalize t

-- Typing preservation is intrinsic: the input and output indices are the
-- same Γ and A.  Agda's structural termination checker certifies termination.
normalization-preserves-typing :
  {Γ : List Code} {A : Code} → E3Term Γ A → E3Normal Γ A
normalization-preserves-typing = normalize

quote-normal : {Γ : List Code} {A : Code} →
               E3Normal Γ A → E3Term Γ A
quote-normal (normal-var x) = term-var x
quote-normal normal-unit = term-unit
quote-normal normal-zero = term-zero
quote-normal (normal-suc n) = term-suc (quote-normal n)
quote-normal (normal-point n) = term-point (quote-normal n)

normalize-quote : {Γ : List Code} {A : Code} →
                  (n : E3Normal Γ A) →
                  normalize (quote-normal n) ≡ n
normalize-quote (normal-var x) = refl
normalize-quote normal-unit = refl
normalize-quote normal-zero = refl
normalize-quote (normal-suc n) = cong normal-suc (normalize-quote n)
normalize-quote (normal-point n) =
  cong normal-point (normalize-quote n)

normalization-replay-stable : {Γ : List Code} {A : Code} →
                              (t : E3Term Γ A) →
                              normalize (quote-normal (normalize t)) ≡
                              normalize t
normalization-replay-stable t = normalize-quote (normalize t)

------------------------------------------------------------------------
-- Renaming, weakening, and substitution naturality
------------------------------------------------------------------------

E3Rename : List Code → List Code → Set
E3Rename Γ Δ = {A : Code} → E3Var Γ A → E3Var Δ A

identity-rename : {Γ : List Code} → E3Rename Γ Γ
identity-rename x = x

weakening-rename : {Γ : List Code} {B : Code} → E3Rename Γ (B ∷ Γ)
weakening-rename = pop

rename-term : {Γ Δ : List Code} {A : Code} →
              E3Rename Γ Δ → E3Term Γ A → E3Term Δ A
rename-term ρ (term-var x) = term-var (ρ x)
rename-term ρ term-unit = term-unit
rename-term ρ term-zero = term-zero
rename-term ρ (term-suc t) = term-suc (rename-term ρ t)
rename-term ρ (term-point t) = term-point (rename-term ρ t)
rename-term ρ (identity-redex t) = identity-redex (rename-term ρ t)
rename-term ρ (trunc-id-redex t) = trunc-id-redex (rename-term ρ t)

rename-normal : {Γ Δ : List Code} {A : Code} →
                E3Rename Γ Δ → E3Normal Γ A → E3Normal Δ A
rename-normal ρ (normal-var x) = normal-var (ρ x)
rename-normal ρ normal-unit = normal-unit
rename-normal ρ normal-zero = normal-zero
rename-normal ρ (normal-suc n) = normal-suc (rename-normal ρ n)
rename-normal ρ (normal-point n) = normal-point (rename-normal ρ n)

normalize-rename : {Γ Δ : List Code} {A : Code} →
                   (ρ : E3Rename Γ Δ) → (t : E3Term Γ A) →
                   normalize (rename-term ρ t) ≡
                   rename-normal ρ (normalize t)
normalize-rename ρ (term-var x) = refl
normalize-rename ρ term-unit = refl
normalize-rename ρ term-zero = refl
normalize-rename ρ (term-suc t) =
  cong normal-suc (normalize-rename ρ t)
normalize-rename ρ (term-point t) =
  cong normal-point (normalize-rename ρ t)
normalize-rename ρ (identity-redex t) = normalize-rename ρ t
normalize-rename ρ (trunc-id-redex t) = normalize-rename ρ t

E3Substitution : List Code → List Code → Set
E3Substitution Γ Δ = {A : Code} → E3Var Γ A → E3Term Δ A

E3NormalSubstitution : List Code → List Code → Set
E3NormalSubstitution Γ Δ =
  {A : Code} → E3Var Γ A → E3Normal Δ A

identity-substitution : {Γ : List Code} → E3Substitution Γ Γ
identity-substitution x = term-var x

normalized-substitution : {Γ Δ : List Code} →
                          E3Substitution Γ Δ →
                          E3NormalSubstitution Γ Δ
normalized-substitution σ x = normalize (σ x)

substitute-term : {Γ Δ : List Code} {A : Code} →
                  E3Substitution Γ Δ → E3Term Γ A → E3Term Δ A
substitute-term σ (term-var x) = σ x
substitute-term σ term-unit = term-unit
substitute-term σ term-zero = term-zero
substitute-term σ (term-suc t) = term-suc (substitute-term σ t)
substitute-term σ (term-point t) = term-point (substitute-term σ t)
substitute-term σ (identity-redex t) = identity-redex (substitute-term σ t)
substitute-term σ (trunc-id-redex t) =
  trunc-id-redex (substitute-term σ t)

substitute-normal : {Γ Δ : List Code} {A : Code} →
                    E3NormalSubstitution Γ Δ →
                    E3Normal Γ A → E3Normal Δ A
substitute-normal σ (normal-var x) = σ x
substitute-normal σ normal-unit = normal-unit
substitute-normal σ normal-zero = normal-zero
substitute-normal σ (normal-suc n) =
  normal-suc (substitute-normal σ n)
substitute-normal σ (normal-point n) =
  normal-point (substitute-normal σ n)

normalize-substitution : {Γ Δ : List Code} {A : Code} →
                         (σ : E3Substitution Γ Δ) →
                         (t : E3Term Γ A) →
                         normalize (substitute-term σ t) ≡
                         substitute-normal
                           (normalized-substitution σ)
                           (normalize t)
normalize-substitution σ (term-var x) = refl
normalize-substitution σ term-unit = refl
normalize-substitution σ term-zero = refl
normalize-substitution σ (term-suc t) =
  cong normal-suc (normalize-substitution σ t)
normalize-substitution σ (term-point t) =
  cong normal-point (normalize-substitution σ t)
normalize-substitution σ (identity-redex t) =
  normalize-substitution σ t
normalize-substitution σ (trunc-id-redex t) =
  normalize-substitution σ t

compose-substitution : {Γ Δ Θ : List Code} →
                       E3Substitution Γ Δ →
                       E3Substitution Δ Θ →
                       E3Substitution Γ Θ
compose-substitution σ τ x = substitute-term τ (σ x)

substitute-identity : {Γ : List Code} {A : Code} →
                      (t : E3Term Γ A) →
                      substitute-term identity-substitution t ≡ t
substitute-identity (term-var x) = refl
substitute-identity term-unit = refl
substitute-identity term-zero = refl
substitute-identity (term-suc t) =
  cong term-suc (substitute-identity t)
substitute-identity (term-point t) =
  cong term-point (substitute-identity t)
substitute-identity (identity-redex t) =
  cong identity-redex (substitute-identity t)
substitute-identity (trunc-id-redex t) =
  cong trunc-id-redex (substitute-identity t)

substitute-composition : {Γ Δ Θ : List Code} {A : Code} →
                         (σ : E3Substitution Γ Δ) →
                         (τ : E3Substitution Δ Θ) →
                         (t : E3Term Γ A) →
                         substitute-term τ (substitute-term σ t) ≡
                         substitute-term (compose-substitution σ τ) t
substitute-composition σ τ (term-var x) = refl
substitute-composition σ τ term-unit = refl
substitute-composition σ τ term-zero = refl
substitute-composition σ τ (term-suc t) =
  cong term-suc (substitute-composition σ τ t)
substitute-composition σ τ (term-point t) =
  cong term-point (substitute-composition σ τ t)
substitute-composition σ τ (identity-redex t) =
  cong identity-redex (substitute-composition σ τ t)
substitute-composition σ τ (trunc-id-redex t) =
  cong trunc-id-redex (substitute-composition σ τ t)

------------------------------------------------------------------------
-- Frozen normal equality with non-erasing provenance
------------------------------------------------------------------------

data Provenance : Set where
  current-generator-origin     : Provenance
  predecessor-reference-origin : Provenance
  library-reference-origin     : Provenance

record ProvenancedTerm (Γ : List Code) (A : Code) : Set where
  constructor term-with-origin
  field
    term-origin : Provenance
    term-syntax : E3Term Γ A

open ProvenancedTerm public

record ProvenancedNormal (Γ : List Code) (A : Code) : Set where
  constructor normal-with-origin
  field
    normal-origin : Provenance
    normal-syntax : E3Normal Γ A

open ProvenancedNormal public

normalize-provenanced : {Γ : List Code} {A : Code} →
                        ProvenancedTerm Γ A → ProvenancedNormal Γ A
normalize-provenanced p =
  normal-with-origin (term-origin p) (normalize (term-syntax p))

-- This is a deliberately strict, family-safe toy relation: equal typed normal
-- forms with exactly equal outer provenance.  In particular, a predecessor or
-- library reference cannot silently become a current generator merely because
-- its term reduces to the same syntax.  It is not the global frozen univalent
-- semantic equality: nested provenance, dependency transport, and semantic
-- equivalences are intentionally absent from this fragment.
record FrozenNormalEqual {Γ : List Code} {A : Code}
                         (left right : ProvenancedTerm Γ A) : Set where
  constructor frozen-normal-equal
  field
    same-normal : normalize (term-syntax left) ≡
                  normalize (term-syntax right)
    same-provenance : term-origin left ≡ term-origin right

open FrozenNormalEqual public

frozen-refl : {Γ : List Code} {A : Code} →
              (p : ProvenancedTerm Γ A) → FrozenNormalEqual p p
frozen-refl p = frozen-normal-equal refl refl

frozen-sym : {Γ : List Code} {A : Code}
             {p q : ProvenancedTerm Γ A} →
             FrozenNormalEqual p q → FrozenNormalEqual q p
frozen-sym (frozen-normal-equal terms origins) =
  frozen-normal-equal (sym terms) (sym origins)

frozen-trans : {Γ : List Code} {A : Code}
               {p q r : ProvenancedTerm Γ A} →
               FrozenNormalEqual p q → FrozenNormalEqual q r →
               FrozenNormalEqual p r
frozen-trans (frozen-normal-equal pq po)
             (frozen-normal-equal qr qo) =
  frozen-normal-equal (trans pq qr) (trans po qo)

provenanced-suc : {Γ : List Code} →
                  ProvenancedTerm Γ nat-code →
                  ProvenancedTerm Γ nat-code
provenanced-suc p = term-with-origin (term-origin p) (term-suc (term-syntax p))

provenanced-point : {Γ : List Code} {A : Code} →
                    ProvenancedTerm Γ A →
                    ProvenancedTerm Γ (trunc-code A)
provenanced-point p =
  term-with-origin (term-origin p) (term-point (term-syntax p))

provenanced-identity-redex : {Γ : List Code} {A : Code} →
                             ProvenancedTerm Γ A → ProvenancedTerm Γ A
provenanced-identity-redex p =
  term-with-origin (term-origin p) (identity-redex (term-syntax p))

frozen-suc-congruence : {Γ : List Code}
                        {p q : ProvenancedTerm Γ nat-code} →
                        FrozenNormalEqual p q →
                        FrozenNormalEqual
                          (provenanced-suc p) (provenanced-suc q)
frozen-suc-congruence (frozen-normal-equal terms origins) =
  frozen-normal-equal (cong normal-suc terms) origins

frozen-point-congruence : {Γ : List Code} {A : Code}
                          {p q : ProvenancedTerm Γ A} →
                          FrozenNormalEqual p q →
                          FrozenNormalEqual
                            (provenanced-point p) (provenanced-point q)
frozen-point-congruence (frozen-normal-equal terms origins) =
  frozen-normal-equal (cong normal-point terms) origins

frozen-redex-congruence : {Γ : List Code} {A : Code}
                          {p q : ProvenancedTerm Γ A} →
                          FrozenNormalEqual p q →
                          FrozenNormalEqual
                            (provenanced-identity-redex p)
                            (provenanced-identity-redex q)
frozen-redex-congruence (frozen-normal-equal terms origins) =
  frozen-normal-equal terms origins

redex-is-frozen-equal : {Γ : List Code} {A : Code} →
                        (p : ProvenancedTerm Γ A) →
                        FrozenNormalEqual (provenanced-identity-redex p) p
redex-is-frozen-equal p = frozen-normal-equal refl refl

rename-provenanced : {Γ Δ : List Code} {A : Code} →
                     E3Rename Γ Δ →
                     ProvenancedTerm Γ A → ProvenancedTerm Δ A
rename-provenanced ρ p =
  term-with-origin (term-origin p) (rename-term ρ (term-syntax p))

rename-respects-frozen : {Γ Δ : List Code} {A : Code} →
                         (ρ : E3Rename Γ Δ) →
                         {p q : ProvenancedTerm Γ A} →
                         FrozenNormalEqual p q →
                         FrozenNormalEqual
                           (rename-provenanced ρ p)
                           (rename-provenanced ρ q)
rename-respects-frozen ρ {p} {q}
  (frozen-normal-equal terms origins) =
  frozen-normal-equal
    (trans (normalize-rename ρ (term-syntax p))
      (trans (cong (rename-normal ρ) terms)
             (sym (normalize-rename ρ (term-syntax q)))))
    origins

NormalSubstitutionEqual : {Γ Δ : List Code} →
                          E3NormalSubstitution Γ Δ →
                          E3NormalSubstitution Γ Δ → Set
NormalSubstitutionEqual σ τ =
  {A : Code} → (x : E3Var _ A) → σ x ≡ τ x

normal-substitution-pointwise : {Γ Δ : List Code}
                                {σ τ : E3NormalSubstitution Γ Δ} →
                                NormalSubstitutionEqual σ τ →
                                {A : Code} → (n : E3Normal Γ A) →
                                substitute-normal σ n ≡
                                substitute-normal τ n
normal-substitution-pointwise equal (normal-var x) = equal x
normal-substitution-pointwise equal normal-unit = refl
normal-substitution-pointwise equal normal-zero = refl
normal-substitution-pointwise equal (normal-suc n) =
  cong normal-suc (normal-substitution-pointwise equal n)
normal-substitution-pointwise equal (normal-point n) =
  cong normal-point (normal-substitution-pointwise equal n)

normal-substitution-congruence : {Γ Δ : List Code}
                                 {σ τ : E3NormalSubstitution Γ Δ}
                                 {A : Code} {n m : E3Normal Γ A} →
                                 NormalSubstitutionEqual σ τ →
                                 n ≡ m →
                                 substitute-normal σ n ≡
                                 substitute-normal τ m
normal-substitution-congruence {n = n} equal refl =
  normal-substitution-pointwise equal n

SubstitutionNormalEqual : {Γ Δ : List Code} →
                          E3Substitution Γ Δ →
                          E3Substitution Γ Δ → Set
SubstitutionNormalEqual σ τ =
  {A : Code} → (x : E3Var _ A) → normalize (σ x) ≡ normalize (τ x)

-- This theorem is syntax-only.  It intentionally does not transport the
-- outer provenance field: correct dependent substitution must rewrite and
-- compose provenance inside substituted expressions, which the toy Origin
-- datatype cannot represent.
substitution-respects-normal-equality :
  {Γ Δ : List Code} {A : Code} →
  {σ τ : E3Substitution Γ Δ} →
  SubstitutionNormalEqual σ τ →
  {left right : E3Term Γ A} →
  normalize left ≡ normalize right →
  normalize (substitute-term σ left) ≡
  normalize (substitute-term τ right)
substitution-respects-normal-equality
  {σ = σ} {τ = τ} substitutions {left} {right} terms =
  trans (normalize-substitution σ left)
    (trans
      (normal-substitution-congruence substitutions terms)
      (sym (normalize-substitution τ right)))

------------------------------------------------------------------------
-- E-4: finite generator kinds and checked-substitution decomposition
------------------------------------------------------------------------

-- These are generator *schemes*.  Their instances retain all intrinsic type
-- indices from Schema2.Sub.  In particular, dimension-image-generator is
-- broader than the still-open finite cubical face-map basis required by C2.
data GeneratorKind : Set where
  weakening-generator         : GeneratorKind
  type-image-generator        : GeneratorKind
  term-image-generator        : GeneratorKind
  library-image-generator     : GeneratorKind
  dimension-image-generator   : GeneratorKind
  cofibration-image-generator : GeneratorKind
  adjacent-exchange-generator : GeneratorKind
  parent-cube-action-generator : GeneratorKind

Basis : Set₁
Basis = GeneratorKind → Set

BasisInclusion : Basis → Basis → Set
BasisInclusion smaller larger =
  (kind : GeneratorKind) → smaller kind → larger kind

data FullSyntacticBasis : GeneratorKind → Set where
  full-weakening   : FullSyntacticBasis weakening-generator
  full-type-image  : FullSyntacticBasis type-image-generator
  full-term-image  : FullSyntacticBasis term-image-generator
  full-library     : FullSyntacticBasis library-image-generator
  full-dimension   : FullSyntacticBasis dimension-image-generator
  full-cofibration : FullSyntacticBasis cofibration-image-generator
  full-exchange    : FullSyntacticBasis adjacent-exchange-generator
  full-parent-cube : FullSyntacticBasis parent-cube-action-generator

infixr 5 _++_

_++_ : {A : Set} → List A → List A → List A
[] ++ ys = ys
(x ∷ xs) ++ ys = x ∷ (xs ++ ys)

-- `generator-kinds` observes an independently checked E-1 Sub value.  It
-- does not define a legal morphism to be a generator word.  Its result is a
-- replayable decomposition trace for that checked syntax only.
generator-kinds : {Γ Δ : SchemaContext} → Sub Γ Δ → List GeneratorKind
generator-kinds identity-sub = []
generator-kinds (compose-sub σ τ) =
  generator-kinds σ ++ generator-kinds τ
generator-kinds weakening-sub = weakening-generator ∷ []
generator-kinds (type-image-sub σ T) =
  generator-kinds σ ++ type-image-generator ∷ []
generator-kinds (term-image-sub σ term) =
  generator-kinds σ ++ term-image-generator ∷ []
generator-kinds (library-image-sub σ term) =
  generator-kinds σ ++ library-image-generator ∷ []
generator-kinds (dimension-image-sub σ dimension) =
  generator-kinds σ ++ dimension-image-generator ∷ []
generator-kinds (cofibration-image-sub σ proof) =
  generator-kinds σ ++ cofibration-image-generator ∷ []
generator-kinds adjacent-exchange-sub =
  adjacent-exchange-generator ∷ []

data BasisCovers (B : Basis) : List GeneratorKind → Set where
  covers-empty : BasisCovers B []
  covers-cons  : {kind : GeneratorKind} {kinds : List GeneratorKind} →
                 B kind → BasisCovers B kinds →
                 BasisCovers B (kind ∷ kinds)

Generates : Basis → {Γ Δ : SchemaContext} → Sub Γ Δ → Set
Generates B σ = BasisCovers B (generator-kinds σ)

lift-coverage : {smaller larger : Basis} →
                BasisInclusion smaller larger →
                {kinds : List GeneratorKind} →
                BasisCovers smaller kinds → BasisCovers larger kinds
lift-coverage inclusion covers-empty = covers-empty
lift-coverage inclusion (covers-cons {kind = kind} member rest) =
  covers-cons (inclusion kind member) (lift-coverage inclusion rest)

lift-generation : {smaller larger : Basis} →
                  BasisInclusion smaller larger →
                  {Γ Δ : SchemaContext} {σ : Sub Γ Δ} →
                  Generates smaller σ → Generates larger σ
lift-generation inclusion evidence = lift-coverage inclusion evidence

full-member : (kind : GeneratorKind) → FullSyntacticBasis kind
full-member weakening-generator = full-weakening
full-member type-image-generator = full-type-image
full-member term-image-generator = full-term-image
full-member library-image-generator = full-library
full-member dimension-image-generator = full-dimension
full-member cofibration-image-generator = full-cofibration
full-member adjacent-exchange-generator = full-exchange
full-member parent-cube-action-generator = full-parent-cube

cover-full-trace : (kinds : List GeneratorKind) →
                   BasisCovers FullSyntacticBasis kinds
cover-full-trace [] = covers-empty
cover-full-trace (kind ∷ kinds) =
  covers-cons (full-member kind) (cover-full-trace kinds)

-- Structural coverage of an already checked E-1 substitution.  This is a
-- theorem about its observable constructor trace, not E-4 completeness for
-- arbitrary semantic context morphisms.
decompose-checked-sub : {Γ Δ : SchemaContext} →
                        (σ : Sub Γ Δ) →
                        Generates FullSyntacticBasis σ
decompose-checked-sub σ = cover-full-trace (generator-kinds σ)

------------------------------------------------------------------------
-- An external morphism view and the exact boundary of decomposition
------------------------------------------------------------------------

record MorphismView (Γ Δ : SchemaContext) : Set₁ where
  constructor morphism-view
  field
    carrier-map : Carrier Δ → Carrier Γ

open MorphismView public

view-checked-sub : {Γ Δ : SchemaContext} →
                   Sub Γ Δ → MorphismView Γ Δ
view-checked-sub σ = morphism-view (apply σ)

record ViewDecomposition (B : Basis) {Γ Δ : SchemaContext}
                         (view : MorphismView Γ Δ) : Set₁ where
  constructor view-decomposition
  field
    checked-word : Sub Γ Δ
    word-generated : Generates B checked-word
    word-sound : (δ : Carrier Δ) →
                 apply checked-word δ ≡ carrier-map view δ

open ViewDecomposition public

decompose-checked-view : {Γ Δ : SchemaContext} →
                         (σ : Sub Γ Δ) →
                         ViewDecomposition FullSyntacticBasis
                           (view-checked-sub σ)
decompose-checked-view σ =
  view-decomposition σ (decompose-checked-sub σ) (λ _ → refl)

-- No inhabitant is constructed for arbitrary MorphismView.  Such an
-- inhabitant would be the extensional completeness theorem still required
-- by E-4, rather than a consequence of the Sub syntax above.
ArbitraryViewCompleteness : Basis → Set₁
ArbitraryViewCompleteness B =
  {Γ Δ : SchemaContext} →
  (view : MorphismView Γ Δ) → ViewDecomposition B view

------------------------------------------------------------------------
-- A sound parent-operation cube-action generator
------------------------------------------------------------------------

-- This relation records the complete observable content of deriving a cube
-- action by applying a parent operation.  Besides the pointwise body equation,
-- all six output faces are proved from the corresponding source faces.
record ParentCubeActionGenerated
       {A B : Set} {at : A}
       (parent : A → B)
       (source : E2.Cube3At A at)
       (action : E2.Cube3At B (parent at)) : Set where
  constructor parent-cube-action-generated
  field
    generated-body : (i j k : Bool) →
      E2.cube3-body action i j k ≡
      parent (E2.cube3-body source i j k)
    generated-i0 : (j k : Bool) →
      E2.cube3-body action false j k ≡ parent at
    generated-i1 : (j k : Bool) →
      E2.cube3-body action true j k ≡ parent at
    generated-j0 : (i k : Bool) →
      E2.cube3-body action i false k ≡ parent at
    generated-j1 : (i k : Bool) →
      E2.cube3-body action i true k ≡ parent at
    generated-k0 : (i j : Bool) →
      E2.cube3-body action i j false ≡ parent at
    generated-k1 : (i j : Bool) →
      E2.cube3-body action i j true ≡ parent at

open ParentCubeActionGenerated public

map-cube3-is-parent-generated :
  {A B : Set} {at : A} →
  (parent : A → B) → (source : E2.Cube3At A at) →
  ParentCubeActionGenerated
    parent source (E2.map-cube3 parent source)
map-cube3-is-parent-generated parent source =
  parent-cube-action-generated
    (λ _ _ _ → refl)
    (λ j k → cong parent (E2.cube3-i0 source j k))
    (λ j k → cong parent (E2.cube3-i1 source j k))
    (λ i k → cong parent (E2.cube3-j0 source i k))
    (λ i k → cong parent (E2.cube3-j1 source i k))
    (λ i j → cong parent (E2.cube3-k0 source i j))
    (λ i j → cong parent (E2.cube3-k1 source i j))

step8-parent-map :
  (signature : E2.Step8OrdinarySignature) →
  (x : E2.S3 (E2.cell-signature signature)) →
  E2.S3 (E2.cell-signature signature) →
  E2.S3 (E2.cell-signature signature)
step8-parent-map signature x y =
  E2.step8-μ-typed signature (y , x)

step8-cell-action-is-parent-generated :
  (signature : E2.Step8OrdinarySignature) →
  (x : E2.S3 (E2.cell-signature signature)) →
  ParentCubeActionGenerated
    (step8-parent-map signature x)
    (E2.p (E2.cell-signature signature))
    (E2.step8-cell-action signature x)
step8-cell-action-is-parent-generated signature x =
  map-cube3-is-parent-generated
    (step8-parent-map signature x)
    (E2.p (E2.cell-signature signature))

------------------------------------------------------------------------
-- P1/M1: monotone generated verdicts; independence remains unavailable
------------------------------------------------------------------------

-- M1-generated evidence carries both the actual sub-basis derivation and a
-- proof that this basis embeds in the eventual basis.  Its final derivation
-- is computed, never asserted by a label.
record M1Generated (smaller eventual : Basis)
                   {Γ Δ : SchemaContext} (σ : Sub Γ Δ) : Set₁ where
  constructor m1-generated
  field
    sub-basis-proof : BasisInclusion smaller eventual
    generating-derivation : Generates smaller σ

open M1Generated public

m1-generated-in-eventual : {smaller eventual : Basis}
                           {Γ Δ : SchemaContext} {σ : Sub Γ Δ} →
                           M1Generated smaller eventual σ →
                           Generates eventual σ
m1-generated-in-eventual {σ = σ} certificate =
  lift-generation
    (sub-basis-proof certificate) {σ = σ}
    (generating-derivation certificate)

GeneratorMonotone : {Row : Set} →
                    (Basis → Row → Set) → Set₁
GeneratorMonotone Generated =
  {smaller larger : Basis} →
  BasisInclusion smaller larger →
  {row : _} → Generated smaller row → Generated larger row

data MembershipVerdict {Row : Set}
     (eventual : Basis) (Generated : Basis → Row → Set)
     (row : Row) : Set₂ where
  generated-from-sub-basis :
    GeneratorMonotone Generated →
    {smaller : Basis} →
    BasisInclusion smaller eventual →
    Generated smaller row →
    MembershipVerdict eventual Generated row

-- There is intentionally no `independent` constructor.  It may be added only
-- by a successor that carries both exhaustive basis coverage and per-class
-- completeness, not merely a decidable membership test.

generated-verdict-is-final : {Row : Set}
                             {eventual : Basis}
                             {Generated : Basis → Row → Set}
                             {row : Row} →
                             GeneratorMonotone Generated →
                             {smaller : Basis} →
                             BasisInclusion smaller eventual →
                             Generated smaller row →
                             Generated eventual row
generated-verdict-is-final monotone inclusion evidence =
  monotone inclusion evidence

------------------------------------------------------------------------
-- M1 instance for the Agda-internal Step-8 parent cube action
------------------------------------------------------------------------

data ParentCubeSubBasis : GeneratorKind → Set where
  parent-cube-only :
    ParentCubeSubBasis parent-cube-action-generator

record Step8ParentActionGenerated
       (B : Basis)
       (signature : E2.Step8OrdinarySignature)
       (x : E2.S3 (E2.cell-signature signature)) : Set where
  constructor step8-parent-action-generated
  field
    parent-cube-generator-member : B parent-cube-action-generator
    parent-cube-derivation :
      ParentCubeActionGenerated
        (step8-parent-map signature x)
        (E2.p (E2.cell-signature signature))
        (E2.step8-cell-action signature x)

open Step8ParentActionGenerated public

data Step8DerivedRow : Set where
  step8-derived-cell-action-row : Step8DerivedRow

Step8Generated :
  (signature : E2.Step8OrdinarySignature) →
  (x : E2.S3 (E2.cell-signature signature)) →
  Basis → Step8DerivedRow → Set
Step8Generated signature x B step8-derived-cell-action-row =
  Step8ParentActionGenerated B signature x

step8-generated-monotone :
  (signature : E2.Step8OrdinarySignature) →
  (x : E2.S3 (E2.cell-signature signature)) →
  GeneratorMonotone (Step8Generated signature x)
step8-generated-monotone signature x inclusion
  {row = step8-derived-cell-action-row}
  (step8-parent-action-generated member derivation) =
  step8-parent-action-generated
    (inclusion parent-cube-action-generator member)
    derivation

step8-generated-in-parent-sub-basis :
  (signature : E2.Step8OrdinarySignature) →
  (x : E2.S3 (E2.cell-signature signature)) →
  Step8Generated signature x ParentCubeSubBasis
    step8-derived-cell-action-row
step8-generated-in-parent-sub-basis signature x =
  step8-parent-action-generated
    parent-cube-only
    (step8-cell-action-is-parent-generated signature x)

step8-cell-action-M1-generated :
  (eventual : Basis) →
  BasisInclusion ParentCubeSubBasis eventual →
  (signature : E2.Step8OrdinarySignature) →
  (x : E2.S3 (E2.cell-signature signature)) →
  MembershipVerdict
    eventual
    (Step8Generated signature x)
    step8-derived-cell-action-row
step8-cell-action-M1-generated eventual inclusion signature x =
  generated-from-sub-basis
    (step8-generated-monotone signature x)
    inclusion
    (step8-generated-in-parent-sub-basis signature x)

------------------------------------------------------------------------
-- Explicit E-3/E-4 scope boundary
------------------------------------------------------------------------

data OpenE3E4Gap : Set where
  arbitrary-Schema2-W-normalization-gap : OpenE3E4Gap
  frozen-univalent-semantic-equality-gap : OpenE3E4Gap
  provenance-correct-dependent-substitution-gap : OpenE3E4Gap
  arbitrary-extensional-morphism-completeness-gap : OpenE3E4Gap
  finite-cubical-face-generator-basis-gap : OpenE3E4Gap
  per-class-generator-basis-completeness-gap : OpenE3E4Gap
  R1-R2-row-generator-membership-gap : OpenE3E4Gap
  parent-operation-naturality-closure-gap : OpenE3E4Gap
  Rust-Agda-Step8-correspondence-gap : OpenE3E4Gap
  general-C6-dependent-coe-hcom-motive-gap : OpenE3E4Gap

open-E3-E4-gaps : List OpenE3E4Gap
open-E3-E4-gaps =
  arbitrary-Schema2-W-normalization-gap ∷
  frozen-univalent-semantic-equality-gap ∷
  provenance-correct-dependent-substitution-gap ∷
  arbitrary-extensional-morphism-completeness-gap ∷
  finite-cubical-face-generator-basis-gap ∷
  per-class-generator-basis-completeness-gap ∷
  R1-R2-row-generator-membership-gap ∷
  parent-operation-naturality-closure-gap ∷
  Rust-Agda-Step8-correspondence-gap ∷
  general-C6-dependent-coe-hcom-motive-gap ∷ []

-- Scope conclusion: this safe module proves typed normalization and a strict
-- family-safe toy equality for the E3Term fragment, plus monotone decomposition
-- for checked E-1 Sub syntax and its observable carrier map.  Its Step-8 M1
-- witness concerns only the Agda-internal map-cube3 term; the Rust/Agda row
-- correspondence remains false/open.  It issues no arbitrary-view or
-- per-class completeness witness, no R1/R2 semantic-row verdict, no
-- independent verdict, no family token, no stage count, and no halt claim.
