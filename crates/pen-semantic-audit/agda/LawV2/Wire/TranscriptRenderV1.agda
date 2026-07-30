{-# OPTIONS --safe --without-K #-}

-- The versioned canonical production transcript, rendered
-- independently by safe Agda (Phase G).
--
-- This module mirrors the Rust `production_transcript.rs` renderer
-- byte-for-byte, but every computed entry comes from the Agda side's
-- own replay machinery: formation levels from `pinfer` and
-- `pexpect-universe`, normal forms from `pnormalize` under the full
-- stored-body delta, synthesis types and dependent application
-- results from `psynthesize-cert`'s proof-carrying derivations, and
-- supplement premise sites from `resolve-site`. A divergence in any
-- semantically important choice between the two implementations is
-- therefore byte-visible in the transcript comparison.
--
-- The transcript is a comparison artifact with its own schema
-- identity (magic `PEN-PROD-TRAN-V1`, version 1), never a decoded
-- input. Numeric fields use the small-value little-endian rendering:
-- every numeric field of the canonical fixture is below 256, and a
-- larger value fails the rendering closed (`nat-to-byte` returns
-- nothing) rather than truncating. Nothing here mints authority.

module LawV2.Wire.TranscriptRenderV1 where

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
  using (check-context-entries; check-slots-from)
open import LawV2.LambdaUnit.ProductionSyntaxV1
open import LawV2.LambdaUnit.ProductionDecodingV1
  using (lookup-pctx)
open import LawV2.Wire.ContextCorrespondenceV1
  using (PSig; total-from; wire-slots-to-psig; wire-context-extend;
         wire-to-ptm; wire-in-scope; ptm-to-wire; lookup-psig-type;
         lookup-psig-body; and-first; and-second;
         term-scoped-in-scope; wire-map-maybe)
open import LawV2.Wire.SemanticReplayV1
  using (wire-maybe-bind; decode-term-checked)
open import LawV2.Wire.NormalizationV1
  using (pnormalize; psig-delta)
open import LawV2.Wire.SynthesisReplayV1
  using (delta-policy-slots)
open import LawV2.Wire.TypingReplayV1
  using (pinfer; pexpect-universe; typing-fuel-v1; psynthesize-cert;
         PSynthesisDerivationV1; psynth-sort; psynth-unit-type;
         psynth-unit; psynth-variable; psynth-global; psynth-pi;
         psynth-lambda; psynth-application; PSynthesisResultV1;
         result-term; result-type; result-derivation)
open import LawV2.Wire.SupplementReplayV1
  using (PremiseSiteV1; premise-site; site-context-entries;
         site-source; site-target; resolve-site)
open import LawV2.Wire.InventoryReplayV1
  using (q0-tag-to-abstract)
open import LawV2.LambdaUnit.ProductionInventoryBridgeV1
  using (ProductionQ0CategoryV1; representation; base-semantic;
         runtime-public; q0-category)

-- --- Small-value little-endian rendering ------------------------------

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

tag-byte : Nat → WireMaybe (List Byte)
tag-byte value = wire-map-maybe (λ b → b ∷ []) (nat-to-byte value)

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

render-id : WireIdV1 → WireMaybe (List Byte)
render-id identifier = wire-just identifier

render-context-entries : ProductionContextWireV1 → WireMaybe (List Byte)
render-context-entries context =
  maybe-append
    (u64-bytes-small (length (entries-oldest-first context)))
    (concat-rows (list-map render-term (entries-oldest-first context)))

-- --- Computed helpers -------------------------------------------------

-- The formation level of a decoded term under a decoded context: the
-- sort level of its kernel-normalized inferred type.
term-formation-level :
  (globals : Nat) (sig : PSig globals) {locals : Nat}
  (context : PCtx globals locals) → PTm globals locals → WireMaybe Nat
term-formation-level globals sig context term =
  wire-maybe-bind (pinfer sig typing-fuel-v1 context term)
    (pexpect-universe sig typing-fuel-v1)

-- The kernel normal form of a decoded term, rendered back to wire.
normalized-wire-term :
  (globals : Nat) (sig : PSig globals) {locals : Nat} →
  PTm globals locals → WireMaybe (List Byte)
normalized-wire-term globals sig term =
  wire-maybe-bind (pnormalize (psig-delta sig) typing-fuel-v1 term)
    (λ normal → render-term (ptm-to-wire normal))

-- One decoded context bundled with its variable count.
record DecodedContextV1 (globals : Nat) : Set where
  constructor decoded-context
  field
    decoded-locals : Nat
    decoded-pctx : PCtx globals decoded-locals

open DecodedContextV1 public

decode-context-checked :
  (globals : Nat) (entries : List WireTermV1) →
  WireMaybe (DecodedContextV1 globals)
decode-context-checked globals entries =
  scoped-branch (check-context-entries globals zero entries) refl
  where
  scoped-branch :
    (scoped : Bool) →
    check-context-entries globals zero entries ≡ scoped →
    WireMaybe (DecodedContextV1 globals)
  scoped-branch false proof = wire-nothing
  scoped-branch true proof =
    wire-just
      (decoded-context (total-from zero entries)
        (wire-context-extend zero pempty entries proof))

-- --- Section 1: signature ---------------------------------------------

render-global-row :
  (globals : Nat) (sig : PSig globals) →
  List GlobalSlotEntryWireV1 → Fin globals → WireMaybe (List Byte)
render-global-row globals sig entries x =
  maybe-append (u32-bytes-small (fin-ordinal x))
    (maybe-append
      (render-term (ptm-to-wire (lookup-psig-type sig x)))
      (maybe-append
        (render-psig-body (lookup-psig-body sig x))
        (wire-maybe-bind
          (term-formation-level globals sig pempty
            (lookup-psig-type sig x))
          u16-bytes-small)))
  where
  render-psig-body :
    WireMaybe (PTm globals zero) → WireMaybe (List Byte)
  render-psig-body wire-nothing = wire-just (byte0 ∷ [])
  render-psig-body (wire-just body) =
    with-tag byte1 (render-term (ptm-to-wire body))

render-signature-section :
  (globals : Nat) (sig : PSig globals) →
  List GlobalSlotEntryWireV1 → WireMaybe (List Byte)
render-signature-section globals sig entries =
  maybe-append (u64-bytes-small (length entries))
    (concat-rows
      (list-map (render-global-row globals sig entries)
        (all-fins globals)))

-- --- Section 2: contexts ----------------------------------------------

render-context-entry-levels :
  (globals : Nat) (sig : PSig globals) {locals : Nat}
  (prefix : PCtx globals locals) (entries : List WireTermV1) →
  check-context-entries globals locals entries ≡ true →
  WireMaybe (List Byte)
render-context-entry-levels globals sig prefix [] proof = wire-just []
render-context-entry-levels globals sig {locals} prefix
  (entry ∷ entries) proof =
  maybe-append
    (maybe-append (render-term entry)
      (wire-maybe-bind
        (term-formation-level globals sig prefix decoded)
        u16-bytes-small))
    (render-context-entry-levels globals sig
      (prefix psnoc decoded) entries
      (and-second
        (LawV2.Wire.ContextChecker.term-scoped globals locals entry)
        (check-context-entries globals (suc locals) entries) proof))
  where
  decoded : PTm globals locals
  decoded =
    wire-to-ptm globals locals entry
      (term-scoped-in-scope globals locals entry
        (and-first
          (LawV2.Wire.ContextChecker.term-scoped globals locals entry)
          (check-context-entries globals (suc locals) entries) proof))

render-context-block :
  (globals : Nat) (sig : PSig globals) → ProductionContextWireV1 →
  WireMaybe (List Byte)
render-context-block globals sig context =
  scoped-branch
    (check-context-entries globals zero
      (entries-oldest-first context))
    refl
  where
  entries : List WireTermV1
  entries = entries-oldest-first context
  scoped-branch :
    (scoped : Bool) →
    check-context-entries globals zero (entries-oldest-first context)
    ≡ scoped → WireMaybe (List Byte)
  scoped-branch false proof = wire-nothing
  scoped-branch true proof =
    maybe-append (u64-bytes-small (length entries))
      (maybe-append
        (render-context-entry-levels globals sig pempty entries proof)
        (concat-rows (list-map row (all-fins locals))))
    where
    locals : Nat
    locals = total-from zero entries
    built : PCtx globals locals
    built = wire-context-extend zero pempty entries proof
    row : Fin locals → WireMaybe (List Byte)
    row x =
      maybe-append (u32-bytes-small (fin-ordinal x))
        (maybe-append
          (u32-bytes-small (locals - suc (fin-ordinal x)))
          (maybe-append (u32-bytes-small (suc (fin-ordinal x)))
            (render-term (ptm-to-wire (lookup-pctx built x)))))

render-contexts-section :
  (globals : Nat) (sig : PSig globals) →
  List ProductionContextWireV1 → WireMaybe (List Byte)
render-contexts-section globals sig contexts =
  maybe-append (u64-bytes-small (length contexts))
    (concat-rows (list-map (render-context-block globals sig) contexts))

-- --- Section 3: conversions -------------------------------------------

path-component-tag : ConversionPathComponentWireV1 → Nat
path-component-tag path-pi-parameter = 0
path-component-tag path-pi-body = 1
path-component-tag path-lambda-parameter = 2
path-component-tag path-lambda-body = 3
path-component-tag path-apply-function = 4
path-component-tag path-apply-argument = 5

disposition-tag : NoRedexDispositionWireV1 → Nat
disposition-tag disposition-sort = 0
disposition-tag disposition-variable = 1
disposition-tag disposition-global-not-enabled = 2
disposition-tag disposition-pi = 3
disposition-tag disposition-lambda = 4
disposition-tag disposition-neutral-application = 5
disposition-tag disposition-unit-type = 6
disposition-tag disposition-unit = 7

render-step : BaseQ0ReductionStepWireV1 → WireMaybe (List Byte)
render-step (step-beta source target) =
  maybe-append (tag-byte 0)
    (maybe-append (render-term source) (render-term target))
render-step (step-transparent-delta source target slot) =
  maybe-append (tag-byte 1)
    (maybe-append (render-term source)
      (maybe-append (render-term target) (u32-bytes-small slot)))
render-step (step-pi-parameter-congruence source target premise) =
  maybe-append (tag-byte 2)
    (maybe-append (render-term source)
      (maybe-append (render-term target) (render-step premise)))
render-step (step-pi-body-congruence source target premise) =
  maybe-append (tag-byte 3)
    (maybe-append (render-term source)
      (maybe-append (render-term target) (render-step premise)))
render-step (step-lambda-parameter-congruence source target premise) =
  maybe-append (tag-byte 4)
    (maybe-append (render-term source)
      (maybe-append (render-term target) (render-step premise)))
render-step (step-lambda-body-congruence source target premise) =
  maybe-append (tag-byte 5)
    (maybe-append (render-term source)
      (maybe-append (render-term target) (render-step premise)))
render-step (step-apply-function-congruence source target premise) =
  maybe-append (tag-byte 6)
    (maybe-append (render-term source)
      (maybe-append (render-term target) (render-step premise)))
render-step (step-apply-argument-congruence source target premise) =
  maybe-append (tag-byte 7)
    (maybe-append (render-term source)
      (maybe-append (render-term target) (render-step premise)))

render-trace : BaseQ0ReductionTraceWireV1 → WireMaybe (List Byte)
render-trace trace =
  maybe-append (render-term (trace-start trace))
    (maybe-append (u64-bytes-small (length (trace-steps trace)))
      (maybe-append
        (concat-rows (list-map render-step (trace-steps trace)))
        (render-term (trace-end trace))))

render-census-entry : NoRedexEntryWireV1 → WireMaybe (List Byte)
render-census-entry entry =
  maybe-append (u64-bytes-small (length (no-redex-path entry)))
    (maybe-append
      (concat-rows
        (list-map (λ component → tag-byte (path-component-tag component))
          (no-redex-path entry)))
      (maybe-append (render-term (no-redex-term entry))
        (tag-byte (disposition-tag (no-redex-disposition entry)))))

-- The kernel-normalized image of one conversion endpoint term under
-- the certificate's decoded context.
normalized-endpoint :
  (globals : Nat) (sig : PSig globals) →
  ProductionContextWireV1 → WireTermV1 → WireMaybe (List Byte)
normalized-endpoint globals sig context term =
  wire-maybe-bind
    (decode-context-checked globals (entries-oldest-first context))
    (λ decoded →
  wire-maybe-bind
    (decode-term-checked globals (decoded-locals decoded) term)
    (λ decoded-term →
  normalized-wire-term globals sig decoded-term))

render-conversion-endpoint :
  (globals : Nat) (sig : PSig globals) →
  ConversionCertificateWireV1 → WireMaybe (List Byte)
render-conversion-endpoint globals sig certificate
  with conversion-endpoint certificate
... | endpoint-has-type expected =
  maybe-append (tag-byte 0) (render-term expected)
... | endpoint-type-formation =
  maybe-append (tag-byte 1)
    (wire-maybe-bind
      (decode-context-checked globals
        (entries-oldest-first (conversion-context certificate)))
      (λ decoded →
    wire-maybe-bind
      (decode-term-checked globals (decoded-locals decoded)
        (conversion-common-normal-form certificate))
      (λ common →
    wire-maybe-bind
      (term-formation-level globals sig (decoded-pctx decoded) common)
      u16-bytes-small)))

render-conversion :
  (globals : Nat) (sig : PSig globals) →
  ConversionCertificateWireV1 → WireMaybe (List Byte)
render-conversion globals sig certificate =
  maybe-append (render-id (conversion-id certificate))
    (maybe-append
      (render-context-entries (conversion-context certificate))
      (maybe-append (render-term (conversion-left certificate))
        (maybe-append (render-term (conversion-right certificate))
          (maybe-append
            (render-conversion-endpoint globals sig certificate)
            (maybe-append
              (render-term (conversion-common-normal-form certificate))
              (maybe-append
                (normalized-endpoint globals sig
                  (conversion-context certificate)
                  (conversion-left certificate))
                (maybe-append
                  (normalized-endpoint globals sig
                    (conversion-context certificate)
                    (conversion-right certificate))
                  (maybe-append
                    (render-trace (conversion-left-trace certificate))
                    (maybe-append
                      (render-trace
                        (conversion-right-trace certificate))
                      (maybe-append
                        (u64-bytes-small
                          (length (conversion-census certificate)))
                        (concat-rows
                          (list-map render-census-entry
                            (conversion-census certificate)))))))))))))

render-conversions-section :
  (globals : Nat) (sig : PSig globals) →
  List ConversionCertificateWireV1 → WireMaybe (List Byte)
render-conversions-section globals sig conversions =
  maybe-append (u64-bytes-small (length conversions))
    (concat-rows (list-map (render-conversion globals sig) conversions))

-- --- Synthesis replay helpers -----------------------------------------

-- The replayed synthesis result of one wire certificate in its own
-- decoded context.
replay-certificate-result :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1) →
  SynthesisCertificateWireV1 → WireMaybe (List Byte × List Byte)
