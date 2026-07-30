{-# OPTIONS --safe --without-K #-}

-- Phase E, synthesis side: semantic type recomputation for decoded
-- synthesis certificates.
--
-- The structural checker verifies shapes and metadata equations; it
-- never recomputes the synthesized types. This module recursively
-- synthesizes the intrinsic subject and type of every code:
--
-- - variables synthesize their exact `lookup-pctx` in-context type,
--   with the ordinal and shift metadata re-verified;
-- - globals synthesize their strict-prior declared type embedded into
--   the local scope;
-- - pi formation combines component sort levels with the kernel `max`;
-- - lambda introduction synthesizes the dependent pi type; and
-- - application elimination resolves its function and argument
--   conversions by identifier (requiring identical derived contexts and
--   full semantic conversion replay), destructures the pi type, and
--   recomputes the dependent result by intrinsic instantiation,
--   comparing it against both the embedded dependent-result term and
--   the certificate's inferred type.
--
-- Conversion mediation enforces the synthesis protocol V2 side
-- conditions exactly: the conversion's left endpoint is the synthesized
-- type, its right endpoint must equal its own common normal form (so
-- the pi is destructured from the census-normal form), and its endpoint
-- judgment must be `TypeFormation`, all in the identical derived
-- context and under full semantic replay. One deliberate remaining
-- delta from the kernel: the recorded dependent result is required to
-- equal the raw intrinsic instantiation, while the kernel records the
-- kernel-normalized instantiation; reconciling the two requires the
-- bounded-normalization bridge and is deferred with the
-- typing-judgment bridge. Nothing here mints authority.

module LawV2.Wire.SynthesisReplayV1 where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe
  using ()
  renaming (Maybe to WireMaybe; just to wire-just; nothing to wire-nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc; _-_)
open import LawV2.Wire.Bytes
  using (length; nat-equal; _and_; _×_; _,_; first; second)
open import LawV2.Wire.ProductionBundleV1
open import LawV2.Wire.ContextChecker
  using (not; nat-less; byte-list-equal; check-context-entries;
         check-slots-from)
open import LawV2.Wire.BundleChecker
  using (wire-term-list-equal)
open import LawV2.LambdaUnit.ProductionSyntaxV1
open import LawV2.LambdaUnit.ProductionDecodingV1
open import LawV2.Wire.ContextCorrespondenceV1
open import LawV2.Wire.SemanticReplayV1

boolean-guard : {A : Set} → Bool → WireMaybe A → WireMaybe A
boolean-guard false continuation = wire-nothing
boolean-guard true continuation = continuation

find-conversion :
  WireIdV1 → List ConversionCertificateWireV1 →
  WireMaybe ConversionCertificateWireV1
find-conversion identifier [] = wire-nothing
find-conversion identifier (certificate ∷ certificates) =
  find-branch (byte-list-equal identifier (conversion-id certificate))
  where
  find-branch : Bool → WireMaybe ConversionCertificateWireV1
  find-branch true = wire-just certificate
  find-branch false = find-conversion identifier certificates

is-type-formation : EndpointJudgmentWireV1 → Bool
is-type-formation endpoint-type-formation = true
is-type-formation (endpoint-has-type _) = false

-- Resolve one application-elimination conversion under the exact V2
-- side conditions: same derived context, `TypeFormation` endpoint, full
-- semantic replay, left endpoint equal to the synthesized type, and
-- right endpoint equal to the common normal form; returns that normal
-- form.
use-conversion :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1)
  (locals : Nat) (context-wire : List WireTermV1)
  (identifier : WireIdV1) (synthesized : PTm globals locals) →
  WireMaybe (PTm globals locals)
use-conversion globals enabled sig conversions locals context-wire
  identifier synthesized =
  wire-maybe-bind (find-conversion identifier conversions)
    (λ certificate →
  boolean-guard
    (wire-term-list-equal
      (entries-oldest-first (conversion-context certificate))
      context-wire)
    (boolean-guard
      (is-type-formation (conversion-endpoint certificate))
      (boolean-guard
        (replay-conversion globals enabled sig certificate)
        (wire-maybe-bind
          (decode-term-checked globals locals
            (conversion-left certificate))
          (λ left →
        wire-maybe-bind
          (decode-term-checked globals locals
            (conversion-right certificate))
          (λ right →
        wire-maybe-bind
          (decode-term-checked globals locals
            (conversion-common-normal-form certificate))
          (λ common →
        boolean-guard (ptm-equal left synthesized)
          (boolean-guard (ptm-equal right common)
            (wire-just common)))))))))

