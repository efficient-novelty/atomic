{-# OPTIONS --safe --without-K #-}

module Schema2 where

open import Agda.Builtin.Bool using (Bool; true; false)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Nat using (Nat)
open import Agda.Builtin.Unit using (⊤; tt)

import KernelBridge as KB

------------------------------------------------------------------------
-- A small constructive universe used by the E-1 context model
------------------------------------------------------------------------

data Trunc (A : Set) : Set where
  point : A → Trunc A

data Code : Set where
  unit-code  : Code
  nat-code   : Code
  trunc-code : Code → Code

El : Code → Set
El unit-code = ⊤
El nat-code = Nat
El (trunc-code A) = Trunc (El A)

record Σ (A : Set) (B : A → Set) : Set where
  constructor _,_
  field
    fst : A
    snd : B fst

open Σ public

------------------------------------------------------------------------
-- Intrinsically formed dependent schema contexts
------------------------------------------------------------------------

data EntryKind : Set where
  universe/type-parameter : EntryKind
  opaque-element          : EntryKind
  library-reference       : EntryKind
  interval-variable       : EntryKind
  cofibration-assumption  : EntryKind

data Holds : Bool → Set where
  holds-true : Holds true

record RawContext : Set₁ where
  constructor raw-context
  field
    RawCarrier : Set
    raw-entries : List EntryKind

open RawContext public

ContextPayload : RawContext → EntryKind → Set
ContextPayload raw universe/type-parameter = ⊤
ContextPayload raw opaque-element = RawCarrier raw → Code
ContextPayload raw library-reference = RawCarrier raw → Code
ContextPayload raw interval-variable = ⊤
ContextPayload raw cofibration-assumption = RawCarrier raw → Bool

payload-family : {raw : RawContext} {kind : EntryKind} →
                 ContextPayload raw kind → RawCarrier raw → Set
payload-family {kind = universe/type-parameter} payload γ = Code
payload-family {kind = opaque-element} A γ = El (A γ)
payload-family {kind = library-reference} A γ = El (A γ)
payload-family {kind = interval-variable} payload γ = Bool
payload-family {kind = cofibration-assumption} φ γ = Holds (φ γ)

extend-raw : (raw : RawContext) → (kind : EntryKind) →
             ContextPayload raw kind → RawContext
extend-raw raw kind payload =
  raw-context
    (Σ (RawCarrier raw) (payload-family payload))
    (kind ∷ raw-entries raw)

data ContextFormation : RawContext → Set₁ where
  empty-formation : ContextFormation (raw-context ⊤ [])
  extend-formation : {raw : RawContext} → ContextFormation raw →
                     (kind : EntryKind) → (payload : ContextPayload raw kind) →
                     ContextFormation (extend-raw raw kind payload)

-- `raw` cannot be paired with arbitrary metadata: the second field has an
-- inhabitant only when it was built by the two formation constructors above.
record SchemaContext : Set₁ where
  constructor formed-context
  field
    raw       : RawContext
    formation : ContextFormation raw

open SchemaContext public

Carrier : SchemaContext → Set
Carrier Γ = RawCarrier (raw Γ)

entries : SchemaContext → List EntryKind
entries Γ = raw-entries (raw Γ)

empty : SchemaContext
empty = formed-context (raw-context ⊤ []) empty-formation

extend : (Γ : SchemaContext) → (kind : EntryKind) →
         ContextPayload (raw Γ) kind → SchemaContext
extend Γ kind payload =
  formed-context
    (extend-raw (raw Γ) kind payload)
    (extend-formation (formation Γ) kind payload)

weaken-payload : {raw : RawContext} {outer inner : EntryKind}
                 {outer-payload : ContextPayload raw outer} →
                 ContextPayload raw inner →
                 ContextPayload (extend-raw raw outer outer-payload) inner
weaken-payload {inner = universe/type-parameter} payload = tt
weaken-payload {inner = opaque-element} A = λ γx → A (fst γx)
weaken-payload {inner = library-reference} A = λ γx → A (fst γx)
weaken-payload {inner = interval-variable} payload = tt
weaken-payload {inner = cofibration-assumption} φ = λ γx → φ (fst γx)

Ty : SchemaContext → Set
Ty Γ = Carrier Γ → Code

Tm : (Γ : SchemaContext) → Ty Γ → Set
Tm Γ A = (γ : Carrier Γ) → El (A γ)

Dim : SchemaContext → Set
Dim Γ = Carrier Γ → Bool

Face : SchemaContext → Set
Face Γ = Carrier Γ → Bool

type-parameter : SchemaContext → SchemaContext
type-parameter Γ = extend Γ universe/type-parameter tt

opaque-context : (Γ : SchemaContext) → Ty Γ → SchemaContext
opaque-context Γ A = extend Γ opaque-element A

library-context : (Γ : SchemaContext) → Ty Γ → SchemaContext
library-context Γ A = extend Γ library-reference A

interval : SchemaContext → SchemaContext
interval Γ = extend Γ interval-variable tt

cofibration : (Γ : SchemaContext) → Face Γ → SchemaContext
cofibration Γ φ = extend Γ cofibration-assumption φ

-- Patterning on both entry tags exposes the corresponding payload families
-- to the checker.  This is the generic legal exchange map; all 25 tag pairs
-- share the same value-level permutation but no tag/payload mismatch is
-- admitted.
exchange-carrier : {Γ : SchemaContext} {left right : EntryKind}
                   {left-payload : ContextPayload (raw Γ) left}
                   {right-payload : ContextPayload (raw Γ) right} →
                   Carrier
                     (extend (extend Γ right right-payload) left
                       (weaken-payload
                         {outer-payload = right-payload}
                         left-payload)) →
                   Carrier
                     (extend (extend Γ left left-payload) right
                       (weaken-payload
                         {outer-payload = left-payload}
                         right-payload))
exchange-carrier {left = universe/type-parameter} {right = universe/type-parameter} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = universe/type-parameter} {right = opaque-element} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = universe/type-parameter} {right = library-reference} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = universe/type-parameter} {right = interval-variable} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = universe/type-parameter} {right = cofibration-assumption} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = opaque-element} {right = universe/type-parameter} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = opaque-element} {right = opaque-element} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = opaque-element} {right = library-reference} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = opaque-element} {right = interval-variable} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = opaque-element} {right = cofibration-assumption} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = library-reference} {right = universe/type-parameter} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = library-reference} {right = opaque-element} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = library-reference} {right = library-reference} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = library-reference} {right = interval-variable} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = library-reference} {right = cofibration-assumption} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = interval-variable} {right = universe/type-parameter} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = interval-variable} {right = opaque-element} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = interval-variable} {right = library-reference} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = interval-variable} {right = interval-variable} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = interval-variable} {right = cofibration-assumption} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = cofibration-assumption} {right = universe/type-parameter} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = cofibration-assumption} {right = opaque-element} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = cofibration-assumption} {right = library-reference} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = cofibration-assumption} {right = interval-variable} ((γ , b) , a) = (γ , a) , b
exchange-carrier {left = cofibration-assumption} {right = cofibration-assumption} ((γ , b) , a) = (γ , a) , b