replay-certificate-result globals enabled sig conversions certificate =
  wire-maybe-bind
    (decode-context-checked globals
      (entries-oldest-first (synthesis-context certificate)))
    (λ decoded →
  wire-maybe-bind
    (psynthesize-cert globals enabled sig conversions typing-fuel-v1
      (decoded-pctx decoded) (synthesis-code certificate))
    (λ result →
  wire-maybe-bind
    (render-term (ptm-to-wire (result-type result)))
    (λ type-bytes →
  wire-maybe-bind
    (render-results
      (collect-dependent-results (result-derivation result)))
    (λ results-bytes →
  wire-just (type-bytes , results-bytes)))))
  where
  collect-dependent-results :
    {enabled′ : List Nat} {sig′ : PSig globals} {locals : Nat}
    {context : PCtx globals locals} {term ty : PTm globals locals} →
    PSynthesisDerivationV1 enabled′ sig′ context term ty →
    List WireTermV1
  collect-dependent-results (psynth-sort level) = []
  collect-dependent-results psynth-unit-type = []
  collect-dependent-results psynth-unit = []
  collect-dependent-results (psynth-variable x) = []
  collect-dependent-results (psynth-global slot) = []
  collect-dependent-results
    (psynth-pi parameter-derivation _ body-derivation) =
    collect-dependent-results parameter-derivation ++
    collect-dependent-results body-derivation
  collect-dependent-results
    (psynth-lambda parameter-derivation _ body-derivation) =
    collect-dependent-results parameter-derivation ++
    collect-dependent-results body-derivation
  collect-dependent-results
    (psynth-application {result-type = result}
      function-derivation argument-derivation _ _ _ _) =
    ptm-to-wire result ∷
    (collect-dependent-results function-derivation ++
     collect-dependent-results argument-derivation)
  render-results : List WireTermV1 → WireMaybe (List Byte)
  render-results results =
    maybe-append (u64-bytes-small (length results))
      (concat-rows (list-map render-term results))

