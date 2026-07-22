{-# OPTIONS --safe --without-K #-}

module Schema2E3E4ClassInduction where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Nat using (Nat; zero; suc)

------------------------------------------------------------------------
-- Small constructive helpers
------------------------------------------------------------------------

data Empty : Set where

cong : {A B : Set} {x y : A} ->
       (f : A -> B) -> x ≡ y -> f x ≡ f y
cong f refl = refl

cong₂ : {A B C : Set}
        (f : A -> B -> C)
        {a a' : A} {b b' : B} ->
        a ≡ a' -> b ≡ b' -> f a b ≡ f a' b'
cong₂ f refl refl = refl

cong₃ : {A B C D : Set}
        (f : A -> B -> C -> D)
        {a a' : A} {b b' : B} {c c' : C} ->
        a ≡ a' -> b ≡ b' -> c ≡ c' ->
        f a b c ≡ f a' b' c'
cong₃ f refl refl refl = refl

cong₄ : {A B C D E : Set}
        (f : A -> B -> C -> D -> E)
        {a a' : A} {b b' : B} {c c' : C} {d d' : D} ->
        a ≡ a' -> b ≡ b' -> c ≡ c' -> d ≡ d' ->
        f a b c d ≡ f a' b' c' d'
cong₄ f refl refl refl refl = refl

cong₅ : {A B C D E F : Set}
        (f : A -> B -> C -> D -> E -> F)
        {a a' : A} {b b' : B} {c c' : C} {d d' : D} {e e' : E} ->
        a ≡ a' -> b ≡ b' -> c ≡ c' -> d ≡ d' -> e ≡ e' ->
        f a b c d e ≡ f a' b' c' d' e'
cong₅ f refl refl refl refl refl = refl

------------------------------------------------------------------------
-- Full normalization/naturality induction for the operational registry
------------------------------------------------------------------------

data Fin : Nat -> Set where
  fzero : {n : Nat} -> Fin (suc n)
  fsuc  : {n : Nat} -> Fin n -> Fin (suc n)

data OTy (n : Nat) : Set where
  parameter : Fin n -> OTy n
  trunc     : OTy n -> OTy n

data OTm (n : Nat) : Set where
  o-var       : Fin n -> OTm n
  trunc-point : OTy n -> OTm n -> OTm n
  reflexivity : OTy n -> OTm n -> OTm n

record OSub (n m : Nat) : Set where
  constructor ordinary-substitution
  field
    type-image : Fin n -> OTy m
    term-image : Fin n -> OTm m

open OSub public

substitute-type : {n m : Nat} -> OSub n m -> OTy n -> OTy m
substitute-type σ (parameter x) = type-image σ x
substitute-type σ (trunc A) = trunc (substitute-type σ A)

substitute-expression : {n m : Nat} -> OSub n m -> OTm n -> OTm m
substitute-expression σ (o-var x) = term-image σ x
substitute-expression σ (trunc-point A t) =
  trunc-point (substitute-type σ A) (substitute-expression σ t)
substitute-expression σ (reflexivity A t) =
  reflexivity (substitute-type σ A) (substitute-expression σ t)

data OrdinaryKind : Set where
  fresh-formation-kind         : OrdinaryKind
  point-or-unit-kind           : OrdinaryKind
  path-constructor-kind        : OrdinaryKind
  recursor-kind                : OrdinaryKind
  inductor-kind                : OrdinaryKind
  trunc-parametric-action-kind : OrdinaryKind
  post-path-operation-kind     : OrdinaryKind
  post-path-coherence-kind     : OrdinaryKind
  cell-action-kind             : OrdinaryKind

data RawOrdinary (n : Nat) : Set where
  fresh-formation         : OTy n -> RawOrdinary n
  point-or-unit           : OTy n -> OTm n -> RawOrdinary n
  path-constructor        : OTy n -> OTm n -> OTm n -> RawOrdinary n
  recursor                : OTy n -> OTy n -> RawOrdinary n
  inductor                : OTy n -> OTy n -> RawOrdinary n
  trunc-parametric-action : OTy n -> OTy n -> RawOrdinary n
  post-path-operation     : OTy n -> Nat -> RawOrdinary n
  post-path-coherence     : OTy n -> OTm n -> OTm n -> RawOrdinary n
  cell-action             : OTy n -> Nat -> RawOrdinary n
  ordinary-redex          : RawOrdinary n -> RawOrdinary n

data NormalOrdinary (n : Nat) : Set where
  n-fresh-formation         : OTy n -> NormalOrdinary n
  n-point-or-unit           : OTy n -> OTm n -> NormalOrdinary n
  n-path-constructor        : OTy n -> OTm n -> OTm n -> NormalOrdinary n
  n-recursor                : OTy n -> OTy n -> NormalOrdinary n
  n-inductor                : OTy n -> OTy n -> NormalOrdinary n
  n-trunc-parametric-action : OTy n -> OTy n -> NormalOrdinary n
  n-post-path-operation     : OTy n -> Nat -> NormalOrdinary n
  n-post-path-coherence     : OTy n -> OTm n -> OTm n -> NormalOrdinary n
  n-cell-action             : OTy n -> Nat -> NormalOrdinary n

normalize-ordinary : {n : Nat} -> RawOrdinary n -> NormalOrdinary n
normalize-ordinary (fresh-formation A) = n-fresh-formation A
normalize-ordinary (point-or-unit A x) = n-point-or-unit A x
normalize-ordinary (path-constructor A x y) = n-path-constructor A x y
normalize-ordinary (recursor A B) = n-recursor A B
normalize-ordinary (inductor A B) = n-inductor A B
normalize-ordinary (trunc-parametric-action A B) =
  n-trunc-parametric-action A B
normalize-ordinary (post-path-operation A arity) =
  n-post-path-operation A arity
normalize-ordinary (post-path-coherence A unit x) =
  n-post-path-coherence A unit x
normalize-ordinary (cell-action A dimension) =
  n-cell-action A dimension
normalize-ordinary (ordinary-redex schema) = normalize-ordinary schema

substitute-raw : {n m : Nat} ->
                 OSub n m -> RawOrdinary n -> RawOrdinary m
substitute-raw σ (fresh-formation A) =
  fresh-formation (substitute-type σ A)
substitute-raw σ (point-or-unit A x) =
  point-or-unit (substitute-type σ A) (substitute-expression σ x)
substitute-raw σ (path-constructor A x y) =
  path-constructor (substitute-type σ A)
                   (substitute-expression σ x)
                   (substitute-expression σ y)
substitute-raw σ (recursor A B) =
  recursor (substitute-type σ A) (substitute-type σ B)
substitute-raw σ (inductor A B) =
  inductor (substitute-type σ A) (substitute-type σ B)
substitute-raw σ (trunc-parametric-action A B) =
  trunc-parametric-action (substitute-type σ A) (substitute-type σ B)
substitute-raw σ (post-path-operation A arity) =
  post-path-operation (substitute-type σ A) arity
substitute-raw σ (post-path-coherence A unit x) =
  post-path-coherence (substitute-type σ A)
                      (substitute-expression σ unit)
                      (substitute-expression σ x)
substitute-raw σ (cell-action A dimension) =
  cell-action (substitute-type σ A) dimension
substitute-raw σ (ordinary-redex schema) =
  ordinary-redex (substitute-raw σ schema)

substitute-normal : {n m : Nat} ->
                    OSub n m -> NormalOrdinary n -> NormalOrdinary m
substitute-normal σ (n-fresh-formation A) =
  n-fresh-formation (substitute-type σ A)
substitute-normal σ (n-point-or-unit A x) =
  n-point-or-unit (substitute-type σ A) (substitute-expression σ x)
substitute-normal σ (n-path-constructor A x y) =
  n-path-constructor (substitute-type σ A)
                     (substitute-expression σ x)
                     (substitute-expression σ y)
substitute-normal σ (n-recursor A B) =
  n-recursor (substitute-type σ A) (substitute-type σ B)
substitute-normal σ (n-inductor A B) =
  n-inductor (substitute-type σ A) (substitute-type σ B)
substitute-normal σ (n-trunc-parametric-action A B) =
  n-trunc-parametric-action (substitute-type σ A) (substitute-type σ B)
substitute-normal σ (n-post-path-operation A arity) =
  n-post-path-operation (substitute-type σ A) arity
substitute-normal σ (n-post-path-coherence A unit x) =
  n-post-path-coherence (substitute-type σ A)
                        (substitute-expression σ unit)
                        (substitute-expression σ x)
substitute-normal σ (n-cell-action A dimension) =
  n-cell-action (substitute-type σ A) dimension

ordinary-normalization-naturality :
  {n m : Nat} -> (sigma : OSub n m) -> (schema : RawOrdinary n) ->
  normalize-ordinary (substitute-raw sigma schema) ≡
  substitute-normal sigma (normalize-ordinary schema)
ordinary-normalization-naturality sigma (fresh-formation A) = refl
ordinary-normalization-naturality sigma (point-or-unit A x) = refl
ordinary-normalization-naturality sigma (path-constructor A x y) = refl
ordinary-normalization-naturality sigma (recursor A B) = refl
ordinary-normalization-naturality sigma (inductor A B) = refl
ordinary-normalization-naturality sigma (trunc-parametric-action A B) = refl
ordinary-normalization-naturality sigma (post-path-operation A arity) = refl
ordinary-normalization-naturality sigma (post-path-coherence A unit x) = refl
ordinary-normalization-naturality sigma (cell-action A dimension) = refl
ordinary-normalization-naturality sigma (ordinary-redex schema) =
  ordinary-normalization-naturality sigma schema

kind-of : {n : Nat} -> NormalOrdinary n -> OrdinaryKind
kind-of (n-fresh-formation A) = fresh-formation-kind
kind-of (n-point-or-unit A x) = point-or-unit-kind
kind-of (n-path-constructor A x y) = path-constructor-kind
kind-of (n-recursor A B) = recursor-kind
kind-of (n-inductor A B) = inductor-kind
kind-of (n-trunc-parametric-action A B) = trunc-parametric-action-kind
kind-of (n-post-path-operation A arity) = post-path-operation-kind
kind-of (n-post-path-coherence A unit x) = post-path-coherence-kind
kind-of (n-cell-action A dimension) = cell-action-kind

data SchemaClass : Set where
  foundation : SchemaClass
  former     : SchemaClass
  map-class  : SchemaClass
  axiomatic  : SchemaClass
  modal      : SchemaClass
  hit-v2     : SchemaClass
  synthesis  : SchemaClass
  unknown    : SchemaClass

class-of : {n : Nat} -> NormalOrdinary n -> SchemaClass
class-of (n-fresh-formation A) = foundation
class-of (n-point-or-unit A x) = foundation
class-of (n-path-constructor A x y) = hit-v2
class-of (n-recursor A B) = former
class-of (n-inductor A B) = former
class-of (n-trunc-parametric-action A B) = map-class
class-of (n-post-path-operation A arity) = map-class
class-of (n-post-path-coherence A unit x) = axiomatic
class-of (n-cell-action A dimension) = hit-v2

record ClassIndexedNaturality {n m : Nat}
                               (sigma : OSub n m)
                               (schema : RawOrdinary n) : Set where
  constructor class-indexed-natural
  field
    normal-form-natural :
      normalize-ordinary (substitute-raw sigma schema) ≡
      substitute-normal sigma (normalize-ordinary schema)
    kind-natural :
      kind-of (normalize-ordinary (substitute-raw sigma schema)) ≡
      kind-of (substitute-normal sigma (normalize-ordinary schema))
    class-natural :
      class-of (normalize-ordinary (substitute-raw sigma schema)) ≡
      class-of (substitute-normal sigma (normalize-ordinary schema))

full-registered-class-indexed-naturality :
  {n m : Nat} -> (sigma : OSub n m) -> (schema : RawOrdinary n) ->
  ClassIndexedNaturality sigma schema
full-registered-class-indexed-naturality sigma (fresh-formation A) =
  class-indexed-natural refl refl refl
full-registered-class-indexed-naturality sigma (point-or-unit A x) =
  class-indexed-natural refl refl refl
full-registered-class-indexed-naturality sigma (path-constructor A x y) =
  class-indexed-natural refl refl refl
full-registered-class-indexed-naturality sigma (recursor A B) =
  class-indexed-natural refl refl refl
full-registered-class-indexed-naturality sigma (inductor A B) =
  class-indexed-natural refl refl refl
full-registered-class-indexed-naturality sigma (trunc-parametric-action A B) =
  class-indexed-natural refl refl refl
full-registered-class-indexed-naturality sigma (post-path-operation A arity) =
  class-indexed-natural refl refl refl
full-registered-class-indexed-naturality sigma (post-path-coherence A unit x) =
  class-indexed-natural refl refl refl
full-registered-class-indexed-naturality sigma (cell-action A dimension) =
  class-indexed-natural refl refl refl
full-registered-class-indexed-naturality sigma (ordinary-redex schema) with
  full-registered-class-indexed-naturality sigma schema
... | class-indexed-natural normal kind class =
  class-indexed-natural normal kind class

-- These are constructive witnesses of the current grammar boundary: no
-- registered normal constructor is Modal, Synthesis, or Unknown.
no-modal-registered-normal : {n : Nat} -> (schema : NormalOrdinary n) ->
                             class-of schema ≡ modal -> Empty
no-modal-registered-normal (n-fresh-formation A) ()
no-modal-registered-normal (n-point-or-unit A x) ()
no-modal-registered-normal (n-path-constructor A x y) ()
no-modal-registered-normal (n-recursor A B) ()
no-modal-registered-normal (n-inductor A B) ()
no-modal-registered-normal (n-trunc-parametric-action A B) ()
no-modal-registered-normal (n-post-path-operation A arity) ()
no-modal-registered-normal (n-post-path-coherence A unit x) ()
no-modal-registered-normal (n-cell-action A dimension) ()

no-synthesis-registered-normal : {n : Nat} -> (schema : NormalOrdinary n) ->
                                 class-of schema ≡ synthesis -> Empty
no-synthesis-registered-normal (n-fresh-formation A) ()
no-synthesis-registered-normal (n-point-or-unit A x) ()
no-synthesis-registered-normal (n-path-constructor A x y) ()
no-synthesis-registered-normal (n-recursor A B) ()
no-synthesis-registered-normal (n-inductor A B) ()
no-synthesis-registered-normal (n-trunc-parametric-action A B) ()
no-synthesis-registered-normal (n-post-path-operation A arity) ()
no-synthesis-registered-normal (n-post-path-coherence A unit x) ()
no-synthesis-registered-normal (n-cell-action A dimension) ()

------------------------------------------------------------------------
-- Dependent cubical dimension action, including coe and hcom
------------------------------------------------------------------------

data Dim (n : Nat) : Set where
  dim-zero : Dim n
  dim-one  : Dim n
  dim-var  : Fin n -> Dim n

DimSub : Nat -> Nat -> Set
DimSub n m = Fin n -> Dim m

weaken-dim : {n : Nat} -> Dim n -> Dim (suc n)
weaken-dim dim-zero = dim-zero
weaken-dim dim-one = dim-one
weaken-dim (dim-var x) = dim-var (fsuc x)

lift-dim-sub : {n m : Nat} -> DimSub n m -> DimSub (suc n) (suc m)
lift-dim-sub sigma fzero = dim-var fzero
lift-dim-sub sigma (fsuc x) = weaken-dim (sigma x)

act-dim : {n m : Nat} -> DimSub n m -> Dim n -> Dim m
act-dim sigma dim-zero = dim-zero
act-dim sigma dim-one = dim-one
act-dim sigma (dim-var x) = sigma x

data CubicalKind : Set where
  point-kind                          : CubicalKind
  motive-base-kind                    : CubicalKind
  endpoint-evaluation-hypothesis-kind : CubicalKind
  path-method-kind                    : CubicalKind
  endpoint-method-hypothesis-kind     : CubicalKind
  path-elim-kind                      : CubicalKind
  endpoint-path-elim-kind             : CubicalKind
  endpoint-elim-neutral-kind          : CubicalKind
  dim-lambda-kind                     : CubicalKind
  dim-app-kind                        : CubicalKind
  coe-kind                            : CubicalKind
  hcom-kind                           : CubicalKind

-- Only the eight constructors accepted by the public cubical context are
-- terms here.  The four endpoint-premise constructors remain named above,
-- but require the private premise context in the Rust kernel.
data CubicalTerm (n : Nat) : Set where
  c-point       : CubicalTerm n
  c-motive-base : Dim n -> CubicalTerm n
  c-path-method : CubicalTerm n -> CubicalTerm n
  c-path-elim   : CubicalTerm n -> CubicalTerm n -> CubicalTerm n -> CubicalTerm n
  c-dim-lambda  : CubicalTerm (suc n) -> CubicalTerm n
  c-dim-app     : CubicalTerm n -> Dim n -> CubicalTerm n
  c-coe         : Dim (suc n) -> Dim n -> Dim n -> CubicalTerm n -> CubicalTerm n
  c-hcom        : Dim (suc n) -> Dim n -> Dim n ->
                  CubicalTerm n -> List (CubicalTerm n) -> CubicalTerm n

data NormalCubical (n : Nat) : Set where
  nc-point       : NormalCubical n
  nc-motive-base : Dim n -> NormalCubical n
  nc-path-method : NormalCubical n -> NormalCubical n
  nc-path-elim   : NormalCubical n -> NormalCubical n ->
                   NormalCubical n -> NormalCubical n
  nc-dim-lambda  : NormalCubical (suc n) -> NormalCubical n
  nc-dim-app     : NormalCubical n -> Dim n -> NormalCubical n
  nc-coe         : Dim (suc n) -> Dim n -> Dim n ->
                   NormalCubical n -> NormalCubical n
  nc-hcom        : Dim (suc n) -> Dim n -> Dim n ->
                   NormalCubical n -> List (NormalCubical n) -> NormalCubical n

mutual
  normalize-cubical-list : {n : Nat} ->
                           List (CubicalTerm n) -> List (NormalCubical n)
  normalize-cubical-list [] = []
  normalize-cubical-list (term ∷ terms) =
    normalize-cubical term ∷ normalize-cubical-list terms

  normalize-cubical : {n : Nat} -> CubicalTerm n -> NormalCubical n
  normalize-cubical c-point = nc-point
  normalize-cubical (c-motive-base r) = nc-motive-base r
  normalize-cubical (c-path-method method) =
    nc-path-method (normalize-cubical method)
  normalize-cubical (c-path-elim base method scrutinee) =
    nc-path-elim (normalize-cubical base)
                 (normalize-cubical method)
                 (normalize-cubical scrutinee)
  normalize-cubical (c-dim-lambda body) =
    nc-dim-lambda (normalize-cubical body)
  normalize-cubical (c-dim-app function argument) =
    nc-dim-app (normalize-cubical function) argument
  normalize-cubical (c-coe family from to term) =
    nc-coe family from to (normalize-cubical term)
  normalize-cubical (c-hcom family from to cap tubes) =
    nc-hcom family from to (normalize-cubical cap)
            (normalize-cubical-list tubes)

mutual
  act-cubical-list : {n m : Nat} -> DimSub n m ->
                     List (CubicalTerm n) -> List (CubicalTerm m)
  act-cubical-list sigma [] = []
  act-cubical-list sigma (term ∷ terms) =
    act-cubical sigma term ∷ act-cubical-list sigma terms

  act-cubical : {n m : Nat} ->
                DimSub n m -> CubicalTerm n -> CubicalTerm m
  act-cubical sigma c-point = c-point
  act-cubical sigma (c-motive-base r) = c-motive-base (act-dim sigma r)
  act-cubical sigma (c-path-method method) =
    c-path-method (act-cubical sigma method)
  act-cubical sigma (c-path-elim base method scrutinee) =
    c-path-elim (act-cubical sigma base)
                (act-cubical sigma method)
                (act-cubical sigma scrutinee)
  act-cubical sigma (c-dim-lambda body) =
    c-dim-lambda (act-cubical (lift-dim-sub sigma) body)
  act-cubical sigma (c-dim-app function argument) =
    c-dim-app (act-cubical sigma function) (act-dim sigma argument)
  act-cubical sigma (c-coe family from to term) =
    c-coe (act-dim (lift-dim-sub sigma) family)
          (act-dim sigma from) (act-dim sigma to)
          (act-cubical sigma term)
  act-cubical sigma (c-hcom family from to cap tubes) =
    c-hcom (act-dim (lift-dim-sub sigma) family)
           (act-dim sigma from) (act-dim sigma to)
           (act-cubical sigma cap) (act-cubical-list sigma tubes)

mutual
  act-normal-list : {n m : Nat} -> DimSub n m ->
                    List (NormalCubical n) -> List (NormalCubical m)
  act-normal-list sigma [] = []
  act-normal-list sigma (term ∷ terms) =
    act-normal sigma term ∷ act-normal-list sigma terms

  act-normal : {n m : Nat} ->
               DimSub n m -> NormalCubical n -> NormalCubical m
  act-normal sigma nc-point = nc-point
  act-normal sigma (nc-motive-base r) = nc-motive-base (act-dim sigma r)
  act-normal sigma (nc-path-method method) =
    nc-path-method (act-normal sigma method)
  act-normal sigma (nc-path-elim base method scrutinee) =
    nc-path-elim (act-normal sigma base)
                 (act-normal sigma method)
                 (act-normal sigma scrutinee)
  act-normal sigma (nc-dim-lambda body) =
    nc-dim-lambda (act-normal (lift-dim-sub sigma) body)
  act-normal sigma (nc-dim-app function argument) =
    nc-dim-app (act-normal sigma function) (act-dim sigma argument)
  act-normal sigma (nc-coe family from to term) =
    nc-coe (act-dim (lift-dim-sub sigma) family)
           (act-dim sigma from) (act-dim sigma to)
           (act-normal sigma term)
  act-normal sigma (nc-hcom family from to cap tubes) =
    nc-hcom (act-dim (lift-dim-sub sigma) family)
            (act-dim sigma from) (act-dim sigma to)
            (act-normal sigma cap) (act-normal-list sigma tubes)

mutual
  normalize-list-action : {n m : Nat} ->
    (sigma : DimSub n m) -> (terms : List (CubicalTerm n)) ->
    normalize-cubical-list (act-cubical-list sigma terms) ≡
    act-normal-list sigma (normalize-cubical-list terms)
  normalize-list-action sigma [] = refl
  normalize-list-action sigma (term ∷ terms) =
    cong₂ _∷_ (normalize-cubical-action sigma term)
              (normalize-list-action sigma terms)

  normalize-cubical-action : {n m : Nat} ->
    (sigma : DimSub n m) -> (term : CubicalTerm n) ->
    normalize-cubical (act-cubical sigma term) ≡
    act-normal sigma (normalize-cubical term)
  normalize-cubical-action sigma c-point = refl
  normalize-cubical-action sigma (c-motive-base r) = refl
  normalize-cubical-action sigma (c-path-method method) =
    cong nc-path-method (normalize-cubical-action sigma method)
  normalize-cubical-action sigma (c-path-elim base method scrutinee) =
    cong₃ nc-path-elim
          (normalize-cubical-action sigma base)
          (normalize-cubical-action sigma method)
          (normalize-cubical-action sigma scrutinee)
  normalize-cubical-action sigma (c-dim-lambda body) =
    cong nc-dim-lambda (normalize-cubical-action (lift-dim-sub sigma) body)
  normalize-cubical-action sigma (c-dim-app function argument) =
    cong (\ body -> nc-dim-app body (act-dim sigma argument))
         (normalize-cubical-action sigma function)
  normalize-cubical-action sigma (c-coe family from to term) =
    cong (\ body -> nc-coe (act-dim (lift-dim-sub sigma) family)
                           (act-dim sigma from) (act-dim sigma to) body)
         (normalize-cubical-action sigma term)
  normalize-cubical-action sigma (c-hcom family from to cap tubes) =
    cong₅ nc-hcom refl refl refl
          (normalize-cubical-action sigma cap)
          (normalize-list-action sigma tubes)

constructor-of : {n : Nat} -> CubicalTerm n -> CubicalKind
constructor-of c-point = point-kind
constructor-of (c-motive-base r) = motive-base-kind
constructor-of (c-path-method method) = path-method-kind
constructor-of (c-path-elim base method scrutinee) = path-elim-kind
constructor-of (c-dim-lambda body) = dim-lambda-kind
constructor-of (c-dim-app function argument) = dim-app-kind
constructor-of (c-coe family from to term) = coe-kind
constructor-of (c-hcom family from to cap tubes) = hcom-kind

data PublicCubicalKind : CubicalKind -> Set where
  public-point       : PublicCubicalKind point-kind
  public-motive-base : PublicCubicalKind motive-base-kind
  public-path-method : PublicCubicalKind path-method-kind
  public-path-elim   : PublicCubicalKind path-elim-kind
  public-dim-lambda  : PublicCubicalKind dim-lambda-kind
  public-dim-app     : PublicCubicalKind dim-app-kind
  public-coe         : PublicCubicalKind coe-kind
  public-hcom        : PublicCubicalKind hcom-kind

dependent-public-cubical-constructor-induction :
  {n : Nat} -> (term : CubicalTerm n) -> PublicCubicalKind (constructor-of term)
dependent-public-cubical-constructor-induction c-point = public-point
dependent-public-cubical-constructor-induction (c-motive-base r) = public-motive-base
dependent-public-cubical-constructor-induction (c-path-method method) = public-path-method
dependent-public-cubical-constructor-induction (c-path-elim base method scrutinee) =
  public-path-elim
dependent-public-cubical-constructor-induction (c-dim-lambda body) = public-dim-lambda
dependent-public-cubical-constructor-induction (c-dim-app function argument) = public-dim-app
dependent-public-cubical-constructor-induction (c-coe family from to term) = public-coe
dependent-public-cubical-constructor-induction (c-hcom family from to cap tubes) = public-hcom

-- Empty by construction in this public mirror.  Supplying this API, and
-- proving its typing/normalization compatibility, is the remaining generic
-- endpoint-premise obligation; it is not silently inferred from the two
-- registered Trunc endpoint bundles.
data PublicEndpointPremiseContext : Set where
