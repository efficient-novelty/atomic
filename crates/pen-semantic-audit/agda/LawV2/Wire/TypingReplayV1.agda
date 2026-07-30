{-# OPTIONS --safe --without-K #-}

-- Phase E typing-judgment bridge, algorithmic layer.
--
-- The semantic replay (`SemanticReplayV1`, `SynthesisReplayV1`) proves
-- that traces reduce and that synthesis recomputes; it never checks
-- that anything is well typed. This module mirrors the kernel's
-- bidirectional discipline (`infer`/`check`/`normalize` in `pen-kernel`
-- `checker.rs`) on the intrinsic production syntax and replays the
-- judgments the kernel replays:
--
-- - `pinfer`/`pcheck` are the fuel-bounded kernel algorithms: variables
--   take their shifted in-context type (`lookup-pctx`), globals their
--   strict-prior declared type, pi/lambda normalize the parameter
--   before extending the context, application destructures the
--   normalized function type and instantiates with the raw argument,
--   and checking compares kernel-normal forms;
-- - `pverify-context` mirrors `verify_context`: each entry must form a
--   type under the normalized prefix, and the normalized context is
--   returned;
-- - `pcheck-signature` mirrors `verify_signature` on the decoded slot
--   table: every declared type forms a type under the strictly prior
--   signature and is its own normal form, and every stored body checks
--   against its declared type and is its own normal form (the wire
--   table must equal the verified signature the kernel would store);
-- - `replay-conversion-typing` mirrors the endpoint replay inside
--   `verify_base_q0_conversion_code_v2`: every trace intermediate is
--   replayed through the endpoint judgment (`HasType` with the one
--   expected type, or `TypeFormation`), every intermediate
--   kernel-normalizes to the common normal form, and the replay-output
--   closure bounds are mirrored. The certified variant returns genuine
--   `PStepsV1` chains under the full stored-body delta for both
--   endpoints plus kernel-delta normality of the common form;
-- - `psynthesize-cert` is the proof-carrying kernel mirror of
--   `verify_code_node`/`finalize_synthesis_node`: every accepted
--   certificate carries a `PSynthesisDerivationV1` derivation, the
--   inductive-relation presentation of the synthesis checker, including
--   semantic conversion mediation under the exact protocol V2 side
--   conditions and the bounded-normalization bridge for dependent
--   application results.
--
-- Fuel is a per-call tree budget (documented deviation from the
-- kernel's shared operation/depth/rewrite budget; both sides fail
-- closed on exhaustion, and exhaustion is never a negative theorem).
-- Nothing here mints authority.

module LawV2.Wire.TypingReplayV1 where

open import Agda.Builtin.Bool using (Bool; false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe
  using ()
  renaming (Maybe to WireMaybe; just to wire-just; nothing to wire-nothing)
open import Agda.Builtin.Nat using (Nat; zero; suc; _-_)
open import LawV2.Wire.Bytes
  using (length; nat-equal; _and_; _++_; _×_; _,_; first; second)
open import LawV2.Wire.ProductionBundleV1
open import LawV2.Wire.ContextChecker
  using (not; nat-less; check-context-entries)
open import LawV2.Wire.BundleChecker
  using (nat-member; step-target; wire-term-list-equal)
open import LawV2.LambdaUnit.ProductionSyntaxV1
open import LawV2.LambdaUnit.ProductionDecodingV1
open import LawV2.Wire.ContextCorrespondenceV1
  using (and-first; and-second; and-intro; pinstantiate; PSig;
         psig-empty; psig-snoc; lookup-psig-type; lookup-psig-body;
         total-from; wire-context-extend; pctx-to-wire)
open import LawV2.Wire.SemanticReplayV1
open import LawV2.Wire.SynthesisReplayV1
  using (boolean-guard; find-conversion; is-type-formation; sort-level;
         pi-parts; fin-checked)
open import LawV2.Wire.NormalizationV1

-- Exact protocol level sets, from `PUBLIC_UNIVERSE_LEVELS_V2` and
-- `CHECKER_OUTPUT_UNIVERSE_LEVELS_V2` in `pen-kernel-synthesis` `v2.rs`
-- (the manifest lists are pinned to the same constants by the
-- structural checker).
public-levels-v1 : List Nat
public-levels-v1 = 0 ∷ 1 ∷ []

checker-levels-v1 : List Nat
checker-levels-v1 = 0 ∷ 1 ∷ 2 ∷ []

-- Sort-level membership over a whole term, mirroring
-- `ensure_term_levels`.
plevels-within :
  List Nat → {globals locals : Nat} → PTm globals locals → Bool
plevels-within allowed (pvar x) = true
plevels-within allowed (psort level) = nat-member level allowed
plevels-within allowed (pglobal slot) = true
plevels-within allowed (ppi parameter body) =
  plevels-within allowed parameter and plevels-within allowed body
plevels-within allowed (plam parameter body) =
  plevels-within allowed parameter and plevels-within allowed body
plevels-within allowed (papp function argument) =
  plevels-within allowed function and plevels-within allowed argument
plevels-within allowed punit-type = true
plevels-within allowed punit = true

pctx-levels-within :
  List Nat → {globals locals : Nat} → PCtx globals locals → Bool
pctx-levels-within allowed pempty = true
pctx-levels-within allowed (context psnoc parameter) =
  pctx-levels-within allowed context and plevels-within allowed parameter

pctx-equal :
  {globals locals : Nat} →
  PCtx globals locals → PCtx globals locals → Bool
pctx-equal pempty pempty = true
pctx-equal (left psnoc left-parameter) (right psnoc right-parameter) =
  pctx-equal left right and ptm-equal left-parameter right-parameter

-- Default tree fuel for the typing layer.
typing-fuel-v1 : Nat
typing-fuel-v1 = 4096

-- `expect_universe`: normalize, then demand a sort. The kernel ignores
-- the context here, and so do we.
pexpect-universe :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} → PTm globals locals → WireMaybe Nat
pexpect-universe sig fuel term =
  wire-maybe-bind (pnormalize (psig-delta sig) fuel term) sort-level

-- The kernel's bidirectional `infer`, on intrinsic terms. Fuel
-- decreases on every recursive call.
pinfer :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} (context : PCtx globals locals) →
  PTm globals locals → WireMaybe (PTm globals locals)

