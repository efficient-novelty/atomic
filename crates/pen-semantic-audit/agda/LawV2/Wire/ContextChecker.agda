{-# OPTIONS --safe --without-K #-}

module LawV2.Wire.ContextChecker where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe using (Maybe; just; nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc)
open import LawV2.Wire.Bytes
open import LawV2.Wire.ProductionBundleV1

not : Bool → Bool
not false = true
not true = false

nat-less : Nat → Nat → Bool
nat-less zero zero = false
nat-less zero (suc right) = true
nat-less (suc left) zero = false
nat-less (suc left) (suc right) = nat-less left right

nat-list-equal : List Nat → List Nat → Bool
nat-list-equal [] [] = true
nat-list-equal [] (_ ∷ _) = false
nat-list-equal (_ ∷ _) [] = false
nat-list-equal (left ∷ lefts) (right ∷ rights) =
  nat-equal left right and nat-list-equal lefts rights

byte-list-equal : List Byte → List Byte → Bool
byte-list-equal [] [] = true
byte-list-equal [] (_ ∷ _) = false
byte-list-equal (_ ∷ _) [] = false
byte-list-equal (left ∷ lefts) (right ∷ rights) =
  byte-equal left right and byte-list-equal lefts rights

byte-values-equal : List Byte → List Nat → Bool
byte-values-equal [] [] = true
byte-values-equal [] (_ ∷ _) = false
byte-values-equal (_ ∷ _) [] = false
byte-values-equal (byte ∷ bytes) (value ∷ values) =
  nat-equal (byte-value byte) value and byte-values-equal bytes values

id-is-32-bytes : WireIdV1 → Bool
id-is-32-bytes identifier = nat-equal (length identifier) 32

expected-profile-id : List Nat
expected-profile-id =
  103 ∷ 102 ∷ 50 ∷ 45 ∷ 115 ∷ 101 ∷ 109 ∷ 97 ∷
  110 ∷ 116 ∷ 105 ∷ 99 ∷ 45 ∷ 97 ∷ 117 ∷ 100 ∷
  105 ∷ 116 ∷ 45 ∷ 108 ∷ 97 ∷ 109 ∷ 98 ∷ 100 ∷
  97 ∷ 45 ∷ 117 ∷ 110 ∷ 105 ∷ 116 ∷ 45 ∷ 118 ∷
  51 ∷ []

expected-synthesis-protocol : List Nat
expected-synthesis-protocol =
  112 ∷ 101 ∷ 110 ∷ 45 ∷ 107 ∷ 101 ∷ 114 ∷ 110 ∷
  101 ∷ 108 ∷ 45 ∷ 115 ∷ 121 ∷ 110 ∷ 116 ∷ 104 ∷
  101 ∷ 115 ∷ 105 ∷ 115 ∷ 47 ∷ 108 ∷ 97 ∷ 109 ∷
  98 ∷ 100 ∷ 97 ∷ 45 ∷ 117 ∷ 110 ∷ 105 ∷ 116 ∷
  47 ∷ 118 ∷ 50 ∷ []

check-manifest-identity-v1 : ManifestSurfaceWireV1 → Bool
check-manifest-identity-v1 manifest =
  nat-equal (semantic-schema-version manifest) 3 and
  (byte-values-equal (profile-id manifest) expected-profile-id and
  (id-is-32-bytes (semantic-manifest-digest manifest) and
  (not (manifest-frozen manifest) and
  (not (live-profile-a-access manifest) and
  (id-is-32-bytes (production-inventory-bridge-digest manifest) and
   id-is-32-bytes (predecessor-delta-policy-binding-digest manifest))))))

check-manifest-universes-v1 : ManifestSurfaceWireV1 → Bool
check-manifest-universes-v1 manifest =
  nat-list-equal (public-universe-levels manifest) (0 ∷ 1 ∷ []) and
  (nat-list-equal (checker-universe-levels manifest) (0 ∷ 1 ∷ 2 ∷ []) and
  (nat-list-equal (formation-witness-levels manifest) (0 ∷ 1 ∷ 2 ∷ 3 ∷ []) and
   nat-equal (maximum-context-entries manifest) 32))

check-manifest-synthesis-v1 : ManifestSurfaceWireV1 → Bool
check-manifest-synthesis-v1 manifest =
  nat-list-equal (synthesis-rule-inventory manifest)
    (0 ∷ 1 ∷ 2 ∷ 3 ∷ 4 ∷ 5 ∷ 6 ∷ 7 ∷ []) and
  (byte-values-equal (synthesis-protocol-id manifest) expected-synthesis-protocol and
   nat-equal (synthesis-schema-version manifest) 2)

check-manifest-surface-v1 : ManifestSurfaceWireV1 → Bool
check-manifest-surface-v1 manifest =
  check-manifest-identity-v1 manifest and
  (check-manifest-universes-v1 manifest and
   check-manifest-synthesis-v1 manifest)

term-scoped : Nat → Nat → WireTermV1 → Bool
term-scoped globals locals (wire-sort level) = nat-less level 2
term-scoped globals locals (wire-variable index) = nat-less index locals
term-scoped globals locals (wire-global-slot slot) = nat-less slot globals
term-scoped globals locals (wire-pi parameter body) =
  term-scoped globals locals parameter and term-scoped globals (suc locals) body
term-scoped globals locals (wire-lambda parameter body) =
  term-scoped globals locals parameter and term-scoped globals (suc locals) body
term-scoped globals locals (wire-apply function argument) =
  term-scoped globals locals function and term-scoped globals locals argument
term-scoped globals locals wire-unit-type = true
term-scoped globals locals wire-unit = true

body-scoped : Nat → Maybe WireTermV1 → Bool
body-scoped globals nothing = true
body-scoped globals (just body) = term-scoped globals zero body

id-member : WireIdV1 → List GlobalSlotEntryWireV1 → Bool
id-member identifier [] = false
id-member identifier (entry ∷ entries) =
  byte-list-equal identifier (global-id entry) or id-member identifier entries
  where
  _or_ : Bool → Bool → Bool
  true or right = true
  false or right = right

ids-unique : List GlobalSlotEntryWireV1 → Bool
ids-unique [] = true
ids-unique (entry ∷ entries) =
  not (id-member (global-id entry) entries) and ids-unique entries

check-slots-from : Nat → List GlobalSlotEntryWireV1 → Bool
check-slots-from expected [] = true
check-slots-from expected (entry ∷ entries) =
  nat-equal (global-slot entry) expected and
  (id-is-32-bytes (global-id entry) and
  (term-scoped expected zero (declaration-type entry) and
  (body-scoped expected (declaration-body entry) and
   check-slots-from (suc expected) entries)))

check-context-entries : Nat → Nat → List WireTermV1 → Bool
check-context-entries globals ordinal [] = true
check-context-entries globals ordinal (entry ∷ entries) =
  term-scoped globals ordinal entry and
  check-context-entries globals (suc ordinal) entries

check-contexts : Nat → List ProductionContextWireV1 → Bool
check-contexts globals [] = true
check-contexts globals (context ∷ contexts) =
  check-context-entries globals zero (entries-oldest-first context) and
  check-contexts globals contexts

lookup-slot : Nat → List GlobalSlotEntryWireV1 → Maybe GlobalSlotEntryWireV1
lookup-slot slot [] = nothing
lookup-slot zero (entry ∷ entries) = just entry
lookup-slot (suc slot) (entry ∷ entries) = lookup-slot slot entries

has-body : Maybe WireTermV1 → Bool
has-body nothing = false
has-body (just body) = true

delta-entry-valid :
  DeltaPolicyEntryWireV1 → List GlobalSlotEntryWireV1 → Bool
delta-entry-valid delta slots
  with lookup-slot (delta-global-slot delta) slots
... | nothing = false
... | just entry =
  byte-list-equal (delta-global-id delta) (global-id entry) and
  has-body (declaration-body entry)

check-delta-tail :
  Nat → List DeltaPolicyEntryWireV1 → List GlobalSlotEntryWireV1 → Bool
check-delta-tail previous [] slots = true
check-delta-tail previous (delta ∷ deltas) slots =
  nat-less previous (delta-global-slot delta) and
  (id-is-32-bytes (delta-global-id delta) and
  (delta-entry-valid delta slots and
   check-delta-tail (delta-global-slot delta) deltas slots))

check-deltas :
  List DeltaPolicyEntryWireV1 → List GlobalSlotEntryWireV1 → Bool
check-deltas [] slots = true
check-deltas (delta ∷ deltas) slots =
  id-is-32-bytes (delta-global-id delta) and
  (delta-entry-valid delta slots and
   check-delta-tail (delta-global-slot delta) deltas slots)

check-signature :
  ProductionSignatureWireV1 → List GlobalSlotEntryWireV1 → Bool
check-signature signature slots =
  id-is-32-bytes (signature-digest signature) and
  (id-is-32-bytes (kernel-protocol-digest signature) and
  (id-is-32-bytes (global-slot-table-digest signature) and
   check-deltas (allowed-transparent-deltas signature) slots))

check-context-global-structure-v1 : ContextGlobalBundleV1 → Bool
check-context-global-structure-v1 bundle =
  check-manifest-surface-v1 (manifest-surface bundle) and
  (ids-unique (global-slot-table bundle) and
  (check-slots-from zero (global-slot-table bundle) and
  (check-signature (production-signature bundle) (global-slot-table bundle) and
   check-contexts (length (global-slot-table bundle)) (production-contexts bundle))))

record CheckedContextGlobalStructureV1
  (bundle : ContextGlobalBundleV1) : Set where
  constructor checked-context-global-v1
  field
    structure-accepted : check-context-global-structure-v1 bundle ≡ true

open CheckedContextGlobalStructureV1 public

check-context-global-result-v1 :
  (bundle : ContextGlobalBundleV1) →
  (result : Bool) →
  check-context-global-structure-v1 bundle ≡ result →
  Maybe (CheckedContextGlobalStructureV1 bundle)
check-context-global-result-v1 bundle false accepted = nothing
check-context-global-result-v1 bundle true accepted =
  just (checked-context-global-v1 accepted)

check-context-global-v1 :
  (bundle : ContextGlobalBundleV1) → Maybe (CheckedContextGlobalStructureV1 bundle)
check-context-global-v1 bundle =
  check-context-global-result-v1 bundle
    (check-context-global-structure-v1 bundle) refl
