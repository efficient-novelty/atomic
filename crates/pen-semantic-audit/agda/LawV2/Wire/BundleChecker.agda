{-# OPTIONS --safe --without-K #-}

-- Structural checking for the complete eleven-section production bundle.
--
-- This module mirrors `pen-production-wire` `validate.rs` over the decoded
-- semantic bundle. On any bundle produced by `decode-production-bundle-v1`
-- the Boolean verdict here is intended to equal the Rust validator verdict
-- on the same bytes; it deliberately proves nothing about kernel typing,
-- conversion soundness, or synthesis soundness, which belong to the later
-- correspondence phases.

module LawV2.Wire.BundleChecker where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe using (Maybe; just; nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc; _-_)
open import LawV2.Wire.Bytes
open import LawV2.Wire.ProductionBundleV1
open import LawV2.Wire.ContextChecker

_or_ : Bool → Bool → Bool
true or right = true
false or right = right

max-context-entries-v1 : Nat
max-context-entries-v1 = 32

-- Exact structural equality of wire terms.
wire-term-equal : WireTermV1 → WireTermV1 → Bool
wire-term-equal (wire-sort left) (wire-sort right) = nat-equal left right
wire-term-equal (wire-variable left) (wire-variable right) =
  nat-equal left right
wire-term-equal (wire-global-slot left) (wire-global-slot right) =
  nat-equal left right
wire-term-equal (wire-pi left-parameter left-body)
  (wire-pi right-parameter right-body) =
  wire-term-equal left-parameter right-parameter and
  wire-term-equal left-body right-body
wire-term-equal (wire-lambda left-parameter left-body)
  (wire-lambda right-parameter right-body) =
  wire-term-equal left-parameter right-parameter and
  wire-term-equal left-body right-body
wire-term-equal (wire-apply left-function left-argument)
  (wire-apply right-function right-argument) =
  wire-term-equal left-function right-function and
  wire-term-equal left-argument right-argument
wire-term-equal wire-unit-type wire-unit-type = true
wire-term-equal wire-unit wire-unit = true
wire-term-equal _ _ = false

wire-term-list-equal : List WireTermV1 → List WireTermV1 → Bool
wire-term-list-equal [] [] = true
wire-term-list-equal [] (_ ∷ _) = false
wire-term-list-equal (_ ∷ _) [] = false
wire-term-list-equal (left ∷ lefts) (right ∷ rights) =
  wire-term-equal left right and wire-term-list-equal lefts rights

wire-context-equal :
  ProductionContextWireV1 → ProductionContextWireV1 → Bool
wire-context-equal left right =
  wire-term-list-equal
    (entries-oldest-first left)
    (entries-oldest-first right)

-- Checker-produced terms admit sort levels {0,1,2}; public terms reuse
-- `term-scoped` from the context checker, which admits {0,1}.
checker-term-scoped : Nat → Nat → WireTermV1 → Bool
checker-term-scoped globals locals (wire-sort level) = nat-less level 3
checker-term-scoped globals locals (wire-variable index) =
  nat-less index locals
checker-term-scoped globals locals (wire-global-slot slot) =
  nat-less slot globals
checker-term-scoped globals locals (wire-pi parameter body) =
  checker-term-scoped globals locals parameter and
  checker-term-scoped globals (suc locals) body
checker-term-scoped globals locals (wire-lambda parameter body) =
  checker-term-scoped globals locals parameter and
  checker-term-scoped globals (suc locals) body
checker-term-scoped globals locals (wire-apply function argument) =
  checker-term-scoped globals locals function and
  checker-term-scoped globals locals argument
checker-term-scoped globals locals wire-unit-type = true
checker-term-scoped globals locals wire-unit = true

-- Every context surface is public and capped at 32 entries, mirroring
-- `MAX_CONTEXT_ENTRIES_V1` in `validate.rs`.
context-within-cap : ProductionContextWireV1 → Bool
context-within-cap context =
  not
    (nat-less max-context-entries-v1
      (length (entries-oldest-first context)))

check-public-context : Nat → ProductionContextWireV1 → Bool
check-public-context globals context =
  context-within-cap context and
  check-context-entries globals zero (entries-oldest-first context)

check-public-contexts : Nat → List ProductionContextWireV1 → Bool
check-public-contexts globals [] = true
check-public-contexts globals (context ∷ contexts) =
  check-public-context globals context and
  check-public-contexts globals contexts

context-local-count : ProductionContextWireV1 → Nat
context-local-count context = length (entries-oldest-first context)

list-within-limit : {A : Set} → List A → Bool
list-within-limit values =
  not (nat-less max-sequence-items-v1 (length values))

nat-member : Nat → List Nat → Bool
nat-member value [] = false
nat-member value (candidate ∷ candidates) =
  nat-equal value candidate or nat-member value candidates

id-list-member : WireIdV1 → List WireIdV1 → Bool
id-list-member identifier [] = false
id-list-member identifier (candidate ∷ candidates) =
  byte-list-equal identifier candidate or
  id-list-member identifier candidates

delta-slot-list : List DeltaPolicyEntryWireV1 → List Nat
delta-slot-list [] = []
delta-slot-list (delta ∷ deltas) =
  delta-global-slot delta ∷ delta-slot-list deltas

conversion-id-list : List ConversionCertificateWireV1 → List WireIdV1
conversion-id-list [] = []
conversion-id-list (certificate ∷ certificates) =
  conversion-id certificate ∷ conversion-id-list certificates

fresh-equation-id-list : List FreshRuleSchemaWireV1 → List WireIdV1
fresh-equation-id-list [] = []
fresh-equation-id-list (schema ∷ schemas) =
  fresh-equation-id schema ∷ fresh-equation-id-list schemas

-- Reduction steps: source/target scoping, transparent-delta slot range,
-- and premise recursion with binder-local weakening for body congruences.
step-source : BaseQ0ReductionStepWireV1 → WireTermV1
step-source (step-beta source _) = source
step-source (step-transparent-delta source _ _) = source
step-source (step-pi-parameter-congruence source _ _) = source
step-source (step-pi-body-congruence source _ _) = source
step-source (step-lambda-parameter-congruence source _ _) = source
step-source (step-lambda-body-congruence source _ _) = source
step-source (step-apply-function-congruence source _ _) = source
step-source (step-apply-argument-congruence source _ _) = source

step-target : BaseQ0ReductionStepWireV1 → WireTermV1
step-target (step-beta _ target) = target
step-target (step-transparent-delta _ target _) = target
step-target (step-pi-parameter-congruence _ target _) = target
step-target (step-pi-body-congruence _ target _) = target
step-target (step-lambda-parameter-congruence _ target _) = target
step-target (step-lambda-body-congruence _ target _) = target
step-target (step-apply-function-congruence _ target _) = target
step-target (step-apply-argument-congruence _ target _) = target

check-reduction-step : Nat → Nat → BaseQ0ReductionStepWireV1 → Bool
check-reduction-step globals locals (step-beta source target) =
  checker-term-scoped globals locals source and
  checker-term-scoped globals locals target
check-reduction-step globals locals
  (step-transparent-delta source target slot) =
  checker-term-scoped globals locals source and
  (checker-term-scoped globals locals target and
   nat-less slot globals)
check-reduction-step globals locals
  (step-pi-parameter-congruence source target premise) =
  checker-term-scoped globals locals source and
  (checker-term-scoped globals locals target and
   check-reduction-step globals locals premise)
check-reduction-step globals locals
  (step-pi-body-congruence source target premise) =
  checker-term-scoped globals locals source and
  (checker-term-scoped globals locals target and
   check-reduction-step globals (suc locals) premise)
check-reduction-step globals locals
  (step-lambda-parameter-congruence source target premise) =
  checker-term-scoped globals locals source and
  (checker-term-scoped globals locals target and
   check-reduction-step globals locals premise)
check-reduction-step globals locals
  (step-lambda-body-congruence source target premise) =
  checker-term-scoped globals locals source and
  (checker-term-scoped globals locals target and
   check-reduction-step globals (suc locals) premise)
check-reduction-step globals locals
  (step-apply-function-congruence source target premise) =
  checker-term-scoped globals locals source and
  (checker-term-scoped globals locals target and
   check-reduction-step globals locals premise)
check-reduction-step globals locals
  (step-apply-argument-congruence source target premise) =
  checker-term-scoped globals locals source and
  (checker-term-scoped globals locals target and
   check-reduction-step globals locals premise)

check-step-chain :
  Nat → Nat → WireTermV1 → List BaseQ0ReductionStepWireV1 → WireTermV1 →
  Bool
check-step-chain globals locals expected [] end =
  wire-term-equal expected end
check-step-chain globals locals expected (step ∷ steps) end =
  wire-term-equal (step-source step) expected and
  (check-reduction-step globals locals step and
   check-step-chain globals locals (step-target step) steps end)

check-reduction-trace : Nat → Nat → BaseQ0ReductionTraceWireV1 → Bool
check-reduction-trace globals locals trace =
  checker-term-scoped globals locals (trace-start trace) and
  (checker-term-scoped globals locals (trace-end trace) and
   check-step-chain globals locals
     (trace-start trace) (trace-steps trace) (trace-end trace))

check-endpoint-judgment : Nat → Nat → EndpointJudgmentWireV1 → Bool
check-endpoint-judgment globals locals (endpoint-has-type expected) =
  checker-term-scoped globals locals expected
check-endpoint-judgment globals locals endpoint-type-formation = true

-- Complete no-redex census recomputation of the common normal form
-- relative to the enabled transparent-delta slots.
is-lambda-term : WireTermV1 → Bool
is-lambda-term (wire-lambda _ _) = true
is-lambda-term _ = false

global-census-branch :
  Bool → List ConversionPathComponentWireV1 → Nat →
  Maybe (List NoRedexEntryWireV1)
global-census-branch true path slot = nothing
global-census-branch false path slot =
  just
    (no-redex-entry-v1 path (wire-global-slot slot)
      disposition-global-not-enabled ∷ [])

apply-census-branch :
  Bool → NoRedexEntryWireV1 → List NoRedexEntryWireV1 →
  List NoRedexEntryWireV1 → Maybe (List NoRedexEntryWireV1)
apply-census-branch true entry function-entries argument-entries = nothing
apply-census-branch false entry function-entries argument-entries =
  just (entry ∷ (function-entries ++ argument-entries))

expected-census-from :
  List Nat → List ConversionPathComponentWireV1 → WireTermV1 →
  Maybe (List NoRedexEntryWireV1)
expected-census-from enabled path (wire-sort level) =
  just (no-redex-entry-v1 path (wire-sort level) disposition-sort ∷ [])
expected-census-from enabled path (wire-variable index) =
  just
    (no-redex-entry-v1 path (wire-variable index) disposition-variable ∷ [])
expected-census-from enabled path (wire-global-slot slot) =
  global-census-branch (nat-member slot enabled) path slot
expected-census-from enabled path (wire-pi parameter body) =
  maybe-bind
    (expected-census-from enabled (path ++ (path-pi-parameter ∷ []))
      parameter)
    (λ parameter-entries →
  maybe-bind
    (expected-census-from enabled (path ++ (path-pi-body ∷ [])) body)
    (λ body-entries →
  just
    (no-redex-entry-v1 path (wire-pi parameter body) disposition-pi ∷
     (parameter-entries ++ body-entries))))
expected-census-from enabled path (wire-lambda parameter body) =
  maybe-bind
    (expected-census-from enabled (path ++ (path-lambda-parameter ∷ []))
      parameter)
    (λ parameter-entries →
  maybe-bind
    (expected-census-from enabled (path ++ (path-lambda-body ∷ [])) body)
    (λ body-entries →
  just
    (no-redex-entry-v1 path (wire-lambda parameter body)
      disposition-lambda ∷
     (parameter-entries ++ body-entries))))
expected-census-from enabled path (wire-apply function argument) =
  maybe-bind
    (expected-census-from enabled (path ++ (path-apply-function ∷ []))
      function)
    (λ function-entries →
  maybe-bind
    (expected-census-from enabled (path ++ (path-apply-argument ∷ []))
      argument)
    (λ argument-entries →
  apply-census-branch (is-lambda-term function)
    (no-redex-entry-v1 path (wire-apply function argument)
      disposition-neutral-application)
    function-entries argument-entries))
expected-census-from enabled path wire-unit-type =
  just (no-redex-entry-v1 path wire-unit-type disposition-unit-type ∷ [])
expected-census-from enabled path wire-unit =
  just (no-redex-entry-v1 path wire-unit disposition-unit ∷ [])

path-component-equal :
  ConversionPathComponentWireV1 → ConversionPathComponentWireV1 → Bool
path-component-equal path-pi-parameter path-pi-parameter = true
path-component-equal path-pi-body path-pi-body = true
path-component-equal path-lambda-parameter path-lambda-parameter = true
path-component-equal path-lambda-body path-lambda-body = true
path-component-equal path-apply-function path-apply-function = true
path-component-equal path-apply-argument path-apply-argument = true
path-component-equal _ _ = false

path-equal :
  List ConversionPathComponentWireV1 →
  List ConversionPathComponentWireV1 → Bool
path-equal [] [] = true
path-equal [] (_ ∷ _) = false
path-equal (_ ∷ _) [] = false
path-equal (left ∷ lefts) (right ∷ rights) =
  path-component-equal left right and path-equal lefts rights

disposition-equal :
  NoRedexDispositionWireV1 → NoRedexDispositionWireV1 → Bool
disposition-equal disposition-sort disposition-sort = true
disposition-equal disposition-variable disposition-variable = true
disposition-equal disposition-global-not-enabled
  disposition-global-not-enabled = true
disposition-equal disposition-pi disposition-pi = true
disposition-equal disposition-lambda disposition-lambda = true
disposition-equal disposition-neutral-application
  disposition-neutral-application = true
disposition-equal disposition-unit-type disposition-unit-type = true
disposition-equal disposition-unit disposition-unit = true
disposition-equal _ _ = false

census-entry-equal : NoRedexEntryWireV1 → NoRedexEntryWireV1 → Bool
census-entry-equal left right =
  path-equal (no-redex-path left) (no-redex-path right) and
  (wire-term-equal (no-redex-term left) (no-redex-term right) and
   disposition-equal
     (no-redex-disposition left) (no-redex-disposition right))

census-equal : List NoRedexEntryWireV1 → List NoRedexEntryWireV1 → Bool
census-equal [] [] = true
census-equal [] (_ ∷ _) = false
census-equal (_ ∷ _) [] = false
census-equal (left ∷ lefts) (right ∷ rights) =
  census-entry-equal left right and census-equal lefts rights

census-matches-branch :
  Maybe (List NoRedexEntryWireV1) → List NoRedexEntryWireV1 → Bool
census-matches-branch nothing supplied = false
census-matches-branch (just expected) supplied =
  census-equal supplied expected

check-census : List Nat → WireTermV1 → List NoRedexEntryWireV1 → Bool
check-census enabled normal-form supplied =
  census-matches-branch
    (expected-census-from enabled [] normal-form) supplied

check-conversion-certificate :
  Nat → List Nat → ConversionCertificateWireV1 → Bool
check-conversion-certificate globals enabled certificate =
  id-is-32-bytes (conversion-id certificate) and
  (check-public-context globals (conversion-context certificate) and
  (checker-term-scoped globals locals (conversion-left certificate) and
  (checker-term-scoped globals locals (conversion-right certificate) and
  (checker-term-scoped globals locals common and
  (check-endpoint-judgment globals locals
    (conversion-endpoint certificate) and
  (check-reduction-trace globals locals left-trace and
  (check-reduction-trace globals locals right-trace and
  (wire-term-equal (trace-start left-trace)
    (conversion-left certificate) and
  (wire-term-equal (trace-start right-trace)
    (conversion-right certificate) and
  (wire-term-equal (trace-end left-trace) common and
  (wire-term-equal (trace-end right-trace) common and
   check-census enabled common (conversion-census certificate))))))))))))
  where
  locals = context-local-count (conversion-context certificate)
  common = conversion-common-normal-form certificate
  left-trace = conversion-left-trace certificate
  right-trace = conversion-right-trace certificate

check-conversions-from :
  Nat → List Nat → List WireIdV1 → List ConversionCertificateWireV1 → Bool
check-conversions-from globals enabled seen [] = true
check-conversions-from globals enabled seen
  (certificate ∷ certificates) =
  not (id-list-member (conversion-id certificate) seen) and
  (check-conversion-certificate globals enabled certificate and
   check-conversions-from globals enabled
     (conversion-id certificate ∷ seen) certificates)

-- Synthesis codes: exact variable-lookup metadata, universe bounds,
-- conversion references, and binder-local context growth.
check-synthesis-code :
  Nat → Nat → List WireIdV1 → SynthesisCodeWireV1 → Bool
check-synthesis-code globals locals conversions (code-sort level) =
  nat-less level 2
check-synthesis-code globals locals conversions code-unit-type = true
check-synthesis-code globals locals conversions code-unit = true
check-synthesis-code globals locals conversions
  (code-variable-lookup index ordinal shift) =
  nat-less index locals and
  (nat-equal ordinal (locals - suc index) and
   nat-equal shift (suc index))
check-synthesis-code globals locals conversions
  (code-global-lookup slot) =
  nat-less slot globals
check-synthesis-code globals locals conversions
  (code-pi-formation parameter body) =
  check-synthesis-code globals locals conversions parameter and
  check-synthesis-code globals (suc locals) conversions body
check-synthesis-code globals locals conversions
  (code-lambda-introduction parameter body) =
  check-synthesis-code globals locals conversions parameter and
  check-synthesis-code globals (suc locals) conversions body
check-synthesis-code globals locals conversions
  (code-application-elimination function argument
    function-conversion argument-conversion result) =
  id-list-member function-conversion conversions and
  (id-list-member argument-conversion conversions and
  (check-synthesis-code globals locals conversions function and
  (check-synthesis-code globals locals conversions argument and
   checker-term-scoped globals locals result)))

code-term-tag-matches : SynthesisCodeWireV1 → WireTermV1 → Bool
code-term-tag-matches (code-sort _) (wire-sort _) = true
code-term-tag-matches code-unit-type wire-unit-type = true
code-term-tag-matches code-unit wire-unit = true
code-term-tag-matches (code-variable-lookup _ _ _) (wire-variable _) = true
code-term-tag-matches (code-global-lookup _) (wire-global-slot _) = true
code-term-tag-matches (code-pi-formation _ _) (wire-pi _ _) = true
code-term-tag-matches (code-lambda-introduction _ _) (wire-lambda _ _) =
  true
code-term-tag-matches (code-application-elimination _ _ _ _ _)
  (wire-apply _ _) = true
code-term-tag-matches _ _ = false

check-synthesis-shape :
  SynthesisCodeWireV1 → WireTermV1 → WireTermV1 → Bool
check-synthesis-shape (code-sort level) (wire-sort actual) inferred =
  nat-equal level actual and
  wire-term-equal inferred (wire-sort (suc level))
check-synthesis-shape code-unit-type wire-unit-type inferred =
  wire-term-equal inferred (wire-sort zero)
check-synthesis-shape code-unit wire-unit inferred =
  wire-term-equal inferred wire-unit-type
check-synthesis-shape (code-variable-lookup index _ _)
  (wire-variable actual) inferred =
  nat-equal index actual
check-synthesis-shape (code-global-lookup slot)
  (wire-global-slot actual) inferred =
  nat-equal slot actual
check-synthesis-shape (code-pi-formation parameter body)
  (wire-pi subject-parameter subject-body) inferred =
  code-term-tag-matches parameter subject-parameter and
  code-term-tag-matches body subject-body
check-synthesis-shape (code-lambda-introduction parameter body)
  (wire-lambda subject-parameter subject-body) inferred =
  code-term-tag-matches parameter subject-parameter and
  code-term-tag-matches body subject-body
check-synthesis-shape
  (code-application-elimination function argument _ _ result)
  (wire-apply subject-function subject-argument) inferred =
  wire-term-equal result inferred and
  (code-term-tag-matches function subject-function and
   code-term-tag-matches argument subject-argument)
check-synthesis-shape _ _ _ = false

check-synthesis-certificate :
  Nat → List WireIdV1 → SynthesisCertificateWireV1 → Bool
check-synthesis-certificate globals conversions certificate =
  id-is-32-bytes (synthesis-id certificate) and
  (check-public-context globals (synthesis-context certificate) and
  (term-scoped globals locals (synthesis-subject certificate) and
  (checker-term-scoped globals locals
    (synthesis-inferred-type certificate) and
  (check-synthesis-shape (synthesis-code certificate)
    (synthesis-subject certificate)
    (synthesis-inferred-type certificate) and
   check-synthesis-code globals locals conversions
     (synthesis-code certificate)))))
  where
  locals = context-local-count (synthesis-context certificate)

check-synthesis-from :
  Nat → List WireIdV1 → List WireIdV1 →
  List SynthesisCertificateWireV1 → Bool
check-synthesis-from globals conversions seen [] = true
check-synthesis-from globals conversions seen
  (certificate ∷ certificates) =
  not (id-list-member (synthesis-id certificate) seen) and
  (check-synthesis-certificate globals conversions certificate and
   check-synthesis-from globals conversions
     (synthesis-id certificate ∷ seen) certificates)

check-formation-level : Maybe Nat → Bool
check-formation-level nothing = true
check-formation-level (just level) = nat-less level 4

check-supplement :
  Nat → List WireIdV1 → ConversionTypingSupplementWireV1 → Bool
check-supplement globals conversions supplement =
  id-list-member (supplement-conversion-id supplement) conversions and
  (check-public-context globals
    (supplement-local-context supplement) and
  (check-formation-level (supplement-formation-level supplement) and
  (check-synthesis-certificate globals conversions
    (supplement-source-code supplement) and
  (check-synthesis-certificate globals conversions
    (supplement-target-code supplement) and
  (wire-context-equal
    (synthesis-context (supplement-source-code supplement))
    (supplement-local-context supplement) and
   wire-context-equal
     (synthesis-context (supplement-target-code supplement))
     (supplement-local-context supplement))))))

check-supplements :
  Nat → List WireIdV1 → List ConversionTypingSupplementWireV1 → Bool
check-supplements globals conversions [] = true
check-supplements globals conversions (supplement ∷ supplements) =
  check-supplement globals conversions supplement and
  check-supplements globals conversions supplements

check-q0-inventory : List Nat → Bool
check-q0-inventory rules =
  nat-list-equal rules (0 ∷ 1 ∷ 2 ∷ 3 ∷ 4 ∷ 5 ∷ 6 ∷ [])

-- Fresh rules: the exact left-linear non-recursive constructor pattern.
term-spine : WireTermV1 → List WireTermV1 → _×_ WireTermV1 (List WireTermV1)
term-spine (wire-apply function argument) arguments =
  term-spine function (argument ∷ arguments)
term-spine head arguments = head , arguments

contains-global-slot : Nat → WireTermV1 → Bool
contains-global-slot target (wire-global-slot slot) =
  nat-equal slot target
contains-global-slot target (wire-pi parameter body) =
  contains-global-slot target parameter or
  contains-global-slot target body
contains-global-slot target (wire-lambda parameter body) =
  contains-global-slot target parameter or
  contains-global-slot target body
contains-global-slot target (wire-apply function argument) =
  contains-global-slot target function or
  contains-global-slot target argument
contains-global-slot target _ = false

check-pattern-arguments : Nat → Nat → List WireTermV1 → Bool
check-pattern-arguments constructor-slot zero (argument ∷ []) =
  wire-term-equal argument (wire-global-slot constructor-slot)
check-pattern-arguments constructor-slot (suc remaining)
  (argument ∷ rest) =
  wire-term-equal argument (wire-variable remaining) and
  check-pattern-arguments constructor-slot remaining rest
check-pattern-arguments constructor-slot _ _ = false

spine-pattern-matches :
  Nat → Nat → Nat → _×_ WireTermV1 (List WireTermV1) → Bool
spine-pattern-matches owner-slot constructor-slot scrutinee
  (head , arguments) =
  wire-term-equal head (wire-global-slot owner-slot) and
  check-pattern-arguments constructor-slot scrutinee arguments

check-fresh-pattern : FreshRuleSchemaWireV1 → Bool
check-fresh-pattern schema =
  spine-pattern-matches
    (fresh-owner-slot schema)
    (fresh-constructor-slot schema)
    (fresh-scrutinee-ordinal schema)
    (term-spine (fresh-left schema) []) and
  not (contains-global-slot (fresh-owner-slot schema) (fresh-right schema))

check-fresh-rule : Nat → FreshRuleSchemaWireV1 → Bool
check-fresh-rule globals schema =
  id-is-32-bytes (fresh-equation-id schema) and
  (nat-less (fresh-owner-slot schema) globals and
  (nat-less (fresh-constructor-slot schema) globals and
  (check-public-context globals (fresh-parameter-context schema) and
  (nat-less (fresh-scrutinee-ordinal schema) locals and
  (nat-equal (fresh-arity schema) locals and
  (term-scoped globals locals (fresh-left schema) and
  (term-scoped globals locals (fresh-right schema) and
  (term-scoped globals locals (fresh-type schema) and
  (nat-equal (suc (fresh-scrutinee-ordinal schema))
    (fresh-arity schema) and
   check-fresh-pattern schema)))))))))
  where
  locals = context-local-count (fresh-parameter-context schema)

check-fresh-rules-from :
  Nat → List WireIdV1 → List FreshRuleSchemaWireV1 → Bool
check-fresh-rules-from globals seen [] = true
check-fresh-rules-from globals seen (schema ∷ schemas) =
  not (id-list-member (fresh-equation-id schema) seen) and
  (check-fresh-rule globals schema and
   check-fresh-rules-from globals
     (fresh-equation-id schema ∷ seen) schemas)

check-family-inventory : List Nat → Bool
check-family-inventory codes = nat-list-equal codes (0 ∷ 1 ∷ 2 ∷ [])

check-family-judgment : Nat → FamilyJudgmentWireV1 → Bool
check-family-judgment globals judgment =
  check-public-context globals (family-judgment-context judgment) and
  (term-scoped globals locals (family-judgment-subject judgment) and
   term-scoped globals locals (family-judgment-type judgment))
  where
  locals = context-local-count (family-judgment-context judgment)

family-payload-id : FamilyPayloadWireV1 → WireIdV1
family-payload-id (family-seed family _ _) = family
family-payload-id (family-generic-public-application family _ _ _) =
  family
family-payload-id (family-generic-equation-action family _ _ _) = family

check-seed-source : Nat → List WireIdV1 → SeedSourceWireV1 → Bool
check-seed-source globals equations (seed-public-head owner) =
  nat-less owner globals
check-seed-source globals equations (seed-public-equation equation) =
  id-list-member equation equations

-- Family references must resolve to strictly earlier payloads, which the
-- `seen` accumulator captures exactly because family identifiers are
-- pairwise distinct.
check-family-payload :
  Nat → List WireIdV1 → List WireIdV1 → FamilyPayloadWireV1 → Bool
check-family-payload globals equations seen
  (family-seed family source judgment) =
  id-is-32-bytes family and
  (check-seed-source globals equations source and
   check-family-judgment globals judgment)
check-family-payload globals equations seen
  (family-generic-public-application family function-family
    argument-family judgment) =
  id-is-32-bytes family and
  (id-list-member function-family seen and
  (id-list-member argument-family seen and
   check-family-judgment globals judgment))
check-family-payload globals equations seen
  (family-generic-equation-action family equation source-family
    judgment) =
  id-is-32-bytes family and
  (id-list-member equation equations and
  (id-list-member source-family seen and
   check-family-judgment globals judgment))

check-family-payloads-from :
  Nat → List WireIdV1 → List WireIdV1 → List FamilyPayloadWireV1 → Bool
check-family-payloads-from globals equations seen [] = true
check-family-payloads-from globals equations seen
  (payload ∷ payloads) =
  not (id-list-member (family-payload-id payload) seen) and
  (check-family-payload globals equations seen payload and
   check-family-payloads-from globals equations
     (family-payload-id payload ∷ seen) payloads)

check-production-bundle-structure-v1 : ProductionBundleSemanticV1 → Bool
check-production-bundle-structure-v1 bundle =
  check-manifest-surface-v1 (bundle-manifest bundle) and
  (list-within-limit (bundle-global-slots bundle) and
  (list-within-limit (bundle-contexts bundle) and
  (list-within-limit (bundle-conversions bundle) and
  (list-within-limit (bundle-supplements bundle) and
  (list-within-limit (bundle-synthesis bundle) and
  (list-within-limit (bundle-fresh-rules bundle) and
  (list-within-limit (bundle-family-payloads bundle) and
  (ids-unique (bundle-global-slots bundle) and
  (check-slots-from zero (bundle-global-slots bundle) and
  (check-signature (bundle-signature bundle)
    (bundle-global-slots bundle) and
  (check-public-contexts globals (bundle-contexts bundle) and
  (check-conversions-from globals enabled []
    (bundle-conversions bundle) and
  (check-synthesis-from globals conversions []
    (bundle-synthesis bundle) and
  (check-supplements globals conversions
    (bundle-supplements bundle) and
  (check-q0-inventory (bundle-q0-rules bundle) and
  (check-fresh-rules-from globals [] (bundle-fresh-rules bundle) and
  (check-family-inventory (bundle-family-codes bundle) and
   check-family-payloads-from globals equations []
     (bundle-family-payloads bundle))))))))))))))))))
  where
  globals = length (bundle-global-slots bundle)
  enabled =
    delta-slot-list
      (allowed-transparent-deltas (bundle-signature bundle))
  conversions = conversion-id-list (bundle-conversions bundle)
  equations = fresh-equation-id-list (bundle-fresh-rules bundle)

record CheckedProductionBundleV1
  (bundle : ProductionBundleSemanticV1) : Set where
  constructor checked-production-bundle-v1
  field
    bundle-structure-accepted :
      check-production-bundle-structure-v1 bundle ≡ true

open CheckedProductionBundleV1 public

check-production-bundle-result-v1 :
  (bundle : ProductionBundleSemanticV1) →
  (result : Bool) →
  check-production-bundle-structure-v1 bundle ≡ result →
  Maybe (CheckedProductionBundleV1 bundle)
check-production-bundle-result-v1 bundle false accepted = nothing
check-production-bundle-result-v1 bundle true accepted =
  just (checked-production-bundle-v1 accepted)

check-production-bundle-v1 :
  (bundle : ProductionBundleSemanticV1) →
  Maybe (CheckedProductionBundleV1 bundle)
check-production-bundle-v1 bundle =
  check-production-bundle-result-v1 bundle
    (check-production-bundle-structure-v1 bundle) refl
