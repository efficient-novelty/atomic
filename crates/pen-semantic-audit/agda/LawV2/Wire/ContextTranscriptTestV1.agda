{-# OPTIONS --safe --without-K #-}

-- Cross-language context/global lookup transcript (development pin).
--
-- The pinned byte literal below is rendered by the Rust side from the
-- cross-language fixture bundle: for every global slot the strict-prior
-- stored declaration, and for every context and variable index the
-- oldest-first ordinal, the shift distance, and the looked-up type
-- computed by iterated de Bruijn shifting. The fixture includes a
-- discriminating well-typed context ([sort 0, variable 0, pi
-- (variable 1) (variable 2)]) whose lookups make the ordinal formula,
-- the oldest-first entry selection, the shift iteration count, and the
-- under-binder shift cutoff all byte-visible: a convention error in any
-- of them changes the pinned bytes. This module recomputes the same
-- transcript from the intrinsic side — `lookup-psig-type`,
-- `lookup-psig-body`, and `lookup-pctx` erased through `ptm-to-wire` —
-- and proves the byte strings identical by refl. This is a development
-- regression vector for the Phase D correspondence theorems, not minted
-- transcript authority (the versioned Phase G transcript codec remains
-- separate work).

module LawV2.Wire.ContextTranscriptTestV1 where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe
  using ()
  renaming (Maybe to WireMaybe; just to wire-just; nothing to wire-nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc; _-_)
open import LawV2.Wire.Bytes
open import LawV2.Wire.ProductionBundleV1
open import LawV2.Wire.ContextChecker
open import LawV2.LambdaUnit.ProductionSyntaxV1
open import LawV2.LambdaUnit.ProductionDecodingV1
open import LawV2.Wire.ContextCorrespondenceV1
open import LawV2.Wire.BundleDecodeTestV1 using (canonical-vector-v1)

context-transcript-v1 : List Nat
context-transcript-v1 =
  4 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  6 ∷ 1 ∷ 7 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 6 ∷ 0 ∷ 2 ∷ 0 ∷ 0 ∷ 
  0 ∷ 3 ∷ 3 ∷ 6 ∷ 0 ∷ 0 ∷ 0 ∷ 5 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 7 ∷ 0 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 3 ∷ 6 ∷ 3 ∷ 6 ∷ 6 ∷ 
  0 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 0 ∷ 
  0 ∷ 6 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  0 ∷ 0 ∷ 2 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 3 ∷ 1 ∷ 
  2 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 3 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 0 ∷ 0 ∷ 
  0 ∷ 1 ∷ 0 ∷ 0 ∷ 0 ∷ 2 ∷ 0 ∷ 0 ∷ 0 ∷ 1 ∷ 2 ∷ 0 ∷ 
  0 ∷ 0 ∷ 2 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 3 ∷ 0 ∷ 
  0 ∷ 0 ∷ 0 ∷ 0 ∷ 0 ∷ 
  []

-- Small-value little-endian rendering: every numeric field of the
-- fixture transcript is below 256, so one significant byte suffices.
list-map : {A B : Set} → (A → B) → List A → List B
list-map function [] = []
list-map function (value ∷ values) = function value ∷ list-map function values

all-fins : (count : Nat) → List (Fin count)
all-fins zero = []
all-fins (suc count) = fzero ∷ list-map fsuc (all-fins count)

maybe-append :
  WireMaybe (List Byte) → WireMaybe (List Byte) → WireMaybe (List Byte)
maybe-append (wire-just left) (wire-just right) = wire-just (left ++ right)
maybe-append wire-nothing right = wire-nothing
maybe-append (wire-just left) wire-nothing = wire-nothing

concat-rows : List (WireMaybe (List Byte)) → WireMaybe (List Byte)
concat-rows [] = wire-just []
concat-rows (row ∷ rows) = maybe-append row (concat-rows rows)

with-tag : Byte → WireMaybe (List Byte) → WireMaybe (List Byte)
with-tag tag = wire-map-maybe (λ bytes → tag ∷ bytes)

u16-bytes-small : Nat → WireMaybe (List Byte)
u16-bytes-small value =
  wire-map-maybe (λ b → b ∷ byte0 ∷ []) (nat-to-byte value)

u32-bytes-small : Nat → WireMaybe (List Byte)
u32-bytes-small value =
  wire-map-maybe (λ b → b ∷ byte0 ∷ byte0 ∷ byte0 ∷ [])
    (nat-to-byte value)

u64-bytes-small : Nat → WireMaybe (List Byte)
u64-bytes-small value =
  wire-map-maybe
    (λ b →
      b ∷ byte0 ∷ byte0 ∷ byte0 ∷ byte0 ∷ byte0 ∷ byte0 ∷ byte0 ∷ [])
    (nat-to-byte value)

render-term : WireTermV1 → WireMaybe (List Byte)
render-term (wire-sort level) = with-tag byte0 (u16-bytes-small level)
render-term (wire-variable index) = with-tag byte1 (u32-bytes-small index)
render-term (wire-global-slot slot) = with-tag byte2 (u32-bytes-small slot)
render-term (wire-pi parameter body) =
  with-tag byte3 (maybe-append (render-term parameter) (render-term body))
render-term (wire-lambda parameter body) =
  with-tag byte4 (maybe-append (render-term parameter) (render-term body))
render-term (wire-apply function argument) =
  with-tag byte5
    (maybe-append (render-term function) (render-term argument))
render-term wire-unit-type = wire-just (byte6 ∷ [])
render-term wire-unit = wire-just (byte7 ∷ [])

render-body : WireMaybe WireTermV1 → WireMaybe (List Byte)
render-body wire-nothing = wire-just (byte0 ∷ [])
render-body (wire-just term) = with-tag byte1 (render-term term)

-- Fixture surfaces extracted from the decoded canonical vector.
fixture-bundle-maybe : WireMaybe ProductionBundleSemanticV1
fixture-bundle-maybe =
  maybe-bind (bytes-from-nats canonical-vector-v1)
    decode-production-bundle-v1

fixture-globals : List GlobalSlotEntryWireV1
fixture-globals with fixture-bundle-maybe
... | wire-just bundle = bundle-global-slots bundle
... | wire-nothing = []

fixture-contexts : List ProductionContextWireV1
fixture-contexts with fixture-bundle-maybe
... | wire-just bundle = bundle-contexts bundle
... | wire-nothing = []

fixture-globals-check : check-slots-from zero fixture-globals ≡ true
fixture-globals-check = refl

fixture-psig : PSig (total-from zero fixture-globals)
fixture-psig = wire-slots-to-psig fixture-globals fixture-globals-check

render-global-row :
  Fin (total-from zero fixture-globals) → WireMaybe (List Byte)
render-global-row x =
  maybe-append (u32-bytes-small (fin-ordinal x))
    (maybe-append
      (render-term (ptm-to-wire (lookup-psig-type fixture-psig x)))
      (render-body
        (wire-map-maybe ptm-to-wire (lookup-psig-body fixture-psig x))))

render-context-rows-checked :
  (globals : Nat) (entries : List WireTermV1) (result : Bool) →
  check-context-entries globals zero entries ≡ result →
  WireMaybe (List Byte)
render-context-rows-checked globals entries false proof = wire-nothing
render-context-rows-checked globals entries true proof =
  maybe-append (u64-bytes-small (length entries))
    (concat-rows (list-map row (all-fins (total-from zero entries))))
  where
  built : PCtx globals (total-from zero entries)
  built = wire-context-extend zero pempty entries proof
  row : Fin (total-from zero entries) → WireMaybe (List Byte)
  row x =
    maybe-append (u32-bytes-small (fin-ordinal x))
      (maybe-append
        (u32-bytes-small
          (total-from zero entries - suc (fin-ordinal x)))
        (maybe-append (u32-bytes-small (suc (fin-ordinal x)))
          (render-term (ptm-to-wire (lookup-pctx built x)))))

render-context-block :
  (globals : Nat) → ProductionContextWireV1 → WireMaybe (List Byte)
render-context-block globals context =
  render-context-rows-checked globals (entries-oldest-first context)
    (check-context-entries globals zero (entries-oldest-first context))
    refl

render-transcript : WireMaybe (List Byte)
render-transcript =
  maybe-append (u64-bytes-small (length fixture-globals))
    (maybe-append
      (concat-rows
        (list-map render-global-row
          (all-fins (total-from zero fixture-globals))))
      (maybe-append (u64-bytes-small (length fixture-contexts))
        (concat-rows
          (list-map
            (render-context-block (total-from zero fixture-globals))
            fixture-contexts))))

-- The intrinsic Agda rendering equals the pinned Rust rendering
-- byte-for-byte.
transcript-agrees :
  render-transcript ≡ bytes-from-nats context-transcript-v1
transcript-agrees = refl
