use pen_search::t_d2_3_candidate_content_stream_v2::{
    issue_t_d2_3_candidate_content_stream_v2, issue_t_d2_3_candidate_content_stream_v2_with_caps,
    open_t_d2_3_candidate_content_stream_v2,
};

fn parse<T: std::str::FromStr>(label: &str, value: &str) -> Result<T, String>
where
    T::Err: std::fmt::Display,
{
    value
        .parse()
        .map_err(|error| format!("invalid {label} `{value}`: {error}"))
}

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => {
            let certificate =
                issue_t_d2_3_candidate_content_stream_v2().map_err(|error| error.to_string())?;
            if !certificate.full_aggregate_candidate_content_stream_exported {
                return Err(format!(
                    "T-D2-3 v2 stopped with named gaps: {}",
                    certificate.open_gaps.join("; ")
                ));
            }
            println!(
                "{}",
                serde_json::to_string_pretty(&certificate)
                    .map_err(|error| format!("certificate serialization failed: {error}"))?
            );
        }
        [command, min_kappa, max_kappa, max_expr_nodes] if command == "bounded" => {
            let certificate = issue_t_d2_3_candidate_content_stream_v2_with_caps(
                parse("min_kappa", min_kappa)?,
                parse("max_kappa", max_kappa)?,
                parse("max_expr_nodes", max_expr_nodes)?,
            )
            .map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&certificate)
                    .map_err(|error| format!("certificate serialization failed: {error}"))?
            );
        }
        [command, kappa, start, count] if command == "sample" => {
            let stream =
                open_t_d2_3_candidate_content_stream_v2().map_err(|error| error.to_string())?;
            let kappa = parse::<u16>("kappa", kappa)?;
            let start = parse::<u128>("start", start)?;
            let count = parse::<u128>("count", count)?;
            let end = start
                .checked_add(count)
                .ok_or_else(|| "sample range exceeds u128".to_owned())?;
            let items = stream
                .chunk(kappa, start..end)
                .map_err(|error| error.to_string())?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&items)
                    .map_err(|error| format!("item serialization failed: {error}"))?
            );
        }
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example t_d2_3_candidate_content_stream_v2 [bounded <min-kappa> <max-kappa> <max-expr-nodes> | sample <kappa> <start> <count>]"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("t-d2-3-candidate-content-stream-v2".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("T-D2-3 v2 worker panicked".into()),
    }
}
