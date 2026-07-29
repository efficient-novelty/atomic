{-# OPTIONS --safe --without-K #-}

module GscInductiveCoreV1 where

open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import Agda.Builtin.String using (String)

data ComputationMode : Set where
  fresh-computation : ComputationMode
  path-computation  : ComputationMode

record ConstructorCode : Set where
  constructor constructor-code
  field
    argument-count : Nat
    recursive-positions : List Nat

record ClosedInductiveCode : Set where
  constructor closed-inductive-code
  field
    constructors : List ConstructorCode
    computation-mode : ComputationMode

data Decision : Set where
  supported : Decision
  unknown   : Decision

data VectorDisposition : Set where
  proven-supported                      : VectorDisposition
  proven-generated-recursive-call-beta : VectorDisposition
  unknown-unsupported-computation-mode  : VectorDisposition

vector-spec-token : String
vector-spec-token =
  "gsc-inductive-core-generic-four-v1/two-nullary=proven-supported;nonrecursive=proven-supported;recursive=proven-generated-recursive-call-beta;path=unknown-unsupported-computation-mode"

length : {A : Set} → List A → Nat
length [] = zero
length (_ ∷ values) = suc (length values)

_+_ : Nat → Nat → Nat
zero + right = right
suc left + right = suc (left + right)

decideMode : ComputationMode → Decision
decideMode fresh-computation = supported
decideMode path-computation = unknown

freshDisposition : List ConstructorCode → VectorDisposition
freshDisposition [] = proven-supported
freshDisposition (constructor-code arguments [] ∷ rest) =
  freshDisposition rest
freshDisposition (constructor-code arguments (_ ∷ recursive) ∷ rest) =
  proven-generated-recursive-call-beta

interpretCode : ClosedInductiveCode → VectorDisposition
interpretCode (closed-inductive-code constructors fresh-computation) =
  freshDisposition constructors
interpretCode (closed-inductive-code constructors path-computation) =
  unknown-unsupported-computation-mode

useParameterCount : ClosedInductiveCode → Nat
useParameterCount code =
  suc (length (ClosedInductiveCode.constructors code))

computeContextSize : ClosedInductiveCode → ConstructorCode → Nat
computeContextSize code ctor =
  useParameterCount code +
  suc (ConstructorCode.argument-count ctor)

twoNullary : ClosedInductiveCode
twoNullary =
  closed-inductive-code
    (constructor-code zero [] ∷ constructor-code zero [] ∷ [])
    fresh-computation

oneNonrecursive : ClosedInductiveCode
oneNonrecursive =
  closed-inductive-code
    (constructor-code (suc zero) [] ∷ [])
    fresh-computation

oneRecursive : ClosedInductiveCode
oneRecursive =
  closed-inductive-code
    (constructor-code (suc zero) (zero ∷ []) ∷ [])
    fresh-computation

pathMode : ClosedInductiveCode
pathMode =
  closed-inductive-code
    (constructor-code zero [] ∷ [])
    path-computation

pathModeFailsClosed :
  decideMode (ClosedInductiveCode.computation-mode pathMode) ≡ unknown
pathModeFailsClosed = refl

twoNullaryUseParameterCount :
  useParameterCount twoNullary ≡ suc (suc (suc zero))
twoNullaryUseParameterCount = refl

nonrecursiveUseParameterCount :
  useParameterCount oneNonrecursive ≡ suc (suc zero)
nonrecursiveUseParameterCount = refl

nonrecursiveComputeContextSize :
  computeContextSize
    oneNonrecursive
    (constructor-code (suc zero) [])
  ≡ suc (suc (suc (suc zero)))
nonrecursiveComputeContextSize = refl

recursiveComputeContextSize :
  computeContextSize
    oneRecursive
    (constructor-code (suc zero) (zero ∷ []))
  ≡ suc (suc (suc (suc zero)))
recursiveComputeContextSize = refl

vectorDispositions : List VectorDisposition
vectorDispositions =
  interpretCode twoNullary ∷
  interpretCode oneNonrecursive ∷
  interpretCode oneRecursive ∷
  interpretCode pathMode ∷
  []

fourVectorDispositions :
  vectorDispositions
  ≡
  (proven-supported ∷
   proven-supported ∷
   proven-generated-recursive-call-beta ∷
   unknown-unsupported-computation-mode ∷
   [])
fourVectorDispositions = refl
