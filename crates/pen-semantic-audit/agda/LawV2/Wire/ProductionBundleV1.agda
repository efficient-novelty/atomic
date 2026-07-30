{-# OPTIONS --safe --without-K #-}

module LawV2.Wire.ProductionBundleV1 where

open import Agda.Builtin.Bool using (Bool)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe using (Maybe; just; nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.Wire.Bytes
open import LawV2.Wire.Decoder
open import LawV2.Wire.Envelope

WireIdV1 : Set
WireIdV1 = List Byte

data WireTermV1 : Set where
  wire-sort : Nat → WireTermV1
  wire-variable : Nat → WireTermV1
  wire-global-slot : Nat → WireTermV1
  wire-pi : WireTermV1 → WireTermV1 → WireTermV1
  wire-lambda : WireTermV1 → WireTermV1 → WireTermV1
  wire-apply : WireTermV1 → WireTermV1 → WireTermV1
  wire-unit-type : WireTermV1
  wire-unit : WireTermV1

record DeltaPolicyEntryWireV1 : Set where
  constructor delta-policy-entry-v1
  field
    delta-global-slot : Nat
    delta-global-id : WireIdV1

open DeltaPolicyEntryWireV1 public

record ManifestSurfaceWireV1 : Set where
  constructor manifest-surface-v1
  field
    semantic-schema-version : Nat
    profile-id : List Byte
    semantic-manifest-digest : WireIdV1
    manifest-frozen : Bool
    live-profile-a-access : Bool
    production-inventory-bridge-digest : WireIdV1
    public-universe-levels : List Nat
    checker-universe-levels : List Nat
    formation-witness-levels : List Nat
    maximum-context-entries : Nat
    synthesis-rule-inventory : List Nat
    predecessor-delta-policy-binding-digest : WireIdV1
    synthesis-protocol-id : List Byte
    synthesis-schema-version : Nat

open ManifestSurfaceWireV1 public

record ProductionSignatureWireV1 : Set where
  constructor production-signature-v1
  field
    signature-digest : WireIdV1
    kernel-protocol-digest : WireIdV1
    global-slot-table-digest : WireIdV1
    allowed-transparent-deltas : List DeltaPolicyEntryWireV1

open ProductionSignatureWireV1 public

record GlobalSlotEntryWireV1 : Set where
  constructor global-slot-entry-v1
  field
    global-slot : Nat
    global-id : WireIdV1
    declaration-type : WireTermV1
    declaration-body : Maybe WireTermV1

open GlobalSlotEntryWireV1 public

record ProductionContextWireV1 : Set where
  constructor production-context-v1
  field
    entries-oldest-first : List WireTermV1

open ProductionContextWireV1 public

record ContextGlobalBundleV1 : Set where
  constructor context-global-bundle-v1
  field
    manifest-surface : ManifestSurfaceWireV1
    production-signature : ProductionSignatureWireV1
    global-slot-table : List GlobalSlotEntryWireV1
    production-contexts : List ProductionContextWireV1
    remaining-envelope : RawProductionEnvelopeV1

open ContextGlobalBundleV1 public

parse-wire-id : Decoder WireIdV1
parse-wire-id = parse-fixed-bytes 32

parse-tag-nat : Decoder Nat
parse-tag-nat =
  parse-byte >>= λ tag →
  pure (byte-value tag)

parse-wire-term-fuel : Nat → Decoder WireTermV1
parse-wire-term-after : Nat → Nat → Decoder WireTermV1

parse-wire-term-fuel zero = fail
parse-wire-term-fuel (suc fuel) =
  parse-byte >>= λ tag →
  parse-wire-term-after fuel (byte-value tag)

parse-wire-term-after fuel zero =
  parse-u16-nat >>= λ level →
  pure (wire-sort level)
parse-wire-term-after fuel (suc zero) =
  parse-u32-nat >>= λ index →
  pure (wire-variable index)
parse-wire-term-after fuel (suc (suc zero)) =
  parse-u32-nat >>= λ slot →
  pure (wire-global-slot slot)
parse-wire-term-after fuel (suc (suc (suc zero))) =
  parse-wire-term-fuel fuel >>= λ parameter →
  parse-wire-term-fuel fuel >>= λ body →
  pure (wire-pi parameter body)
parse-wire-term-after fuel (suc (suc (suc (suc zero)))) =
  parse-wire-term-fuel fuel >>= λ parameter →
  parse-wire-term-fuel fuel >>= λ body →
  pure (wire-lambda parameter body)
parse-wire-term-after fuel (suc (suc (suc (suc (suc zero))))) =
  parse-wire-term-fuel fuel >>= λ function →
  parse-wire-term-fuel fuel >>= λ argument →
  pure (wire-apply function argument)
parse-wire-term-after fuel (suc (suc (suc (suc (suc (suc zero)))))) =
  pure wire-unit-type
parse-wire-term-after fuel (suc (suc (suc (suc (suc (suc (suc zero))))))) =
  pure wire-unit
parse-wire-term-after fuel (suc (suc (suc (suc (suc (suc (suc (suc tag)))))))) =
  fail

parse-wire-term : Decoder WireTermV1
parse-wire-term input = parse-wire-term-fuel (length input) input

parse-delta-policy-entry : Decoder DeltaPolicyEntryWireV1
parse-delta-policy-entry =
  parse-u32-nat >>= λ slot →
  parse-wire-id >>= λ identifier →
  pure (delta-policy-entry-v1 slot identifier)

parse-manifest-surface : Decoder ManifestSurfaceWireV1
parse-manifest-surface =
  parse-u16-nat >>= λ schema →
  parse-byte-string >>= λ profile →
  parse-wire-id >>= λ manifest-digest →
  expect-byte byte0 >>= λ _ →
  parse-boolean >>= λ frozen →
  parse-boolean >>= λ live →
  parse-wire-id >>= λ inventory-digest →
  parse-list parse-u16-nat >>= λ public-levels →
  parse-list parse-u16-nat >>= λ checker-levels →
  parse-list parse-u16-nat >>= λ formation-levels →
  parse-u16-nat >>= λ maximum-context →
  parse-list parse-tag-nat >>= λ synthesis-inventory →
  parse-wire-id >>= λ delta-policy-digest →
  parse-byte-string >>= λ synthesis-protocol →
  parse-u16-nat >>= λ synthesis-schema →
  pure
    (manifest-surface-v1
      schema
      profile
      manifest-digest
      frozen
      live
      inventory-digest
      public-levels
      checker-levels
      formation-levels
      maximum-context
      synthesis-inventory
      delta-policy-digest
      synthesis-protocol
      synthesis-schema)

parse-production-signature : Decoder ProductionSignatureWireV1
parse-production-signature =
  parse-wire-id >>= λ signature →
  parse-wire-id >>= λ kernel →
  parse-wire-id >>= λ slots →
  parse-list parse-delta-policy-entry >>= λ deltas →
  pure (production-signature-v1 signature kernel slots deltas)

parse-global-slot-entry : Decoder GlobalSlotEntryWireV1
parse-global-slot-entry =
  parse-u32-nat >>= λ slot →
  parse-wire-id >>= λ identifier →
  parse-wire-term >>= λ ty →
  parse-option parse-wire-term >>= λ body →
  pure (global-slot-entry-v1 slot identifier ty body)

parse-global-slot-table : Decoder (List GlobalSlotEntryWireV1)
parse-global-slot-table = parse-list parse-global-slot-entry

parse-production-context : Decoder ProductionContextWireV1
parse-production-context =
  parse-list parse-wire-term >>= λ entries →
  pure (production-context-v1 entries)

parse-production-contexts : Decoder (List ProductionContextWireV1)
parse-production-contexts = parse-list parse-production-context

maybe-bind : {A B : Set} → Maybe A → (A → Maybe B) → Maybe B
maybe-bind nothing continuation = nothing
maybe-bind (just value) continuation = continuation value

decode-context-global-sections-v1 :
  RawProductionEnvelopeV1 → Maybe ContextGlobalBundleV1
decode-context-global-sections-v1 envelope =
  maybe-bind
    (run-complete parse-manifest-surface (second (manifest-section envelope)))
    (λ manifest →
  maybe-bind
    (run-complete parse-production-signature (second (signature-section envelope)))
    (λ signature →
  maybe-bind
    (run-complete parse-global-slot-table (second (global-slots-section envelope)))
    (λ slots →
  maybe-bind
    (run-complete parse-production-contexts (second (contexts-section envelope)))
    (λ contexts →
  just (context-global-bundle-v1 manifest signature slots contexts envelope)))))

decode-context-global-v1 : List Byte → Maybe ContextGlobalBundleV1
decode-context-global-v1 input =
  maybe-bind (decode-envelope-v1 input) decode-context-global-sections-v1