pcompare-normal :
  {globals locals : Nat} →
  WireMaybe (PTm globals locals) → WireMaybe (PTm globals locals) → Bool
pcompare-normal (wire-just left) (wire-just right) = ptm-equal left right
pcompare-normal _ _ = false

-- The kernel's `check`: normalize the inferred and the expected type
-- and compare exactly.
pcheck :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} (context : PCtx globals locals) →
  PTm globals locals → PTm globals locals → Bool
pcheck sig fuel context term expected =
  pcompare-normal
    (wire-maybe-bind (pinfer sig fuel context term)
      (pnormalize (psig-delta sig) fuel))
    (pnormalize (psig-delta sig) fuel expected)

pinfer sig zero context term = wire-nothing
pinfer sig (suc fuel) context (pvar x) =
  wire-just (lookup-pctx context x)
pinfer sig (suc fuel) context (psort level) =
  wire-just (psort (suc level))
pinfer sig (suc fuel) context (pglobal slot) =
  wire-just (pembed-closed (lookup-psig-type sig slot))
pinfer sig (suc fuel) context (ppi parameter body) =
  wire-maybe-bind (pinfer sig fuel context parameter)
    (λ parameter-sort →
  wire-maybe-bind (pexpect-universe sig fuel parameter-sort)
    (λ parameter-level →
  wire-maybe-bind (pnormalize (psig-delta sig) fuel parameter)
    (λ normal-parameter →
  wire-maybe-bind
    (pinfer sig fuel (context psnoc normal-parameter) body)
    (λ body-sort →
  wire-maybe-bind (pexpect-universe sig fuel body-sort) (λ body-level →
  wire-just (psort (parameter-level max body-level)))))))
pinfer sig (suc fuel) context (plam parameter body) =
  wire-maybe-bind (pinfer sig fuel context parameter)
    (λ parameter-sort →
  wire-maybe-bind (pexpect-universe sig fuel parameter-sort) (λ _ →
  wire-maybe-bind (pnormalize (psig-delta sig) fuel parameter)
    (λ normal-parameter →
  wire-maybe-bind
    (pinfer sig fuel (context psnoc normal-parameter) body)
    (λ body-type →
  wire-just (ppi normal-parameter body-type)))))
pinfer sig (suc fuel) context (papp function argument) =
  wire-maybe-bind (pinfer sig fuel context function) (λ function-type →
  wire-maybe-bind (pnormalize (psig-delta sig) fuel function-type)
    (λ normal-function-type →
  wire-maybe-bind (pi-parts normal-function-type) (λ parts →
  boolean-guard (pcheck sig fuel context argument (first parts))
    (wire-just (pinstantiate (second parts) argument)))))
pinfer sig (suc fuel) context punit-type = wire-just (psort zero)
pinfer sig (suc fuel) context punit = wire-just punit-type

-- `verify_context`: each entry forms a type under the normalized
-- prefix; the normalized context is returned.
pverify-context :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} → PCtx globals locals →
  WireMaybe (PCtx globals locals)
pverify-context sig fuel pempty = wire-just pempty
pverify-context sig fuel (context psnoc parameter) =
  wire-maybe-bind (pverify-context sig fuel context) (λ normalized →
  wire-maybe-bind (pinfer sig fuel normalized parameter)
    (λ parameter-sort →
  wire-maybe-bind (pexpect-universe sig fuel parameter-sort) (λ _ →
  wire-maybe-bind (pnormalize (psig-delta sig) fuel parameter)
    (λ normal-parameter →
  wire-just (normalized psnoc normal-parameter)))))

-- `verify_signature` on the decoded chronological slot table: the wire
-- table must be exactly the normalized verified signature the kernel
-- would store, so every declared type and stored body must be its own
-- kernel normal form in addition to being well typed under the strictly
-- prior prefix.
pcheck-signature-entry :
  {count : Nat} (sig : PSig count) (fuel : Nat) →
  PTm count zero → WireMaybe (PTm count zero) → Bool
pcheck-signature-entry sig fuel ty wire-nothing =
  is-wire-just
    (wire-maybe-bind (pinfer sig fuel pempty ty)
      (pexpect-universe sig fuel)) and
  pcompare-normal (pnormalize (psig-delta sig) fuel ty) (wire-just ty)
pcheck-signature-entry sig fuel ty (wire-just body) =
  is-wire-just
    (wire-maybe-bind (pinfer sig fuel pempty ty)
      (pexpect-universe sig fuel)) and
  (pcompare-normal (pnormalize (psig-delta sig) fuel ty) (wire-just ty)
   and
   (pcheck sig fuel pempty body ty and
    pcompare-normal (pnormalize (psig-delta sig) fuel body)
      (wire-just body)))

pcheck-signature : {count : Nat} (fuel : Nat) → PSig count → Bool
pcheck-signature fuel psig-empty = true
pcheck-signature fuel (psig-snoc sig ty body) =
  pcheck-signature fuel sig and pcheck-signature-entry sig fuel ty body

-- Kernel-delta conversion evidence for a replayed conversion: both
-- decoded endpoints reach the decoded common form by genuine intrinsic
-- steps under the full stored-body delta, and the common form is
-- kernel-delta normal (hence admits no outgoing step by
-- `pdelta-normal-no-step`). This complements the policy-delta evidence
-- in `SemanticReplayV1`.
record ConversionKernelEvidenceV1
  {globals : Nat} (sig : PSig globals)
  {locals : Nat}
  (left right common : PTm globals locals) : Set where
  constructor conversion-kernel-evidence
  field
    left-kernel-chain : PStepsV1 (psig-delta sig) left common
    right-kernel-chain : PStepsV1 (psig-delta sig) right common
    common-kernel-normal :
      pdelta-normal (psig-delta sig) common ≡ true

