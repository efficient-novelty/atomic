{-# OPTIONS --safe --without-K #-}

module LawV2.Wire.Envelope where

open import Agda.Builtin.Bool using (false; true)
open import Agda.Builtin.Equality using (_≡_; refl)
open import Agda.Builtin.List using (List; []; _∷_)
open import Agda.Builtin.Maybe using (Maybe; just; nothing)
open import LawV2.Wire.Bytes
open import LawV2.Wire.Decoder

-- ASCII `PEN-PROD-WIRE-V1`, expressed intrinsically as sixteen bytes.
byte45 byte49 byte68 byte69 byte73 byte78 : Byte
byte79 byte80 byte82 byte86 byte87 : Byte
byte45 = bits true false true true false true false false
byte49 = bits true false false false true true false false
byte68 = bits false false true false false false true false
byte69 = bits true false true false false false true false
byte73 = bits true false false true false false true false
byte78 = bits false true true true false false true false
byte79 = bits true true true true false false true false
byte80 = bits false false false false true false true false
byte82 = bits false true false false true false true false
byte86 = bits false true true false true false true false
byte87 = bits true true true false true false true false

production-magic-v1 : List Byte
production-magic-v1 =
  byte80 ∷ byte69 ∷ byte78 ∷ byte45 ∷
  byte80 ∷ byte82 ∷ byte79 ∷ byte68 ∷
  byte45 ∷ byte87 ∷ byte73 ∷ byte82 ∷
  byte69 ∷ byte45 ∷ byte86 ∷ byte49 ∷ []

production-header-v1 : List Byte
production-header-v1 =
  production-magic-v1 ++
  (byte1 ∷ byte0 ∷ byte11 ∷ byte0 ∷ [])

record RawProductionEnvelopeV1 : Set where
  constructor raw-envelope-v1
  field
    manifest-section : U64 × List Byte
    signature-section : U64 × List Byte
    global-slots-section : U64 × List Byte
    contexts-section : U64 × List Byte
    conversions-section : U64 × List Byte
    conversion-supplements-section : U64 × List Byte
    synthesis-section : U64 × List Byte
    q0-inventory-section : U64 × List Byte
    fresh-rules-section : U64 × List Byte
    family-inventory-section : U64 × List Byte
    family-payloads-section : U64 × List Byte

open RawProductionEnvelopeV1 public

record CanonicalProductionEnvelopeV1 : Set where
  constructor canonical-envelope-v1
  field
    canonical-manifest-section : CanonicalSizedBytes
    canonical-signature-section : CanonicalSizedBytes
    canonical-global-slots-section : CanonicalSizedBytes
    canonical-contexts-section : CanonicalSizedBytes
    canonical-conversions-section : CanonicalSizedBytes
    canonical-conversion-supplements-section : CanonicalSizedBytes
    canonical-synthesis-section : CanonicalSizedBytes
    canonical-q0-inventory-section : CanonicalSizedBytes
    canonical-fresh-rules-section : CanonicalSizedBytes
    canonical-family-inventory-section : CanonicalSizedBytes
    canonical-family-payloads-section : CanonicalSizedBytes

open CanonicalProductionEnvelopeV1 public

raw-sized : CanonicalSizedBytes → U64 × List Byte
raw-sized section = encoded-length section , payload section

erase-envelope-v1 : CanonicalProductionEnvelopeV1 → RawProductionEnvelopeV1
erase-envelope-v1 envelope =
  raw-envelope-v1
    (raw-sized (canonical-manifest-section envelope))
    (raw-sized (canonical-signature-section envelope))
    (raw-sized (canonical-global-slots-section envelope))
    (raw-sized (canonical-contexts-section envelope))
    (raw-sized (canonical-conversions-section envelope))
    (raw-sized (canonical-conversion-supplements-section envelope))
    (raw-sized (canonical-synthesis-section envelope))
    (raw-sized (canonical-q0-inventory-section envelope))
    (raw-sized (canonical-fresh-rules-section envelope))
    (raw-sized (canonical-family-inventory-section envelope))
    (raw-sized (canonical-family-payloads-section envelope))

encode-section : Byte → CanonicalSizedBytes → List Byte
encode-section tag section = tag ∷ encode-sized-bytes section

encode-envelope-v1 : CanonicalProductionEnvelopeV1 → List Byte
encode-envelope-v1 envelope =
  production-header-v1 ++
  encode-section byte1 (canonical-manifest-section envelope) ++
  encode-section byte2 (canonical-signature-section envelope) ++
  encode-section byte3 (canonical-global-slots-section envelope) ++
  encode-section byte4 (canonical-contexts-section envelope) ++
  encode-section byte5 (canonical-conversions-section envelope) ++
  encode-section byte6 (canonical-conversion-supplements-section envelope) ++
  encode-section byte7 (canonical-synthesis-section envelope) ++
  encode-section byte8 (canonical-q0-inventory-section envelope) ++
  encode-section byte9 (canonical-fresh-rules-section envelope) ++
  encode-section byte10 (canonical-family-inventory-section envelope) ++
  encode-section byte11 (canonical-family-payloads-section envelope) ++ []

parse-section : Byte → Decoder (U64 × List Byte)
parse-section tag =
  expect-byte tag >>= λ _ →
  parse-sized-bytes

parse-envelope-v1 : Decoder RawProductionEnvelopeV1
parse-envelope-v1 =
  expect-byte byte80 >>= λ _ →
  expect-byte byte69 >>= λ _ →
  expect-byte byte78 >>= λ _ →
  expect-byte byte45 >>= λ _ →
  expect-byte byte80 >>= λ _ →
  expect-byte byte82 >>= λ _ →
  expect-byte byte79 >>= λ _ →
  expect-byte byte68 >>= λ _ →
  expect-byte byte45 >>= λ _ →
  expect-byte byte87 >>= λ _ →
  expect-byte byte73 >>= λ _ →
  expect-byte byte82 >>= λ _ →
  expect-byte byte69 >>= λ _ →
  expect-byte byte45 >>= λ _ →
  expect-byte byte86 >>= λ _ →
  expect-byte byte49 >>= λ _ →
  expect-byte byte1 >>= λ _ →
  expect-byte byte0 >>= λ _ →
  expect-byte byte11 >>= λ _ →
  expect-byte byte0 >>= λ _ →
  parse-section byte1 >>= λ manifest →
  parse-section byte2 >>= λ signature →
  parse-section byte3 >>= λ global-slots →
  parse-section byte4 >>= λ contexts →
  parse-section byte5 >>= λ conversions →
  parse-section byte6 >>= λ conversion-supplements →
  parse-section byte7 >>= λ synthesis →
  parse-section byte8 >>= λ q0-inventory →
  parse-section byte9 >>= λ fresh-rules →
  parse-section byte10 >>= λ family-inventory →
  parse-section byte11 >>= λ family-payloads →
  pure
    (raw-envelope-v1
      manifest
      signature
      global-slots
      contexts
      conversions
      conversion-supplements
      synthesis
      q0-inventory
      fresh-rules
      family-inventory
      family-payloads)

decode-envelope-v1 : List Byte → Maybe RawProductionEnvelopeV1
decode-envelope-v1 input
  with parse-envelope-v1 input
... | nothing = nothing
... | just (envelope , []) = just envelope
... | just (envelope , _ ∷ _) = nothing
parse-section-byte1-encode :
  (section : CanonicalSizedBytes) →
  (rest : List Byte) →
  parse-section byte1 (encode-section byte1 section ++ rest)
  ≡
  just (raw-sized section , rest)
parse-section-byte1-encode section rest
  rewrite parse-sized-bytes-encode section rest = refl

parse-section-byte2-encode :
  (section : CanonicalSizedBytes) →
  (rest : List Byte) →
  parse-section byte2 (encode-section byte2 section ++ rest)
  ≡
  just (raw-sized section , rest)
parse-section-byte2-encode section rest
  rewrite parse-sized-bytes-encode section rest = refl

parse-section-byte3-encode :
  (section : CanonicalSizedBytes) →
  (rest : List Byte) →
  parse-section byte3 (encode-section byte3 section ++ rest)
  ≡
  just (raw-sized section , rest)
parse-section-byte3-encode section rest
  rewrite parse-sized-bytes-encode section rest = refl

parse-section-byte4-encode :
  (section : CanonicalSizedBytes) →
  (rest : List Byte) →
  parse-section byte4 (encode-section byte4 section ++ rest)
  ≡
  just (raw-sized section , rest)
parse-section-byte4-encode section rest
  rewrite parse-sized-bytes-encode section rest = refl

parse-section-byte5-encode :
  (section : CanonicalSizedBytes) →
  (rest : List Byte) →
  parse-section byte5 (encode-section byte5 section ++ rest)
  ≡
  just (raw-sized section , rest)
parse-section-byte5-encode section rest
  rewrite parse-sized-bytes-encode section rest = refl

parse-section-byte6-encode :
  (section : CanonicalSizedBytes) →
  (rest : List Byte) →
  parse-section byte6 (encode-section byte6 section ++ rest)
  ≡
  just (raw-sized section , rest)
parse-section-byte6-encode section rest
  rewrite parse-sized-bytes-encode section rest = refl

parse-section-byte7-encode :
  (section : CanonicalSizedBytes) →
  (rest : List Byte) →
  parse-section byte7 (encode-section byte7 section ++ rest)
  ≡
  just (raw-sized section , rest)
parse-section-byte7-encode section rest
  rewrite parse-sized-bytes-encode section rest = refl

parse-section-byte8-encode :
  (section : CanonicalSizedBytes) →
  (rest : List Byte) →
  parse-section byte8 (encode-section byte8 section ++ rest)
  ≡
  just (raw-sized section , rest)
parse-section-byte8-encode section rest
  rewrite parse-sized-bytes-encode section rest = refl

parse-section-byte9-encode :
  (section : CanonicalSizedBytes) →
  (rest : List Byte) →
  parse-section byte9 (encode-section byte9 section ++ rest)
  ≡
  just (raw-sized section , rest)
parse-section-byte9-encode section rest
  rewrite parse-sized-bytes-encode section rest = refl

parse-section-byte10-encode :
  (section : CanonicalSizedBytes) →
  (rest : List Byte) →
  parse-section byte10 (encode-section byte10 section ++ rest)
  ≡
  just (raw-sized section , rest)
parse-section-byte10-encode section rest
  rewrite parse-sized-bytes-encode section rest = refl

parse-section-byte11-encode :
  (section : CanonicalSizedBytes) →
  (rest : List Byte) →
  parse-section byte11 (encode-section byte11 section ++ rest)
  ≡
  just (raw-sized section , rest)
parse-section-byte11-encode section rest
  rewrite parse-sized-bytes-encode section rest = refl

parse-envelope-v1-encode :
  (envelope : CanonicalProductionEnvelopeV1) →
  parse-envelope-v1 (encode-envelope-v1 envelope)
  ≡
  just (erase-envelope-v1 envelope , [])
parse-envelope-v1-encode
  (canonical-envelope-v1 s1 s2 s3 s4 s5 s6 s7 s8 s9 s10 s11)
  rewrite parse-section-byte1-encode s1
            (encode-section byte2 s2 ++
             encode-section byte3 s3 ++
             encode-section byte4 s4 ++
             encode-section byte5 s5 ++
             encode-section byte6 s6 ++
             encode-section byte7 s7 ++
             encode-section byte8 s8 ++
             encode-section byte9 s9 ++
             encode-section byte10 s10 ++
             encode-section byte11 s11 ++ [])
        | parse-section-byte2-encode s2
            (encode-section byte3 s3 ++
             encode-section byte4 s4 ++
             encode-section byte5 s5 ++
             encode-section byte6 s6 ++
             encode-section byte7 s7 ++
             encode-section byte8 s8 ++
             encode-section byte9 s9 ++
             encode-section byte10 s10 ++
             encode-section byte11 s11 ++ [])
        | parse-section-byte3-encode s3
            (encode-section byte4 s4 ++
             encode-section byte5 s5 ++
             encode-section byte6 s6 ++
             encode-section byte7 s7 ++
             encode-section byte8 s8 ++
             encode-section byte9 s9 ++
             encode-section byte10 s10 ++
             encode-section byte11 s11 ++ [])
        | parse-section-byte4-encode s4
            (encode-section byte5 s5 ++
             encode-section byte6 s6 ++
             encode-section byte7 s7 ++
             encode-section byte8 s8 ++
             encode-section byte9 s9 ++
             encode-section byte10 s10 ++
             encode-section byte11 s11 ++ [])
        | parse-section-byte5-encode s5
            (encode-section byte6 s6 ++
             encode-section byte7 s7 ++
             encode-section byte8 s8 ++
             encode-section byte9 s9 ++
             encode-section byte10 s10 ++
             encode-section byte11 s11 ++ [])
        | parse-section-byte6-encode s6
            (encode-section byte7 s7 ++
             encode-section byte8 s8 ++
             encode-section byte9 s9 ++
             encode-section byte10 s10 ++
             encode-section byte11 s11 ++ [])
        | parse-section-byte7-encode s7
            (encode-section byte8 s8 ++
             encode-section byte9 s9 ++
             encode-section byte10 s10 ++
             encode-section byte11 s11 ++ [])
        | parse-section-byte8-encode s8
            (encode-section byte9 s9 ++
             encode-section byte10 s10 ++
             encode-section byte11 s11 ++ [])
        | parse-section-byte9-encode s9
            (encode-section byte10 s10 ++
             encode-section byte11 s11 ++ [])
        | parse-section-byte10-encode s10
            (encode-section byte11 s11 ++ [])
        | parse-section-byte11-encode s11 [] = refl

decode-envelope-v1-encode :
  (envelope : CanonicalProductionEnvelopeV1) →
  decode-envelope-v1 (encode-envelope-v1 envelope)
  ≡
  just (erase-envelope-v1 envelope)
decode-envelope-v1-encode envelope
  rewrite parse-envelope-v1-encode envelope = refl