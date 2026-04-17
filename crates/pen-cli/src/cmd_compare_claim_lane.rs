use crate::claim_evidence::{
    build_claim_compare_summary, load_run, render_claim_compare_text, write_text,
};
use crate::cli::CompareClaimLaneArgs;
use anyhow::Result;

pub fn compare_claim_lane(args: CompareClaimLaneArgs) -> Result<String> {
    let guarded = load_run(&args.guarded_run)?;
    let claim = load_run(&args.claim_run)?;
    let summary = build_claim_compare_summary(&guarded, &claim);
    let text = render_claim_compare_text(&summary);

    if let Some(path) = args.json_out.as_deref() {
        write_text(path, &(serde_json::to_string_pretty(&summary)? + "\n"))?;
    }
    if let Some(path) = args.text_out.as_deref() {
        write_text(path, &text)?;
    }

    Ok(text)
}