exchange-carrier-inverse : {Γ : SchemaContext} {left right : EntryKind}
                           {left-payload : ContextPayload (raw Γ) left}
                           {right-payload : ContextPayload (raw Γ) right} →
                           (δ : Carrier
                             (extend (extend Γ right right-payload) left
                               (weaken-payload
                                 {outer-payload = right-payload}
                                 left-payload))) →
                           exchange-carrier
                             {Γ = Γ} {left = right} {right = left}
                             {left-payload = right-payload}
                             {right-payload = left-payload}
                             (exchange-carrier
                               {Γ = Γ} {left = left} {right = right}
                               {left-payload = left-payload}
                               {right-payload = right-payload}
                               δ)
                           ≡ δ
exchange-carrier-inverse {left = universe/type-parameter} {right = universe/type-parameter} _ = refl
exchange-carrier-inverse {left = universe/type-parameter} {right = opaque-element} _ = refl
exchange-carrier-inverse {left = universe/type-parameter} {right = library-reference} _ = refl
exchange-carrier-inverse {left = universe/type-parameter} {right = interval-variable} _ = refl
exchange-carrier-inverse {left = universe/type-parameter} {right = cofibration-assumption} _ = refl
exchange-carrier-inverse {left = opaque-element} {right = universe/type-parameter} _ = refl
exchange-carrier-inverse {left = opaque-element} {right = opaque-element} _ = refl
exchange-carrier-inverse {left = opaque-element} {right = library-reference} _ = refl
exchange-carrier-inverse {left = opaque-element} {right = interval-variable} _ = refl
exchange-carrier-inverse {left = opaque-element} {right = cofibration-assumption} _ = refl
exchange-carrier-inverse {left = library-reference} {right = universe/type-parameter} _ = refl
exchange-carrier-inverse {left = library-reference} {right = opaque-element} _ = refl
exchange-carrier-inverse {left = library-reference} {right = library-reference} _ = refl
exchange-carrier-inverse {left = library-reference} {right = interval-variable} _ = refl
exchange-carrier-inverse {left = library-reference} {right = cofibration-assumption} _ = refl
exchange-carrier-inverse {left = interval-variable} {right = universe/type-parameter} _ = refl
exchange-carrier-inverse {left = interval-variable} {right = opaque-element} _ = refl
exchange-carrier-inverse {left = interval-variable} {right = library-reference} _ = refl
exchange-carrier-inverse {left = interval-variable} {right = interval-variable} _ = refl
exchange-carrier-inverse {left = interval-variable} {right = cofibration-assumption} _ = refl
exchange-carrier-inverse {left = cofibration-assumption} {right = universe/type-parameter} _ = refl
exchange-carrier-inverse {left = cofibration-assumption} {right = opaque-element} _ = refl
exchange-carrier-inverse {left = cofibration-assumption} {right = library-reference} _ = refl
exchange-carrier-inverse {left = cofibration-assumption} {right = interval-variable} _ = refl
exchange-carrier-inverse {left = cofibration-assumption} {right = cofibration-assumption} _ = refl

