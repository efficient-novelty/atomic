module StepWitness where

open import Agda.Primitive using (Set)

open import AbstractionBarrier using (AbstractionBarrier; PublicInterface; SealedLayer)
open import BridgePayload using (BridgePayload)
-- Keep the conditional L1/L2 cardinality proof in every self-contained
-- exported bundle.  The imported module documents (and types) the semantic
-- hypotheses; importing it here makes Agda verify it alongside each step.
open import CountingLemmas using (l1-basis-size; l2-basis-size)
-- Extraction-guarded provenance is an injection into charged local roles or
-- previously live demand tags, plus per-schema validity of the assigned
-- anchor. Its debt-free specialization is bar-blind.
open import ProvenanceBound using
  (provenance-at-most; debt-free-local-at-most-nine)
-- Keep the P5 theorem-domain audit and conditional internality boundary in
-- every self-contained export as well.
open import P5RecordBoundary using
  (no-survivor-dominant; transparent-internality;
   guarded-step15-flow-internality)
-- The certified Step-16 support-local bound is part of each exported audit
-- bundle.  Its semantic injection remains an explicit proof obligation.
open import CertifiedHalt using
  (bar16-above-nine; support-local-below-nine; support-local-below-bar16;
   debt-free-step16-at-most-four; debt-free-step16-at-most-nine;
   nine-kappa-below-bar16;
   class-ceiling-below-maximum; certified-maximum-below-bar16)
-- Phase 4 of the semantic normalization program: the typed ValidAnchor
-- witnesses and debt-free inputs that DISCHARGE the conditional AtMost
-- boundary for the four certified falsifiers (AtMost Marginal (4 * kappa)
-- per stratum) and the identity TransparentExtension instantiating the
-- guarded Step-15 internality theorem (AtMost Marginal zero). Importing
-- them here makes every exported bundle's typecheck re-verify the
-- discharged theorems.
open import ProvenanceWitness16 using
  (hit-at-most-four-kappa; temporal-at-most-four-kappa;
   single-l15-at-most-four-kappa; inheritance-at-most-four-kappa;
   guarded-step15-marginal-zero)

record StepWitness (payload : BridgePayload) (carrier : Set) : Set1 where
  constructor mkStepWitness
  field
    sealedLayer : SealedLayer
    barrier : AbstractionBarrier payload sealedLayer
    exportCarrier :
      carrier -> PublicInterface.schema (SealedLayer.publicInterface sealedLayer)