open ConversionKernelEvidenceV1 public

psteps-cast-target :
  {globals : Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {source from to : PTm globals locals} →
  from ≡ to →
  PStepsV1 delta source from → PStepsV1 delta source to
psteps-cast-target refl chain = chain

pdelta-normal-cast :
  {globals : Nat}
  {delta : Fin globals → WireMaybe (PTm globals zero)}
  {locals : Nat} {from to : PTm globals locals} →
  from ≡ to →
  pdelta-normal delta from ≡ true → pdelta-normal delta to ≡ true
pdelta-normal-cast refl proof = proof

private
  normalize-to-branch :
    {globals : Nat} (sig : PSig globals)
    {locals : Nat} {term : PTm globals locals}
    (common : PTm globals locals)
    (result : PNormalizeResultV1 (psig-delta sig) term)
    (matched : Bool) →
    ptm-equal (normal-form result) common ≡ matched →
    WireMaybe
      (PStepsV1 (psig-delta sig) term common ×
       (pdelta-normal (psig-delta sig) common ≡ true))
  normalize-to-branch sig common result false matched-equation =
    wire-nothing
  normalize-to-branch sig common result true matched-equation =
    wire-just
      (psteps-cast-target
        (ptm-equal-sound (normal-form result) common matched-equation)
        (reduction result) ,
       pdelta-normal-cast
        (ptm-equal-sound (normal-form result) common matched-equation)
        (normality result))

-- Normalize `term` and require the result to be exactly `common`,
-- returning the reduction chain and the normality of `common`.
normalize-to-evidence :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} (term common : PTm globals locals) →
  WireMaybe
    (PStepsV1 (psig-delta sig) term common ×
     (pdelta-normal (psig-delta sig) common ≡ true))
normalize-to-evidence sig fuel term common =
  wire-maybe-bind (pnormalize-cert (psig-delta sig) fuel term)
    (λ result →
      normalize-to-branch sig common result
        (ptm-equal (normal-form result) common) refl)

-- Boolean form for the intermediate loop.
pnormalizes-to :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} → PTm globals locals → PTm globals locals → Bool
pnormalizes-to sig fuel term common =
  pcompare-normal
    (pnormalize (psig-delta sig) fuel term) (wire-just common)

-- One trace intermediate under the recorded endpoint judgment,
-- mirroring `verify_open_judgment` inside the conversion replay: the
-- judgment must replay, and the replayed output term must equal the
-- common normal form.
check-intermediate-has-type :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} (context : PCtx globals locals)
  (expected common term : PTm globals locals) → Bool
check-intermediate-has-type sig fuel context expected common term =
  pcheck sig fuel context term expected and
  pnormalizes-to sig fuel term common

check-intermediate-formation :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} (context : PCtx globals locals)
  (common term : PTm globals locals) → Bool
check-intermediate-formation sig fuel context common term =
  is-wire-just
    (wire-maybe-bind (pinfer sig fuel context term)
      (pexpect-universe sig fuel)) and
  pnormalizes-to sig fuel term common

check-intermediates-has-type :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} (context : PCtx globals locals)
  (expected common : PTm globals locals) →
  List (PTm globals locals) → Bool
check-intermediates-has-type sig fuel context expected common [] = true
check-intermediates-has-type sig fuel context expected common
  (term ∷ terms) =
  check-intermediate-has-type sig fuel context expected common term and
  check-intermediates-has-type sig fuel context expected common terms

check-intermediates-formation :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} (context : PCtx globals locals)
  (common : PTm globals locals) → List (PTm globals locals) → Bool
check-intermediates-formation sig fuel context common [] = true
check-intermediates-formation sig fuel context common (term ∷ terms) =
  check-intermediate-formation sig fuel context common term and
  check-intermediates-formation sig fuel context common terms

-- Endpoint dispatch: for `HasType` the expected type must itself form a
-- type (`ty_sort` + `expect_universe` in the kernel replay) and its
-- normal form must satisfy the replay-output closure levels; every
-- intermediate is then checked against it.
check-endpoint-judgment-typing :
  (globals : Nat) {locals-index : Nat}
  (sig : PSig globals) (fuel : Nat)
  (context : PCtx globals locals-index) →
  EndpointJudgmentWireV1 →
  (common : PTm globals locals-index) →
  List (PTm globals locals-index) → Bool
check-endpoint-judgment-typing globals sig fuel context
  endpoint-type-formation common terms =
  check-intermediates-formation sig fuel context common terms
check-endpoint-judgment-typing globals {locals-index} sig fuel context
  (endpoint-has-type raw-expected) common terms =
  expected-branch
    (decode-term-checked globals locals-index raw-expected)
  where
  expected-branch : WireMaybe (PTm globals locals-index) → Bool
  expected-branch wire-nothing = false
  expected-branch (wire-just expected) =
    is-wire-just
      (wire-maybe-bind (pinfer sig fuel context expected)
        (pexpect-universe sig fuel)) and
    (normal-expected-closure
      (pnormalize (psig-delta sig) fuel expected) and
     check-intermediates-has-type sig fuel context expected common
       terms)
    where
    normal-expected-closure : WireMaybe (PTm globals locals-index) → Bool
    normal-expected-closure wire-nothing = false
    normal-expected-closure (wire-just normal-expected) =
      plevels-within checker-levels-v1 normal-expected

decode-term-list-checked :
  (globals locals : Nat) → List WireTermV1 →
  WireMaybe (List (PTm globals locals))
