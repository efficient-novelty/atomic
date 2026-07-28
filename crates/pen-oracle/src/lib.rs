//! Quarantined reference data for legacy replay, tests, and post-run decoding.
//!
//! Nothing in this crate is law-level input. Production discovery crates must
//! not depend on `pen-oracle`; its reference telescopes, human labels, expected
//! values, and expected hashes are oracle/testimony data only.

#![forbid(unsafe_code)]

use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::telescope::Telescope;
use schemars::JsonSchema;
use serde::Serialize;

/// Number of entries in the quarantined legacy reference trace.
///
/// This is oracle metadata, never a lawful termination condition.
pub const ORACLE_REFERENCE_STAGE_COUNT: usize = 15;

/// Provenance of the copied reference telescope payloads.
pub const REFERENCE_TELESCOPE_ORACLE_SOURCE: &str =
    "crates/pen-core/src/telescope.rs:Telescope::reference (quarantine copy)";

/// Provenance of the copied human-facing labels.
pub const HUMAN_LABEL_ORACLE_SOURCE: &str =
    "crates/pen-cli/src/human.rs:step_label (quarantine copy)";

/// Provenance of the legacy numerical and candidate-hash testimony.
pub const LEGACY_BAR_V1_TESTIMONY_SOURCE: &str = "runs/step15-live/checkpoints/steps/step-NN.json";

/// Human-facing decoder labels. These names are oracle metadata and must not
/// participate in production discovery or acceptance.
pub const ORACLE_HUMAN_LABEL_TESTIMONY: [&str; ORACLE_REFERENCE_STAGE_COUNT] = [
    "Universe",
    "Unit",
    "Witness",
    "Pi",
    "S1",
    "Trunc",
    "S2",
    "S3",
    "Hopf",
    "Cohesion",
    "Connections",
    "Curvature",
    "Metric",
    "Hilbert",
    "DCT",
];

/// Frozen legacy structural-ν observations.
///
/// These values are testimony from `legacy_bar_v1`; they are not the
/// semantic-family register and must not be used by a law-level selector.
pub const EXPECTED_LEGACY_BAR_V1_STRUCTURAL_NU_TESTIMONY: [u32; ORACLE_REFERENCE_STAGE_COUNT] =
    [1, 1, 2, 5, 7, 8, 10, 18, 17, 19, 26, 34, 46, 62, 103];

/// Frozen legacy clause-κ observations (not encoded bit costs).
pub const EXPECTED_LEGACY_BAR_V1_CLAUSE_KAPPA_TESTIMONY: [u16; ORACLE_REFERENCE_STAGE_COUNT] =
    [2, 1, 1, 3, 3, 3, 3, 5, 4, 4, 5, 6, 7, 9, 8];

/// Frozen legacy candidate hashes, copied from the accepted step checkpoints.
pub const EXPECTED_LEGACY_BAR_V1_CANDIDATE_HASH_TESTIMONY: [&str; ORACLE_REFERENCE_STAGE_COUNT] = [
    "blake3:61631d63a5877aad1b32b1e71aa6f0e555317b65732f327e3f38e32c222e29e7",
    "blake3:a2dfff0fb8ce1073119da3893e1d195247c234af66b9a7de17c85d5cf718f555",
    "blake3:934b5599bb0f28abf0be9652982caec5e0ff6d7d204ddaa4e66f23c37155457e",
    "blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407",
    "blake3:043c7990d42d91f972544a3cbcae33ca65e48eaab82d605a15cb505ed0ad26d0",
    "blake3:a2ff7bc69a678e8c257824465993f8aadf71c05676562887da12dce3ae6cfe1d",
    "blake3:30f190b67aab5179a6de8dccee6c29c699b80c12bef676bf2e9f9377595a7906",
    "blake3:e01de7b95b0dda56a1062add33387a37709e23d3d6ea6ce4efaf27adacb868c0",
    "blake3:f57e3a5aa44003adb5f8054013e549e4f3065757d88922313478e8e445825491",
    "blake3:93289041755cf4c4e0396029273822630028999d945359b10bb88e2376b9788e",
    "blake3:03cc71839428ceab088e386e31e6c72b7ca4a1e12c9309557efc76826dcb88cd",
    "blake3:0d06e3b14bfd7c1bd16d9f66039e016f84a6162ce85584f7c641753b833b1dcb",
    "blake3:b09b3f832bef16953747c213e6b8a548f594339dc7db9148e7ae60a6c53a4cf1",
    "blake3:1ff4820f2272c022a5fece1032aa9647e41167a3dd8f62765e84e22f5d90b19c",
    "blake3:e919c8419bbafde89e3e99ff25f348f3c8b679ed22685e84d3cdec634ebf90d4",
];

