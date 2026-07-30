{-# OPTIONS --safe --without-K #-}

-- Phase E binder-local conversion-typing supplements.
--
-- The wire structural layer checks only that a supplement names an
-- existing conversion, that its formation level is a witness level,
-- and that its two synthesis certificates share its local context; the
-- step path is structurally unchecked and the Rust synthesis crate
-- records the binder-local replay as an open frontier
-- (`NestedCongruenceReplayFrontierV2`). This module defines and checks
-- the semantic contract that discharges that frontier on the Agda
-- side (per lesson 4.9: derive each binder-local context and compare
-- the supplied supplement to the derived context):
--
-- - a step path addresses one congruence premise: the trace selector
--   (0 = left, 1 = right), the step ordinal in that trace, and one
--   zero per premise descent (every congruence constructor has exactly
--   one premise). Descending through a pi-body or lambda-body
--   congruence extends the derived context with the source binder's
--   parameter; other frames leave it unchanged;
-- - the supplement's local context must equal the derived context
--   exactly, and its two certificates must have the premise's recorded
--   source and target as their subjects;
-- - for a `HasType` endpoint the formation level must be absent and
--   both certificates must record exactly the claimed type; for a
--   `TypeFormation` endpoint the formation level must be present and
--   both certificates must record exactly that sort — the existential
--   formation-level recovery;
-- - both certificates must pass the full typed synthesis replay
--   (`replay-synthesis-typed`), in the derived binder-local context;
-- - coverage: every congruence premise position of every conversion
--   must be addressed by exactly one supplement. A conversion whose
--   premise cannot be supplemented (for example because the premise
--   type would exceed the representable levels) therefore fails
--   closed rather than passing unverified.
--
-- `typing-check-bundle` is the whole-bundle typing verdict: the slot
-- table replays as a kernel verified signature, every standalone
-- context verifies, every conversion passes the endpoint-typing
-- replay, every synthesis certificate passes the typed replay, and
-- every supplement verifies with full premise coverage. It runs
-- strictly after (and does not replace) the structural checker and the
-- semantic replay. Nothing here mints authority.

module LawV2.Wire.SupplementReplayV1 where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe
  using ()
  renaming (Maybe to WireMaybe; just to wire-just; nothing to wire-nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.Wire.Bytes
  using (length; nat-equal; _and_; _++_; _×_; _,_; first; second)
open import LawV2.Wire.ProductionBundleV1
open import LawV2.Wire.ContextChecker
  using (not; nat-list-equal; byte-list-equal; check-context-entries;
         check-slots-from)
open import LawV2.Wire.BundleChecker
  using (wire-term-equal; wire-term-list-equal; step-source;
         step-target)
open import LawV2.LambdaUnit.ProductionSyntaxV1
open import LawV2.Wire.ContextCorrespondenceV1
  using (PSig; total-from; wire-slots-to-psig; list-entry;
         wire-context-extend)
open import LawV2.Wire.SemanticReplayV1
  using (wire-maybe-bind; is-wire-just)
open import LawV2.Wire.SynthesisReplayV1
  using (boolean-guard; find-conversion; delta-policy-slots)
open import LawV2.Wire.TypingReplayV1

-- One resolved congruence premise: the derived binder-local context
-- (oldest-first wire entries) and the premise's recorded source and
-- target.
record PremiseSiteV1 : Set where
  constructor premise-site
  field
    site-context-entries : List WireTermV1
    site-source : WireTermV1
    site-target : WireTermV1

open PremiseSiteV1 public

-- The premise of one congruence step, with the derived context
-- extension. Body congruences require the recorded source to expose
-- the binder whose parameter extends the context; beta and delta steps
-- have no premise.
step-child :
  List WireTermV1 → BaseQ0ReductionStepWireV1 →
  WireMaybe (List WireTermV1 × BaseQ0ReductionStepWireV1)
step-child context (step-beta _ _) = wire-nothing
step-child context (step-transparent-delta _ _ _) = wire-nothing
step-child context (step-pi-parameter-congruence _ _ premise) =
  wire-just (context , premise)
step-child context
  (step-pi-body-congruence (wire-pi parameter _) _ premise) =
  wire-just ((context ++ (parameter ∷ [])) , premise)
step-child context (step-pi-body-congruence _ _ premise) = wire-nothing
step-child context (step-lambda-parameter-congruence _ _ premise) =
  wire-just (context , premise)
step-child context
  (step-lambda-body-congruence (wire-lambda parameter _) _ premise) =
  wire-just ((context ++ (parameter ∷ [])) , premise)
step-child context (step-lambda-body-congruence _ _ premise) =
  wire-nothing
step-child context (step-apply-function-congruence _ _ premise) =
  wire-just (context , premise)
step-child context (step-apply-argument-congruence _ _ premise) =
  wire-just (context , premise)

resolve-descents :
  List WireTermV1 → BaseQ0ReductionStepWireV1 → List Nat →
  WireMaybe PremiseSiteV1
resolve-descents context step [] = wire-nothing
resolve-descents context step (zero ∷ []) =
  wire-maybe-bind (step-child context step) (λ child →
    wire-just
      (premise-site (first child)
        (step-source (second child))
        (step-target (second child))))
resolve-descents context step (zero ∷ (descent ∷ descents)) =
  wire-maybe-bind (step-child context step) (λ child →
    resolve-descents (first child) (second child)
      (descent ∷ descents))
resolve-descents context step (suc _ ∷ _) = wire-nothing

trace-of-selector :
  ConversionCertificateWireV1 → Nat →
  WireMaybe BaseQ0ReductionTraceWireV1
trace-of-selector certificate zero =
  wire-just (conversion-left-trace certificate)
trace-of-selector certificate (suc zero) =
  wire-just (conversion-right-trace certificate)
trace-of-selector certificate (suc (suc _)) = wire-nothing

resolve-site :
  ConversionCertificateWireV1 → List Nat → WireMaybe PremiseSiteV1
resolve-site certificate
  (selector ∷ (index ∷ (descent ∷ descents))) =
  wire-maybe-bind (trace-of-selector certificate selector) (λ trace →
  wire-maybe-bind (list-entry (trace-steps trace) index) (λ step →
  resolve-descents
    (entries-oldest-first (conversion-context certificate))
    step (descent ∷ descents)))
resolve-site certificate _ = wire-nothing

-- Every premise position inside one step, as descent suffixes.
step-descent-paths : BaseQ0ReductionStepWireV1 → List (List Nat)
step-descent-premise : BaseQ0ReductionStepWireV1 → List (List Nat)

prefix-zero : List (List Nat) → List (List Nat)
prefix-zero [] = []
prefix-zero (path ∷ paths) = (zero ∷ path) ∷ prefix-zero paths

step-descent-paths (step-beta _ _) = []
step-descent-paths (step-transparent-delta _ _ _) = []
step-descent-paths (step-pi-parameter-congruence _ _ premise) =
  step-descent-premise premise
step-descent-paths (step-pi-body-congruence _ _ premise) =
  step-descent-premise premise
step-descent-paths (step-lambda-parameter-congruence _ _ premise) =
  step-descent-premise premise
step-descent-paths (step-lambda-body-congruence _ _ premise) =
  step-descent-premise premise
step-descent-paths (step-apply-function-congruence _ _ premise) =
  step-descent-premise premise
step-descent-paths (step-apply-argument-congruence _ _ premise) =
  step-descent-premise premise

step-descent-premise premise =
  (zero ∷ []) ∷ prefix-zero (step-descent-paths premise)

-- Every premise path of one trace: `selector ∷ ordinal ∷ descents`.
trace-premise-paths :
  Nat → Nat → List BaseQ0ReductionStepWireV1 → List (List Nat)
trace-premise-paths selector ordinal [] = []
trace-premise-paths selector ordinal (step ∷ steps) =
  attach (step-descent-paths step) ++
  trace-premise-paths selector (suc ordinal) steps
  where
  attach : List (List Nat) → List (List Nat)
  attach [] = []
  attach (descents ∷ rest) =
    (selector ∷ (ordinal ∷ descents)) ∷ attach rest

conversion-premise-paths :
  ConversionCertificateWireV1 → List (List Nat)
conversion-premise-paths certificate =
  trace-premise-paths 0 zero
    (trace-steps (conversion-left-trace certificate)) ++
  trace-premise-paths 1 zero
    (trace-steps (conversion-right-trace certificate))

-- Exactly-one coverage: each premise path of each conversion must be
-- addressed by exactly one supplement naming that conversion.
count-supplements-at :
  WireIdV1 → List Nat → List ConversionTypingSupplementWireV1 → Nat
count-supplements-at conversion path [] = zero
count-supplements-at conversion path (supplement ∷ supplements) =
  add-match
    (byte-list-equal (supplement-conversion-id supplement) conversion
     and
     nat-list-equal (supplement-step-path supplement) path)
  where
  add-match : Bool → Nat
  add-match true =
    suc (count-supplements-at conversion path supplements)
  add-match false = count-supplements-at conversion path supplements

paths-covered :
  WireIdV1 → List (List Nat) →
  List ConversionTypingSupplementWireV1 → Bool
paths-covered conversion [] supplements = true
paths-covered conversion (path ∷ paths) supplements =
  nat-equal (count-supplements-at conversion path supplements) 1 and
  paths-covered conversion paths supplements

conversions-covered :
  List ConversionCertificateWireV1 →
  List ConversionTypingSupplementWireV1 → Bool
conversions-covered [] supplements = true
conversions-covered (certificate ∷ certificates) supplements =
  paths-covered (conversion-id certificate)
    (conversion-premise-paths certificate) supplements and
  conversions-covered certificates supplements

-- The endpoint claim: for `HasType` both certificates record exactly
-- the claimed type and no formation level is present; for
-- `TypeFormation` the recovered existential level is present and both
-- certificates record exactly that sort.
check-supplement-endpoint :
  EndpointJudgmentWireV1 → WireMaybe Nat →
  SynthesisCertificateWireV1 → SynthesisCertificateWireV1 → Bool
check-supplement-endpoint (endpoint-has-type expected) wire-nothing
  source-code target-code =
  wire-term-equal (synthesis-inferred-type source-code) expected and
  wire-term-equal (synthesis-inferred-type target-code) expected
check-supplement-endpoint (endpoint-has-type expected) (wire-just _)
  source-code target-code = false
check-supplement-endpoint endpoint-type-formation wire-nothing
  source-code target-code = false
check-supplement-endpoint endpoint-type-formation (wire-just level)
  source-code target-code =
  wire-term-equal (synthesis-inferred-type source-code)
    (wire-sort level) and
  wire-term-equal (synthesis-inferred-type target-code)
    (wire-sort level)

check-supplement-typing :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1) (fuel : Nat) →
  ConversionTypingSupplementWireV1 → Bool
check-supplement-typing globals enabled sig conversions fuel
  supplement =
  site-branch
    (wire-maybe-bind
      (find-conversion (supplement-conversion-id supplement)
        conversions)
      (λ certificate →
        resolve-site certificate (supplement-step-path supplement)))
  where
  site-branch : WireMaybe PremiseSiteV1 → Bool
  site-branch wire-nothing = false
  site-branch (wire-just site) =
    wire-term-list-equal (site-context-entries site)
      (entries-oldest-first (supplement-local-context supplement)) and
    (wire-term-equal (site-source site)
      (synthesis-subject (supplement-source-code supplement)) and
    (wire-term-equal (site-target site)
      (synthesis-subject (supplement-target-code supplement)) and
    (check-supplement-endpoint (supplement-endpoint supplement)
      (supplement-formation-level supplement)
      (supplement-source-code supplement)
      (supplement-target-code supplement) and
    (replay-synthesis-typed globals enabled sig conversions fuel
      (supplement-source-code supplement) and
     replay-synthesis-typed globals enabled sig conversions fuel
       (supplement-target-code supplement)))))

check-supplements-typing :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1) (fuel : Nat) →
  List ConversionTypingSupplementWireV1 → Bool