decode-term-list-checked globals locals [] = wire-just []
decode-term-list-checked globals locals (term ∷ terms) =
  wire-maybe-bind (decode-term-checked globals locals term) (λ decoded →
  wire-maybe-bind (decode-term-list-checked globals locals terms)
    (λ rest → wire-just (decoded ∷ rest)))

step-targets-wire : List BaseQ0ReductionStepWireV1 → List WireTermV1
step-targets-wire [] = []
step-targets-wire (step ∷ steps) =
  step-target step ∷ step-targets-wire steps

-- The certified conversion-typing replay result: the decoded surface
-- plus kernel-delta evidence. Every Boolean side condition of the
-- kernel endpoint replay is folded into construction, so acceptance is
-- exactly `wire-just`.
record ConversionTypingReplayV1
  {globals : Nat} (sig : PSig globals) : Set where
  constructor conversion-typing-replay
  field
    replay-locals : Nat
    replay-context : PCtx globals replay-locals
    replay-left replay-right replay-common : PTm globals replay-locals
    replay-evidence :
      ConversionKernelEvidenceV1 sig
        replay-left replay-right replay-common

open ConversionTypingReplayV1 public

private
  conversion-context-branch :
    {globals : Nat} (sig : PSig globals) (fuel : Nat)
    (certificate : ConversionCertificateWireV1) (scoped : Bool) →
    check-context-entries globals zero
      (entries-oldest-first (conversion-context certificate))
    ≡ scoped →
    WireMaybe (ConversionTypingReplayV1 sig)
  conversion-context-branch sig fuel certificate false proof =
    wire-nothing
  conversion-context-branch {globals} sig fuel certificate true proof =
    wire-maybe-bind (pverify-context sig fuel raw-context)
      (λ normalized →
    boolean-guard (pctx-levels-within public-levels-v1 normalized)
    (wire-maybe-bind
      (decode-term-checked globals locals (conversion-left certificate))
      (λ left →
    wire-maybe-bind
      (decode-term-checked globals locals
        (conversion-right certificate))
      (λ right →
    wire-maybe-bind
      (decode-term-checked globals locals
        (conversion-common-normal-form certificate))
      (λ common →
    wire-maybe-bind
      (decode-term-list-checked globals locals
        (step-targets-wire
          (trace-steps (conversion-left-trace certificate))))
      (λ left-targets →
    wire-maybe-bind
      (decode-term-list-checked globals locals
        (step-targets-wire
          (trace-steps (conversion-right-trace certificate))))
      (λ right-targets →
    boolean-guard
      (check-endpoint-judgment-typing globals sig fuel normalized
        (conversion-endpoint certificate) common
        (left ∷ (left-targets ++ (right ∷ right-targets))))
    (boolean-guard (plevels-within checker-levels-v1 common)
    (wire-maybe-bind (normalize-to-evidence sig fuel left common)
      (λ left-evidence →
    wire-maybe-bind (normalize-to-evidence sig fuel right common)
      (λ right-evidence →
    wire-just
      (conversion-typing-replay locals raw-context left right common
        (conversion-kernel-evidence
          (first left-evidence)
          (first right-evidence)
          (second left-evidence))))))))))))))
    where
    entries : List WireTermV1
    entries = entries-oldest-first (conversion-context certificate)
    locals : Nat
    locals = total-from zero entries
    raw-context : PCtx globals locals
    raw-context = wire-context-extend zero pempty entries proof

replay-conversion-typing-cert :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  (certificate : ConversionCertificateWireV1) →
  WireMaybe (ConversionTypingReplayV1 sig)
replay-conversion-typing-cert {globals} sig fuel certificate =
  conversion-context-branch sig fuel certificate
    (check-context-entries globals zero
      (entries-oldest-first (conversion-context certificate)))
    refl

replay-conversion-typing :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  (certificate : ConversionCertificateWireV1) → Bool
replay-conversion-typing sig fuel certificate =
  is-wire-just (replay-conversion-typing-cert sig fuel certificate)

