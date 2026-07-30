{-# OPTIONS --safe --without-K #-}

module LawV2.Wire.ProductionBundleV1 where

open import Agda.Builtin.Bool using (Bool; false; true)
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

-- Exact Rust resource bounds (`pen-production-wire` `codec.rs`): sequence
-- counts, byte-string lengths, whole-bundle size, and the shared
-- term/step/synthesis recursion budget. The recursion budget is one shared
-- counter in Rust, so nested payloads must thread one fuel value here.
max-sequence-items-v1 : Nat
max-sequence-items-v1 = 1000000

-- The Rust per-section byte bound is not re-checked at the envelope
-- layer here: because the section and bundle limits are the same
-- constant, any section length claim above the limit already exceeds
-- the remaining input under the whole-bundle cap and fails
-- `take-exact`. If these two limits ever diverge, the envelope needs
-- its own explicit per-section bound.
max-section-bytes-v1 : Nat
max-section-bytes-v1 = 67108864

max-bundle-bytes-v1 : Nat
max-bundle-bytes-v1 = 67108864

term-recursion-limit-v1 : Nat
term-recursion-limit-v1 = 256

limit-exceeded : Nat → Nat → Bool
limit-exceeded zero zero = false
limit-exceeded zero (suc value) = true
limit-exceeded (suc limit) zero = false
limit-exceeded (suc limit) (suc value) = limit-exceeded limit value

parse-count-branch : {A : Set} → Bool → Nat → Decoder A → Decoder (List A)
parse-count-branch true count parser = fail
parse-count-branch false count parser = parse-many count parser

parse-bounded-list : {A : Set} → Nat → Decoder A → Decoder (List A)
parse-bounded-list limit parser =
  parse-u64-nat >>= λ count →
  parse-count-branch (limit-exceeded limit count) count parser

parse-bytes-branch : Bool → Nat → Decoder (List Byte)
parse-bytes-branch true count = fail
parse-bytes-branch false count = parse-fixed-bytes count

parse-bounded-byte-string : Nat → Decoder (List Byte)
parse-bounded-byte-string limit =
  parse-u64-nat >>= λ count →
  parse-bytes-branch (limit-exceeded limit count) count

parse-tag-branch : Bool → Nat → Decoder Nat
parse-tag-branch true value = fail
parse-tag-branch false value = pure value

parse-tag-below : Nat → Decoder Nat
parse-tag-below bound =
  parse-tag-nat >>= λ value →
  parse-tag-branch (limit-exceeded bound value) value

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

-- One top-level payload receives exactly the Rust recursion budget of 256
-- nested levels; length-based fuel would accept deeper terms than Rust.
parse-wire-term : Decoder WireTermV1
parse-wire-term = parse-wire-term-fuel term-recursion-limit-v1

parse-delta-policy-entry : Decoder DeltaPolicyEntryWireV1
parse-delta-policy-entry =
  parse-u32-nat >>= λ slot →
  parse-wire-id >>= λ identifier →
  pure (delta-policy-entry-v1 slot identifier)

parse-manifest-surface : Decoder ManifestSurfaceWireV1
parse-manifest-surface =
  parse-u16-nat >>= λ schema →
  parse-bounded-byte-string max-section-bytes-v1 >>= λ profile →
  parse-wire-id >>= λ manifest-digest →
  expect-byte byte0 >>= λ _ →
  parse-boolean >>= λ frozen →
  parse-boolean >>= λ live →
  parse-wire-id >>= λ inventory-digest →
  parse-wire-id >>= λ delta-policy-digest →
  parse-bounded-byte-string max-section-bytes-v1 >>= λ synthesis-protocol →
  parse-u16-nat >>= λ synthesis-schema →
  parse-bounded-list max-sequence-items-v1 parse-u16-nat >>= λ public-levels →
  parse-bounded-list max-sequence-items-v1 parse-u16-nat >>= λ checker-levels →
  parse-bounded-list max-sequence-items-v1 parse-u16-nat >>= λ formation-levels →
  parse-u16-nat >>= λ maximum-context →
  parse-bounded-list max-sequence-items-v1 parse-tag-nat >>= λ synthesis-inventory →
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
  parse-bounded-list max-sequence-items-v1 parse-delta-policy-entry >>= λ deltas →
  pure (production-signature-v1 signature kernel slots deltas)

parse-global-slot-entry : Decoder GlobalSlotEntryWireV1
parse-global-slot-entry =
  parse-u32-nat >>= λ slot →
  parse-wire-id >>= λ identifier →
  parse-wire-term >>= λ ty →
  parse-option parse-wire-term >>= λ body →
  pure (global-slot-entry-v1 slot identifier ty body)

parse-global-slot-table : Decoder (List GlobalSlotEntryWireV1)
parse-global-slot-table =
  parse-bounded-list max-sequence-items-v1 parse-global-slot-entry

parse-production-context : Decoder ProductionContextWireV1
parse-production-context =
  parse-bounded-list max-sequence-items-v1 parse-wire-term >>= λ entries →
  pure (production-context-v1 entries)

parse-production-contexts : Decoder (List ProductionContextWireV1)
parse-production-contexts =
  parse-bounded-list max-sequence-items-v1 parse-production-context

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

-- Sections 5-11: semantic payload mirrors of `pen-production-wire`
-- `model.rs`, decoded byte-exactly per `codec.rs`.

data ConversionPathComponentWireV1 : Set where
  path-pi-parameter : ConversionPathComponentWireV1
  path-pi-body : ConversionPathComponentWireV1
  path-lambda-parameter : ConversionPathComponentWireV1
  path-lambda-body : ConversionPathComponentWireV1
  path-apply-function : ConversionPathComponentWireV1
  path-apply-argument : ConversionPathComponentWireV1

data NoRedexDispositionWireV1 : Set where
  disposition-sort : NoRedexDispositionWireV1
  disposition-variable : NoRedexDispositionWireV1
  disposition-global-not-enabled : NoRedexDispositionWireV1
  disposition-pi : NoRedexDispositionWireV1
  disposition-lambda : NoRedexDispositionWireV1
  disposition-neutral-application : NoRedexDispositionWireV1
  disposition-unit-type : NoRedexDispositionWireV1
  disposition-unit : NoRedexDispositionWireV1

record NoRedexEntryWireV1 : Set where
  constructor no-redex-entry-v1
  field
    no-redex-path : List ConversionPathComponentWireV1
    no-redex-term : WireTermV1
    no-redex-disposition : NoRedexDispositionWireV1

open NoRedexEntryWireV1 public

data BaseQ0ReductionStepWireV1 : Set where
  step-beta :
    WireTermV1 → WireTermV1 → BaseQ0ReductionStepWireV1
  step-transparent-delta :
    WireTermV1 → WireTermV1 → Nat → BaseQ0ReductionStepWireV1
  step-pi-parameter-congruence :
    WireTermV1 → WireTermV1 → BaseQ0ReductionStepWireV1 →
    BaseQ0ReductionStepWireV1
  step-pi-body-congruence :
    WireTermV1 → WireTermV1 → BaseQ0ReductionStepWireV1 →
    BaseQ0ReductionStepWireV1
  step-lambda-parameter-congruence :
    WireTermV1 → WireTermV1 → BaseQ0ReductionStepWireV1 →
    BaseQ0ReductionStepWireV1
  step-lambda-body-congruence :
    WireTermV1 → WireTermV1 → BaseQ0ReductionStepWireV1 →
    BaseQ0ReductionStepWireV1
  step-apply-function-congruence :
    WireTermV1 → WireTermV1 → BaseQ0ReductionStepWireV1 →
    BaseQ0ReductionStepWireV1
  step-apply-argument-congruence :
    WireTermV1 → WireTermV1 → BaseQ0ReductionStepWireV1 →
    BaseQ0ReductionStepWireV1

record BaseQ0ReductionTraceWireV1 : Set where
  constructor reduction-trace-v1
  field
    trace-start : WireTermV1
    trace-steps : List BaseQ0ReductionStepWireV1
    trace-end : WireTermV1

open BaseQ0ReductionTraceWireV1 public

data EndpointJudgmentWireV1 : Set where
  endpoint-has-type : WireTermV1 → EndpointJudgmentWireV1
  endpoint-type-formation : EndpointJudgmentWireV1

record ConversionCertificateWireV1 : Set where
  constructor conversion-certificate-v1
  field
    conversion-id : WireIdV1
    conversion-context : ProductionContextWireV1
    conversion-left : WireTermV1
    conversion-right : WireTermV1
    conversion-endpoint : EndpointJudgmentWireV1
    conversion-common-normal-form : WireTermV1
    conversion-left-trace : BaseQ0ReductionTraceWireV1
    conversion-right-trace : BaseQ0ReductionTraceWireV1
    conversion-census : List NoRedexEntryWireV1

open ConversionCertificateWireV1 public

data SynthesisCodeWireV1 : Set where
  code-sort : Nat → SynthesisCodeWireV1
  code-unit-type : SynthesisCodeWireV1
  code-unit : SynthesisCodeWireV1
  code-variable-lookup : Nat → Nat → Nat → SynthesisCodeWireV1
  code-global-lookup : Nat → SynthesisCodeWireV1
  code-pi-formation :
    SynthesisCodeWireV1 → SynthesisCodeWireV1 → SynthesisCodeWireV1
  code-lambda-introduction :
    SynthesisCodeWireV1 → SynthesisCodeWireV1 → SynthesisCodeWireV1
  code-application-elimination :
    SynthesisCodeWireV1 → SynthesisCodeWireV1 → WireIdV1 → WireIdV1 →
    WireTermV1 → SynthesisCodeWireV1

record SynthesisCertificateWireV1 : Set where
  constructor synthesis-certificate-v1
  field
    synthesis-id : WireIdV1
    synthesis-context : ProductionContextWireV1
    synthesis-subject : WireTermV1
    synthesis-inferred-type : WireTermV1
    synthesis-code : SynthesisCodeWireV1

open SynthesisCertificateWireV1 public

record ConversionTypingSupplementWireV1 : Set where
  constructor conversion-typing-supplement-v1
  field
    supplement-conversion-id : WireIdV1
    supplement-step-path : List Nat
    supplement-local-context : ProductionContextWireV1
    supplement-endpoint : EndpointJudgmentWireV1
    supplement-source-code : SynthesisCertificateWireV1
    supplement-target-code : SynthesisCertificateWireV1
    supplement-formation-level : Maybe Nat

open ConversionTypingSupplementWireV1 public

record FreshRuleSchemaWireV1 : Set where
  constructor fresh-rule-schema-v1
  field
    fresh-equation-id : WireIdV1
    fresh-owner-slot : Nat
    fresh-constructor-slot : Nat
    fresh-parameter-context : ProductionContextWireV1
    fresh-left : WireTermV1
    fresh-right : WireTermV1
    fresh-type : WireTermV1
    fresh-scrutinee-ordinal : Nat
    fresh-arity : Nat

open FreshRuleSchemaWireV1 public

record FamilyJudgmentWireV1 : Set where
  constructor family-judgment-v1
  field
    family-judgment-context : ProductionContextWireV1
    family-judgment-subject : WireTermV1
    family-judgment-type : WireTermV1

open FamilyJudgmentWireV1 public

data SeedSourceWireV1 : Set where
  seed-public-head : Nat → SeedSourceWireV1
  seed-public-equation : WireIdV1 → SeedSourceWireV1

data FamilyPayloadWireV1 : Set where
  family-seed :
    WireIdV1 → SeedSourceWireV1 → FamilyJudgmentWireV1 → FamilyPayloadWireV1
  family-generic-public-application :
    WireIdV1 → WireIdV1 → WireIdV1 → FamilyJudgmentWireV1 →
    FamilyPayloadWireV1
  family-generic-equation-action :
    WireIdV1 → WireIdV1 → WireIdV1 → FamilyJudgmentWireV1 →
    FamilyPayloadWireV1

parse-path-component-after : Nat → Decoder ConversionPathComponentWireV1
parse-path-component-after zero = pure path-pi-parameter
parse-path-component-after (suc zero) = pure path-pi-body
parse-path-component-after (suc (suc zero)) = pure path-lambda-parameter
parse-path-component-after (suc (suc (suc zero))) = pure path-lambda-body
parse-path-component-after (suc (suc (suc (suc zero)))) =
  pure path-apply-function
parse-path-component-after (suc (suc (suc (suc (suc zero))))) =
  pure path-apply-argument
parse-path-component-after (suc (suc (suc (suc (suc (suc _)))))) = fail

parse-path-component : Decoder ConversionPathComponentWireV1
parse-path-component =
  parse-byte >>= λ tag →
  parse-path-component-after (byte-value tag)

parse-no-redex-disposition-after : Nat → Decoder NoRedexDispositionWireV1
parse-no-redex-disposition-after zero = pure disposition-sort
parse-no-redex-disposition-after (suc zero) = pure disposition-variable
parse-no-redex-disposition-after (suc (suc zero)) =
  pure disposition-global-not-enabled
parse-no-redex-disposition-after (suc (suc (suc zero))) = pure disposition-pi
parse-no-redex-disposition-after (suc (suc (suc (suc zero)))) =
  pure disposition-lambda
parse-no-redex-disposition-after (suc (suc (suc (suc (suc zero))))) =
  pure disposition-neutral-application
parse-no-redex-disposition-after (suc (suc (suc (suc (suc (suc zero)))))) =
  pure disposition-unit-type
parse-no-redex-disposition-after
  (suc (suc (suc (suc (suc (suc (suc zero))))))) =
  pure disposition-unit
parse-no-redex-disposition-after
  (suc (suc (suc (suc (suc (suc (suc (suc _)))))))) = fail

parse-no-redex-disposition : Decoder NoRedexDispositionWireV1
parse-no-redex-disposition =
  parse-byte >>= λ tag →
  parse-no-redex-disposition-after (byte-value tag)

parse-no-redex-entry : Decoder NoRedexEntryWireV1
parse-no-redex-entry =
  parse-bounded-list max-sequence-items-v1 parse-path-component >>= λ path →
  parse-wire-term >>= λ term →
  parse-no-redex-disposition >>= λ disposition →
  pure (no-redex-entry-v1 path term disposition)

parse-no-redex-census : Decoder (List NoRedexEntryWireV1)
parse-no-redex-census =
  parse-bounded-list max-sequence-items-v1 parse-no-redex-entry

-- Reduction steps share the single Rust recursion budget with the terms
-- they contain, so the fuel threads through both parsers.
parse-reduction-step-fuel : Nat → Decoder BaseQ0ReductionStepWireV1
parse-reduction-step-after :
  Nat → Nat → WireTermV1 → WireTermV1 → Decoder BaseQ0ReductionStepWireV1

parse-reduction-step-fuel zero = fail
parse-reduction-step-fuel (suc fuel) =
  parse-byte >>= λ tag →
  parse-wire-term-fuel fuel >>= λ source →
  parse-wire-term-fuel fuel >>= λ target →
  parse-reduction-step-after fuel (byte-value tag) source target

parse-reduction-step-after fuel zero source target =
  pure (step-beta source target)
parse-reduction-step-after fuel (suc zero) source target =
  parse-u32-nat >>= λ slot →
  pure (step-transparent-delta source target slot)
parse-reduction-step-after fuel (suc (suc zero)) source target =
  parse-reduction-step-fuel fuel >>= λ premise →
  pure (step-pi-parameter-congruence source target premise)
parse-reduction-step-after fuel (suc (suc (suc zero))) source target =
  parse-reduction-step-fuel fuel >>= λ premise →
  pure (step-pi-body-congruence source target premise)
parse-reduction-step-after fuel (suc (suc (suc (suc zero)))) source target =
  parse-reduction-step-fuel fuel >>= λ premise →
  pure (step-lambda-parameter-congruence source target premise)
parse-reduction-step-after fuel (suc (suc (suc (suc (suc zero)))))
  source target =
  parse-reduction-step-fuel fuel >>= λ premise →
  pure (step-lambda-body-congruence source target premise)
parse-reduction-step-after fuel (suc (suc (suc (suc (suc (suc zero))))))
  source target =
  parse-reduction-step-fuel fuel >>= λ premise →
  pure (step-apply-function-congruence source target premise)
parse-reduction-step-after fuel
  (suc (suc (suc (suc (suc (suc (suc zero))))))) source target =
  parse-reduction-step-fuel fuel >>= λ premise →
  pure (step-apply-argument-congruence source target premise)
parse-reduction-step-after fuel
  (suc (suc (suc (suc (suc (suc (suc (suc _)))))))) source target = fail

parse-reduction-step : Decoder BaseQ0ReductionStepWireV1
parse-reduction-step = parse-reduction-step-fuel term-recursion-limit-v1

parse-reduction-trace : Decoder BaseQ0ReductionTraceWireV1
parse-reduction-trace =
  parse-wire-term >>= λ start →
  parse-bounded-list max-sequence-items-v1 parse-reduction-step >>= λ steps →
  parse-wire-term >>= λ end →
  pure (reduction-trace-v1 start steps end)

parse-endpoint-judgment-after : Nat → Decoder EndpointJudgmentWireV1
parse-endpoint-judgment-after zero =
  parse-wire-term >>= λ expected →
  pure (endpoint-has-type expected)
parse-endpoint-judgment-after (suc zero) = pure endpoint-type-formation
parse-endpoint-judgment-after (suc (suc _)) = fail

parse-endpoint-judgment : Decoder EndpointJudgmentWireV1
parse-endpoint-judgment =
  parse-byte >>= λ tag →
  parse-endpoint-judgment-after (byte-value tag)

parse-conversion-certificate : Decoder ConversionCertificateWireV1
parse-conversion-certificate =
  parse-wire-id >>= λ identifier →
  parse-production-context >>= λ context →
  parse-wire-term >>= λ left →
  parse-wire-term >>= λ right →
  parse-endpoint-judgment >>= λ endpoint →
  parse-wire-term >>= λ common →
  parse-reduction-trace >>= λ left-trace →
  parse-reduction-trace >>= λ right-trace →
  parse-no-redex-census >>= λ census →
  pure
    (conversion-certificate-v1
      identifier context left right endpoint common
      left-trace right-trace census)

parse-conversion-certificates : Decoder (List ConversionCertificateWireV1)
parse-conversion-certificates =
  parse-bounded-list max-sequence-items-v1 parse-conversion-certificate

-- Synthesis codes also share the Rust recursion budget with their
-- embedded dependent-result term.
parse-synthesis-code-fuel : Nat → Decoder SynthesisCodeWireV1
parse-synthesis-code-after : Nat → Nat → Decoder SynthesisCodeWireV1

parse-synthesis-code-fuel zero = fail
parse-synthesis-code-fuel (suc fuel) =
  parse-byte >>= λ tag →
  parse-synthesis-code-after fuel (byte-value tag)

parse-synthesis-code-after fuel zero =
  parse-u16-nat >>= λ level →
  pure (code-sort level)
parse-synthesis-code-after fuel (suc zero) = pure code-unit-type
parse-synthesis-code-after fuel (suc (suc zero)) = pure code-unit
parse-synthesis-code-after fuel (suc (suc (suc zero))) =
  parse-u32-nat >>= λ index →
  parse-u32-nat >>= λ ordinal →
  parse-u32-nat >>= λ shift →
  pure (code-variable-lookup index ordinal shift)
parse-synthesis-code-after fuel (suc (suc (suc (suc zero)))) =
  parse-u32-nat >>= λ slot →
  pure (code-global-lookup slot)
parse-synthesis-code-after fuel (suc (suc (suc (suc (suc zero))))) =
  parse-synthesis-code-fuel fuel >>= λ parameter →
  parse-synthesis-code-fuel fuel >>= λ body →
  pure (code-pi-formation parameter body)
parse-synthesis-code-after fuel (suc (suc (suc (suc (suc (suc zero)))))) =
  parse-synthesis-code-fuel fuel >>= λ parameter →
  parse-synthesis-code-fuel fuel >>= λ body →
  pure (code-lambda-introduction parameter body)
parse-synthesis-code-after fuel
  (suc (suc (suc (suc (suc (suc (suc zero))))))) =
  parse-synthesis-code-fuel fuel >>= λ function →
  parse-synthesis-code-fuel fuel >>= λ argument →
  parse-wire-id >>= λ function-conversion →
  parse-wire-id >>= λ argument-conversion →
  parse-wire-term-fuel fuel >>= λ result →
  pure
    (code-application-elimination
      function argument function-conversion argument-conversion result)
parse-synthesis-code-after fuel
  (suc (suc (suc (suc (suc (suc (suc (suc _)))))))) = fail

parse-synthesis-code : Decoder SynthesisCodeWireV1
parse-synthesis-code = parse-synthesis-code-fuel term-recursion-limit-v1

parse-synthesis-certificate : Decoder SynthesisCertificateWireV1
parse-synthesis-certificate =
  parse-wire-id >>= λ identifier →
  parse-production-context >>= λ context →
  parse-wire-term >>= λ subject →
  parse-wire-term >>= λ inferred →
  parse-synthesis-code >>= λ code →
  pure (synthesis-certificate-v1 identifier context subject inferred code)

parse-synthesis-certificates : Decoder (List SynthesisCertificateWireV1)
parse-synthesis-certificates =
  parse-bounded-list max-sequence-items-v1 parse-synthesis-certificate

parse-conversion-typing-supplement : Decoder ConversionTypingSupplementWireV1
parse-conversion-typing-supplement =
  parse-wire-id >>= λ identifier →
  parse-bounded-list max-sequence-items-v1 parse-u32-nat >>= λ step-path →
  parse-production-context >>= λ context →
  parse-endpoint-judgment >>= λ endpoint →
  parse-synthesis-certificate >>= λ source →
  parse-synthesis-certificate >>= λ target →
  parse-option parse-u16-nat >>= λ formation →
  pure
    (conversion-typing-supplement-v1
      identifier step-path context endpoint source target formation)

parse-conversion-typing-supplements :
  Decoder (List ConversionTypingSupplementWireV1)
parse-conversion-typing-supplements =
  parse-bounded-list max-sequence-items-v1 parse-conversion-typing-supplement

parse-q0-inventory : Decoder (List Nat)
parse-q0-inventory = parse-bounded-list max-sequence-items-v1 (parse-tag-below 6)

parse-fresh-rule-schema : Decoder FreshRuleSchemaWireV1
parse-fresh-rule-schema =
  parse-wire-id >>= λ equation →
  parse-u32-nat >>= λ owner →
  parse-u32-nat >>= λ constructor-slot →
  parse-production-context >>= λ context →
  parse-wire-term >>= λ left →
  parse-wire-term >>= λ right →
  parse-wire-term >>= λ ty →
  parse-u32-nat >>= λ scrutinee →
  parse-u16-nat >>= λ arity →
  pure
    (fresh-rule-schema-v1
      equation owner constructor-slot context left right ty scrutinee arity)

parse-fresh-rule-schemas : Decoder (List FreshRuleSchemaWireV1)
parse-fresh-rule-schemas =
  parse-bounded-list max-sequence-items-v1 parse-fresh-rule-schema

parse-family-inventory : Decoder (List Nat)
parse-family-inventory =
  parse-bounded-list max-sequence-items-v1 (parse-tag-below 2)

parse-family-judgment : Decoder FamilyJudgmentWireV1
parse-family-judgment =
  parse-production-context >>= λ context →
  parse-wire-term >>= λ subject →
  parse-wire-term >>= λ ty →
  pure (family-judgment-v1 context subject ty)

parse-seed-source-after : Nat → Decoder SeedSourceWireV1
parse-seed-source-after zero =
  parse-u32-nat >>= λ owner →
  pure (seed-public-head owner)
parse-seed-source-after (suc zero) =
  parse-wire-id >>= λ equation →
  pure (seed-public-equation equation)
parse-seed-source-after (suc (suc _)) = fail

parse-seed-source : Decoder SeedSourceWireV1
parse-seed-source =
  parse-byte >>= λ tag →
  parse-seed-source-after (byte-value tag)

parse-family-payload-after : Nat → Decoder FamilyPayloadWireV1
parse-family-payload-after zero =
  parse-wire-id >>= λ family →
  parse-seed-source >>= λ source →
  parse-family-judgment >>= λ judgment →
  pure (family-seed family source judgment)
parse-family-payload-after (suc zero) =
  parse-wire-id >>= λ family →
  parse-wire-id >>= λ function-family →
  parse-wire-id >>= λ argument-family →
  parse-family-judgment >>= λ judgment →
  pure
    (family-generic-public-application
      family function-family argument-family judgment)
parse-family-payload-after (suc (suc zero)) =
  parse-wire-id >>= λ family →
  parse-wire-id >>= λ equation →
  parse-wire-id >>= λ source-family →
  parse-family-judgment >>= λ judgment →
  pure
    (family-generic-equation-action family equation source-family judgment)
parse-family-payload-after (suc (suc (suc _))) = fail

parse-family-payload : Decoder FamilyPayloadWireV1
parse-family-payload =
  parse-byte >>= λ tag →
  parse-family-payload-after (byte-value tag)

parse-family-payloads : Decoder (List FamilyPayloadWireV1)
parse-family-payloads =
  parse-bounded-list max-sequence-items-v1 parse-family-payload

record ProductionBundleSemanticV1 : Set where
  constructor production-bundle-semantic-v1
  field
    bundle-manifest : ManifestSurfaceWireV1
    bundle-signature : ProductionSignatureWireV1
    bundle-global-slots : List GlobalSlotEntryWireV1
    bundle-contexts : List ProductionContextWireV1
    bundle-conversions : List ConversionCertificateWireV1
    bundle-supplements : List ConversionTypingSupplementWireV1
    bundle-synthesis : List SynthesisCertificateWireV1
    bundle-q0-rules : List Nat
    bundle-fresh-rules : List FreshRuleSchemaWireV1
    bundle-family-codes : List Nat
    bundle-family-payloads : List FamilyPayloadWireV1

open ProductionBundleSemanticV1 public

decode-production-bundle-sections-v1 :
  RawProductionEnvelopeV1 → Maybe ProductionBundleSemanticV1
decode-production-bundle-sections-v1 envelope =
  maybe-bind
    (run-complete parse-manifest-surface
      (second (manifest-section envelope)))
    (λ manifest →
  maybe-bind
    (run-complete parse-production-signature
      (second (signature-section envelope)))
    (λ signature →
  maybe-bind
    (run-complete parse-global-slot-table
      (second (global-slots-section envelope)))
    (λ slots →
  maybe-bind
    (run-complete parse-production-contexts
      (second (contexts-section envelope)))
    (λ contexts →
  maybe-bind
    (run-complete parse-conversion-certificates
      (second (conversions-section envelope)))
    (λ conversions →
  maybe-bind
    (run-complete parse-conversion-typing-supplements
      (second (conversion-supplements-section envelope)))
    (λ supplements →
  maybe-bind
    (run-complete parse-synthesis-certificates
      (second (synthesis-section envelope)))
    (λ synthesis →
  maybe-bind
    (run-complete parse-q0-inventory
      (second (q0-inventory-section envelope)))
    (λ q0-rules →
  maybe-bind
    (run-complete parse-fresh-rule-schemas
      (second (fresh-rules-section envelope)))
    (λ fresh-rules →
  maybe-bind
    (run-complete parse-family-inventory
      (second (family-inventory-section envelope)))
    (λ family-codes →
  maybe-bind
    (run-complete parse-family-payloads
      (second (family-payloads-section envelope)))
    (λ family-payloads →
  just
    (production-bundle-semantic-v1
      manifest signature slots contexts conversions supplements
      synthesis q0-rules fresh-rules family-codes family-payloads))))))))))))

decode-bundle-branch : Bool → List Byte → Maybe ProductionBundleSemanticV1
decode-bundle-branch true input = nothing
decode-bundle-branch false input =
  maybe-bind (decode-envelope-v1 input) decode-production-bundle-sections-v1

-- The whole-bundle byte bound mirrors `MAX_CANONICAL_BUNDLE_BYTES_V1`.
decode-production-bundle-v1 : List Byte → Maybe ProductionBundleSemanticV1
decode-production-bundle-v1 input =
  decode-bundle-branch
    (limit-exceeded max-bundle-bytes-v1 (length input))
    input