{-# OPTIONS --safe --without-K #-}

-- Canonical Agda-side encoders and parse/encode round-trip theorems for
-- the semantic wire payloads.
--
-- Canonical structures carry the exact little-endian words (following the
-- `CanonicalSizedBytes` pattern in `Decoder.agda`); erasure computes the
-- semantic value, so no numeric conversion function is trusted. The
-- recursive payloads (terms, reduction steps) receive fuel-generalized
-- round trips whose depth witnesses mirror the shared Rust recursion
-- budget.

module LawV2.Wire.BundleEncode where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe using (Maybe; just; nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.Wire.Bytes
open import LawV2.Wire.Decoder
open import LawV2.Wire.ProductionBundleV1

sym : {A : Set} {left right : A} → left ≡ right → right ≡ left
sym refl = refl

++-assoc :
  {A : Set} (first second third : List A) →
  (first ++ second) ++ third ≡ first ++ (second ++ third)
++-assoc [] second third = refl
++-assoc (x ∷ first) second third
  rewrite ++-assoc first second third = refl

++-right-identity : {A : Set} (values : List A) → values ++ [] ≡ values
++-right-identity [] = refl
++-right-identity (x ∷ values)
  rewrite ++-right-identity values = refl

map : {A B : Set} → (A → B) → List A → List B
map function [] = []
map function (value ∷ values) = function value ∷ map function values

concat-encode : {A : Set} → (A → List Byte) → List A → List Byte
concat-encode encode-item [] = []
concat-encode encode-item (value ∷ values) =
  encode-item value ++ concat-encode encode-item values

-- Canonical 32-byte identifiers.
record CanonicalIdV1 : Set where
  constructor canonical-id-v1
  field
    id-bytes : List Byte
    id-length-canonical : length id-bytes ≡ 32

open CanonicalIdV1 public

encode-canonical-id : CanonicalIdV1 → List Byte
encode-canonical-id identifier = id-bytes identifier

parse-wire-id-encode :
  (identifier : CanonicalIdV1) (rest : List Byte) →
  parse-wire-id (encode-canonical-id identifier ++ rest)
  ≡ just (id-bytes identifier , rest)
parse-wire-id-encode (canonical-id-v1 bytes canonical) rest
  rewrite sym canonical
        | take-exact-append bytes rest = refl

-- Canonical wire terms carry the exact numeric words.
data CanonicalTermV1 : Set where
  canonical-sort : U16 → CanonicalTermV1
  canonical-variable : U32 → CanonicalTermV1
  canonical-global-slot : U32 → CanonicalTermV1
  canonical-pi : CanonicalTermV1 → CanonicalTermV1 → CanonicalTermV1
  canonical-lambda : CanonicalTermV1 → CanonicalTermV1 → CanonicalTermV1
  canonical-apply : CanonicalTermV1 → CanonicalTermV1 → CanonicalTermV1
  canonical-unit-type : CanonicalTermV1
  canonical-unit : CanonicalTermV1

erase-canonical-term : CanonicalTermV1 → WireTermV1
erase-canonical-term (canonical-sort level) = wire-sort (u16-value level)
erase-canonical-term (canonical-variable index) =
  wire-variable (u32-value index)
erase-canonical-term (canonical-global-slot slot) =
  wire-global-slot (u32-value slot)
erase-canonical-term (canonical-pi parameter body) =
  wire-pi (erase-canonical-term parameter) (erase-canonical-term body)
erase-canonical-term (canonical-lambda parameter body) =
  wire-lambda (erase-canonical-term parameter) (erase-canonical-term body)
erase-canonical-term (canonical-apply function argument) =
  wire-apply (erase-canonical-term function) (erase-canonical-term argument)
erase-canonical-term canonical-unit-type = wire-unit-type
erase-canonical-term canonical-unit = wire-unit

encode-canonical-term : CanonicalTermV1 → List Byte
encode-canonical-term (canonical-sort level) = byte0 ∷ encode-u16 level
encode-canonical-term (canonical-variable index) = byte1 ∷ encode-u32 index
encode-canonical-term (canonical-global-slot slot) = byte2 ∷ encode-u32 slot
encode-canonical-term (canonical-pi parameter body) =
  byte3 ∷
  (encode-canonical-term parameter ++ encode-canonical-term body)
encode-canonical-term (canonical-lambda parameter body) =
  byte4 ∷
  (encode-canonical-term parameter ++ encode-canonical-term body)
encode-canonical-term (canonical-apply function argument) =
  byte5 ∷
  (encode-canonical-term function ++ encode-canonical-term argument)
encode-canonical-term canonical-unit-type = byte6 ∷ []
encode-canonical-term canonical-unit = byte7 ∷ []

-- Depth witnesses mirroring the shared Rust recursion budget: a term
-- fits fuel `suc n` when its children fit fuel `n`.
data TermFitsV1 : Nat → CanonicalTermV1 → Set where
  fits-sort :
    {fuel : Nat} {level : U16} →
    TermFitsV1 (suc fuel) (canonical-sort level)
  fits-variable :
    {fuel : Nat} {index : U32} →
    TermFitsV1 (suc fuel) (canonical-variable index)
  fits-global-slot :
    {fuel : Nat} {slot : U32} →
    TermFitsV1 (suc fuel) (canonical-global-slot slot)
  fits-pi :
    {fuel : Nat} {parameter body : CanonicalTermV1} →
    TermFitsV1 fuel parameter → TermFitsV1 fuel body →
    TermFitsV1 (suc fuel) (canonical-pi parameter body)
  fits-lambda :
    {fuel : Nat} {parameter body : CanonicalTermV1} →
    TermFitsV1 fuel parameter → TermFitsV1 fuel body →
    TermFitsV1 (suc fuel) (canonical-lambda parameter body)
  fits-apply :
    {fuel : Nat} {function argument : CanonicalTermV1} →
    TermFitsV1 fuel function → TermFitsV1 fuel argument →
    TermFitsV1 (suc fuel) (canonical-apply function argument)
  fits-unit-type :
    {fuel : Nat} → TermFitsV1 (suc fuel) canonical-unit-type
  fits-unit :
    {fuel : Nat} → TermFitsV1 (suc fuel) canonical-unit

parse-term-fuel-encode :
  (fuel : Nat) (term : CanonicalTermV1) (rest : List Byte) →
  TermFitsV1 fuel term →
  parse-wire-term-fuel fuel (encode-canonical-term term ++ rest)
  ≡ just (erase-canonical-term term , rest)
parse-term-fuel-encode (suc fuel) (canonical-sort (little16 b0 b1))
  rest fits-sort = refl
parse-term-fuel-encode (suc fuel)
  (canonical-variable (little32 b0 b1 b2 b3)) rest fits-variable = refl
parse-term-fuel-encode (suc fuel)
  (canonical-global-slot (little32 b0 b1 b2 b3)) rest
  fits-global-slot = refl
parse-term-fuel-encode (suc fuel) (canonical-pi parameter body) rest
  (fits-pi fits-parameter fits-body)
  rewrite ++-assoc (encode-canonical-term parameter)
            (encode-canonical-term body) rest
        | parse-term-fuel-encode fuel parameter
            (encode-canonical-term body ++ rest) fits-parameter
        | parse-term-fuel-encode fuel body rest fits-body
        = refl
parse-term-fuel-encode (suc fuel) (canonical-lambda parameter body) rest
  (fits-lambda fits-parameter fits-body)
  rewrite ++-assoc (encode-canonical-term parameter)
            (encode-canonical-term body) rest
        | parse-term-fuel-encode fuel parameter
            (encode-canonical-term body ++ rest) fits-parameter
        | parse-term-fuel-encode fuel body rest fits-body
        = refl
parse-term-fuel-encode (suc fuel) (canonical-apply function argument) rest
  (fits-apply fits-function fits-argument)
  rewrite ++-assoc (encode-canonical-term function)
            (encode-canonical-term argument) rest
        | parse-term-fuel-encode fuel function
            (encode-canonical-term argument ++ rest) fits-function
        | parse-term-fuel-encode fuel argument rest fits-argument
        = refl
parse-term-fuel-encode (suc fuel) canonical-unit-type rest
  fits-unit-type = refl
parse-term-fuel-encode (suc fuel) canonical-unit rest fits-unit = refl

-- Terms fitting the full Rust recursion budget round-trip through the
-- top-level term parser.
record BoundedCanonicalTermV1 : Set where
  constructor bounded-canonical-term
  field
    bounded-term : CanonicalTermV1
    bounded-term-fits : TermFitsV1 term-recursion-limit-v1 bounded-term

open BoundedCanonicalTermV1 public

encode-bounded-term : BoundedCanonicalTermV1 → List Byte
encode-bounded-term term = encode-canonical-term (bounded-term term)

erase-bounded-term : BoundedCanonicalTermV1 → WireTermV1
erase-bounded-term term = erase-canonical-term (bounded-term term)

parse-wire-term-encode :
  (term : BoundedCanonicalTermV1) (rest : List Byte) →
  parse-wire-term (encode-bounded-term term ++ rest)
  ≡ just (erase-bounded-term term , rest)
parse-wire-term-encode (bounded-canonical-term term fits) rest =
  parse-term-fuel-encode term-recursion-limit-v1 term rest fits

-- Canonical reduction steps share the fuel budget with their terms.
data CanonicalStepV1 : Set where
  canonical-step-beta :
    CanonicalTermV1 → CanonicalTermV1 → CanonicalStepV1
  canonical-step-transparent-delta :
    CanonicalTermV1 → CanonicalTermV1 → U32 → CanonicalStepV1
  canonical-step-pi-parameter-congruence :
    CanonicalTermV1 → CanonicalTermV1 → CanonicalStepV1 → CanonicalStepV1
  canonical-step-pi-body-congruence :
    CanonicalTermV1 → CanonicalTermV1 → CanonicalStepV1 → CanonicalStepV1
  canonical-step-lambda-parameter-congruence :
    CanonicalTermV1 → CanonicalTermV1 → CanonicalStepV1 → CanonicalStepV1
  canonical-step-lambda-body-congruence :
    CanonicalTermV1 → CanonicalTermV1 → CanonicalStepV1 → CanonicalStepV1
  canonical-step-apply-function-congruence :
    CanonicalTermV1 → CanonicalTermV1 → CanonicalStepV1 → CanonicalStepV1
  canonical-step-apply-argument-congruence :
    CanonicalTermV1 → CanonicalTermV1 → CanonicalStepV1 → CanonicalStepV1

erase-canonical-step : CanonicalStepV1 → BaseQ0ReductionStepWireV1
erase-canonical-step (canonical-step-beta source target) =
  step-beta (erase-canonical-term source) (erase-canonical-term target)
erase-canonical-step
  (canonical-step-transparent-delta source target slot) =
  step-transparent-delta
    (erase-canonical-term source) (erase-canonical-term target)
    (u32-value slot)
erase-canonical-step
  (canonical-step-pi-parameter-congruence source target premise) =
  step-pi-parameter-congruence
    (erase-canonical-term source) (erase-canonical-term target)
    (erase-canonical-step premise)
erase-canonical-step
  (canonical-step-pi-body-congruence source target premise) =
  step-pi-body-congruence
    (erase-canonical-term source) (erase-canonical-term target)
    (erase-canonical-step premise)
erase-canonical-step
  (canonical-step-lambda-parameter-congruence source target premise) =
  step-lambda-parameter-congruence
    (erase-canonical-term source) (erase-canonical-term target)
    (erase-canonical-step premise)
erase-canonical-step
  (canonical-step-lambda-body-congruence source target premise) =
  step-lambda-body-congruence
    (erase-canonical-term source) (erase-canonical-term target)
    (erase-canonical-step premise)
erase-canonical-step
  (canonical-step-apply-function-congruence source target premise) =
  step-apply-function-congruence
    (erase-canonical-term source) (erase-canonical-term target)
    (erase-canonical-step premise)
erase-canonical-step
  (canonical-step-apply-argument-congruence source target premise) =
  step-apply-argument-congruence
    (erase-canonical-term source) (erase-canonical-term target)
    (erase-canonical-step premise)

encode-canonical-step : CanonicalStepV1 → List Byte
encode-canonical-step (canonical-step-beta source target) =
  byte0 ∷
  (encode-canonical-term source ++ encode-canonical-term target)
encode-canonical-step
  (canonical-step-transparent-delta source target slot) =
  byte1 ∷
  (encode-canonical-term source ++
   (encode-canonical-term target ++ encode-u32 slot))
encode-canonical-step
  (canonical-step-pi-parameter-congruence source target premise) =
  byte2 ∷
  (encode-canonical-term source ++
   (encode-canonical-term target ++ encode-canonical-step premise))
encode-canonical-step
  (canonical-step-pi-body-congruence source target premise) =
  byte3 ∷
  (encode-canonical-term source ++
   (encode-canonical-term target ++ encode-canonical-step premise))
encode-canonical-step
  (canonical-step-lambda-parameter-congruence source target premise) =
  byte4 ∷
  (encode-canonical-term source ++
   (encode-canonical-term target ++ encode-canonical-step premise))
encode-canonical-step
  (canonical-step-lambda-body-congruence source target premise) =
  byte5 ∷
  (encode-canonical-term source ++
   (encode-canonical-term target ++ encode-canonical-step premise))
encode-canonical-step
  (canonical-step-apply-function-congruence source target premise) =
  byte6 ∷
  (encode-canonical-term source ++
   (encode-canonical-term target ++ encode-canonical-step premise))
encode-canonical-step
  (canonical-step-apply-argument-congruence source target premise) =
  byte7 ∷
  (encode-canonical-term source ++
   (encode-canonical-term target ++ encode-canonical-step premise))

data StepFitsV1 : Nat → CanonicalStepV1 → Set where
  fits-step-beta :
    {fuel : Nat} {source target : CanonicalTermV1} →
    TermFitsV1 fuel source → TermFitsV1 fuel target →
    StepFitsV1 (suc fuel) (canonical-step-beta source target)
  fits-step-transparent-delta :
    {fuel : Nat} {source target : CanonicalTermV1} {slot : U32} →
    TermFitsV1 fuel source → TermFitsV1 fuel target →
    StepFitsV1 (suc fuel)
      (canonical-step-transparent-delta source target slot)
  fits-step-pi-parameter-congruence :
    {fuel : Nat} {source target : CanonicalTermV1}
    {premise : CanonicalStepV1} →
    TermFitsV1 fuel source → TermFitsV1 fuel target →
    StepFitsV1 fuel premise →
    StepFitsV1 (suc fuel)
      (canonical-step-pi-parameter-congruence source target premise)
  fits-step-pi-body-congruence :
    {fuel : Nat} {source target : CanonicalTermV1}
    {premise : CanonicalStepV1} →
    TermFitsV1 fuel source → TermFitsV1 fuel target →
    StepFitsV1 fuel premise →
    StepFitsV1 (suc fuel)
      (canonical-step-pi-body-congruence source target premise)
  fits-step-lambda-parameter-congruence :
    {fuel : Nat} {source target : CanonicalTermV1}
    {premise : CanonicalStepV1} →
    TermFitsV1 fuel source → TermFitsV1 fuel target →
    StepFitsV1 fuel premise →
    StepFitsV1 (suc fuel)
      (canonical-step-lambda-parameter-congruence source target premise)
  fits-step-lambda-body-congruence :
    {fuel : Nat} {source target : CanonicalTermV1}
    {premise : CanonicalStepV1} →
    TermFitsV1 fuel source → TermFitsV1 fuel target →
    StepFitsV1 fuel premise →
    StepFitsV1 (suc fuel)
      (canonical-step-lambda-body-congruence source target premise)
  fits-step-apply-function-congruence :
    {fuel : Nat} {source target : CanonicalTermV1}
    {premise : CanonicalStepV1} →
    TermFitsV1 fuel source → TermFitsV1 fuel target →
    StepFitsV1 fuel premise →
    StepFitsV1 (suc fuel)
      (canonical-step-apply-function-congruence source target premise)
  fits-step-apply-argument-congruence :
    {fuel : Nat} {source target : CanonicalTermV1}
    {premise : CanonicalStepV1} →
    TermFitsV1 fuel source → TermFitsV1 fuel target →
    StepFitsV1 fuel premise →
    StepFitsV1 (suc fuel)
      (canonical-step-apply-argument-congruence source target premise)

parse-step-fuel-encode :
  (fuel : Nat) (step : CanonicalStepV1) (rest : List Byte) →
  StepFitsV1 fuel step →
  parse-reduction-step-fuel fuel (encode-canonical-step step ++ rest)
  ≡ just (erase-canonical-step step , rest)
parse-step-fuel-encode (suc fuel) (canonical-step-beta source target)
  rest (fits-step-beta fits-source fits-target)
  rewrite ++-assoc (encode-canonical-term source)
            (encode-canonical-term target) rest
        | parse-term-fuel-encode fuel source
            (encode-canonical-term target ++ rest) fits-source
        | parse-term-fuel-encode fuel target rest fits-target
        = refl
parse-step-fuel-encode (suc fuel)
  (canonical-step-transparent-delta source target
    (little32 b0 b1 b2 b3))
  rest (fits-step-transparent-delta fits-source fits-target)
  rewrite ++-assoc (encode-canonical-term source)
            (encode-canonical-term target ++
             encode-u32 (little32 b0 b1 b2 b3))
            rest
        | ++-assoc (encode-canonical-term target)
            (encode-u32 (little32 b0 b1 b2 b3)) rest
        | parse-term-fuel-encode fuel source
            (encode-canonical-term target ++
             (encode-u32 (little32 b0 b1 b2 b3) ++ rest))
            fits-source
        | parse-term-fuel-encode fuel target
            (encode-u32 (little32 b0 b1 b2 b3) ++ rest)
            fits-target
        = refl
parse-step-fuel-encode (suc fuel)
  (canonical-step-pi-parameter-congruence source target premise)
  rest
  (fits-step-pi-parameter-congruence fits-source fits-target
    fits-premise)
  rewrite ++-assoc (encode-canonical-term source)
            (encode-canonical-term target ++ encode-canonical-step premise)
            rest
        | ++-assoc (encode-canonical-term target)
            (encode-canonical-step premise) rest
        | parse-term-fuel-encode fuel source
            (encode-canonical-term target ++
             (encode-canonical-step premise ++ rest))
            fits-source
        | parse-term-fuel-encode fuel target
            (encode-canonical-step premise ++ rest) fits-target
        | parse-step-fuel-encode fuel premise rest fits-premise
        = refl
parse-step-fuel-encode (suc fuel)
  (canonical-step-pi-body-congruence source target premise)
  rest
  (fits-step-pi-body-congruence fits-source fits-target fits-premise)
  rewrite ++-assoc (encode-canonical-term source)
            (encode-canonical-term target ++ encode-canonical-step premise)
            rest
        | ++-assoc (encode-canonical-term target)
            (encode-canonical-step premise) rest
        | parse-term-fuel-encode fuel source
            (encode-canonical-term target ++
             (encode-canonical-step premise ++ rest))
            fits-source
        | parse-term-fuel-encode fuel target
            (encode-canonical-step premise ++ rest) fits-target
        | parse-step-fuel-encode fuel premise rest fits-premise
        = refl
parse-step-fuel-encode (suc fuel)
  (canonical-step-lambda-parameter-congruence source target premise)
  rest
  (fits-step-lambda-parameter-congruence fits-source fits-target
    fits-premise)
  rewrite ++-assoc (encode-canonical-term source)
            (encode-canonical-term target ++ encode-canonical-step premise)
            rest
        | ++-assoc (encode-canonical-term target)
            (encode-canonical-step premise) rest
        | parse-term-fuel-encode fuel source
            (encode-canonical-term target ++
             (encode-canonical-step premise ++ rest))
            fits-source
        | parse-term-fuel-encode fuel target
            (encode-canonical-step premise ++ rest) fits-target
        | parse-step-fuel-encode fuel premise rest fits-premise
        = refl
parse-step-fuel-encode (suc fuel)
  (canonical-step-lambda-body-congruence source target premise)
  rest
  (fits-step-lambda-body-congruence fits-source fits-target
    fits-premise)
  rewrite ++-assoc (encode-canonical-term source)
            (encode-canonical-term target ++ encode-canonical-step premise)
            rest
        | ++-assoc (encode-canonical-term target)
            (encode-canonical-step premise) rest
        | parse-term-fuel-encode fuel source
            (encode-canonical-term target ++
             (encode-canonical-step premise ++ rest))
            fits-source
        | parse-term-fuel-encode fuel target
            (encode-canonical-step premise ++ rest) fits-target
        | parse-step-fuel-encode fuel premise rest fits-premise
        = refl
parse-step-fuel-encode (suc fuel)
  (canonical-step-apply-function-congruence source target premise)
  rest
  (fits-step-apply-function-congruence fits-source fits-target
    fits-premise)
  rewrite ++-assoc (encode-canonical-term source)
            (encode-canonical-term target ++ encode-canonical-step premise)
            rest
        | ++-assoc (encode-canonical-term target)
            (encode-canonical-step premise) rest
        | parse-term-fuel-encode fuel source
            (encode-canonical-term target ++
             (encode-canonical-step premise ++ rest))
            fits-source
        | parse-term-fuel-encode fuel target
            (encode-canonical-step premise ++ rest) fits-target
        | parse-step-fuel-encode fuel premise rest fits-premise
        = refl
parse-step-fuel-encode (suc fuel)
  (canonical-step-apply-argument-congruence source target premise)
  rest
  (fits-step-apply-argument-congruence fits-source fits-target
    fits-premise)
  rewrite ++-assoc (encode-canonical-term source)
            (encode-canonical-term target ++ encode-canonical-step premise)
            rest
        | ++-assoc (encode-canonical-term target)
            (encode-canonical-step premise) rest
        | parse-term-fuel-encode fuel source
            (encode-canonical-term target ++
             (encode-canonical-step premise ++ rest))
            fits-source
        | parse-term-fuel-encode fuel target
            (encode-canonical-step premise ++ rest) fits-target
        | parse-step-fuel-encode fuel premise rest fits-premise
        = refl

record BoundedCanonicalStepV1 : Set where
  constructor bounded-canonical-step
  field
    bounded-step : CanonicalStepV1
    bounded-step-fits : StepFitsV1 term-recursion-limit-v1 bounded-step

open BoundedCanonicalStepV1 public

encode-bounded-step : BoundedCanonicalStepV1 → List Byte
encode-bounded-step step = encode-canonical-step (bounded-step step)

erase-bounded-step : BoundedCanonicalStepV1 → BaseQ0ReductionStepWireV1
erase-bounded-step step = erase-canonical-step (bounded-step step)

parse-reduction-step-encode :
  (step : BoundedCanonicalStepV1) (rest : List Byte) →
  parse-reduction-step (encode-bounded-step step ++ rest)
  ≡ just (erase-bounded-step step , rest)
parse-reduction-step-encode (bounded-canonical-step step fits) rest =
  parse-step-fuel-encode term-recursion-limit-v1 step rest fits

-- Canonical synthesis codes: the third recursive payload sharing the
-- Rust recursion budget, embedding conversion identifiers and one
-- dependent-result term.
data CanonicalSynthCodeV1 : Set where
  canonical-code-sort : U16 → CanonicalSynthCodeV1
  canonical-code-unit-type : CanonicalSynthCodeV1
  canonical-code-unit : CanonicalSynthCodeV1
  canonical-code-variable-lookup :
    U32 → U32 → U32 → CanonicalSynthCodeV1
  canonical-code-global-lookup : U32 → CanonicalSynthCodeV1
  canonical-code-pi-formation :
    CanonicalSynthCodeV1 → CanonicalSynthCodeV1 → CanonicalSynthCodeV1
  canonical-code-lambda-introduction :
    CanonicalSynthCodeV1 → CanonicalSynthCodeV1 → CanonicalSynthCodeV1
  canonical-code-application-elimination :
    CanonicalSynthCodeV1 → CanonicalSynthCodeV1 → CanonicalIdV1 →
    CanonicalIdV1 → CanonicalTermV1 → CanonicalSynthCodeV1

erase-canonical-code : CanonicalSynthCodeV1 → SynthesisCodeWireV1
erase-canonical-code (canonical-code-sort level) =
  code-sort (u16-value level)
erase-canonical-code canonical-code-unit-type = code-unit-type
erase-canonical-code canonical-code-unit = code-unit
erase-canonical-code
  (canonical-code-variable-lookup index ordinal shift) =
  code-variable-lookup (u32-value index) (u32-value ordinal)
    (u32-value shift)
erase-canonical-code (canonical-code-global-lookup slot) =
  code-global-lookup (u32-value slot)
erase-canonical-code (canonical-code-pi-formation parameter body) =
  code-pi-formation
    (erase-canonical-code parameter) (erase-canonical-code body)
erase-canonical-code
  (canonical-code-lambda-introduction parameter body) =
  code-lambda-introduction
    (erase-canonical-code parameter) (erase-canonical-code body)
erase-canonical-code
  (canonical-code-application-elimination function argument
    function-conversion argument-conversion result) =
  code-application-elimination
    (erase-canonical-code function) (erase-canonical-code argument)
    (id-bytes function-conversion) (id-bytes argument-conversion)
    (erase-canonical-term result)

encode-canonical-code : CanonicalSynthCodeV1 → List Byte
encode-canonical-code (canonical-code-sort level) =
  byte0 ∷ encode-u16 level
encode-canonical-code canonical-code-unit-type = byte1 ∷ []
encode-canonical-code canonical-code-unit = byte2 ∷ []
encode-canonical-code
  (canonical-code-variable-lookup index ordinal shift) =
  byte3 ∷
  (encode-u32 index ++ (encode-u32 ordinal ++ encode-u32 shift))
encode-canonical-code (canonical-code-global-lookup slot) =
  byte4 ∷ encode-u32 slot
encode-canonical-code (canonical-code-pi-formation parameter body) =
  byte5 ∷
  (encode-canonical-code parameter ++ encode-canonical-code body)
encode-canonical-code
  (canonical-code-lambda-introduction parameter body) =
  byte6 ∷
  (encode-canonical-code parameter ++ encode-canonical-code body)
encode-canonical-code
  (canonical-code-application-elimination function argument
    function-conversion argument-conversion result) =
  byte7 ∷
  (encode-canonical-code function ++
   (encode-canonical-code argument ++
    (encode-canonical-id function-conversion ++
     (encode-canonical-id argument-conversion ++
      encode-canonical-term result))))

data CodeFitsV1 : Nat → CanonicalSynthCodeV1 → Set where
  fits-code-sort :
    {fuel : Nat} {level : U16} →
    CodeFitsV1 (suc fuel) (canonical-code-sort level)
  fits-code-unit-type :
    {fuel : Nat} → CodeFitsV1 (suc fuel) canonical-code-unit-type
  fits-code-unit :
    {fuel : Nat} → CodeFitsV1 (suc fuel) canonical-code-unit
  fits-code-variable-lookup :
    {fuel : Nat} {index ordinal shift : U32} →
    CodeFitsV1 (suc fuel)
      (canonical-code-variable-lookup index ordinal shift)
  fits-code-global-lookup :
    {fuel : Nat} {slot : U32} →
    CodeFitsV1 (suc fuel) (canonical-code-global-lookup slot)
  fits-code-pi-formation :
    {fuel : Nat} {parameter body : CanonicalSynthCodeV1} →
    CodeFitsV1 fuel parameter → CodeFitsV1 fuel body →
    CodeFitsV1 (suc fuel) (canonical-code-pi-formation parameter body)
  fits-code-lambda-introduction :
    {fuel : Nat} {parameter body : CanonicalSynthCodeV1} →
    CodeFitsV1 fuel parameter → CodeFitsV1 fuel body →
    CodeFitsV1 (suc fuel)
      (canonical-code-lambda-introduction parameter body)
  fits-code-application-elimination :
    {fuel : Nat} {function argument : CanonicalSynthCodeV1}
    {function-conversion argument-conversion : CanonicalIdV1}
    {result : CanonicalTermV1} →
    CodeFitsV1 fuel function → CodeFitsV1 fuel argument →
    TermFitsV1 fuel result →
    CodeFitsV1 (suc fuel)
      (canonical-code-application-elimination function argument
        function-conversion argument-conversion result)

parse-code-fuel-encode :
  (fuel : Nat) (code : CanonicalSynthCodeV1) (rest : List Byte) →
  CodeFitsV1 fuel code →
  parse-synthesis-code-fuel fuel (encode-canonical-code code ++ rest)
  ≡ just (erase-canonical-code code , rest)
parse-code-fuel-encode (suc fuel) (canonical-code-sort (little16 b0 b1))
  rest fits-code-sort = refl
parse-code-fuel-encode (suc fuel) canonical-code-unit-type rest
  fits-code-unit-type = refl
parse-code-fuel-encode (suc fuel) canonical-code-unit rest
  fits-code-unit = refl
parse-code-fuel-encode (suc fuel)
  (canonical-code-variable-lookup (little32 a0 a1 a2 a3)
    (little32 c0 c1 c2 c3) (little32 d0 d1 d2 d3))
  rest fits-code-variable-lookup = refl
parse-code-fuel-encode (suc fuel)
  (canonical-code-global-lookup (little32 b0 b1 b2 b3)) rest
  fits-code-global-lookup = refl
parse-code-fuel-encode (suc fuel)
  (canonical-code-pi-formation parameter body) rest
  (fits-code-pi-formation fits-parameter fits-body)
  rewrite ++-assoc (encode-canonical-code parameter)
            (encode-canonical-code body) rest
        | parse-code-fuel-encode fuel parameter
            (encode-canonical-code body ++ rest) fits-parameter
        | parse-code-fuel-encode fuel body rest fits-body
        = refl
parse-code-fuel-encode (suc fuel)
  (canonical-code-lambda-introduction parameter body) rest
  (fits-code-lambda-introduction fits-parameter fits-body)
  rewrite ++-assoc (encode-canonical-code parameter)
            (encode-canonical-code body) rest
        | parse-code-fuel-encode fuel parameter
            (encode-canonical-code body ++ rest) fits-parameter
        | parse-code-fuel-encode fuel body rest fits-body
        = refl
parse-code-fuel-encode (suc fuel)
  (canonical-code-application-elimination function argument
    function-conversion argument-conversion result)
  rest
  (fits-code-application-elimination fits-function fits-argument
    fits-result)
  rewrite ++-assoc (encode-canonical-code function)
            (encode-canonical-code argument ++
             (encode-canonical-id function-conversion ++
              (encode-canonical-id argument-conversion ++
               encode-canonical-term result)))
            rest
        | ++-assoc (encode-canonical-code argument)
            (encode-canonical-id function-conversion ++
             (encode-canonical-id argument-conversion ++
              encode-canonical-term result))
            rest
        | ++-assoc (encode-canonical-id function-conversion)
            (encode-canonical-id argument-conversion ++
             encode-canonical-term result)
            rest
        | ++-assoc (encode-canonical-id argument-conversion)
            (encode-canonical-term result) rest
        | parse-code-fuel-encode fuel function
            (encode-canonical-code argument ++
             (encode-canonical-id function-conversion ++
              (encode-canonical-id argument-conversion ++
               (encode-canonical-term result ++ rest))))
            fits-function
        | parse-code-fuel-encode fuel argument
            (encode-canonical-id function-conversion ++
             (encode-canonical-id argument-conversion ++
              (encode-canonical-term result ++ rest)))
            fits-argument
        | parse-wire-id-encode function-conversion
            (encode-canonical-id argument-conversion ++
             (encode-canonical-term result ++ rest))
        | parse-wire-id-encode argument-conversion
            (encode-canonical-term result ++ rest)
        | parse-term-fuel-encode fuel result rest fits-result
        = refl

record BoundedCanonicalCodeV1 : Set where
  constructor bounded-canonical-code
  field
    bounded-code : CanonicalSynthCodeV1
    bounded-code-fits : CodeFitsV1 term-recursion-limit-v1 bounded-code

open BoundedCanonicalCodeV1 public

encode-bounded-code : BoundedCanonicalCodeV1 → List Byte
encode-bounded-code code = encode-canonical-code (bounded-code code)

erase-bounded-code : BoundedCanonicalCodeV1 → SynthesisCodeWireV1
erase-bounded-code code = erase-canonical-code (bounded-code code)

parse-synthesis-code-encode :
  (code : BoundedCanonicalCodeV1) (rest : List Byte) →
  parse-synthesis-code (encode-bounded-code code ++ rest)
  ≡ just (erase-bounded-code code , rest)
parse-synthesis-code-encode (bounded-canonical-code code fits) rest =
  parse-code-fuel-encode term-recursion-limit-v1 code rest fits

-- Counted lists carry their exact length word, mirroring the u64 counts
-- written by the Rust codec.
record CountedList (A : Set) : Set where
  constructor counted-list
  field
    count-word : U64
    counted-items : List A
    count-canonical : u64-value count-word ≡ length counted-items

open CountedList public

encode-counted-list :
  {A : Set} → (A → List Byte) → CountedList A → List Byte
encode-counted-list encode-item values =
  encode-u64 (count-word values) ++
  concat-encode encode-item (counted-items values)

parse-many-encode :
  {A B : Set} (parser : Decoder B)
  (encode-item : A → List Byte) (erase-item : A → B) →
  ((item : A) (rest : List Byte) →
    parser (encode-item item ++ rest) ≡ just (erase-item item , rest)) →
  (values : List A) (rest : List Byte) →
  parse-many (length values) parser
    (concat-encode encode-item values ++ rest)
  ≡ just (map erase-item values , rest)
parse-many-encode parser encode-item erase-item item-round [] rest = refl
parse-many-encode parser encode-item erase-item item-round
  (value ∷ values) rest
  rewrite ++-assoc (encode-item value)
            (concat-encode encode-item values) rest
        | item-round value (concat-encode encode-item values ++ rest)
        | parse-many-encode parser encode-item erase-item item-round
            values rest
        = refl

parse-bounded-list-encode :
  {A B : Set} (limit : Nat) (parser : Decoder B)
  (encode-item : A → List Byte) (erase-item : A → B) →
  ((item : A) (rest : List Byte) →
    parser (encode-item item ++ rest) ≡ just (erase-item item , rest)) →
  (values : CountedList A) (rest : List Byte) →
  limit-exceeded limit (length (counted-items values)) ≡ false →
  parse-bounded-list limit parser
    (encode-counted-list encode-item values ++ rest)
  ≡ just (map erase-item (counted-items values) , rest)
parse-bounded-list-encode limit parser encode-item erase-item item-round
  (counted-list (little64 b0 b1 b2 b3 b4 b5 b6 b7) values canonical)
  rest within
  rewrite canonical
        | within
        | parse-many-encode parser encode-item erase-item item-round
            values rest
        = refl

run-complete-encode :
  {A : Set} (parser : Decoder A) (input : List Byte) (value : A) →
  parser input ≡ just (value , []) →
  run-complete parser input ≡ just value
run-complete-encode parser input value complete
  rewrite complete = refl

-- Round trips are stated against `encode ++ rest`; the empty-rest
-- instance retracts to the exact section payload.
transport-parser-input :
  {A : Set} (parser : Decoder A) {left right : List Byte} →
  left ≡ right → {result : Maybe (A × List Byte)} →
  parser left ≡ result → parser right ≡ result
transport-parser-input parser refl equal = equal

parser-complete-of-padded :
  {A : Set} (parser : Decoder A) (bytes : List Byte) (value : A) →
  parser (bytes ++ []) ≡ just (value , []) →
  parser bytes ≡ just (value , [])
parser-complete-of-padded parser bytes value padded =
  transport-parser-input parser (++-right-identity bytes) padded

-- Section 4 payload: one production context is a counted list of bounded
-- canonical terms.
CanonicalContextV1 : Set
CanonicalContextV1 = CountedList BoundedCanonicalTermV1

encode-canonical-context : CanonicalContextV1 → List Byte
encode-canonical-context = encode-counted-list encode-bounded-term

erase-canonical-context : CanonicalContextV1 → ProductionContextWireV1
erase-canonical-context context =
  production-context-v1 (map erase-bounded-term (counted-items context))

parse-production-context-encode :
  (context : CanonicalContextV1) (rest : List Byte) →
  limit-exceeded max-sequence-items-v1
    (length (counted-items context)) ≡ false →
  parse-production-context (encode-canonical-context context ++ rest)
  ≡ just (erase-canonical-context context , rest)
parse-production-context-encode context rest within
  rewrite parse-bounded-list-encode max-sequence-items-v1 parse-wire-term
            encode-bounded-term erase-bounded-term parse-wire-term-encode
            context rest within
        = refl

-- Section 8 and section 10 payloads: counted lists of range-guarded tag
-- bytes.
record BoundedTagV1 (bound : Nat) : Set where
  constructor bounded-tag
  field
    tag-byte : Byte
    tag-within : limit-exceeded bound (byte-value tag-byte) ≡ false

open BoundedTagV1 public

encode-bounded-tag : {bound : Nat} → BoundedTagV1 bound → List Byte
encode-bounded-tag tag = tag-byte tag ∷ []

erase-bounded-tag : {bound : Nat} → BoundedTagV1 bound → Nat
erase-bounded-tag tag = byte-value (tag-byte tag)

parse-tag-below-encode :
  (bound : Nat) (tag : BoundedTagV1 bound) (rest : List Byte) →
  parse-tag-below bound (encode-bounded-tag tag ++ rest)
  ≡ just (erase-bounded-tag tag , rest)
parse-tag-below-encode bound (bounded-tag byte within) rest
  rewrite within = refl

CanonicalQ0InventoryV1 : Set
CanonicalQ0InventoryV1 = CountedList (BoundedTagV1 6)

encode-canonical-q0-inventory : CanonicalQ0InventoryV1 → List Byte
encode-canonical-q0-inventory = encode-counted-list encode-bounded-tag

erase-canonical-q0-inventory : CanonicalQ0InventoryV1 → List Nat
erase-canonical-q0-inventory inventory =
  map erase-bounded-tag (counted-items inventory)

parse-q0-inventory-encode :
  (inventory : CanonicalQ0InventoryV1) (rest : List Byte) →
  limit-exceeded max-sequence-items-v1
    (length (counted-items inventory)) ≡ false →
  parse-q0-inventory (encode-canonical-q0-inventory inventory ++ rest)
  ≡ just (erase-canonical-q0-inventory inventory , rest)
parse-q0-inventory-encode inventory rest within
  rewrite parse-bounded-list-encode max-sequence-items-v1
            (parse-tag-below 6) encode-bounded-tag erase-bounded-tag
            (parse-tag-below-encode 6) inventory rest within
        = refl

-- The complete-payload form used by the section decoders: encoding a
-- canonical Q0 inventory as an exact section payload round-trips through
-- `run-complete`.
q0-inventory-section-round :
  (inventory : CanonicalQ0InventoryV1) →
  limit-exceeded max-sequence-items-v1
    (length (counted-items inventory)) ≡ false →
  run-complete parse-q0-inventory
    (encode-canonical-q0-inventory inventory)
  ≡ just (erase-canonical-q0-inventory inventory)
q0-inventory-section-round inventory within =
  run-complete-encode parse-q0-inventory
    (encode-canonical-q0-inventory inventory)
    (erase-canonical-q0-inventory inventory)
    (parser-complete-of-padded parse-q0-inventory
      (encode-canonical-q0-inventory inventory)
      (erase-canonical-q0-inventory inventory)
      (parse-q0-inventory-encode inventory [] within))

CanonicalFamilyInventoryV1 : Set
CanonicalFamilyInventoryV1 = CountedList (BoundedTagV1 2)

encode-canonical-family-inventory :
  CanonicalFamilyInventoryV1 → List Byte
encode-canonical-family-inventory = encode-counted-list encode-bounded-tag

erase-canonical-family-inventory :
  CanonicalFamilyInventoryV1 → List Nat
erase-canonical-family-inventory inventory =
  map erase-bounded-tag (counted-items inventory)

parse-family-inventory-encode :
  (inventory : CanonicalFamilyInventoryV1) (rest : List Byte) →
  limit-exceeded max-sequence-items-v1
    (length (counted-items inventory)) ≡ false →
  parse-family-inventory
    (encode-canonical-family-inventory inventory ++ rest)
  ≡ just (erase-canonical-family-inventory inventory , rest)
parse-family-inventory-encode inventory rest within
  rewrite parse-bounded-list-encode max-sequence-items-v1
            (parse-tag-below 2) encode-bounded-tag erase-bounded-tag
            (parse-tag-below-encode 2) inventory rest within
        = refl