-- The inductive-relation presentation of the typed synthesis checker,
-- mirroring the conversion side's `PStepV1` package: every constructor
-- is one `verify_code_node` rule with its semantic premises —
-- sub-derivations, policy-delta conversion mediation
-- (`ConversionReplayEvidenceV1`, protocol V2 side conditions expressed
-- by the indices: left endpoint equal to the synthesized type, right
-- endpoint equal to the census-normal common form), kernel-delta
-- normality of binder parameters (the kernel's parameter
-- self-normality replay), and the bounded-normalization bridge for the
-- dependent application result (the recorded type is reached from the
-- raw intrinsic instantiation by genuine kernel-delta steps and is
-- itself kernel-delta normal).
data PSynthesisDerivationV1
  {globals : Nat} (enabled : List Nat) (sig : PSig globals) :
  {locals : Nat} → PCtx globals locals →
  PTm globals locals → PTm globals locals → Set where

  psynth-sort :
    {locals : Nat} {context : PCtx globals locals} (level : Nat) →
    PSynthesisDerivationV1 enabled sig context
      (psort level) (psort (suc level))

  psynth-unit-type :
    {locals : Nat} {context : PCtx globals locals} →
    PSynthesisDerivationV1 enabled sig context punit-type (psort zero)

  psynth-unit :
    {locals : Nat} {context : PCtx globals locals} →
    PSynthesisDerivationV1 enabled sig context punit punit-type

  psynth-variable :
    {locals : Nat} {context : PCtx globals locals} (x : Fin locals) →
    PSynthesisDerivationV1 enabled sig context
      (pvar x) (lookup-pctx context x)

  psynth-global :
    {locals : Nat} {context : PCtx globals locals}
    (slot : Fin globals) →
    PSynthesisDerivationV1 enabled sig context
      (pglobal slot) (pembed-closed (lookup-psig-type sig slot))

  psynth-pi :
    {locals : Nat} {context : PCtx globals locals}
    {parameter : PTm globals locals}
    {body : PTm globals (suc locals)}
    {parameter-level body-level : Nat} →
    PSynthesisDerivationV1 enabled sig context
      parameter (psort parameter-level) →
    pdelta-normal (psig-delta sig) parameter ≡ true →
    PSynthesisDerivationV1 enabled sig (context psnoc parameter)
      body (psort body-level) →
    PSynthesisDerivationV1 enabled sig context
      (ppi parameter body) (psort (parameter-level max body-level))

  psynth-lambda :
    {locals : Nat} {context : PCtx globals locals}
    {parameter : PTm globals locals}
    {body body-type : PTm globals (suc locals)}
    {parameter-level : Nat} →
    PSynthesisDerivationV1 enabled sig context
      parameter (psort parameter-level) →
    pdelta-normal (psig-delta sig) parameter ≡ true →
    PSynthesisDerivationV1 enabled sig (context psnoc parameter)
      body body-type →
    PSynthesisDerivationV1 enabled sig context
      (plam parameter body) (ppi parameter body-type)

  psynth-application :
    {locals : Nat} {context : PCtx globals locals}
    {function argument function-type argument-type :
      PTm globals locals}
    {pi-parameter : PTm globals locals}
    {pi-body : PTm globals (suc locals)}
    {result-type : PTm globals locals} →
    PSynthesisDerivationV1 enabled sig context function function-type →
    PSynthesisDerivationV1 enabled sig context argument argument-type →
    ConversionReplayEvidenceV1 enabled sig
      function-type (ppi pi-parameter pi-body)
      (ppi pi-parameter pi-body) →
    ConversionReplayEvidenceV1 enabled sig
      argument-type pi-parameter pi-parameter →
    PStepsV1 (psig-delta sig)
      (pinstantiate pi-body argument) result-type →
    pdelta-normal (psig-delta sig) result-type ≡ true →
    PSynthesisDerivationV1 enabled sig context
      (papp function argument) result-type

record PSynthesisResultV1
  {globals : Nat} (enabled : List Nat) (sig : PSig globals)
  {locals : Nat} (context : PCtx globals locals) : Set where
  constructor psynthesis-result
  field
    result-term : PTm globals locals
    result-type : PTm globals locals
    result-derivation :
      PSynthesisDerivationV1 enabled sig context
        result-term result-type

open PSynthesisResultV1 public

-- `finalize_synthesis_node`: replay the raw inferred type as a
-- `TypeFormation` judgment, take the kernel-normal type, and replay the
-- subject against it as a `HasType` judgment, with the replay-output
-- closure bounds (`ensure_public_term` on the subject,
-- `ensure_checker_output_term` and `ensure_replay_output_closure` on
-- the outputs). The returned certificate carries the reduction chain
-- from the raw type to the kernel-normal type.
pfinalize-cert :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} (context : PCtx globals locals)
  (term raw-type : PTm globals locals) →
  WireMaybe (PNormalizeResultV1 (psig-delta sig) raw-type)
pfinalize-cert sig fuel context term raw-type =
  boolean-guard (plevels-within public-levels-v1 term)
  (wire-maybe-bind
    (wire-maybe-bind (pinfer sig fuel context raw-type)
      (pexpect-universe sig fuel))
    (λ _ →
  wire-maybe-bind (pnormalize-cert (psig-delta sig) fuel raw-type)
    (λ normalized-result →
  boolean-guard
    (plevels-within checker-levels-v1 (normal-form normalized-result))
  (wire-maybe-bind
    (wire-maybe-bind
      (pinfer sig fuel context (normal-form normalized-result))
      (pexpect-universe sig fuel))
    (λ _ →
  boolean-guard
    (pcheck sig fuel context term (normal-form normalized-result))
  (wire-maybe-bind (pnormalize (psig-delta sig) fuel term)
    (λ normal-term →
  boolean-guard (plevels-within checker-levels-v1 normal-term)
    (wire-just normalized-result))))))))

private
  self-normal-branch :
    {globals : Nat} (sig : PSig globals)
    {locals : Nat} (term : PTm globals locals)
    (result : PNormalizeResultV1 (psig-delta sig) term)
    (matched : Bool) →
    ptm-equal (normal-form result) term ≡ matched →
    WireMaybe (pdelta-normal (psig-delta sig) term ≡ true)
  self-normal-branch sig term result false matched-equation =
    wire-nothing
  self-normal-branch sig term result true matched-equation =
    wire-just
      (pdelta-normal-cast
        (ptm-equal-sound (normal-form result) term matched-equation)
        (normality result))

-- The kernel's parameter self-normality replay
-- (`replay_parameter_type_formation` plus the
-- `normal_parameter == parameter_node.term()` comparison), returning
-- kernel-delta normality of the term itself.
pself-normal-evidence :
  {globals : Nat} (sig : PSig globals) (fuel : Nat)
  {locals : Nat} (term : PTm globals locals) →
  WireMaybe (pdelta-normal (psig-delta sig) term ≡ true)
pself-normal-evidence sig fuel term =
  wire-maybe-bind (pnormalize-cert (psig-delta sig) fuel term)
    (λ result →
      self-normal-branch sig term result
        (ptm-equal (normal-form result) term) refl)

-- `expect_sort_v2`: the recorded node type must be a literal sort at a
-- checker-output level. Matching the result record refines the
-- derivation's type index to that sort.
expect-sort-result :
  {A : Set} {globals : Nat}
  {enabled : List Nat} {sig : PSig globals}
  {locals : Nat} {context : PCtx globals locals} →
  PSynthesisResultV1 enabled sig context →
  ((level : Nat) (term : PTm globals locals) →
    PSynthesisDerivationV1 enabled sig context term (psort level) →
    WireMaybe A) →
  WireMaybe A