------------------------------------------------------------------------
-- Lookup, weakening, substitution, and legal exchange
------------------------------------------------------------------------

mutual
  -- Only the listed typed constructors create context morphisms.  There is
  -- no public constructor from an arbitrary carrier function.
  data Sub : SchemaContext → SchemaContext → Set₁ where
    identity-sub : {Γ : SchemaContext} → Sub Γ Γ
    compose-sub  : {Γ Δ Θ : SchemaContext} →
                   Sub Γ Δ → Sub Δ Θ → Sub Γ Θ
    weakening-sub : {Γ : SchemaContext} {kind : EntryKind}
                    {payload : ContextPayload (raw Γ) kind} →
                    Sub Γ (extend Γ kind payload)
    type-image-sub : {Γ Δ : SchemaContext} →
                     (σ : Sub Γ Δ) → Ty Δ → Sub (type-parameter Γ) Δ
    term-image-sub : {Γ Δ : SchemaContext} {A : Ty Γ} →
                     (σ : Sub Γ Δ) →
                     ((δ : Carrier Δ) → El (A (apply σ δ))) →
                     Sub (opaque-context Γ A) Δ
    library-image-sub : {Γ Δ : SchemaContext} {A : Ty Γ} →
                        (σ : Sub Γ Δ) →
                        ((δ : Carrier Δ) → El (A (apply σ δ))) →
                        Sub (library-context Γ A) Δ
    dimension-image-sub : {Γ Δ : SchemaContext} →
                          (σ : Sub Γ Δ) → Dim Δ → Sub (interval Γ) Δ
    cofibration-image-sub : {Γ Δ : SchemaContext} {φ : Face Γ} →
                            (σ : Sub Γ Δ) →
                            ((δ : Carrier Δ) → Holds (φ (apply σ δ))) →
                            Sub (cofibration Γ φ) Δ
    adjacent-exchange-sub : {Γ : SchemaContext} {left right : EntryKind}
                            {left-payload : ContextPayload (raw Γ) left}
                            {right-payload : ContextPayload (raw Γ) right} →
                            Sub
                              (extend (extend Γ left left-payload) right
                                (weaken-payload
                                  {outer-payload = left-payload}
                                  right-payload))
                              (extend (extend Γ right right-payload) left
                                (weaken-payload
                                  {outer-payload = right-payload}
                                  left-payload))

  apply : {Γ Δ : SchemaContext} → Sub Γ Δ → Carrier Δ → Carrier Γ
  apply identity-sub γ = γ
  apply (compose-sub σ τ) θ = apply σ (apply τ θ)
  apply weakening-sub γa = fst γa
  apply (type-image-sub σ B) δ = apply σ δ , B δ
  apply (term-image-sub σ term) δ = apply σ δ , term δ
  apply (library-image-sub σ term) δ = apply σ δ , term δ
  apply (dimension-image-sub σ dimension) δ = apply σ δ , dimension δ
  apply (cofibration-image-sub σ proof) δ = apply σ δ , proof δ
  apply
    (adjacent-exchange-sub
      {Γ = Γ} {left = left} {right = right}
      {left-payload = left-payload}
      {right-payload = right-payload}) δ =
    exchange-carrier
      {Γ = Γ} {left = left} {right = right}
      {left-payload = left-payload}
      {right-payload = right-payload}
      δ

