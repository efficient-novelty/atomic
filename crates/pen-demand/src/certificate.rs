use crate::census::{
    RelativeInputBudget, compute_relative_census_preflighted, preflight_relative_inputs,
};
use crate::{
    FiniteDemandDomain, InstanceId, LibrarySeeds, OpaqueWindow, RelativeCensus,
    RelativeCensusOutcome, StructuralSupport, UnknownReason, domain_digest, library_digest,
    window_digest,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, Kernel, VerifiedSignature};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const RELATIVE_CENSUS_CERTIFICATE_VERSION: u16 = 2;

/// Replay data only. Deserializing or constructing this DTO does not confer a
/// verified capability.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedRelativeCensusCertificate {
    pub version: u16,
    pub kernel_digest: Digest,
    pub normalizer_digest: Digest,
    pub domain_digest: Digest,
    pub signature_digest: Digest,
    pub window_digest: Digest,
    pub library_digest: Digest,
    pub support: Vec<StructuralSupport>,
    pub active: Vec<InstanceId>,
    pub reached: Vec<InstanceId>,
    pub unreached: Vec<InstanceId>,
    pub layers: Vec<Vec<InstanceId>>,
}

impl UncheckedRelativeCensusCertificate {
    /// Construct a replay claim from a completed relative census. The result
    /// remains unchecked until `verify_relative_census_certificate` succeeds.
    pub fn claim(
        kernel: &Kernel,
        signature: &VerifiedSignature,
        domain: &FiniteDemandDomain,
        window: &OpaqueWindow,
        library: &LibrarySeeds,
        census: &RelativeCensus,
    ) -> RelativeCensusOutcome<Self> {
        let mut preflight = RelativeInputBudget::for_kernel(kernel);
        if let Err(reason) = preflight_relative_inputs(&mut preflight, domain, window, library)
            .and_then(|()| preflight_census(&mut preflight, census))
        {
            return RelativeCensusOutcome::Unknown(reason);
        }
        RelativeCensusOutcome::CompleteRelative(Self {
            version: RELATIVE_CENSUS_CERTIFICATE_VERSION,
            kernel_digest: kernel.kernel_protocol_digest(),
            normalizer_digest: kernel.normalizer_protocol_digest(),
            domain_digest: domain_digest(domain),
            signature_digest: signature.digest().clone(),
            window_digest: window_digest(window),
            library_digest: library_digest(library),
            support: census.support.clone(),
            active: census.active.clone(),
            reached: census.reached.clone(),
            unreached: census.unreached.clone(),
            layers: census.layers.clone(),
        })
    }
}

impl CanonicalEncode for UncheckedRelativeCensusCertificate {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.version);
        self.kernel_digest.encode_canonical(encoder);
        self.normalizer_digest.encode_canonical(encoder);
        self.domain_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.window_digest.encode_canonical(encoder);
        self.library_digest.encode_canonical(encoder);
        encoder.sequence(&self.support);
        encoder.sequence(&self.active);
        encoder.sequence(&self.reached);
        encoder.sequence(&self.unreached);
        encoder.u64(self.layers.len() as u64);
        for layer in &self.layers {
            encoder.sequence(layer);
        }
    }
}

/// Successful replay capability. Its fields are private and the type has no
/// deserialization implementation.
#[derive(Clone, Debug)]
pub struct VerifiedRelativeCensus {
    certificate_digest: Digest,
    kernel_digest: Digest,
    normalizer_digest: Digest,
    domain_digest: Digest,
    signature_digest: Digest,
    window_digest: Digest,
    library_digest: Digest,
    census: RelativeCensus,
}

impl VerifiedRelativeCensus {
    pub fn certificate_digest(&self) -> &Digest {
        &self.certificate_digest
    }

    pub fn kernel_digest(&self) -> &Digest {
        &self.kernel_digest
    }

    pub fn normalizer_digest(&self) -> &Digest {
        &self.normalizer_digest
    }

