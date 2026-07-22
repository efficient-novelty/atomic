mod act_local_provenance_v3 {
    pub use pen_search::act_local_provenance_v3::*;
}

#[path = "../src/act_local_semantic_provenance_v4.rs"]
mod act_local_semantic_provenance_v4;

fn main() {
    match act_local_semantic_provenance_v4::issue_reference_act_local_semantic_sequence_v4() {
        Ok(packages) => {
            let vector = packages
                .iter()
                .map(|package| package.semantic_family_nu)
                .collect::<Vec<_>>();
            let gaps = packages
                .iter()
                .map(|package| package.v3_role_schema_gap_count)
                .sum::<usize>();
            let proved = packages
                .iter()
                .map(|package| package.proved_role_declaration_count)
                .sum::<usize>();
            let impossible = packages
                .iter()
                .map(|package| package.impossible_role_declaration_count)
                .sum::<usize>();
            let residual = packages
                .iter()
                .map(|package| package.named_role_residual_count)
                .sum::<usize>();
            println!(
                "non_authoritative_extraction_floor={vector:?} gaps={gaps} proved={proved} impossible={impossible} named_residual={residual}"
            );
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