-- --- Section 4: supplements -------------------------------------------

find-conversion-by-id :
  List ConversionCertificateWireV1 → WireIdV1 →
  WireMaybe ConversionCertificateWireV1
find-conversion-by-id [] identifier = wire-nothing
find-conversion-by-id (certificate ∷ certificates) identifier =
  found-branch
    (LawV2.Wire.ContextChecker.byte-list-equal
      (conversion-id certificate) identifier)
  where
  found-branch : Bool → WireMaybe ConversionCertificateWireV1
  found-branch true = wire-just certificate
  found-branch false = find-conversion-by-id certificates identifier

render-supplement :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1) →
  ConversionTypingSupplementWireV1 → WireMaybe (List Byte)
render-supplement globals enabled sig conversions supplement =
  wire-maybe-bind
    (find-conversion-by-id conversions
      (supplement-conversion-id supplement))
    (λ certificate →
  wire-maybe-bind
    (resolve-site certificate (supplement-step-path supplement))
    (λ site →
  wire-maybe-bind
    (replay-certificate-result globals enabled sig conversions
      (supplement-source-code supplement))
    (λ source-result →
  wire-maybe-bind
    (replay-certificate-result globals enabled sig conversions
      (supplement-target-code supplement))
    (λ target-result →
  maybe-append (render-id (supplement-conversion-id supplement))
    (maybe-append
      (u64-bytes-small (length (supplement-step-path supplement)))
      (maybe-append
        (concat-rows
          (list-map u32-bytes-small
            (supplement-step-path supplement)))
        (maybe-append
          (u64-bytes-small (length (site-context-entries site)))
          (maybe-append
            (concat-rows
              (list-map render-term (site-context-entries site)))
            (maybe-append (render-term (site-source site))
              (maybe-append (render-term (site-target site))
                (maybe-append
                  (render-supplement-endpoint
                    (supplement-endpoint supplement))
                  (maybe-append
                    (render-formation-level
                      (supplement-formation-level supplement))
                    (maybe-append
                      (render-id
                        (synthesis-id
                          (supplement-source-code supplement)))
                      (maybe-append (wire-just (first source-result))
                        (maybe-append
                          (render-id
                            (synthesis-id
                              (supplement-target-code supplement)))
                          (wire-just
                            (first target-result)))))))))))))))))
  where
  render-supplement-endpoint :
    EndpointJudgmentWireV1 → WireMaybe (List Byte)
  render-supplement-endpoint (endpoint-has-type expected) =
    maybe-append (tag-byte 0) (render-term expected)
  render-supplement-endpoint endpoint-type-formation = tag-byte 1
  render-formation-level : WireMaybe Nat → WireMaybe (List Byte)
  render-formation-level wire-nothing = tag-byte 0
  render-formation-level (wire-just level) =
    maybe-append (tag-byte 1) (u16-bytes-small level)