id-sub : {Γ : SchemaContext} → Sub Γ Γ
id-sub = identity-sub

infixr 9 _∘s_

_∘s_ : {Γ Δ Θ : SchemaContext} → Sub Γ Δ → Sub Δ Θ → Sub Γ Θ
_∘s_ = compose-sub

weaken : {Γ : SchemaContext} {kind : EntryKind}
         {payload : ContextPayload (raw Γ) kind} →
         Sub Γ (extend Γ kind payload)
weaken = weakening-sub

substTy : {Γ Δ : SchemaContext} → Ty Γ → Sub Γ Δ → Ty Δ
substTy A σ δ = A (apply σ δ)

substTm : {Γ Δ : SchemaContext} {A : Ty Γ} →
          Tm Γ A → (σ : Sub Γ Δ) → Tm Δ (substTy A σ)
substTm term σ δ = term (apply σ δ)

substDim : {Γ Δ : SchemaContext} → Dim Γ → Sub Γ Δ → Dim Δ
substDim dimension σ δ = dimension (apply σ δ)

substFace : {Γ Δ : SchemaContext} → Face Γ → Sub Γ Δ → Face Δ
substFace face σ δ = face (apply σ δ)

top-type : {Γ : SchemaContext} → Ty (type-parameter Γ)
top-type (_ , B) = B

top-opaque : {Γ : SchemaContext} {A : Ty Γ} →
             Tm (opaque-context Γ A) (λ γa → A (fst γa))
top-opaque (_ , a) = a

top-library : {Γ : SchemaContext} {A : Ty Γ} →
              Tm (library-context Γ A) (λ γa → A (fst γa))
top-library (_ , a) = a

top-interval : {Γ : SchemaContext} → Dim (interval Γ)
top-interval (_ , i) = i

top-cofibration : {Γ : SchemaContext} {φ : Face Γ} →
                  (γ : Carrier (cofibration Γ φ)) →
                  Holds (φ (fst γ))
top-cofibration (_ , proof) = proof

-- A type-parameter image is an arbitrary well-formed target type, not only
-- another variable.  An element image is a genuine target term at the
-- already-substituted dependent type.
extend-type-image : {Γ Δ : SchemaContext} →
                    (σ : Sub Γ Δ) → Ty Δ → Sub (type-parameter Γ) Δ
extend-type-image = type-image-sub

extend-term-image : {Γ Δ : SchemaContext} {A : Ty Γ} →
                    (σ : Sub Γ Δ) → Tm Δ (substTy A σ) →
                    Sub (opaque-context Γ A) Δ
extend-term-image = term-image-sub

extend-library-image : {Γ Δ : SchemaContext} {A : Ty Γ} →
                       (σ : Sub Γ Δ) → Tm Δ (substTy A σ) →
                       Sub (library-context Γ A) Δ