/// Frozen legacy canonical hashes, copied from the trajectory fixture and
/// accepted step reports.
pub const EXPECTED_LEGACY_BAR_V1_CANONICAL_HASH_TESTIMONY: [&str; ORACLE_REFERENCE_STAGE_COUNT] = [
    "blake3:23ab301fcb806f4f151974b767119ea66f8ee39bcf5b7d9c95924fc2902663b2",
    "blake3:cd72c5935198d40c0ef48ae172c9ccba1ef9c90435494569121c8dccd3466476",
    "blake3:2e85e04d43f4ef6ae6c9b00f4460ba7b8507b8c371743c97b7262ca014fd2987",
    "blake3:c7a99c10bbc01197aec9fa0c4605b4763dda6dc22ea95310fae37e9ff5ee65a1",
    "blake3:6db82a1b3484f3373986e9f2986d6472521448379495ab3b66da36c0d2507085",
    "blake3:05d6509e1d870b0169bf74091e2a4ac86e7a337540c6dc53957ec18aa02ac28a",
    "blake3:8fd79581fd71b586ff52b3be14ade3ec63cf520232577617aa7918bbdc80ece4",
    "blake3:c6fc7673d107487f21fb29064e7ce8c9418f9fb2f0688613380e116ea77230aa",
    "blake3:db6a9c039fc70d0f2595143ed87bd948041943105ba106556b71ae8c09c9f777",
    "blake3:09bed8af6371d3ac8be4c5d4bd59321753f2a027ff9685808b8bab6ce8ac4ef5",
    "blake3:0b747d5eea40ef4cece4f40b64c7dbdff0db9ed74f209e5c6c79346f7165ae27",
    "blake3:fd636363062e2cc44259d330cd96e319979d8ae7ff02eb065599bb2541d03cac",
    "blake3:4eefcc446fb659c101cb538dcf4b06695c165d6e977e64ab7580a2451af0f5ae",
    "blake3:435eaca75694e1e58b26739b2f6d8c5d77cc0c601c2681e63ced81692f25c67d",
    "blake3:6f4b65c28060999af4bac3fa46a9da3bd8e1bbdbc99f41ecc94291012a0573a4",
];

/// One row of explicitly non-lawful legacy testimony.
#[derive(Clone, Copy, Debug, Eq, JsonSchema, PartialEq, Serialize)]
pub struct LegacyBarV1Testimony {
    pub stage: u32,
    pub oracle_human_label: &'static str,
    pub expected_legacy_structural_nu: u32,
    pub expected_legacy_clause_kappa: u16,
    pub expected_legacy_candidate_hash: &'static str,
    pub expected_legacy_canonical_hash: &'static str,
}

/// Frozen testimony rows for the complete legacy reference trace.
pub const EXPECTED_LEGACY_BAR_V1_TESTIMONY: [LegacyBarV1Testimony; ORACLE_REFERENCE_STAGE_COUNT] = [
    testimony(1),
    testimony(2),
    testimony(3),
    testimony(4),
    testimony(5),
    testimony(6),
    testimony(7),
    testimony(8),
    testimony(9),
    testimony(10),
    testimony(11),
    testimony(12),
    testimony(13),
    testimony(14),
    testimony(15),
];

const fn testimony(stage: u32) -> LegacyBarV1Testimony {
    let index = (stage - 1) as usize;
    LegacyBarV1Testimony {
        stage,
        oracle_human_label: ORACLE_HUMAN_LABEL_TESTIMONY[index],
        expected_legacy_structural_nu: EXPECTED_LEGACY_BAR_V1_STRUCTURAL_NU_TESTIMONY[index],
        expected_legacy_clause_kappa: EXPECTED_LEGACY_BAR_V1_CLAUSE_KAPPA_TESTIMONY[index],
        expected_legacy_candidate_hash: EXPECTED_LEGACY_BAR_V1_CANDIDATE_HASH_TESTIMONY[index],
        expected_legacy_canonical_hash: EXPECTED_LEGACY_BAR_V1_CANONICAL_HASH_TESTIMONY[index],
    }
}

/// Look up a human-facing post-run decoder label.
///
/// Returns `None` outside the quarantined oracle trace.
pub fn oracle_human_label(stage: u32) -> Option<&'static str> {
    oracle_index(stage).map(|index| ORACLE_HUMAN_LABEL_TESTIMONY[index])
}