render-supplements-section :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1) →
  List ConversionTypingSupplementWireV1 → WireMaybe (List Byte)
render-supplements-section globals enabled sig conversions
  supplements =
  maybe-append (u64-bytes-small (length supplements))
    (concat-rows
      (list-map (render-supplement globals enabled sig conversions)
        supplements))

-- --- Section 5: synthesis ---------------------------------------------

render-code : SynthesisCodeWireV1 → WireMaybe (List Byte)
render-code (code-sort level) =
  maybe-append (tag-byte 0) (u16-bytes-small level)
render-code code-unit-type = tag-byte 1
render-code code-unit = tag-byte 2
render-code (code-variable-lookup index ordinal shift) =
  maybe-append (tag-byte 3)
    (maybe-append (u32-bytes-small index)
      (maybe-append (u32-bytes-small ordinal) (u32-bytes-small shift)))
render-code (code-global-lookup slot) =
  maybe-append (tag-byte 4) (u32-bytes-small slot)
render-code (code-pi-formation parameter body) =
  maybe-append (tag-byte 5)
    (maybe-append (render-code parameter) (render-code body))
render-code (code-lambda-introduction parameter body) =
  maybe-append (tag-byte 6)
    (maybe-append (render-code parameter) (render-code body))
render-code
  (code-application-elimination function argument
    function-conversion argument-conversion dependent-result) =
  maybe-append (tag-byte 7)
    (maybe-append (render-code function)
      (maybe-append (render-code argument)
        (maybe-append (render-id function-conversion)
          (maybe-append (render-id argument-conversion)
            (render-term dependent-result)))))