expect-sort-result
  (psynthesis-result term (psort level) derivation) continue =
  boolean-guard (nat-member level checker-levels-v1)
    (continue level term derivation)
expect-sort-result (psynthesis-result term (pvar _) _) continue =
  wire-nothing
expect-sort-result (psynthesis-result term (pglobal _) _) continue =
  wire-nothing
expect-sort-result (psynthesis-result term (ppi _ _) _) continue =
  wire-nothing
expect-sort-result (psynthesis-result term (plam _ _) _) continue =
  wire-nothing
expect-sort-result (psynthesis-result term (papp _ _) _) continue =
  wire-nothing
expect-sort-result (psynthesis-result term punit-type _) continue =
  wire-nothing
expect-sort-result (psynthesis-result term punit _) continue =
  wire-nothing

-- Resolve one mediating conversion under the protocol V2 side
-- conditions and return its census-normal common form together with the
-- policy-delta replay evidence, indexed at the synthesized type.
record ResolvedConversionV1
  {globals : Nat} (enabled : List Nat) (sig : PSig globals)
  {locals : Nat} (synthesized : PTm globals locals) : Set where
  constructor resolved-conversion
  field
    resolved-common : PTm globals locals
    resolved-evidence :
      ConversionReplayEvidenceV1 enabled sig
        synthesized resolved-common resolved-common

open ResolvedConversionV1 public

private
  cast-conversion-evidence :
    {globals : Nat} {enabled : List Nat} {sig : PSig globals}
    {locals : Nat}
    {left synthesized right common : PTm globals locals} →
    left ≡ synthesized → right ≡ common →
    ConversionReplayEvidenceV1 enabled sig left right common →
    ConversionReplayEvidenceV1 enabled sig synthesized common common
  cast-conversion-evidence refl refl evidence = evidence

  resolve-branch :
    {globals : Nat} (enabled : List Nat) (sig : PSig globals)
    {locals : Nat}
    (synthesized left right common : PTm globals locals)
    (certificate : ConversionCertificateWireV1)
    (replayed : Bool) →
    replay-conversion-endpoints
      (policy-body enabled sig)
      (λ slot → nat-member slot enabled)
      left right common certificate ≡ replayed →
    (left-matches : Bool) →
    ptm-equal left synthesized ≡ left-matches →
    (right-matches : Bool) →
    ptm-equal right common ≡ right-matches →
    WireMaybe (ResolvedConversionV1 enabled sig synthesized)
  resolve-branch enabled sig synthesized left right common certificate
    false replay-equation left-matches left-equation right-matches
    right-equation = wire-nothing
  resolve-branch enabled sig synthesized left right common certificate
    true replay-equation false left-equation right-matches
    right-equation = wire-nothing
  resolve-branch enabled sig synthesized left right common certificate
    true replay-equation true left-equation false right-equation =
    wire-nothing
  resolve-branch enabled sig synthesized left right common certificate
    true replay-equation true left-equation true right-equation =
    wire-just
      (resolved-conversion common
        (cast-conversion-evidence
          (ptm-equal-sound left synthesized left-equation)
          (ptm-equal-sound right common right-equation)
          (replay-conversion-endpoints-sound enabled sig
            left right common certificate replay-equation)))

use-conversion-cert :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1)
  (locals : Nat) (context-wire : List WireTermV1)
  (identifier : WireIdV1) (synthesized : PTm globals locals) →
  WireMaybe (ResolvedConversionV1 enabled sig synthesized)
use-conversion-cert globals enabled sig conversions locals context-wire
  identifier synthesized =
  wire-maybe-bind (find-conversion identifier conversions)
    (λ certificate →
  boolean-guard
    (wire-term-list-equal
      (entries-oldest-first (conversion-context certificate))
      context-wire)
  (boolean-guard
    (is-type-formation (conversion-endpoint certificate))
  (wire-maybe-bind
    (decode-term-checked globals locals (conversion-left certificate))
    (λ left →
  wire-maybe-bind
    (decode-term-checked globals locals (conversion-right certificate))
    (λ right →
  wire-maybe-bind
    (decode-term-checked globals locals
      (conversion-common-normal-form certificate))
    (λ common →
  resolve-branch enabled sig synthesized left right common certificate
    (replay-conversion-endpoints
      (policy-body enabled sig)
      (λ slot → nat-member slot enabled)
      left right common certificate) refl
    (ptm-equal left synthesized) refl
    (ptm-equal right common) refl))))))

-- Match the resolved common form as a dependent product, refining the
-- evidence's index, exactly as the kernel destructures the pi from the
-- census-normal common form.
expect-pi-resolved :
  {A : Set} {globals : Nat}
  {enabled : List Nat} {sig : PSig globals}
  {locals : Nat} {synthesized : PTm globals locals} →
  ResolvedConversionV1 enabled sig synthesized →
  ((pi-parameter : PTm globals locals)
   (pi-body : PTm globals (suc locals)) →
   ConversionReplayEvidenceV1 enabled sig synthesized
     (ppi pi-parameter pi-body) (ppi pi-parameter pi-body) →
   WireMaybe A) →
  WireMaybe A
expect-pi-resolved
  (resolved-conversion (ppi pi-parameter pi-body) evidence) continue =
  continue pi-parameter pi-body evidence
expect-pi-resolved (resolved-conversion (pvar _) _) continue =
  wire-nothing
expect-pi-resolved (resolved-conversion (psort _) _) continue =
  wire-nothing
expect-pi-resolved (resolved-conversion (pglobal _) _) continue =
  wire-nothing
expect-pi-resolved (resolved-conversion (plam _ _) _) continue =
  wire-nothing
expect-pi-resolved (resolved-conversion (papp _ _) _) continue =
  wire-nothing
expect-pi-resolved (resolved-conversion punit-type _) continue =
  wire-nothing