/// Look up the frozen legacy testimony row for `stage`.
pub fn legacy_bar_v1_testimony(stage: u32) -> Option<&'static LegacyBarV1Testimony> {
    oracle_index(stage).map(|index| &EXPECTED_LEGACY_BAR_V1_TESTIMONY[index])
}

fn oracle_index(stage: u32) -> Option<usize> {
    let index = usize::try_from(stage.checked_sub(1)?).ok()?;
    (index < ORACLE_REFERENCE_STAGE_COUNT).then_some(index)
}

/// Return the quarantined telescope fixture for a legacy reference step.
///
/// This mirrors the legacy `Telescope::reference` API during the quarantine
/// transition, including its empty-telescope result for an unknown step. New
/// law-level code must not call this function.
pub fn reference_telescope(step: u32) -> Telescope {
    match step {
        1 => Telescope::new(vec![
            oracle_clause(Expr::Univ),
            oracle_clause(Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))),
        ]),
        2 => Telescope::new(vec![oracle_clause(Expr::App(
            Box::new(Expr::Univ),
            Box::new(Expr::Var(1)),
        ))]),
        3 => Telescope::new(vec![oracle_clause(Expr::App(
            Box::new(Expr::Lib(2)),
            Box::new(Expr::Var(1)),
        ))]),
        4 => Telescope::new(vec![
            oracle_clause(Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(2)),
            )))),
            oracle_clause(Expr::App(
                Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                Box::new(Expr::Var(3)),
            )),
            oracle_clause(Expr::App(
                Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                Box::new(Expr::Var(2)),
            )),
        ]),
        5 => Telescope::new(vec![
            oracle_clause(Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Var(1)),
            oracle_clause(Expr::PathCon(1)),
        ]),
        6 => Telescope::new(vec![
            oracle_clause(Expr::Trunc(Box::new(Expr::Var(1)))),
            oracle_clause(Expr::App(
                Box::new(Expr::Trunc(Box::new(Expr::Var(1)))),
                Box::new(Expr::Var(2)),
            )),
            oracle_clause(Expr::PathCon(1)),
        ]),
        7 => Telescope::new(vec![
            oracle_clause(Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Var(1)),
            oracle_clause(Expr::PathCon(2)),
        ]),
        8 => Telescope::new(vec![
            oracle_clause(Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Var(1)),
            oracle_clause(Expr::PathCon(3)),
            oracle_clause(Expr::Lam(Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Lam(Box::new(Expr::Var(2)))),
        ]),
        9 => Telescope::new(vec![
            oracle_clause(Expr::Pi(Box::new(Expr::Lib(8)), Box::new(Expr::Lib(7)))),
            oracle_clause(Expr::App(Box::new(Expr::Lib(5)), Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(8)),
                Box::new(Expr::Lib(7)),
            )))),
            oracle_clause(Expr::Pi(Box::new(Expr::Lib(7)), Box::new(Expr::Lib(8)))),
        ]),
        10 => Telescope::new(vec![
            oracle_clause(Expr::Flat(Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Sharp(Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Disc(Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Shape(Box::new(Expr::Var(1)))),
        ]),
        11 => Telescope::new(vec![
            oracle_clause(Expr::Pi(
                Box::new(Expr::Lib(10)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            )),
            oracle_clause(Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(2)),
            )))),
            oracle_clause(Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Var(1)))),
                Box::new(Expr::Var(1)),
            )),
            oracle_clause(Expr::App(Box::new(Expr::Lib(10)), Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Lam(Box::new(Expr::Var(1)))),
        ]),
        12 => Telescope::new(vec![
            oracle_clause(Expr::Pi(
                Box::new(Expr::Lib(11)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            )),
            oracle_clause(Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(11)),
                Box::new(Expr::Var(1)),
            )))),
            oracle_clause(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Lib(11)))),
            oracle_clause(Expr::App(
                Box::new(Expr::Lib(11)),
                Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            )),
            oracle_clause(Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(2)),
            )))),
            oracle_clause(Expr::Pi(Box::new(Expr::Lib(11)), Box::new(Expr::Lib(11)))),
        ]),
        13 => Telescope::new(vec![
            oracle_clause(Expr::Sigma(
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            )),
            oracle_clause(Expr::Pi(
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
                Box::new(Expr::Lib(11)),
            )),
            oracle_clause(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            )),
            oracle_clause(Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(2)),
            )))),
            oracle_clause(Expr::Pi(Box::new(Expr::Lib(12)), Box::new(Expr::Lib(12)))),
            oracle_clause(Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Var(1)),
            )))),
            oracle_clause(Expr::Pi(Box::new(Expr::Lib(12)), Box::new(Expr::Var(1)))),
        ]),
        14 => Telescope::new(vec![
            oracle_clause(Expr::Sigma(
                Box::new(Expr::Pi(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Univ))),
                )),
                Box::new(Expr::Var(1)),
            )),
            oracle_clause(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            )),
            oracle_clause(Expr::Pi(
                Box::new(Expr::Lam(Box::new(Expr::Var(1)))),
                Box::new(Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(2)))),
            )),
            oracle_clause(Expr::Sigma(
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
                Box::new(Expr::Pi(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))),
            )),
            oracle_clause(Expr::Pi(Box::new(Expr::Lib(13)), Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Pi(Box::new(Expr::Lib(12)), Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Pi(Box::new(Expr::Lib(11)), Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Lam(Box::new(Expr::Pi(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Univ),
            )))),
        ]),
        15 => Telescope::new(vec![
            oracle_clause(Expr::Next(Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Eventually(Box::new(Expr::Var(1)))),
            oracle_clause(Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
                Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
            )),
            oracle_clause(Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(10)),
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            )))),
            oracle_clause(Expr::Pi(
                Box::new(Expr::Flat(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Next(Box::new(Expr::Flat(Box::new(Expr::Var(1)))))),
            )),
            oracle_clause(Expr::Pi(
                Box::new(Expr::Sharp(Box::new(Expr::Eventually(Box::new(
                    Expr::Var(1),
                ))))),
                Box::new(Expr::Eventually(Box::new(Expr::Sharp(Box::new(
                    Expr::Var(1),
                ))))),
            )),
            oracle_clause(Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
                Box::new(Expr::Var(2)),
            )))),
            oracle_clause(Expr::Pi(
                Box::new(Expr::Next(Box::new(Expr::Next(Box::new(Expr::Var(1)))))),
                Box::new(Expr::Next(Box::new(Expr::Var(1)))),
            )),
        ]),
        _ => Telescope::default(),
    }
}