render-synthesis :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1) →
  SynthesisCertificateWireV1 → WireMaybe (List Byte)
render-synthesis globals enabled sig conversions certificate =
  wire-maybe-bind
    (replay-certificate-result globals enabled sig conversions
      certificate)
    (λ result →
  maybe-append (render-id (synthesis-id certificate))
    (maybe-append
      (render-context-entries (synthesis-context certificate))
      (maybe-append (render-term (synthesis-subject certificate))
        (maybe-append (wire-just (first result))
          (maybe-append (render-code (synthesis-code certificate))
            (wire-just (second result)))))))

render-synthesis-section :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1) →
  List SynthesisCertificateWireV1 → WireMaybe (List Byte)
render-synthesis-section globals enabled sig conversions
  certificates =
  maybe-append (u64-bytes-small (length certificates))
    (concat-rows
      (list-map (render-synthesis globals enabled sig conversions)
        certificates))

-- --- Sections 6-8: inventories ----------------------------------------

category-tag : ProductionQ0CategoryV1 → Nat
category-tag representation = 0
category-tag base-semantic = 1
category-tag runtime-public = 2

render-q0-row : Nat → WireMaybe (List Byte)
render-q0-row tag =
  maybe-append (tag-byte tag)
    (wire-maybe-bind (q0-tag-to-abstract tag) (λ abstract-tag →
      tag-byte (category-tag (q0-category abstract-tag))))