expect-pi-resolved (resolved-conversion punit _) continue =
  wire-nothing

private
  argument-common-branch :
    {globals : Nat} {enabled : List Nat} {sig : PSig globals}
    {locals : Nat}
    {argument-type : PTm globals locals}
    (pi-parameter : PTm globals locals)
    (resolved : ResolvedConversionV1 enabled sig argument-type)
    (matched : Bool) →
    ptm-equal (resolved-common resolved) pi-parameter ≡ matched →
    WireMaybe
      (ConversionReplayEvidenceV1 enabled sig
        argument-type pi-parameter pi-parameter)
  argument-common-branch pi-parameter resolved false matched-equation =
    wire-nothing
  argument-common-branch pi-parameter
    (resolved-conversion common evidence) true matched-equation =
    wire-just
      (cast-argument-evidence
        (ptm-equal-sound common pi-parameter matched-equation)
        evidence)
    where
    cast-argument-evidence :
      {globals : Nat} {enabled : List Nat} {sig : PSig globals}
      {locals : Nat}
      {argument-type from to : PTm globals locals} →
      from ≡ to →
      ConversionReplayEvidenceV1 enabled sig argument-type from from →
      ConversionReplayEvidenceV1 enabled sig argument-type to to
    cast-argument-evidence refl evidence = evidence

  dependent-result-branch :
    {globals : Nat} {enabled : List Nat} {sig : PSig globals}
    {locals : Nat} {context : PCtx globals locals}
    {function argument function-type argument-type :
      PTm globals locals}
    {pi-parameter : PTm globals locals}
    {pi-body : PTm globals (suc locals)}
    (recorded : PTm globals locals)
    (function-derivation :
      PSynthesisDerivationV1 enabled sig context
        function function-type)
    (argument-derivation :
      PSynthesisDerivationV1 enabled sig context
        argument argument-type)
    (function-evidence :
      ConversionReplayEvidenceV1 enabled sig
        function-type (ppi pi-parameter pi-body)
        (ppi pi-parameter pi-body))
    (argument-evidence :
      ConversionReplayEvidenceV1 enabled sig
        argument-type pi-parameter pi-parameter)
    (normalized-result :
      PNormalizeResultV1 (psig-delta sig)
        (pinstantiate pi-body argument))
    (matched : Bool) →
    ptm-equal (normal-form normalized-result) recorded ≡ matched →
    WireMaybe
      (PSynthesisResultV1 enabled sig context)
  dependent-result-branch recorded function-derivation
    argument-derivation function-evidence argument-evidence
    normalized-result false matched-equation = wire-nothing
  dependent-result-branch
    {function = function} {argument = argument}
    recorded function-derivation
    argument-derivation function-evidence argument-evidence
    normalized-result true matched-equation =
    wire-just
      (psynthesis-result (papp function argument)
        (normal-form normalized-result)
        (psynth-application function-derivation argument-derivation
          function-evidence argument-evidence
          (reduction normalized-result)
          (normality normalized-result)))

-- The proof-carrying kernel mirror of `verify_code_node`. The context
-- argument is the verified (self-normal) context; sub-nodes extend it
-- with kernel-normal parameters exactly as `extend_verified_context`
-- does.
psynthesize-cert :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1) (fuel : Nat)
  {locals : Nat} (context : PCtx globals locals) →
  SynthesisCodeWireV1 →
  WireMaybe (PSynthesisResultV1 enabled sig context)

private
  finalize-exact :
    {globals : Nat} {enabled : List Nat} (sig : PSig globals)
    (fuel : Nat)
    {locals : Nat} (context : PCtx globals locals)
    (term raw-type : PTm globals locals) →
    PSynthesisDerivationV1 enabled sig context term raw-type →
    WireMaybe (PSynthesisResultV1 enabled sig context)
  finalize-exact sig fuel context term raw-type derivation =
    wire-maybe-bind (pfinalize-cert sig fuel context term raw-type)
      (λ normalized-result →
    boolean-guard (ptm-equal (normal-form normalized-result) raw-type)
      (wire-just (psynthesis-result term raw-type derivation)))

psynthesize-cert globals enabled sig conversions fuel context
  (code-sort level) =
  boolean-guard (nat-member level public-levels-v1)
    (finalize-exact sig fuel context (psort level) (psort (suc level))
      (psynth-sort level))
psynthesize-cert globals enabled sig conversions fuel context
  code-unit-type =
  finalize-exact sig fuel context punit-type (psort zero)
    psynth-unit-type
psynthesize-cert globals enabled sig conversions fuel context
  code-unit =
  finalize-exact sig fuel context punit punit-type psynth-unit
psynthesize-cert globals enabled sig conversions fuel {locals} context
  (code-variable-lookup index ordinal shift) =
  boolean-guard (nat-equal ordinal (locals - suc index))
  (boolean-guard (nat-equal shift (suc index))
  (wire-maybe-bind (fin-checked index locals) (λ x →
    finalize-exact sig fuel context (pvar x) (lookup-pctx context x)
      (psynth-variable x))))
psynthesize-cert globals enabled sig conversions fuel context
  (code-global-lookup slot) =
  wire-maybe-bind (fin-checked slot globals) (λ x →
  -- `ensure_checker_output_term(declaration.ty)`: sort levels are
  -- unaffected by the closed embedding, so the check runs on the
  -- stored declaration directly.
  boolean-guard
    (plevels-within checker-levels-v1 (lookup-psig-type sig x))
    (finalize-exact sig fuel context (pglobal x)
      (pembed-closed (lookup-psig-type sig x)) (psynth-global x)))