sort-level : {globals locals : Nat} → PTm globals locals → WireMaybe Nat
sort-level (psort level) = wire-just level
sort-level _ = wire-nothing

pi-parts :
  {globals locals : Nat} → PTm globals locals →
  WireMaybe (PTm globals locals × PTm globals (suc locals))
pi-parts (ppi parameter body) = wire-just (parameter , body)
pi-parts _ = wire-nothing

fin-checked :
  (index count : Nat) → WireMaybe (Fin count)
fin-checked index count = branch (nat-less index count) refl
  where
  branch :
    (result : Bool) → nat-less index count ≡ result →
    WireMaybe (Fin count)
  branch false proof = wire-nothing
  branch true proof = wire-just (fin-from-less index count proof)

-- Recursive intrinsic synthesis of (subject, type) for every code.
psynthesize :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1)
  (locals : Nat) (context : PCtx globals locals) →
  SynthesisCodeWireV1 →
  WireMaybe (PTm globals locals × PTm globals locals)
psynthesize globals enabled sig conversions locals context
  (code-sort level) =
  wire-just (psort level , psort (suc level))
psynthesize globals enabled sig conversions locals context
  code-unit-type =
  wire-just (punit-type , psort zero)
psynthesize globals enabled sig conversions locals context code-unit =
  wire-just (punit , punit-type)
psynthesize globals enabled sig conversions locals context
  (code-variable-lookup index ordinal shift) =
  boolean-guard (nat-equal ordinal (locals - suc index))
    (boolean-guard (nat-equal shift (suc index))
      (wire-maybe-bind (fin-checked index locals) (λ x →
        wire-just (pvar x , lookup-pctx context x))))
psynthesize globals enabled sig conversions locals context
  (code-global-lookup slot) =
  wire-maybe-bind (fin-checked slot globals) (λ x →
    wire-just
      (pglobal x , pembed-closed (lookup-psig-type sig x)))
psynthesize globals enabled sig conversions locals context
  (code-pi-formation parameter body) =
  wire-maybe-bind
    (psynthesize globals enabled sig conversions locals context
      parameter)
    (λ parameter-result →
  wire-maybe-bind (sort-level (second parameter-result))
    (λ parameter-level →
  wire-maybe-bind
    (psynthesize globals enabled sig conversions (suc locals)
      (context psnoc first parameter-result) body)
    (λ body-result →
  wire-maybe-bind (sort-level (second body-result)) (λ body-level →
  wire-just
    (ppi (first parameter-result) (first body-result) ,
     psort (parameter-level max body-level))))))
psynthesize globals enabled sig conversions locals context
  (code-lambda-introduction parameter body) =
  wire-maybe-bind
    (psynthesize globals enabled sig conversions locals context
      parameter)
    (λ parameter-result →
  wire-maybe-bind (sort-level (second parameter-result)) (λ _ →
  wire-maybe-bind
    (psynthesize globals enabled sig conversions (suc locals)
      (context psnoc first parameter-result) body)
    (λ body-result →
  wire-just
    (plam (first parameter-result) (first body-result) ,
     ppi (first parameter-result) (second body-result)))))
psynthesize globals enabled sig conversions locals context
  (code-application-elimination function argument
    function-conversion argument-conversion dependent-result) =
  wire-maybe-bind
    (psynthesize globals enabled sig conversions locals context
      function)
    (λ function-result →
  wire-maybe-bind
    (psynthesize globals enabled sig conversions locals context
      argument)
    (λ argument-result →
  wire-maybe-bind
    (use-conversion globals enabled sig conversions locals
      (pctx-to-wire context) function-conversion
      (second function-result))
    (λ pi-type →
  wire-maybe-bind (pi-parts pi-type) (λ parts →
  wire-maybe-bind
    (use-conversion globals enabled sig conversions locals
      (pctx-to-wire context) argument-conversion
      (second argument-result))
    (λ argument-target →
  boolean-guard (ptm-equal argument-target (first parts))
    (wire-maybe-bind
      (decode-term-checked globals locals dependent-result)
      (λ recorded-result →
    boolean-guard
      (ptm-equal recorded-result
        (pinstantiate (second parts) (first argument-result)))
      (wire-just
        (papp (first function-result) (first argument-result) ,
         pinstantiate (second parts) (first argument-result))))))))))