render-q0-section : List Nat → WireMaybe (List Byte)
render-q0-section rules =
  maybe-append (u64-bytes-small (length rules))
    (concat-rows (list-map render-q0-row rules))

render-fresh-rule :
  (globals : Nat) (sig : PSig globals) →
  FreshRuleSchemaWireV1 → WireMaybe (List Byte)
render-fresh-rule globals sig schema =
  maybe-append (render-id (fresh-equation-id schema))
    (maybe-append (u32-bytes-small (fresh-owner-slot schema))
      (maybe-append (u32-bytes-small (fresh-constructor-slot schema))
        (maybe-append (u16-bytes-small (fresh-arity schema))
          (maybe-append
            (u32-bytes-small (fresh-scrutinee-ordinal schema))
            (maybe-append
              (render-context-entries (fresh-parameter-context schema))
              (maybe-append (render-term (fresh-left schema))
                (maybe-append (render-term (fresh-right schema))
                  (maybe-append (render-term (fresh-type schema))
                    (wire-maybe-bind
                      (fresh-type-level)
                      u16-bytes-small)))))))))
  where
  fresh-type-level : WireMaybe Nat
  fresh-type-level =
    wire-maybe-bind
      (decode-context-checked globals
        (entries-oldest-first (fresh-parameter-context schema)))
      (λ decoded →
    wire-maybe-bind
      (decode-term-checked globals (decoded-locals decoded)
        (fresh-type schema))
      (λ ty →
    term-formation-level globals sig (decoded-pctx decoded) ty))

render-fresh-section :
  (globals : Nat) (sig : PSig globals) →
  List FreshRuleSchemaWireV1 → WireMaybe (List Byte)
render-fresh-section globals sig schemas =
  maybe-append (u64-bytes-small (length schemas))
    (concat-rows (list-map (render-fresh-rule globals sig) schemas))

render-family-normalized-type :
  (globals : Nat) (sig : PSig globals) →
  FamilyJudgmentWireV1 → WireMaybe (List Byte)
render-family-normalized-type globals sig judgment =
  wire-maybe-bind
    (decode-context-checked globals
      (entries-oldest-first (family-judgment-context judgment)))
    (λ decoded →
  wire-maybe-bind
    (decode-term-checked globals (decoded-locals decoded)
      (family-judgment-type judgment))
    (λ ty →
  normalized-wire-term globals sig ty))

