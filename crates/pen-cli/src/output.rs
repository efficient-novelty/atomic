use crate::narrative::{NarrativeOutputConfig, render_run_narrative};
use crate::report::{StepReport, render_debug_report, render_standard_report};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputStyle {
    Standard,
    Debug,
}

impl OutputStyle {
    pub const fn from_debug(debug: bool) -> Self {
        if debug { Self::Debug } else { Self::Standard }
    }
}

pub fn render_run_output(
    style: OutputStyle,
    run_id: &str,
    steps: &[StepReport],
    narrative: Option<NarrativeOutputConfig<'_>>,
) -> String {
    let base = match style {
        OutputStyle::Standard => render_standard_report(run_id, steps),
        OutputStyle::Debug => render_debug_report(run_id, steps),
    };
    match narrative {
        Some(narrative) => format!("{base}\n\n{}", render_run_narrative(steps, narrative)),
        None => base,
    }
}

#[cfg(test)]
mod tests {
    use super::{OutputStyle, render_run_output};
    use crate::narrative::NarrativeOutputConfig;
    use crate::report::generate_steps_with_config_and_runtime;
    use pen_search::config::RuntimeConfig;
    use pen_search::diversify::FrontierRuntimeLimits;

    #[test]
    fn current_claim_step_fifteen_run_output_with_narrative_uses_the_stored_demo_closure_surface() {
        let config = RuntimeConfig::from_toml_str(include_str!(
            "../../../configs/desktop_claim_shadow_smoke.toml"
        ))
        .expect("claim config should parse");
        let mut steps =
            generate_steps_with_config_and_runtime(15, &config, FrontierRuntimeLimits::unlimited())
                .expect("claim steps should build")
                .steps;
        let stored_closure = {
            let step = steps.last_mut().expect("late claim step should exist");
            let stored_closure = step.search_stats.demo_closure.clone();
            step.search_stats.demo_funnel.exact_bound_screened = 7;
            step.search_stats.demo_funnel.exact_bound_pruned = 1;
            stored_closure
        };

        let text = render_run_output(
            OutputStyle::Standard,
            "run-1",
            &steps,
            Some(NarrativeOutputConfig::from_runtime(&config)),
        );

        assert_eq!(steps.last().expect("late claim step").accepted.nu, 103);
        assert!(text.contains("run run-1"));
        assert!(text.contains("claim narrative"));
        assert!(text.contains(&format!(
            "{}% frontier_total={} certified_nonwinning={}",
            stored_closure.closure_percent,
            stored_closure.frontier_total_seen,
            stored_closure.frontier_certified_nonwinning
        )));
        assert!(!text.contains("14% frontier_total=7 certified_nonwinning=1"));
    }
}