    pub fn domain_digest(&self) -> &Digest {
        &self.domain_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn window_digest(&self) -> &Digest {
        &self.window_digest
    }

    pub fn library_digest(&self) -> &Digest {
        &self.library_digest
    }

    pub fn census(&self) -> &RelativeCensus {
        &self.census
    }
}

pub fn verify_relative_census_certificate(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    domain: &FiniteDemandDomain,
    window: &OpaqueWindow,
    library: &LibrarySeeds,
    operation_limit: u64,
    certificate: &UncheckedRelativeCensusCertificate,
) -> RelativeCensusOutcome<VerifiedRelativeCensus> {
    let mut preflight = RelativeInputBudget::for_kernel(kernel);
    if let Err(reason) = preflight_relative_inputs(&mut preflight, domain, window, library)
        .and_then(|()| preflight_certificate(&mut preflight, certificate))
    {
        return RelativeCensusOutcome::Unknown(reason);
    }

    let expected_kernel = kernel.kernel_protocol_digest();
    let expected_normalizer = kernel.normalizer_protocol_digest();
    let expected_domain = domain_digest(domain);
    let expected_window = window_digest(window);
    let expected_library = library_digest(library);
    if certificate.version != RELATIVE_CENSUS_CERTIFICATE_VERSION
        || certificate.kernel_digest != expected_kernel
        || certificate.normalizer_digest != expected_normalizer
        || certificate.domain_digest != expected_domain
        || certificate.signature_digest != *signature.digest()
        || certificate.window_digest != expected_window
        || certificate.library_digest != expected_library
    {
        return RelativeCensusOutcome::Unknown(UnknownReason::Unsupported);
    }

    let recomputed = match compute_relative_census_preflighted(
        kernel,
        signature,
        domain,
        window,
        library,
        operation_limit,
    ) {
        RelativeCensusOutcome::CompleteRelative(census) => census,
        RelativeCensusOutcome::Unknown(reason) => {
            return RelativeCensusOutcome::Unknown(reason);
        }
    };
    if certificate.support.as_slice() != recomputed.support.as_slice()
        || certificate.active.as_slice() != recomputed.active.as_slice()
        || certificate.reached.as_slice() != recomputed.reached.as_slice()
        || certificate.unreached.as_slice() != recomputed.unreached.as_slice()
        || certificate.layers.as_slice() != recomputed.layers.as_slice()
    {
        return RelativeCensusOutcome::Unknown(UnknownReason::Unsupported);
    }

    RelativeCensusOutcome::CompleteRelative(VerifiedRelativeCensus {
        certificate_digest: Digest::of_canonical(
            "pen-demand/relative-census-certificate/v2",
            certificate,
        ),
        kernel_digest: expected_kernel,
        normalizer_digest: expected_normalizer,
        domain_digest: expected_domain,
        signature_digest: signature.digest().clone(),
        window_digest: expected_window,
        library_digest: expected_library,
        census: recomputed,
    })
}

fn preflight_certificate(
    budget: &mut RelativeInputBudget,
    certificate: &UncheckedRelativeCensusCertificate,
) -> Result<(), UnknownReason> {
    budget.charge()?;
    preflight_census_fields(
        budget,
        &certificate.support,
        &certificate.active,
        &certificate.reached,
        &certificate.unreached,
        &certificate.layers,
    )
}

fn preflight_census(
    budget: &mut RelativeInputBudget,
    census: &RelativeCensus,
) -> Result<(), UnknownReason> {
    budget.charge()?;
    preflight_census_fields(
        budget,
        &census.support,
        &census.active,
        &census.reached,
        &census.unreached,
        &census.layers,
    )
}

fn preflight_census_fields(
    budget: &mut RelativeInputBudget,
    support: &[StructuralSupport],
    active: &[InstanceId],
    reached: &[InstanceId],
    unreached: &[InstanceId],
    layers: &[Vec<InstanceId>],
) -> Result<(), UnknownReason> {
    budget.charge_len(support.len())?;
    for entry in support {
        budget.charge()?;
        budget.charge_len(entry.globals.len())?;
        budget.charge_len(entry.window_hits.len())?;
    }
    budget.charge_len(active.len())?;
    budget.charge_len(reached.len())?;
    budget.charge_len(unreached.len())?;
    budget.charge_len(layers.len())?;
    for layer in layers {
        budget.charge()?;
        budget.charge_len(layer.len())?;
    }
    Ok(())
}