check-supplements-typing globals enabled sig conversions fuel [] = true
check-supplements-typing globals enabled sig conversions fuel
  (supplement ∷ supplements) =
  check-supplement-typing globals enabled sig conversions fuel
    supplement and
  check-supplements-typing globals enabled sig conversions fuel
    supplements

-- Standalone context surfaces (section 4): each must replay through
-- kernel context verification.
check-context-typing :
  (globals : Nat) (sig : PSig globals) (fuel : Nat) →
  ProductionContextWireV1 → Bool
check-context-typing globals sig fuel context =
  scoped-branch
    (check-context-entries globals zero
      (entries-oldest-first context))
    refl
  where
  scoped-branch :
    (scoped : Bool) →
    check-context-entries globals zero (entries-oldest-first context)
    ≡ scoped → Bool
  scoped-branch false proof = false
  scoped-branch true proof =
    is-wire-just
      (pverify-context sig fuel
        (wire-context-extend zero pempty
          (entries-oldest-first context) proof))

check-contexts-typing :
  (globals : Nat) (sig : PSig globals) (fuel : Nat) →
  List ProductionContextWireV1 → Bool
check-contexts-typing globals sig fuel [] = true
check-contexts-typing globals sig fuel (context ∷ contexts) =
  check-context-typing globals sig fuel context and
  check-contexts-typing globals sig fuel contexts