extend-library-image = library-image-sub

extend-dimension-image : {Γ Δ : SchemaContext} →
                         (σ : Sub Γ Δ) → Dim Δ → Sub (interval Γ) Δ
extend-dimension-image = dimension-image-sub

extend-cofibration-image : {Γ Δ : SchemaContext} {φ : Face Γ} →
                           (σ : Sub Γ Δ) →
                           ((δ : Carrier Δ) → Holds (substFace φ σ δ)) →
                           Sub (cofibration Γ φ) Δ
extend-cofibration-image = cofibration-image-sub

-- Adjacent exchange is available only for two families independently pulled
-- back from Γ.  A genuinely dependent second family has no inhabitant of
-- this signature, so illegal exchange is unrepresentable here.
exchange : {Γ : SchemaContext} {left right : EntryKind}
           {left-payload : ContextPayload (raw Γ) left}
           {right-payload : ContextPayload (raw Γ) right} →
           Sub
             (extend (extend Γ left left-payload) right
               (weaken-payload {outer-payload = left-payload} right-payload))
             (extend (extend Γ right right-payload) left
               (weaken-payload {outer-payload = right-payload} left-payload))
exchange = adjacent-exchange-sub

exchange-inverse : {Γ : SchemaContext} {left right : EntryKind}
                   {left-payload : ContextPayload (raw Γ) left}
                   {right-payload : ContextPayload (raw Γ) right} →
                   (δ : Carrier
                     (extend (extend Γ right right-payload) left
                       (weaken-payload
                         {outer-payload = right-payload}
                         left-payload))) →
                   apply
                     (exchange
                       {Γ} {left = right} {right = left}
                       {left-payload = right-payload}
                       {right-payload = left-payload})
                     (apply
                       (exchange
                         {Γ} {left = left} {right = right}
                         {left-payload = left-payload}
                         {right-payload = right-payload})
                       δ)
                   ≡ δ
exchange-inverse
  {Γ = Γ} {left = left} {right = right}
  {left-payload = left-payload}
  {right-payload = right-payload} δ =
  exchange-carrier-inverse
    {Γ = Γ} {left = left} {right = right}
    {left-payload = left-payload}
    {right-payload = right-payload}
    δ

------------------------------------------------------------------------
-- Identity, composition, and preservation laws (pointwise, without K)
------------------------------------------------------------------------

substTy-id : {Γ : SchemaContext} → (A : Ty Γ) →
             (γ : Carrier Γ) → substTy A (id-sub {Γ}) γ ≡ A γ
substTy-id A γ = refl

substTm-id : {Γ : SchemaContext} {A : Ty Γ} → (term : Tm Γ A) →
             (γ : Carrier Γ) → substTm term (id-sub {Γ}) γ ≡ term γ
substTm-id term γ = refl

substTy-compose : {Γ Δ Θ : SchemaContext} →
                  (A : Ty Γ) → (σ : Sub Γ Δ) → (τ : Sub Δ Θ) →
                  (θ : Carrier Θ) →
                  substTy (substTy A σ) τ θ ≡ substTy A (σ ∘s τ) θ
substTy-compose A σ τ θ = refl

substTm-compose : {Γ Δ Θ : SchemaContext} {A : Ty Γ} →
                  (term : Tm Γ A) → (σ : Sub Γ Δ) → (τ : Sub Δ Θ) →
                  (θ : Carrier Θ) →
                  substTm (substTm term σ) τ θ ≡
                  substTm term (σ ∘s τ) θ
substTm-compose term σ τ θ = refl

-- Typing preservation is intrinsic in this signature: the codomain of
-- `substTm` is exactly the substituted source type.
record PreservationWitness
       {Γ Δ : SchemaContext} (A : Ty Γ) (term : Tm Γ A)
       (σ : Sub Γ Δ) : Set where
  constructor preserves
  field
    image : Tm Δ (substTy A σ)
    agrees : (δ : Carrier Δ) → image δ ≡ substTm term σ δ