/// Return every quarantined reference telescope in legacy stage order.
///
/// The fixed range is oracle metadata and must not be used as a production
/// discovery endpoint.
pub fn all_reference_telescopes() -> Vec<(u32, Telescope)> {
    (1..=u32::try_from(ORACLE_REFERENCE_STAGE_COUNT).expect("oracle stage count fits u32"))
        .map(|stage| (stage, reference_telescope(stage)))
        .collect()
}

fn oracle_clause(expr: Expr) -> ClauseRec {
    let role = match &expr {
        Expr::Univ
        | Expr::Pi(_, _)
        | Expr::Sigma(_, _)
        | Expr::Id(_, _, _)
        | Expr::Trunc(_)
        | Expr::Flat(_)
        | Expr::Sharp(_)
        | Expr::Disc(_)
        | Expr::Shape(_)
        | Expr::Next(_)
        | Expr::Eventually(_)
        | Expr::Bang(_)
        | Expr::WhyNot(_)
        | Expr::Lib(_)
        | Expr::Susp(_) => ClauseRole::Formation,
        Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ) => ClauseRole::Formation,
        Expr::App(left, _) if matches!(left.as_ref(), Expr::Lam(_)) => ClauseRole::Elimination,
        Expr::App(_, _) | Expr::Var(_) | Expr::Lam(_) | Expr::Refl(_) => ClauseRole::Introduction,
        Expr::PathCon(_) => ClauseRole::PathAttach,
    };
    ClauseRec::new(role, expr)
}

#[cfg(test)]
mod tests {
    use super::{
        EXPECTED_LEGACY_BAR_V1_CANDIDATE_HASH_TESTIMONY,
        EXPECTED_LEGACY_BAR_V1_CANONICAL_HASH_TESTIMONY,
        EXPECTED_LEGACY_BAR_V1_CLAUSE_KAPPA_TESTIMONY,
        EXPECTED_LEGACY_BAR_V1_STRUCTURAL_NU_TESTIMONY, EXPECTED_LEGACY_BAR_V1_TESTIMONY,
        ORACLE_HUMAN_LABEL_TESTIMONY, ORACLE_REFERENCE_STAGE_COUNT, all_reference_telescopes,
        legacy_bar_v1_testimony, oracle_human_label, reference_telescope,
    };
    use pen_core::hash::blake3_hex;
    use pen_core::telescope::Telescope;
    use std::collections::BTreeSet;

    #[test]
    fn quarantine_copy_matches_the_legacy_reference_source() {
        for stage in 1..=ORACLE_REFERENCE_STAGE_COUNT as u32 {
            assert_eq!(
                reference_telescope(stage),
                Telescope::reference(stage),
                "quarantined telescope drift at stage {stage}"
            );
        }
        assert_eq!(reference_telescope(0), Telescope::default());
        assert_eq!(reference_telescope(16), Telescope::default());
    }

