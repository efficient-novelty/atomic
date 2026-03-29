module BridgePayload where

open import Agda.Primitive using (Set)

record NuClaim : Set1 where
  constructor mkNuClaim
  field
    nuGWellFormed : Set
    nuHWellFormed : Set
    nuCWellFormed : Set
    nuTotalConsistent : Set

record BridgePayload : Set1 where
  constructor mkBridgePayload
  field
    canonicalKeyContract : Set
    decodeNonInterferenceContract : Set
    importFootprintOpaqueContract : Set
    nuClaimContract : NuClaim

record ContractWitness (payload : BridgePayload) : Set1 where
  constructor mkContractWitness
  field
    canonicalKeySound : BridgePayload.canonicalKeyContract payload
    decodeNonInterference : BridgePayload.decodeNonInterferenceContract payload
    importFootprintOpaque : BridgePayload.importFootprintOpaqueContract payload
    nuClaimWellFormed :
      NuClaim.nuTotalConsistent (BridgePayload.nuClaimContract payload)