render-family-judgment-surface :
  FamilyJudgmentWireV1 → WireMaybe (List Byte)
render-family-judgment-surface judgment =
  maybe-append
    (render-context-entries (family-judgment-context judgment))
    (maybe-append (render-term (family-judgment-subject judgment))
      (render-term (family-judgment-type judgment)))

render-family :
  (globals : Nat) (sig : PSig globals) →
  FamilyPayloadWireV1 → WireMaybe (List Byte)
render-family globals sig (family-seed family source judgment) =
  maybe-append (tag-byte 0)
    (maybe-append (render-id family)
      (maybe-append (render-seed-source source)
        (maybe-append (render-family-judgment-surface judgment)
          (render-family-normalized-type globals sig judgment))))
  where
  render-seed-source : SeedSourceWireV1 → WireMaybe (List Byte)
  render-seed-source (seed-public-head owner) =
    maybe-append (tag-byte 0) (u32-bytes-small owner)
  render-seed-source (seed-public-equation equation) =
    maybe-append (tag-byte 1) (render-id equation)
render-family globals sig
  (family-generic-public-application family function argument
    judgment) =
  maybe-append (tag-byte 1)
    (maybe-append (render-id family)
      (maybe-append (render-id function)
        (maybe-append (render-id argument)
          (maybe-append (render-family-judgment-surface judgment)
            (render-family-normalized-type globals sig judgment)))))
render-family globals sig
  (family-generic-equation-action family equation source judgment) =
  maybe-append (tag-byte 2)
    (maybe-append (render-id family)
      (maybe-append (render-id equation)
        (maybe-append (render-id source)
          (maybe-append (render-family-judgment-surface judgment)
            (render-family-normalized-type globals sig judgment)))))

render-families-section :
  (globals : Nat) (sig : PSig globals) →
  List FamilyPayloadWireV1 → WireMaybe (List Byte)
render-families-section globals sig payloads =
  maybe-append (u64-bytes-small (length payloads))
    (concat-rows (list-map (render-family globals sig) payloads))

-- --- The whole transcript ---------------------------------------------

transcript-magic : List Nat
transcript-magic =
  80 ∷ 69 ∷ 78 ∷ 45 ∷ 80 ∷ 82 ∷ 79 ∷ 68 ∷ 45 ∷ 84 ∷ 82 ∷ 65 ∷
  78 ∷ 45 ∷ 86 ∷ 49 ∷ []

render-canonical-transcript :
  ProductionBundleSemanticV1 → WireMaybe (List Byte)
render-canonical-transcript bundle =
  slots-branch (check-slots-from zero (bundle-global-slots bundle))
    refl
  where
  slots-branch :
    (result : Bool) →
    check-slots-from zero (bundle-global-slots bundle) ≡ result →
    WireMaybe (List Byte)
  slots-branch false proof = wire-nothing
  slots-branch true proof =
    maybe-append (bytes-from-nats transcript-magic)
      (maybe-append (u16-bytes-small 1)
        (maybe-append
          (render-signature-section globals sig
            (bundle-global-slots bundle))
          (maybe-append
            (render-contexts-section globals sig
              (bundle-contexts bundle))
            (maybe-append
              (render-conversions-section globals sig
                (bundle-conversions bundle))
              (maybe-append
                (render-supplements-section globals enabled sig
                  (bundle-conversions bundle)
                  (bundle-supplements bundle))
                (maybe-append
                  (render-synthesis-section globals enabled sig
                    (bundle-conversions bundle)
                    (bundle-synthesis bundle))
                  (maybe-append
                    (render-q0-section (bundle-q0-rules bundle))
                    (maybe-append
                      (render-fresh-section globals sig
                        (bundle-fresh-rules bundle))
                      (render-families-section globals sig
                        (bundle-family-payloads bundle))))))))))
    where
    globals : Nat
    globals = total-from zero (bundle-global-slots bundle)
    sig : PSig globals
    sig = wire-slots-to-psig (bundle-global-slots bundle) proof
    enabled : List Nat
    enabled =
      delta-policy-slots
        (allowed-transparent-deltas (bundle-signature bundle))