all-conversions-typing :
  {globals : Nat} (sig : PSig globals) (fuel : Nat) →
  List ConversionCertificateWireV1 → Bool
all-conversions-typing sig fuel [] = true
all-conversions-typing sig fuel (certificate ∷ certificates) =
  replay-conversion-typing sig fuel certificate and
  all-conversions-typing sig fuel certificates

all-synthesis-typing :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1) (fuel : Nat) →
  List SynthesisCertificateWireV1 → Bool
all-synthesis-typing globals enabled sig conversions fuel [] = true
all-synthesis-typing globals enabled sig conversions fuel
  (certificate ∷ certificates) =
  replay-synthesis-typed globals enabled sig conversions fuel
    certificate and
  all-synthesis-typing globals enabled sig conversions fuel
    certificates

-- The whole-bundle typing verdict. Runs strictly after the structural
-- checker and the semantic replay; every section that carries
-- judgments is replayed through the kernel-mirroring typing layer.
typing-check-bundle : ProductionBundleSemanticV1 → Bool
typing-check-bundle bundle =
  slots-branch (check-slots-from zero (bundle-global-slots bundle))
    refl
  where
  slots-branch :
    (result : Bool) →
    check-slots-from zero (bundle-global-slots bundle) ≡ result → Bool
  slots-branch false proof = false
  slots-branch true proof =
    pcheck-signature typing-fuel-v1 sig and
    (check-contexts-typing globals sig typing-fuel-v1
      (bundle-contexts bundle) and
    (all-conversions-typing sig typing-fuel-v1
      (bundle-conversions bundle) and
    (all-synthesis-typing globals enabled sig
      (bundle-conversions bundle) typing-fuel-v1
      (bundle-synthesis bundle) and
    (check-supplements-typing globals enabled sig
      (bundle-conversions bundle) typing-fuel-v1
      (bundle-supplements bundle) and
     conversions-covered (bundle-conversions bundle)
       (bundle-supplements bundle)))))
    where
    globals : Nat
    globals = total-from zero (bundle-global-slots bundle)
    sig : PSig globals
    sig = wire-slots-to-psig (bundle-global-slots bundle) proof
    enabled : List Nat
    enabled =
      delta-policy-slots
        (allowed-transparent-deltas (bundle-signature bundle))
