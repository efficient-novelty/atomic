use pen_search::uc1_prediction_score::{
    emit_uc1_prediction_score_create_new, issue_uc1_prediction_score_from_directory,
    replay_uc1_prediction_score_directory,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, directory] if command == "inspect" => {
            let certificate = issue_uc1_prediction_score_from_directory(Path::new(directory))?;
            println!(
                "P1={:?} P2={:?} P3={:?} zone={} U-T-authorized={} digest={}",
                certificate.p1.status,
                certificate.p2.status,
                certificate.p3.status,
                certificate
                    .outcome
                    .registered_outcome_zone
                    .as_deref()
                    .unwrap_or("none"),
                certificate
                    .outcome
                    .construction_tasks_u_t1_through_u_t4_authorized,
                certificate.result_digest,
            );
        }
        [command, directory] if command == "create-new" => {
            let certificate = emit_uc1_prediction_score_create_new(Path::new(directory))?;
            println!(
                "created UC-1 score: P1={:?} P2={:?} P3={:?} digest={}",
                certificate.p1.status,
                certificate.p2.status,
                certificate.p3.status,
                certificate.result_digest,
            );
        }
        [command, directory] if command == "replay" => {
            let replay = replay_uc1_prediction_score_directory(Path::new(directory));
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err(format!("UC-1 score replay failed: {:?}", replay.errors).into());
            }
        }
        _ => {
            return Err(
                "usage: uc1_prediction_score <inspect|create-new|replay> <artifact-directory>"
                    .into(),
            );
        }
    }
    Ok(())
}
