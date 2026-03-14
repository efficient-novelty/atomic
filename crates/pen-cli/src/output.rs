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

pub fn render_run_output(style: OutputStyle, run_id: &str, steps: &[StepReport]) -> String {
    match style {
        OutputStyle::Standard => render_standard_report(run_id, steps),
        OutputStyle::Debug => render_debug_report(run_id, steps),
    }
}
