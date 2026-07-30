{-# OPTIONS --safe --without-K #-}

-- Phase F inventory correspondence: the Q0 inventory, fresh-rule
-- schemas, and family payloads of wire sections 8-11 receive their
-- exact semantic reading.
--
-- The structural checker pins only shapes: the exact seven-tag Q0
-- list, the exact fresh pattern (owner head applied to distinct
-- variables and the constructor in the scrutinee position, with an
-- owner-free right-hand side), the exact three-code family inventory,
-- and strictly-prior family references. This module adds the semantic
-- layer:
--
-- - the wire Q0 tags decode onto the abstract inventory codes
--   (`AbstractQ0TagV1`) with round trips in both directions, the
--   checker-accepted inventory decodes to exactly the seven abstract
--   rules in order, and the representation/base-semantic/
--   runtime-public classification of the decoded inventory is proven
--   as the exact category list; every abstract reduction step's Q0
--   tag is a member of the decoded inventory. This mechanizes the
--   Rust-enum erasure named by the
--   `rust-q0-tag-erasure-not-mechanized` frontier marker;
-- - every fresh-rule schema is replayed through the kernel-mirroring
--   typing layer (the parameter context verifies, the recorded type
--   forms a type, and both sides check against it), the owner and
--   constructor must be distinct bodyless declarations (a bodyful
--   owner would be erased by transparent delta, exactly as the kernel
--   normalizer forbids), the arity must be at least two (parameters
--   plus scrutinee, the normalizer's telescope discipline), and no
--   two rules may share an `(owner, constructor)` pair (the pairwise
--   pattern disjointness of the restricted-Q0 confluence
--   certificate). The decoded rule yields genuine abstract
--   `fresh-equation-step` instances for every match substitution,
--   with substitution stability inherited from `substitution-step`
--   and the composed-match equations from
--   `fresh-left-naturality`/`fresh-right-naturality`; the structural
--   owner-freeness of the right-hand side transports to the decoded
--   term. Constructor-set coverage (every constructor of an owner
--   datatype has a rule) is NOT stateable from the wire alone - the
--   wire carries no datatype registry - and remains an obligation of
--   the native-carrier phase; the coverage checked here is coverage
--   of the argument telescope by the pattern plus at-most-one rule
--   per `(owner, constructor)` pair;
-- - every family payload decodes exactly: seeds must judge their
--   public head (`PublicHead` seeds bind `global slot` at its exact
--   declared type; `PublicEquation` seeds bind the equation's owner
--   head, mirroring the Rust seed subject
--   `Equation { owner_head, .. }`), applications must judge the
--   application of their components' subjects in the components'
--   shared context (the wire image of the dropped `context_witness`),
--   and equation actions must preserve their source family's type.
--   Every judgment is replayed through the typing layer. The decoded
--   payloads build genuine `Family` values (equation references enter
--   as owner-head seeds), the wire payload tag equals the abstract
--   `family-code` of the decoded value - mechanizing the
--   `rust-family-payload-erasure-not-mechanized` frontier marker -
--   and family naturality transports: the decoded code is invariant
--   under every substitution of the decoded value, and the
--   identity/composition functor laws hold at decoded values.
--
-- Known registered gaps, not silently papered over: the wire drops
-- the Rust `hole_ordinal`/`context_witness` fields of
-- `GenericEquationAction`, so the rewrite-step relation between an
-- action's subject and its source's subject belongs to the Phase J
-- rewrite authority, not to this layer; and heterogeneous judgment
-- contexts across independent family trees decode per-tree at that
-- tree's context.
--
-- `inventory-check-bundle` is the whole-bundle inventory verdict. It
-- runs strictly after (and does not replace) the structural checker,
-- the semantic replay, and the typing verdict. Nothing here mints
-- correspondence authority: these are theorem packages consumed later
-- by the private factory after replay and transcript agreement.

module LawV2.Wire.InventoryReplayV1 where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe
  using ()
  renaming (Maybe to WireMaybe; just to wire-just; nothing to wire-nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.Wire.Bytes
  using (nat-equal; _and_; _×_; _,_)
open import LawV2.Wire.ProductionBundleV1
open import LawV2.Wire.ContextChecker
  using (not; nat-less; nat-list-equal; byte-list-equal;
         check-context-entries; check-slots-from)
open import LawV2.Wire.BundleChecker
  using (_or_; wire-term-equal; wire-term-list-equal;
         contains-global-slot; context-local-count; family-payload-id;
         check-q0-inventory; check-family-inventory)
open import LawV2.LambdaUnit.ProductionSyntaxV1
open import LawV2.LambdaUnit.SubstitutionReduction
  using (Step; beta-step; public-delta-step; fresh-equation-step;
         pi-parameter-congruence-step; pi-body-congruence-step;
         lambda-parameter-congruence-step;
         lambda-body-congruence-step;
         apply-function-congruence-step;
         apply-argument-congruence-step; substitution-step;
         fresh-left-naturality; fresh-right-naturality)
open import LawV2.LambdaUnit.FamilyNaturality
  using (Family; seed; generic-public-application;
         generic-equation-action; substitute-family; family-identity;
         family-composition; generic-public-application-naturality;
         generic-equation-action-naturality)
open import LawV2.LambdaUnit.ProductionInventoryBridgeV1
  using (AbstractQ0TagV1; q0-de-bruijn; q0-sequential-substitution;
         q0-beta; q0-provenance-preserving-delta; q0-unit;
         q0-telescope-flattening;
         q0-fresh-nonrecursive-constructor-computation;
         ProductionQ0CategoryV1; representation; base-semantic;
         runtime-public; q0-category; Q0ClassificationV1;
         q0-classification-total; abstract-step-q0-tag;
         ProductionFamilyCodeV1; family-code;
         FamilyClassificationV1; family-classification-total;
         family-code-substitution)
  renaming (family-seed to code-seed;
            family-generic-public-application
              to code-generic-public-application;
            family-generic-equation-action
              to code-generic-equation-action)
open import LawV2.Wire.ContextCorrespondenceV1
  using (PSig; total-from; wire-slots-to-psig; list-entry;
         wire-context-extend; wire-in-scope; wire-to-ptm;
         fin-from-less; and-first; and-second)
open import LawV2.Wire.SemanticReplayV1
  using (wire-maybe-bind; is-wire-just; decode-term-checked)
open import LawV2.Wire.TypingReplayV1
  using (pinfer; pcheck; pverify-context; pexpect-universe;
         typing-fuel-v1)

-- --- Boolean soundness helpers ---------------------------------------

nat-equal-sound :
  (left right : Nat) → nat-equal left right ≡ true → left ≡ right
nat-equal-sound zero zero proof = refl
nat-equal-sound zero (suc right) ()
nat-equal-sound (suc left) zero ()
nat-equal-sound (suc left) (suc right) proof =
  cong suc (nat-equal-sound left right proof)

nat-list-equal-sound :
  (left right : List Nat) → nat-list-equal left right ≡ true →
  left ≡ right
nat-list-equal-sound [] [] proof = refl
nat-list-equal-sound [] (y ∷ ys) ()
nat-list-equal-sound (x ∷ xs) [] ()
nat-list-equal-sound (x ∷ xs) (y ∷ ys) proof =
  cong₂ _∷_
    (nat-equal-sound x y
      (and-first (nat-equal x y) (nat-list-equal xs ys) proof))
    (nat-list-equal-sound xs ys
      (and-second (nat-equal x y) (nat-list-equal xs ys) proof))

or-first-false :
  (a b : Bool) → (a or b) ≡ false → a ≡ false
or-first-false false b proof = refl
or-first-false true b ()

or-second-false :
  (a b : Bool) → (a or b) ≡ false → b ≡ false
or-second-false false b proof = proof
or-second-false true b ()

wire-just-injective :
  {A : Set} {left right : A} →
  wire-just left ≡ wire-just right → left ≡ right
wire-just-injective refl = refl

-- --- Q0 inventory correspondence -------------------------------------

-- The wire byte tags 0..6 decode onto the abstract Q0 rule codes in
-- the exact `Q0RuleWireV1` declaration order.
q0-tag-to-abstract : Nat → WireMaybe AbstractQ0TagV1
q0-tag-to-abstract 0 = wire-just q0-de-bruijn
q0-tag-to-abstract 1 = wire-just q0-sequential-substitution
q0-tag-to-abstract 2 = wire-just q0-beta
q0-tag-to-abstract 3 = wire-just q0-provenance-preserving-delta
q0-tag-to-abstract 4 = wire-just q0-unit
q0-tag-to-abstract 5 = wire-just q0-telescope-flattening
q0-tag-to-abstract 6 =
  wire-just q0-fresh-nonrecursive-constructor-computation
q0-tag-to-abstract _ = wire-nothing

abstract-q0-to-tag : AbstractQ0TagV1 → Nat
abstract-q0-to-tag q0-de-bruijn = 0
abstract-q0-to-tag q0-sequential-substitution = 1
abstract-q0-to-tag q0-beta = 2
abstract-q0-to-tag q0-provenance-preserving-delta = 3
abstract-q0-to-tag q0-unit = 4
abstract-q0-to-tag q0-telescope-flattening = 5
abstract-q0-to-tag q0-fresh-nonrecursive-constructor-computation = 6

q0-tag-round-trip :
  (tag : AbstractQ0TagV1) →
  q0-tag-to-abstract (abstract-q0-to-tag tag) ≡ wire-just tag
q0-tag-round-trip q0-de-bruijn = refl
q0-tag-round-trip q0-sequential-substitution = refl
q0-tag-round-trip q0-beta = refl
q0-tag-round-trip q0-provenance-preserving-delta = refl
q0-tag-round-trip q0-unit = refl
q0-tag-round-trip q0-telescope-flattening = refl
q0-tag-round-trip q0-fresh-nonrecursive-constructor-computation = refl

abstract-q0-to-tag-injective :
  (left right : AbstractQ0TagV1) →
  abstract-q0-to-tag left ≡ abstract-q0-to-tag right → left ≡ right
abstract-q0-to-tag-injective left right proof =
  wire-just-injective
    (trans (sym (q0-tag-round-trip left))
      (trans (cong q0-tag-to-abstract proof)
        (q0-tag-round-trip right)))

exact-abstract-q0-inventory-v1 : List AbstractQ0TagV1
exact-abstract-q0-inventory-v1 =
  q0-de-bruijn ∷
  q0-sequential-substitution ∷
  q0-beta ∷
  q0-provenance-preserving-delta ∷
  q0-unit ∷
  q0-telescope-flattening ∷
  q0-fresh-nonrecursive-constructor-computation ∷ []

decode-q0-inventory : List Nat → WireMaybe (List AbstractQ0TagV1)
decode-q0-inventory [] = wire-just []
decode-q0-inventory (tag ∷ tags) =
  wire-maybe-bind (q0-tag-to-abstract tag) (λ decoded →
  wire-maybe-bind (decode-q0-inventory tags) (λ rest →
  wire-just (decoded ∷ rest)))

-- The structurally accepted Q0 section decodes to exactly the seven
-- abstract rules in order: the mechanized Rust-tag erasure.
q0-inventory-decodes :
  (rules : List Nat) → check-q0-inventory rules ≡ true →
  decode-q0-inventory rules ≡ wire-just exact-abstract-q0-inventory-v1
q0-inventory-decodes rules proof =
  cong decode-q0-inventory
    (nat-list-equal-sound rules
      (0 ∷ 1 ∷ 2 ∷ 3 ∷ 4 ∷ 5 ∷ 6 ∷ []) proof)

map-q0-category :
  List AbstractQ0TagV1 → List ProductionQ0CategoryV1
map-q0-category [] = []
map-q0-category (tag ∷ tags) =
  q0-category tag ∷ map-q0-category tags

exact-q0-category-list-v1 : List ProductionQ0CategoryV1
exact-q0-category-list-v1 =
  representation ∷
  representation ∷
  base-semantic ∷
  base-semantic ∷
  base-semantic ∷
  representation ∷
  runtime-public ∷ []

-- The representation/base-semantic/runtime-public classification of
-- the decoded inventory, pinned as the exact category list. Each
-- element additionally carries its `Q0ClassificationV1` record via
-- `q0-classification-total`.
q0-classification-exact :
  map-q0-category exact-abstract-q0-inventory-v1
  ≡ exact-q0-category-list-v1
q0-classification-exact = refl

q0-inventory-classified :
  (tag : AbstractQ0TagV1) →
  Q0ClassificationV1 tag (q0-category tag)
q0-inventory-classified = q0-classification-total

q0-member : AbstractQ0TagV1 → List AbstractQ0TagV1 → Bool
q0-member tag [] = false
q0-member tag (candidate ∷ rest) =
  nat-equal (abstract-q0-to-tag tag) (abstract-q0-to-tag candidate)
  or q0-member tag rest

-- Every abstract reduction step is classified by a rule of the exact
-- decoded inventory: the base rewrites map to their tags directly and
-- congruence frames inherit their premise's tag, so no step of the
-- replayed conversion layer falls outside section 8.
step-q0-tag-inventoried :
  {V : Set} {left right : Tm V} (step : Step {V} left right) →
  q0-member (abstract-step-q0-tag step)
    exact-abstract-q0-inventory-v1 ≡ true
step-q0-tag-inventoried (beta-step parameter-type body argument) =
  refl
step-q0-tag-inventoried (public-delta-step identifier body) = refl
step-q0-tag-inventoried (fresh-equation-step left right match) = refl
step-q0-tag-inventoried (pi-parameter-congruence-step body premise) =
  step-q0-tag-inventoried premise
step-q0-tag-inventoried (pi-body-congruence-step parameter premise) =
  step-q0-tag-inventoried premise
step-q0-tag-inventoried
  (lambda-parameter-congruence-step body premise) =
  step-q0-tag-inventoried premise
step-q0-tag-inventoried
  (lambda-body-congruence-step parameter premise) =
  step-q0-tag-inventoried premise
step-q0-tag-inventoried
  (apply-function-congruence-step argument premise) =
  step-q0-tag-inventoried premise
step-q0-tag-inventoried
  (apply-argument-congruence-step function premise) =
  step-q0-tag-inventoried premise

-- --- Slot-table helpers ----------------------------------------------

-- The structural checker pins chronological slots (entry i has slot
-- i), so plain list indexing is the slot lookup.
slot-body-absent : WireMaybe GlobalSlotEntryWireV1 → Bool
slot-body-absent wire-nothing = false
slot-body-absent (wire-just entry) = body-check (declaration-body entry)
  where
  body-check : WireMaybe WireTermV1 → Bool
  body-check wire-nothing = true
  body-check (wire-just body) = false

slot-bodyless : List GlobalSlotEntryWireV1 → Nat → Bool
slot-bodyless entries slot = slot-body-absent (list-entry entries slot)

-- --- Typed judgment replay -------------------------------------------

-- One judgment `context ⊢ subject : ty` replayed through the
-- kernel-mirroring typing layer: the context builds and verifies, the
-- type forms a type, and the subject checks against it. This is the
-- wire image of `verify_open_judgments` for a single judgment.
check-judgment-typing :
  (globals : Nat) (sig : PSig globals) (fuel : Nat) →
  FamilyJudgmentWireV1 → Bool
check-judgment-typing globals sig fuel judgment =
  context-branch
    (check-context-entries globals zero
      (entries-oldest-first (family-judgment-context judgment)))
    refl
  where
  context-branch :
    (scoped : Bool) →
    check-context-entries globals zero
      (entries-oldest-first (family-judgment-context judgment))
    ≡ scoped → Bool
  context-branch false proof = false
  context-branch true proof =
    typed-branch
      (decode-term-checked globals locals
        (family-judgment-subject judgment))
      (decode-term-checked globals locals
        (family-judgment-type judgment))
    where
    entries : List WireTermV1
    entries = entries-oldest-first (family-judgment-context judgment)
    locals : Nat
    locals = total-from zero entries
    context : PCtx globals locals
    context = wire-context-extend zero pempty entries proof
    typed-branch :
      WireMaybe (PTm globals locals) →
      WireMaybe (PTm globals locals) → Bool
    typed-branch (wire-just subject) (wire-just ty) =
      is-wire-just (pverify-context sig fuel context) and
      (is-wire-just
        (wire-maybe-bind (pinfer sig fuel context ty)
          (pexpect-universe sig fuel)) and
       pcheck sig fuel context subject ty)
    typed-branch _ _ = false

-- --- Fresh-rule inventory layer --------------------------------------

find-fresh-rule :
  List FreshRuleSchemaWireV1 → WireIdV1 →
  WireMaybe FreshRuleSchemaWireV1
find-fresh-rule [] equation = wire-nothing
find-fresh-rule (schema ∷ schemas) equation =
  found-branch (byte-list-equal (fresh-equation-id schema) equation)
  where
  found-branch : Bool → WireMaybe FreshRuleSchemaWireV1
  found-branch true = wire-just schema
  found-branch false = find-fresh-rule schemas equation

-- One fresh rule beyond the structural pattern: at least two
-- telescope entries (parameters plus scrutinee), distinct bodyless
-- owner and constructor (a bodyful owner would be delta-erased and is
-- forbidden by the kernel normalizer; bodylessness also keeps the
-- owner outside every transparent-delta policy), and both sides of
-- the equation replay against the recorded type under the parameter
-- context.
check-fresh-rule-inventory :
  (globals : Nat) (entries : List GlobalSlotEntryWireV1)
  (sig : PSig globals) (fuel : Nat) → FreshRuleSchemaWireV1 → Bool
check-fresh-rule-inventory globals entries sig fuel schema =
  nat-less 1 (fresh-arity schema) and
  (not
    (nat-equal (fresh-owner-slot schema)
      (fresh-constructor-slot schema)) and
  (slot-bodyless entries (fresh-owner-slot schema) and
  (slot-bodyless entries (fresh-constructor-slot schema) and
  (check-judgment-typing globals sig fuel
    (family-judgment-v1 (fresh-parameter-context schema)
      (fresh-left schema) (fresh-type schema)) and
   check-judgment-typing globals sig fuel
     (family-judgment-v1 (fresh-parameter-context schema)
       (fresh-right schema) (fresh-type schema))))))

owner-constructor-member : Nat → Nat → List (Nat × Nat) → Bool
owner-constructor-member owner head-constructor [] = false
owner-constructor-member owner head-constructor
  ((seen-owner , seen-constructor) ∷ rest) =
  (nat-equal owner seen-owner and
   nat-equal head-constructor seen-constructor)
  or owner-constructor-member owner head-constructor rest

-- Pairwise `(owner, constructor)` disjointness across the whole rule
-- list: the wire image of `constructor_patterns_are_pairwise_disjoint`
-- from the restricted-Q0 confluence certificate. With left-linearity
-- from the structural pattern, the accepted rule set is orthogonal.
check-fresh-rules-inventory :
  (globals : Nat) (entries : List GlobalSlotEntryWireV1)
  (sig : PSig globals) (fuel : Nat) → List (Nat × Nat) →
  List FreshRuleSchemaWireV1 → Bool
check-fresh-rules-inventory globals entries sig fuel seen [] = true
check-fresh-rules-inventory globals entries sig fuel seen
  (schema ∷ schemas) =
  not
    (owner-constructor-member (fresh-owner-slot schema)
      (fresh-constructor-slot schema) seen) and
  (check-fresh-rule-inventory globals entries sig fuel schema and
   check-fresh-rules-inventory globals entries sig fuel
     ((fresh-owner-slot schema , fresh-constructor-slot schema) ∷ seen)
     schemas)

-- --- Family payload inventory layer ----------------------------------

family-payload-judgment : FamilyPayloadWireV1 → FamilyJudgmentWireV1
family-payload-judgment (family-seed family source judgment) = judgment
family-payload-judgment
  (family-generic-public-application family function argument
    judgment) = judgment
family-payload-judgment
  (family-generic-equation-action family equation source judgment) =
  judgment

find-judgment :
  List (WireIdV1 × FamilyJudgmentWireV1) → WireIdV1 →
  WireMaybe FamilyJudgmentWireV1
find-judgment [] family = wire-nothing
find-judgment ((seen-id , judgment) ∷ rest) family =
  found-branch (byte-list-equal seen-id family)
  where
  found-branch : Bool → WireMaybe FamilyJudgmentWireV1
  found-branch true = wire-just judgment
  found-branch false = find-judgment rest family

judgment-context-entries : FamilyJudgmentWireV1 → List WireTermV1
judgment-context-entries judgment =
  entries-oldest-first (family-judgment-context judgment)

-- A seed judgment must bind exactly its public head: the subject is
-- the head's global and the recorded type is the head's exact declared
-- type (not merely a convertible presentation of it).
check-seed-binding :
  (entries : List GlobalSlotEntryWireV1) (owner : Nat) →
  FamilyJudgmentWireV1 → Bool
check-seed-binding entries owner judgment =
  declared-branch (list-entry entries owner)
  where
  declared-branch : WireMaybe GlobalSlotEntryWireV1 → Bool
  declared-branch wire-nothing = false
  declared-branch (wire-just entry) =
    wire-term-equal (family-judgment-subject judgment)
      (wire-global-slot owner) and
    wire-term-equal (family-judgment-type judgment)
      (declaration-type entry)

check-family-payload-inventory :
  (globals : Nat) (entries : List GlobalSlotEntryWireV1)
  (sig : PSig globals) (fresh : List FreshRuleSchemaWireV1)
  (fuel : Nat) (seen : List (WireIdV1 × FamilyJudgmentWireV1)) →
  FamilyPayloadWireV1 → Bool
check-family-payload-inventory globals entries sig fresh fuel seen
  (family-seed family (seed-public-head owner) judgment) =
  check-seed-binding entries owner judgment and
  check-judgment-typing globals sig fuel judgment
check-family-payload-inventory globals entries sig fresh fuel seen
  (family-seed family (seed-public-equation equation) judgment) =
  rule-branch (find-fresh-rule fresh equation)
  where
  rule-branch : WireMaybe FreshRuleSchemaWireV1 → Bool
  rule-branch wire-nothing = false
  rule-branch (wire-just rule) =
    check-seed-binding entries (fresh-owner-slot rule) judgment and
    check-judgment-typing globals sig fuel judgment
check-family-payload-inventory globals entries sig fresh fuel seen
  (family-generic-public-application family function argument
    judgment) =
  components-branch (find-judgment seen function)
    (find-judgment seen argument)
  where
  components-branch :
    WireMaybe FamilyJudgmentWireV1 →
    WireMaybe FamilyJudgmentWireV1 → Bool
  components-branch (wire-just function-judgment)
    (wire-just argument-judgment) =
    wire-term-list-equal (judgment-context-entries judgment)
      (judgment-context-entries function-judgment) and
    (wire-term-list-equal (judgment-context-entries judgment)
      (judgment-context-entries argument-judgment) and
    (wire-term-equal (family-judgment-subject judgment)
      (wire-apply (family-judgment-subject function-judgment)
        (family-judgment-subject argument-judgment)) and
     check-judgment-typing globals sig fuel judgment))
  components-branch _ _ = false
check-family-payload-inventory globals entries sig fresh fuel seen
  (family-generic-equation-action family equation source judgment) =
  is-wire-just (find-fresh-rule fresh equation) and
  source-branch (find-judgment seen source)
  where
  source-branch : WireMaybe FamilyJudgmentWireV1 → Bool
  source-branch wire-nothing = false
  source-branch (wire-just source-judgment) =
    wire-term-list-equal (judgment-context-entries judgment)
      (judgment-context-entries source-judgment) and
    (wire-term-equal (family-judgment-type judgment)
      (family-judgment-type source-judgment) and
     check-judgment-typing globals sig fuel judgment)

check-family-payloads-inventory :
  (globals : Nat) (entries : List GlobalSlotEntryWireV1)
  (sig : PSig globals) (fresh : List FreshRuleSchemaWireV1)
  (fuel : Nat) (seen : List (WireIdV1 × FamilyJudgmentWireV1)) →
  List FamilyPayloadWireV1 → Bool
check-family-payloads-inventory globals entries sig fresh fuel seen
  [] = true
check-family-payloads-inventory globals entries sig fresh fuel seen
  (payload ∷ payloads) =
  check-family-payload-inventory globals entries sig fresh fuel seen
    payload and
  check-family-payloads-inventory globals entries sig fresh fuel
    ((family-payload-id payload , family-payload-judgment payload)
      ∷ seen)
    payloads

-- --- Whole-bundle inventory verdict ----------------------------------

-- The Phase F verdict: the exact Q0 and family inventories, the
-- typed/disjoint fresh-rule layer, and the exact family payload
-- discipline. Runs strictly after the structural checker, the
-- semantic replay, and the typing verdict; sections 9 and 11 are
-- invisible to those layers, so this verdict is the first semantic
-- claim about them.
inventory-check-bundle : ProductionBundleSemanticV1 → Bool
inventory-check-bundle bundle =
  slots-branch (check-slots-from zero (bundle-global-slots bundle))
    refl
  where
  slots-branch :
    (result : Bool) →
    check-slots-from zero (bundle-global-slots bundle) ≡ result → Bool
  slots-branch false proof = false
  slots-branch true proof =
    check-q0-inventory (bundle-q0-rules bundle) and
    (check-fresh-rules-inventory globals (bundle-global-slots bundle)
      sig typing-fuel-v1 [] (bundle-fresh-rules bundle) and
    (check-family-inventory (bundle-family-codes bundle) and
     check-family-payloads-inventory globals
       (bundle-global-slots bundle) sig (bundle-fresh-rules bundle)
       typing-fuel-v1 [] (bundle-family-payloads bundle)))
    where
    globals : Nat
    globals = total-from zero (bundle-global-slots bundle)
    sig : PSig globals
    sig = wire-slots-to-psig (bundle-global-slots bundle) proof

-- --- Decoded fresh rules and substitution stability ------------------

record DecodedFreshRuleV1 (globals : Nat) : Set where
  constructor decoded-fresh-rule
  field
    fresh-vars : Nat
    fresh-left-ptm : PTm globals fresh-vars
    fresh-right-ptm : PTm globals fresh-vars
    fresh-type-ptm : PTm globals fresh-vars

open DecodedFreshRuleV1 public

decode-fresh-rule :
  (globals : Nat) → FreshRuleSchemaWireV1 →
  WireMaybe (DecodedFreshRuleV1 globals)
decode-fresh-rule globals schema =
  wire-maybe-bind
    (decode-term-checked globals locals (fresh-left schema)) (λ left →
  wire-maybe-bind
    (decode-term-checked globals locals (fresh-right schema)) (λ right →
  wire-maybe-bind
    (decode-term-checked globals locals (fresh-type schema)) (λ ty →
  wire-just (decoded-fresh-rule locals left right ty))))
  where
  locals : Nat
  locals = context-local-count (fresh-parameter-context schema)

-- Every match substitution from the pattern variables instantiates
-- the decoded rule as a genuine abstract step: the wire image of
-- `fresh-equation-step`, with the pattern-variable set given by the
-- parameter context.
decoded-fresh-step :
  {globals : Nat} (rule : DecodedFreshRuleV1 globals) {W : Set}
  (match : PVar (fresh-vars rule) → Tm W) →
  Step
    (substitute match (decode-ptm (fresh-left-ptm rule)))
    (substitute match (decode-ptm (fresh-right-ptm rule)))
decoded-fresh-step rule match =
  fresh-equation-step
    (decode-ptm (fresh-left-ptm rule))
    (decode-ptm (fresh-right-ptm rule))
    match

-- Substitution stability of the decoded rule schema: substituting an
-- accepted instance is again an accepted instance. The step transport
-- is `substitution-step`; the composed-match endpoint equations are
-- the inventoried fresh-equation stability obligations
-- (`fresh-left-naturality`/`fresh-right-naturality`).
decoded-fresh-step-stability :
  {globals : Nat} (rule : DecodedFreshRuleV1 globals) {W X : Set}
  (match : PVar (fresh-vars rule) → Tm W) (σ : W → Tm X) →
  Step
    (substitute σ
      (substitute match (decode-ptm (fresh-left-ptm rule))))
    (substitute σ
      (substitute match (decode-ptm (fresh-right-ptm rule))))
decoded-fresh-step-stability rule match σ =
  substitution-step σ (decoded-fresh-step rule match)

decoded-fresh-left-composed :
  {globals : Nat} (rule : DecodedFreshRuleV1 globals) {W X : Set}
  (match : PVar (fresh-vars rule) → Tm W) (σ : W → Tm X) →
  substitute σ (substitute match (decode-ptm (fresh-left-ptm rule)))
  ≡ substitute (match then σ) (decode-ptm (fresh-left-ptm rule))
decoded-fresh-left-composed rule match σ =
  fresh-left-naturality (decode-ptm (fresh-left-ptm rule)) match σ

decoded-fresh-right-composed :
  {globals : Nat} (rule : DecodedFreshRuleV1 globals) {W X : Set}
  (match : PVar (fresh-vars rule) → Tm W) (σ : W → Tm X) →
  substitute σ (substitute match (decode-ptm (fresh-right-ptm rule)))
  ≡ substitute (match then σ) (decode-ptm (fresh-right-ptm rule))
decoded-fresh-right-composed rule match σ =
  fresh-right-naturality (decode-ptm (fresh-right-ptm rule)) match σ

-- --- Non-recursion transport -----------------------------------------

-- The argument orientation mirrors `contains-global-slot` exactly:
-- the term's identifier is compared first.
tm-contains-global : {V : Set} → Nat → Tm V → Bool
tm-contains-global target (var x) = false
tm-contains-global target (sort level) = false
tm-contains-global target (global identifier) =
  nat-equal identifier target
tm-contains-global target (pi parameter body) =
  tm-contains-global target parameter or
  tm-contains-global target body
tm-contains-global target (lam parameter body) =
  tm-contains-global target parameter or
  tm-contains-global target body
tm-contains-global target (app function argument) =
  tm-contains-global target function or
  tm-contains-global target argument
tm-contains-global target unit-type = false
tm-contains-global target unit = false

fin-ordinal-of-from-less :
  (slot limit : Nat) (proof : nat-less slot limit ≡ true) →
  fin-ordinal (fin-from-less slot limit proof) ≡ slot
fin-ordinal-of-from-less zero zero ()
fin-ordinal-of-from-less (suc slot) zero ()
fin-ordinal-of-from-less zero (suc limit) proof = refl
fin-ordinal-of-from-less (suc slot) (suc limit) proof =
  cong suc (fin-ordinal-of-from-less slot limit proof)

-- The structural owner-freeness Boolean transports to the decoded
-- abstract term: a wire term that never mentions a global slot
-- decodes to a term that never mentions that global identifier. With
-- the structural `check-fresh-pattern` conjunct this is the intrinsic
-- non-recursion statement for every accepted fresh rule.
contains-global-transport :
  (globals locals : Nat) (term : WireTermV1)
  (scope-proof : wire-in-scope globals locals term ≡ true)
  (slot : Nat) →
  contains-global-slot slot term ≡ false →
  tm-contains-global slot
    (decode-ptm (wire-to-ptm globals locals term scope-proof))
  ≡ false
contains-global-transport globals locals (wire-sort level) scope-proof
  slot proof = refl
contains-global-transport globals locals (wire-variable index)
  scope-proof slot proof = refl
contains-global-transport globals locals (wire-global-slot target)
  scope-proof slot proof =
  ordinal-branch
    (fin-ordinal-of-from-less target globals scope-proof)
  where
  ordinal-branch :
    fin-ordinal (fin-from-less target globals scope-proof) ≡ target →
    nat-equal
      (fin-ordinal (fin-from-less target globals scope-proof)) slot
    ≡ false
  ordinal-branch ordinal-proof rewrite ordinal-proof = proof
contains-global-transport globals locals (wire-pi parameter body)
  scope-proof slot proof
  rewrite
    contains-global-transport globals locals parameter
      (and-first (wire-in-scope globals locals parameter)
        (wire-in-scope globals (suc locals) body) scope-proof)
      slot
      (or-first-false (contains-global-slot slot parameter)
        (contains-global-slot slot body) proof)
  | contains-global-transport globals (suc locals) body
      (and-second (wire-in-scope globals locals parameter)
        (wire-in-scope globals (suc locals) body) scope-proof)
      slot
      (or-second-false (contains-global-slot slot parameter)
        (contains-global-slot slot body) proof)
  = refl
contains-global-transport globals locals (wire-lambda parameter body)
  scope-proof slot proof
  rewrite
    contains-global-transport globals locals parameter
      (and-first (wire-in-scope globals locals parameter)
        (wire-in-scope globals (suc locals) body) scope-proof)
      slot
      (or-first-false (contains-global-slot slot parameter)
        (contains-global-slot slot body) proof)
  | contains-global-transport globals (suc locals) body
      (and-second (wire-in-scope globals locals parameter)
        (wire-in-scope globals (suc locals) body) scope-proof)
      slot
      (or-second-false (contains-global-slot slot parameter)
        (contains-global-slot slot body) proof)
  = refl
contains-global-transport globals locals (wire-apply function argument)
  scope-proof slot proof
  rewrite
    contains-global-transport globals locals function
      (and-first (wire-in-scope globals locals function)
        (wire-in-scope globals locals argument) scope-proof)
      slot
      (or-first-false (contains-global-slot slot function)
        (contains-global-slot slot argument) proof)
  | contains-global-transport globals locals argument
      (and-second (wire-in-scope globals locals function)
        (wire-in-scope globals locals argument) scope-proof)
      slot
      (or-second-false (contains-global-slot slot function)
        (contains-global-slot slot argument) proof)
  = refl
contains-global-transport globals locals wire-unit-type scope-proof
  slot proof = refl
contains-global-transport globals locals wire-unit scope-proof slot
  proof = refl

-- --- Family payload decoding and naturality transport ----------------

wire-family-payload-code : FamilyPayloadWireV1 → Nat
wire-family-payload-code (family-seed family source judgment) = 0
wire-family-payload-code
  (family-generic-public-application family function argument
    judgment) = 1
wire-family-payload-code
  (family-generic-equation-action family equation source judgment) = 2

abstract-family-code-to-tag : ProductionFamilyCodeV1 → Nat
abstract-family-code-to-tag code-seed = 0
abstract-family-code-to-tag code-generic-public-application = 1
abstract-family-code-to-tag code-generic-equation-action = 2

family-code-tag-to-abstract : Nat → WireMaybe ProductionFamilyCodeV1
family-code-tag-to-abstract 0 = wire-just code-seed
family-code-tag-to-abstract 1 =
  wire-just code-generic-public-application
family-code-tag-to-abstract 2 =
  wire-just code-generic-equation-action
family-code-tag-to-abstract _ = wire-nothing

family-code-round-trip :
  (code : ProductionFamilyCodeV1) →
  family-code-tag-to-abstract (abstract-family-code-to-tag code)
  ≡ wire-just code
family-code-round-trip code-seed = refl
family-code-round-trip code-generic-public-application = refl
family-code-round-trip code-generic-equation-action = refl

exact-abstract-family-inventory-v1 : List ProductionFamilyCodeV1
exact-abstract-family-inventory-v1 =
  code-seed ∷
  code-generic-public-application ∷
  code-generic-equation-action ∷ []

decode-family-inventory :
  List Nat → WireMaybe (List ProductionFamilyCodeV1)
decode-family-inventory [] = wire-just []
decode-family-inventory (tag ∷ tags) =
  wire-maybe-bind (family-code-tag-to-abstract tag) (λ decoded →
  wire-maybe-bind (decode-family-inventory tags) (λ rest →
  wire-just (decoded ∷ rest)))

family-inventory-decodes :
  (codes : List Nat) → check-family-inventory codes ≡ true →
  decode-family-inventory codes
  ≡ wire-just exact-abstract-family-inventory-v1
family-inventory-decodes codes proof =
  cong decode-family-inventory
    (nat-list-equal-sound codes (0 ∷ 1 ∷ 2 ∷ []) proof)

-- Decoded family environments at one judgment context. The strictly
-- prior wire references resolve against the already decoded prefix;
-- the shared-context discipline of the Boolean layer puts every tree
-- at its own single context, so the decode is per-tree at that
-- context's variable set. The variable-set index is explicit
-- everywhere because `PVar` is a defined type function that unification
-- cannot invert.
FamilyEnvV1 : (locals : Nat) → Set
FamilyEnvV1 locals = List (WireIdV1 × Family (PVar locals))

find-family :
  (locals : Nat) → FamilyEnvV1 locals → WireIdV1 →
  WireMaybe (Family (PVar locals))
find-family locals [] family = wire-nothing
find-family locals ((seen-id , value) ∷ rest) family =
  found-branch (byte-list-equal seen-id family)
  where
  found-branch : Bool → WireMaybe (Family (PVar locals))
  found-branch true = wire-just value
  found-branch false = find-family locals rest family

-- The equation component of an action enters the family world as the
-- seed of the equation's owner head - the same representative a
-- `PublicEquation` seed payload of that equation binds, mirroring the
-- Rust seed subject `Equation { owner_head, .. }`.
equation-seed-term :
  (globals locals : Nat) (fresh : List FreshRuleSchemaWireV1)
  (equation : WireIdV1) → WireMaybe (Tm (PVar locals))
equation-seed-term globals locals fresh equation =
  wire-maybe-bind (find-fresh-rule fresh equation) (λ rule →
  wire-maybe-bind
    (decode-term-checked globals locals
      (wire-global-slot (fresh-owner-slot rule))) (λ owner →
  wire-just (decode-ptm owner)))

decode-family-value :
  (globals locals : Nat) (fresh : List FreshRuleSchemaWireV1)
  (env : FamilyEnvV1 locals) → FamilyPayloadWireV1 →
  WireMaybe (Family (PVar locals))
decode-family-value globals locals fresh env
  (family-seed family source judgment) =
  wire-maybe-bind
    (decode-term-checked globals locals
      (family-judgment-subject judgment)) (λ subject →
  wire-just (seed (decode-ptm subject)))
decode-family-value globals locals fresh env
  (family-generic-public-application family function argument
    judgment) =
  wire-maybe-bind (find-family locals env function) (λ function-value →
  wire-maybe-bind (find-family locals env argument) (λ argument-value →
  wire-just
    (generic-public-application function-value argument-value)))
decode-family-value globals locals fresh env
  (family-generic-equation-action family equation source judgment) =
  wire-maybe-bind (equation-seed-term globals locals fresh equation)
    (λ equation-term →
  wire-maybe-bind (find-family locals env source) (λ source-value →
  wire-just
    (generic-equation-action (seed equation-term) source-value)))

decode-family-payloads :
  (globals locals : Nat) (fresh : List FreshRuleSchemaWireV1)
  (env : FamilyEnvV1 locals) →
  List FamilyPayloadWireV1 → WireMaybe (FamilyEnvV1 locals)
decode-family-payloads globals locals fresh env [] = wire-just env
decode-family-payloads globals locals fresh env
  (payload ∷ payloads) =
  wire-maybe-bind
    (decode-family-value globals locals fresh env payload) (λ value →
  decode-family-payloads globals locals fresh
    ((family-payload-id payload , value) ∷ env) payloads)

-- The wire payload tag is exactly the abstract `family-code` of the
-- decoded value: the mechanized Rust family-payload erasure.
decode-family-code-correspondence :
  (globals locals : Nat) (fresh : List FreshRuleSchemaWireV1)
  (env : FamilyEnvV1 locals) (payload : FamilyPayloadWireV1)
  (value : Family (PVar locals)) →
  decode-family-value globals locals fresh env payload
  ≡ wire-just value →
  abstract-family-code-to-tag (family-code value)
  ≡ wire-family-payload-code payload
decode-family-code-correspondence globals locals fresh env
  (family-seed family source judgment) value proof =
  seed-branch
    (decode-term-checked globals locals
      (family-judgment-subject judgment))
    proof
  where
  seed-branch :
    (subject : WireMaybe (PTm globals locals)) →
    wire-maybe-bind subject
      (λ decoded → wire-just (seed (decode-ptm decoded)))
    ≡ wire-just value →
    abstract-family-code-to-tag (family-code value) ≡ 0
  seed-branch wire-nothing ()
  seed-branch (wire-just decoded) bind-proof
    rewrite sym (wire-just-injective bind-proof) = refl
decode-family-code-correspondence globals locals fresh env
  (family-generic-public-application family function argument
    judgment) value proof =
  application-branch (find-family locals env function)
    (find-family locals env argument) proof
  where
  application-branch :
    (function-value argument-value :
      WireMaybe (Family (PVar locals))) →
    wire-maybe-bind function-value (λ f →
      wire-maybe-bind argument-value (λ a →
        wire-just (generic-public-application f a)))
    ≡ wire-just value →
    abstract-family-code-to-tag (family-code value) ≡ 1
  application-branch wire-nothing argument-value ()
  application-branch (wire-just f) wire-nothing ()
  application-branch (wire-just f) (wire-just a) bind-proof
    rewrite sym (wire-just-injective bind-proof) = refl
decode-family-code-correspondence globals locals fresh env
  (family-generic-equation-action family equation source judgment)
  value proof =
  action-branch (equation-seed-term globals locals fresh equation)
    (find-family locals env source) proof
  where
  action-branch :
    (equation-term : WireMaybe (Tm (PVar locals)))
    (source-value : WireMaybe (Family (PVar locals))) →
    wire-maybe-bind equation-term (λ e →
      wire-maybe-bind source-value (λ s →
        wire-just (generic-equation-action (seed e) s)))
    ≡ wire-just value →
    abstract-family-code-to-tag (family-code value) ≡ 2
  action-branch wire-nothing source-value ()
  action-branch (wire-just e) wire-nothing ()
  action-branch (wire-just e) (wire-just s) bind-proof
    rewrite sym (wire-just-injective bind-proof) = refl

-- Family naturality transported through the exact payload decoding:
-- the wire code of a decoded payload is invariant under every
-- substitution of the decoded value, so the section-10/11
-- classification is a substitution invariant of the decoded object,
-- not an artifact of its presentation.
decoded-family-code-substitution-invariant :
  (globals locals : Nat) (fresh : List FreshRuleSchemaWireV1)
  (env : FamilyEnvV1 locals) (payload : FamilyPayloadWireV1)
  (value : Family (PVar locals)) →
  decode-family-value globals locals fresh env payload
  ≡ wire-just value →
  {W : Set} (σ : PVar locals → Tm W) →
  abstract-family-code-to-tag
    (family-code (substitute-family σ value))
  ≡ wire-family-payload-code payload
decoded-family-code-substitution-invariant globals locals fresh env
  payload value proof σ =
  trans
    (cong abstract-family-code-to-tag
      (family-code-substitution σ value))
    (decode-family-code-correspondence globals locals fresh env
      payload value proof)

-- The functor laws of `substitute-family` at decoded values: named
-- corollaries of `family-identity` and `family-composition`, so the
-- decoded objects carry the complete naturality package.
decoded-family-identity :
  {globals locals : Nat} (fresh : List FreshRuleSchemaWireV1)
  (env : FamilyEnvV1 locals) (payload : FamilyPayloadWireV1)
  (value : Family (PVar locals)) →
  decode-family-value globals locals fresh env payload
  ≡ wire-just value →
  substitute-family identity-substitution value ≡ value
decoded-family-identity fresh env payload value proof =
  family-identity value

decoded-family-composition :
  {globals locals : Nat} (fresh : List FreshRuleSchemaWireV1)
  (env : FamilyEnvV1 locals) (payload : FamilyPayloadWireV1)
  (value : Family (PVar locals)) →
  decode-family-value globals locals fresh env payload
  ≡ wire-just value →
  {W X : Set} (σ : PVar locals → Tm W) (τ : W → Tm X) →
  substitute-family τ (substitute-family σ value)
  ≡ substitute-family (σ then τ) value
decoded-family-composition fresh env payload value proof σ τ =
  family-composition σ τ value

-- The constructor-commutation naturality equations at decoded
-- component values, re-stated at the wire layer so the transported
-- package is explicit.
decoded-application-naturality :
  {locals : Nat} {W : Set} (σ : PVar locals → Tm W)
  (function-value argument-value : Family (PVar locals)) →
  substitute-family σ
    (generic-public-application function-value argument-value)
  ≡ generic-public-application
      (substitute-family σ function-value)
      (substitute-family σ argument-value)
decoded-application-naturality σ function-value argument-value =
  generic-public-application-naturality σ function-value
    argument-value

decoded-action-naturality :
  {locals : Nat} {W : Set} (σ : PVar locals → Tm W)
  (equation-value source-value : Family (PVar locals)) →
  substitute-family σ
    (generic-equation-action equation-value source-value)
  ≡ generic-equation-action
      (substitute-family σ equation-value)
      (substitute-family σ source-value)
decoded-action-naturality σ equation-value source-value =
  generic-equation-action-naturality σ equation-value source-value

-- The classification record for every decoded family, via the total
-- classification of the abstract layer.
decoded-family-classification :
  {locals : Nat} (value : Family (PVar locals)) →
  FamilyClassificationV1 value (family-code value)
decoded-family-classification value =
  family-classification-total value