-- Certificate-level semantic verdict: the recomputed subject and type
-- must equal the decoded recorded subject and inferred type.
replay-synthesis-with-context :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1)
  (locals : Nat) (context : PCtx globals locals)
  (certificate : SynthesisCertificateWireV1) → Bool
replay-synthesis-with-context globals enabled sig conversions locals
  context certificate =
  verdict
    (psynthesize globals enabled sig conversions locals context
      (synthesis-code certificate))
    (decode-term-checked globals locals
      (synthesis-subject certificate))
    (decode-term-checked globals locals
      (synthesis-inferred-type certificate))
  where
  verdict :
    WireMaybe (PTm globals locals × PTm globals locals) →
    WireMaybe (PTm globals locals) → WireMaybe (PTm globals locals) →
    Bool
  verdict (wire-just result) (wire-just subject) (wire-just inferred) =
    ptm-equal (first result) subject and
    ptm-equal (second result) inferred
  verdict _ _ _ = false

replay-synthesis-certificate :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1)
  (certificate : SynthesisCertificateWireV1) → Bool
replay-synthesis-certificate globals enabled sig conversions
  certificate =
  branch
    (check-context-entries globals zero
      (entries-oldest-first (synthesis-context certificate)))
    refl
  where
  branch :
    (result : Bool) →
    check-context-entries globals zero
      (entries-oldest-first (synthesis-context certificate)) ≡ result →
    Bool
  branch false proof = false
  branch true proof =
    replay-synthesis-with-context globals enabled sig conversions
      (total-from zero
        (entries-oldest-first (synthesis-context certificate)))
      (wire-context-extend zero pempty
        (entries-oldest-first (synthesis-context certificate)) proof)
      certificate

-- Whole-bundle semantic verdict: every conversion certificate replays
-- as genuine intrinsic reductions to a stepless normal form, and every
-- synthesis certificate recomputes to its recorded subject and type.
-- This runs strictly after (and does not replace) the structural
-- checker.
all-conversions-replay :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals) →
  List ConversionCertificateWireV1 → Bool
all-conversions-replay globals enabled sig [] = true
all-conversions-replay globals enabled sig
  (certificate ∷ certificates) =
  replay-conversion globals enabled sig certificate and
  all-conversions-replay globals enabled sig certificates

all-synthesis-replay :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1) →
  List SynthesisCertificateWireV1 → Bool
all-synthesis-replay globals enabled sig conversions [] = true
all-synthesis-replay globals enabled sig conversions
  (certificate ∷ certificates) =
  replay-synthesis-certificate globals enabled sig conversions
    certificate and
  all-synthesis-replay globals enabled sig conversions certificates

delta-policy-slots : List DeltaPolicyEntryWireV1 → List Nat
delta-policy-slots [] = []
delta-policy-slots (entry ∷ entries) =
  delta-global-slot entry ∷ delta-policy-slots entries

semantic-check-bundle : ProductionBundleSemanticV1 → Bool
semantic-check-bundle bundle =
  branch
    (check-slots-from zero (bundle-global-slots bundle)) refl
  where
  branch :
    (result : Bool) →
    check-slots-from zero (bundle-global-slots bundle) ≡ result → Bool
  branch false proof = false
  branch true proof =
    all-conversions-replay
      (total-from zero (bundle-global-slots bundle))
      (delta-policy-slots
        (allowed-transparent-deltas (bundle-signature bundle)))
      (wire-slots-to-psig (bundle-global-slots bundle) proof)
      (bundle-conversions bundle) and
    all-synthesis-replay
      (total-from zero (bundle-global-slots bundle))
      (delta-policy-slots
        (allowed-transparent-deltas (bundle-signature bundle)))
      (wire-slots-to-psig (bundle-global-slots bundle) proof)
      (bundle-conversions bundle)
      (bundle-synthesis bundle)