    #[test]
    fn all_reference_telescopes_are_complete_and_ordered() {
        let all = all_reference_telescopes();
        assert_eq!(all.len(), ORACLE_REFERENCE_STAGE_COUNT);
        for (index, (stage, telescope)) in all.iter().enumerate() {
            assert_eq!(*stage, u32::try_from(index + 1).expect("stage fits u32"));
            assert_eq!(*telescope, reference_telescope(*stage));
        }
    }

    #[test]
    fn decoder_labels_are_oracle_only_and_total_on_the_fixture() {
        for (index, expected) in ORACLE_HUMAN_LABEL_TESTIMONY.iter().enumerate() {
            let stage = u32::try_from(index + 1).expect("stage fits u32");
            assert_eq!(oracle_human_label(stage), Some(*expected));
        }
        assert_eq!(oracle_human_label(0), None);
        assert_eq!(oracle_human_label(16), None);
    }

    #[test]
    fn legacy_testimony_rows_align_with_the_independent_fixture_columns() {
        for (index, row) in EXPECTED_LEGACY_BAR_V1_TESTIMONY.iter().enumerate() {
            let stage = u32::try_from(index + 1).expect("stage fits u32");
            assert_eq!(row.stage, stage);
            assert_eq!(row.oracle_human_label, ORACLE_HUMAN_LABEL_TESTIMONY[index]);
            assert_eq!(
                row.expected_legacy_structural_nu,
                EXPECTED_LEGACY_BAR_V1_STRUCTURAL_NU_TESTIMONY[index]
            );
            assert_eq!(
                row.expected_legacy_clause_kappa,
                EXPECTED_LEGACY_BAR_V1_CLAUSE_KAPPA_TESTIMONY[index]
            );
            assert_eq!(
                row.expected_legacy_candidate_hash,
                EXPECTED_LEGACY_BAR_V1_CANDIDATE_HASH_TESTIMONY[index]
            );
            assert_eq!(
                row.expected_legacy_canonical_hash,
                EXPECTED_LEGACY_BAR_V1_CANONICAL_HASH_TESTIMONY[index]
            );
            assert_eq!(
                reference_telescope(stage).kappa(),
                usize::from(row.expected_legacy_clause_kappa)
            );
            assert_eq!(legacy_bar_v1_testimony(stage), Some(row));
        }
        assert_eq!(legacy_bar_v1_testimony(0), None);
        assert_eq!(legacy_bar_v1_testimony(16), None);
    }

    #[test]
    fn candidate_hash_testimony_is_well_formed_and_unique() {
        for hashes in [
            EXPECTED_LEGACY_BAR_V1_CANDIDATE_HASH_TESTIMONY,
            EXPECTED_LEGACY_BAR_V1_CANONICAL_HASH_TESTIMONY,
        ] {
            let unique = hashes.into_iter().collect::<BTreeSet<_>>();
            assert_eq!(unique.len(), ORACLE_REFERENCE_STAGE_COUNT);
            assert!(
                unique.iter().all(|hash| {
                    hash.strip_prefix("blake3:").is_some_and(|hex| {
                        hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit())
                    })
                }),
                "every testimony hash must be a full BLAKE3 hexadecimal digest"
            );
        }
    }

    #[test]
    fn candidate_hash_testimony_replays_from_quarantined_telescopes() {
        for stage in 1..=ORACLE_REFERENCE_STAGE_COUNT as u32 {
            let telescope = reference_telescope(stage);
            let encoded = serde_json::to_vec(&telescope).expect("serialize telescope");
            let replayed = format!("blake3:{}", blake3_hex(&encoded));
            assert_eq!(
                replayed,
                EXPECTED_LEGACY_BAR_V1_CANDIDATE_HASH_TESTIMONY[(stage - 1) as usize],
                "candidate hash drift at oracle stage {stage}"
            );
        }
    }

    #[test]
    fn legacy_vectors_remain_the_frozen_checkpoint_values() {
        assert_eq!(
            EXPECTED_LEGACY_BAR_V1_STRUCTURAL_NU_TESTIMONY,
            [1, 1, 2, 5, 7, 8, 10, 18, 17, 19, 26, 34, 46, 62, 103]
        );
        assert_eq!(
            EXPECTED_LEGACY_BAR_V1_CLAUSE_KAPPA_TESTIMONY,
            [2, 1, 1, 3, 3, 3, 3, 5, 4, 4, 5, 6, 7, 9, 8]
        );
    }
}