psynthesize-cert globals enabled sig conversions fuel context
  (code-pi-formation parameter-code body-code) =
  wire-maybe-bind
    (psynthesize-cert globals enabled sig conversions fuel context
      parameter-code)
    (λ parameter-result →
  expect-sort-result parameter-result
    (λ parameter-level parameter-term parameter-derivation →
  wire-maybe-bind (pself-normal-evidence sig fuel parameter-term)
    (λ parameter-normal →
  wire-maybe-bind
    (psynthesize-cert globals enabled sig conversions fuel
      (context psnoc parameter-term) body-code)
    (λ body-result →
  expect-sort-result body-result
    (λ body-level body-term body-derivation →
  finalize-exact sig fuel context (ppi parameter-term body-term)
    (psort (parameter-level max body-level))
    (psynth-pi parameter-derivation parameter-normal
      body-derivation))))))
psynthesize-cert globals enabled sig conversions fuel context
  (code-lambda-introduction parameter-code body-code) =
  wire-maybe-bind
    (psynthesize-cert globals enabled sig conversions fuel context
      parameter-code)
    (λ parameter-result →
  expect-sort-result parameter-result
    (λ parameter-level parameter-term parameter-derivation →
  wire-maybe-bind (pself-normal-evidence sig fuel parameter-term)
    (λ parameter-normal →
  wire-maybe-bind
    (psynthesize-cert globals enabled sig conversions fuel
      (context psnoc parameter-term) body-code)
    (λ body-result →
  finalize-exact sig fuel context
    (plam parameter-term (result-term body-result))
    (ppi parameter-term (result-type body-result))
    (psynth-lambda parameter-derivation parameter-normal
      (result-derivation body-result))))))
psynthesize-cert globals enabled sig conversions fuel {locals} context
  (code-application-elimination function-code argument-code
    function-conversion argument-conversion dependent-result) =
  wire-maybe-bind
    (psynthesize-cert globals enabled sig conversions fuel context
      function-code)
    (λ function-result →
  wire-maybe-bind
    (psynthesize-cert globals enabled sig conversions fuel context
      argument-code)
    (λ argument-result →
  wire-maybe-bind
    (use-conversion-cert globals enabled sig conversions locals
      (pctx-to-wire context) function-conversion
      (result-type function-result))
    (λ function-resolved →
  expect-pi-resolved function-resolved
    (λ pi-parameter pi-body function-evidence →
  wire-maybe-bind
    (use-conversion-cert globals enabled sig conversions locals
      (pctx-to-wire context) argument-conversion
      (result-type argument-result))
    (λ argument-resolved →
  wire-maybe-bind
    (argument-common-branch pi-parameter argument-resolved
      (ptm-equal (resolved-common argument-resolved) pi-parameter)
      refl)
    (λ argument-evidence →
  boolean-guard
    (is-wire-just
      (wire-maybe-bind (pinfer sig fuel context pi-parameter)
        (pexpect-universe sig fuel)))
  (boolean-guard
    (pcheck sig fuel context (result-term argument-result)
      pi-parameter)
  (wire-maybe-bind
    (decode-term-checked globals locals dependent-result)
    (λ recorded →
  wire-maybe-bind
    (pfinalize-cert sig fuel context
      (papp (result-term function-result)
        (result-term argument-result))
      (pinstantiate pi-body (result-term argument-result)))
    (λ normalized-result →
  dependent-result-branch recorded
    (result-derivation function-result)
    (result-derivation argument-result)
    function-evidence argument-evidence normalized-result
    (ptm-equal (normal-form normalized-result) recorded)
    refl))))))))))

-- Certificate-level typed replay: the wire context must be exactly its
-- own kernel verification (`verified_context.normalized_wire() ==
-- *context` in `verify_synthesis_code_v2`) with public levels, and the
-- recomputed subject and kernel-normal type must equal the recorded
-- subject and inferred type.
private
  synthesis-context-branch :
    (globals : Nat) (enabled : List Nat) (sig : PSig globals)
    (conversions : List ConversionCertificateWireV1) (fuel : Nat)
    (certificate : SynthesisCertificateWireV1) (scoped : Bool) →
    check-context-entries globals zero
      (entries-oldest-first (synthesis-context certificate))
    ≡ scoped →
    Bool
  synthesis-context-branch globals enabled sig conversions fuel
    certificate false proof = false
  synthesis-context-branch globals enabled sig conversions fuel
    certificate true proof =
    verified-branch (pverify-context sig fuel raw-context)
    where
    entries : List WireTermV1
    entries = entries-oldest-first (synthesis-context certificate)
    locals : Nat
    locals = total-from zero entries
    raw-context : PCtx globals locals
    raw-context = wire-context-extend zero pempty entries proof
    result-branch :
      WireMaybe (PSynthesisResultV1 enabled sig raw-context) →
      WireMaybe (PTm globals locals) →
      WireMaybe (PTm globals locals) → Bool
    result-branch (wire-just result) (wire-just subject)
      (wire-just inferred) =
      ptm-equal (result-term result) subject and
      ptm-equal (result-type result) inferred
    result-branch _ _ _ = false
    verified-branch : WireMaybe (PCtx globals locals) → Bool
    verified-branch wire-nothing = false
    verified-branch (wire-just normalized) =
      pctx-equal normalized raw-context and
      (pctx-levels-within public-levels-v1 raw-context and
       result-branch
         (psynthesize-cert globals enabled sig conversions fuel
           raw-context (synthesis-code certificate))
         (decode-term-checked globals locals
           (synthesis-subject certificate))
         (decode-term-checked globals locals
           (synthesis-inferred-type certificate)))

replay-synthesis-typed :
  (globals : Nat) (enabled : List Nat) (sig : PSig globals)
  (conversions : List ConversionCertificateWireV1) (fuel : Nat)
  (certificate : SynthesisCertificateWireV1) → Bool
replay-synthesis-typed globals enabled sig conversions fuel
  certificate =
  synthesis-context-branch globals enabled sig conversions fuel
    certificate
    (check-context-entries globals zero
      (entries-oldest-first (synthesis-context certificate)))
    refl