preservation : {Γ Δ : SchemaContext} {A : Ty Γ} →
               (term : Tm Γ A) → (σ : Sub Γ Δ) →
               PreservationWitness A term σ
preservation term σ = preserves (substTm term σ) (λ _ → refl)

------------------------------------------------------------------------
-- Genuine non-variable typed instance
------------------------------------------------------------------------

ΓA : SchemaContext
ΓA = type-parameter empty

A : Ty ΓA
A (_ , B) = B

ΓA-x : SchemaContext
ΓA-x = opaque-context ΓA (λ γ → trunc-code (A γ))

ΓA-a : SchemaContext
ΓA-a = opaque-context ΓA A

-- The source endpoint x : Trunc(A) is instantiated by point(a), an actual
-- expression image.  The coarse KernelBridge variable-image fragment cannot
-- state this map; its typing is carried by this function's codomain.
point-instance : Sub ΓA-x ΓA-a
point-instance = extend-term-image weaken (λ { (γ , a) → point a })

point-instance-preserves : (δ : Carrier ΓA-a) →
                           snd (apply point-instance δ) ≡ point (snd δ)
point-instance-preserves _ = refl

------------------------------------------------------------------------
-- The four registered TRUNC-ER endpoint maps
------------------------------------------------------------------------

ΓA-x-y : SchemaContext
ΓA-x-y = opaque-context ΓA-x (λ γx → trunc-code (A (fst γx)))

x-endpoint : Tm ΓA-x-y (λ γxy → trunc-code (A (fst (fst γxy))))
x-endpoint ((γ , x) , y) = x

y-endpoint : Tm ΓA-x-y (λ γxy → trunc-code (A (fst (fst γxy))))
y-endpoint = snd

trunc-base : Sub ΓA ΓA-x-y
trunc-base = weaken ∘s weaken

trunc-identity : Sub ΓA-x-y ΓA-x-y
trunc-identity = id-sub

trunc-swap : Sub ΓA-x-y ΓA-x-y
trunc-swap =
  extend-term-image (extend-term-image trunc-base y-endpoint) x-endpoint

trunc-collapse-x : Sub ΓA-x-y ΓA-x-y
trunc-collapse-x =
  extend-term-image (extend-term-image trunc-base x-endpoint) x-endpoint

trunc-collapse-y : Sub ΓA-x-y ΓA-x-y
trunc-collapse-y =
  extend-term-image (extend-term-image trunc-base y-endpoint) y-endpoint

swap-x : (δ : Carrier ΓA-x-y) →
         substTm x-endpoint trunc-swap δ ≡ y-endpoint δ
swap-x _ = refl

swap-y : (δ : Carrier ΓA-x-y) →
         substTm y-endpoint trunc-swap δ ≡ x-endpoint δ
swap-y _ = refl

collapse-x-left : (δ : Carrier ΓA-x-y) →
                  substTm x-endpoint trunc-collapse-x δ ≡ x-endpoint δ
collapse-x-left _ = refl

collapse-x-right : (δ : Carrier ΓA-x-y) →
                   substTm y-endpoint trunc-collapse-x δ ≡ x-endpoint δ
collapse-x-right _ = refl

collapse-y-left : (δ : Carrier ΓA-x-y) →
                  substTm x-endpoint trunc-collapse-y δ ≡ y-endpoint δ
collapse-y-left _ = refl

collapse-y-right : (δ : Carrier ΓA-x-y) →
                   substTm y-endpoint trunc-collapse-y δ ≡ y-endpoint δ
collapse-y-right _ = refl

------------------------------------------------------------------------
-- Explicit extension of the shipped restricted KernelBridge theorem
------------------------------------------------------------------------

kernel-restricted-sort-preservation :
  {source target : KB.ParameterSort} →
  KB.RestrictedVariableImage source target → source ≡ target
kernel-restricted-sort-preservation = KB.restricted-variable-sort-preservation

-- Scope boundary: this E-1 module proves context/substitution structure only.
-- It defines no Schema2 constructor inventory, normalizer, classifier, score,
-- acceptance comparison, or global-halt theorem.
