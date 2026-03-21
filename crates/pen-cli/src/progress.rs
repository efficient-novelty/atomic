use crate::report::{StepProgressObserver, StepReport};
use std::io::{self, IsTerminal, Write};

pub fn format_millis(millis: u64) -> String {
    if millis < 1_000 {
        return format!("{millis}ms");
    }
    if millis < 60_000 {
        let whole = millis / 1_000;
        let tenths = (millis % 1_000) / 100;
        return format!("{whole}.{tenths}s");
    }

    let total_seconds = millis / 1_000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{minutes}m{seconds:02}s")
}

pub fn format_seconds(seconds: u32) -> String {
    format_millis(u64::from(seconds) * 1_000)
}

pub fn render_goal_line(label: &str, current: u64, target: Option<u64>, noun: &str) -> String {
    match target {
        Some(target) if target > 0 => {
            let status = if current >= target { "hit" } else { "miss" };
            format!(
                "{label:<12} [{}] {current} / {target} {noun} ({status})",
                render_bar(current, target)
            )
        }
        _ => format!("{label:<12} {current} {noun}"),
    }
}

pub fn render_limit_line(label: &str, current: u64, limit: Option<u64>, noun: &str) -> String {
    match limit {
        Some(limit) if limit > 0 => {
            let status = if current <= limit {
                "within limit"
            } else {
                "over limit"
            };
            format!(
                "{label:<12} [{}] {current} / {limit} {noun} ({status})",
                render_bar(current.min(limit), limit)
            )
        }
        _ => format!("{label:<12} {current} {noun}"),
    }
}

pub fn render_bar(current: u64, target: u64) -> String {
    const WIDTH: u64 = 16;
    if target == 0 {
        return "-".repeat(WIDTH as usize);
    }
    let filled = ((current.min(target) * WIDTH) + target.saturating_sub(1)) / target;
    let mut bar = String::with_capacity(WIDTH as usize);
    for index in 0..WIDTH {
        bar.push(if index < filled { '#' } else { '.' });
    }
    bar
}

pub struct TerminalStepProgress<W: Write> {
    writer: W,
    until_step: u32,
}

impl TerminalStepProgress<io::Stderr> {
    pub fn stderr(until_step: u32) -> Option<Self> {
        let stderr = io::stderr();
        if stderr.is_terminal() {
            Some(Self::new(stderr, until_step))
        } else {
            None
        }
    }
}

impl<W: Write> TerminalStepProgress<W> {
    pub fn new(writer: W, until_step: u32) -> Self {
        Self { writer, until_step }
    }

    fn emit_line(&mut self, line: String) {
        let _ = writeln!(self.writer, "{line}");
        let _ = self.writer.flush();
    }

    fn total_steps(&self, step_index: u32) -> u32 {
        self.until_step.max(step_index)
    }
}

impl<W: Write> StepProgressObserver for TerminalStepProgress<W> {
    fn on_step_started(&mut self, step_index: u32, label: &str) {
        self.emit_line(format!(
            "[progress] step {step_index}/{total} ({label}) started",
            total = self.total_steps(step_index)
        ));
    }

    fn on_step_completed(&mut self, step: &StepReport) {
        let duration = step.search_stats.search_timing.step_wall_clock_millis;
        let mut details = vec![
            format!("nu={}", step.accepted.nu),
            format!("kappa={}", step.accepted.clause_kappa),
        ];
        if duration > 0 {
            details.push(format!("time={}", format_millis(duration)));
        }

        let generated = step.search_stats.demo_funnel.generated_raw_prefixes;
        let exact_screened = step.search_stats.demo_funnel.exact_bound_screened;
        if generated > 0 || exact_screened > 0 {
            details.push(format!("generated={generated}"));
            details.push(format!("exact={exact_screened}"));
            details.push(format!(
                "full_eval={}",
                step.search_stats.full_telescopes_evaluated
            ));
        } else {
            details.push(format!(
                "evaluated={}",
                step.search_stats.evaluated_candidates
            ));
            details.push(format!(
                "canonical={}",
                step.search_stats.canonical_candidates
            ));
        }

        self.emit_line(format!(
            "[progress] step {step_index}/{total} ({label}) complete | {details}",
            step_index = step.step_index,
            total = self.total_steps(step.step_index),
            label = step.label,
            details = details.join(" | "),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::{TerminalStepProgress, format_millis, render_goal_line, render_limit_line};
    use crate::report::{StepProgressObserver, replay_reference_steps};

    #[test]
    fn progress_helpers_render_human_readable_units() {
        assert_eq!(format_millis(250), "250ms");
        assert_eq!(format_millis(1_200), "1.2s");
        assert_eq!(format_millis(61_000), "1m01s");
    }

    #[test]
    fn progress_helpers_render_goal_and_limit_bars() {
        assert!(render_goal_line("breadth", 5, Some(10), "generated").contains("(miss)"));
        assert!(render_limit_line("cap", 5, Some(10), "evaluated").contains("within limit"));
    }

    #[test]
    fn terminal_progress_streams_step_start_and_completion_lines() {
        let step = replay_reference_steps(1, 2)
            .expect("reference replay should succeed")
            .into_iter()
            .next()
            .expect("step should exist");
        let mut buffer = Vec::new();
        let mut progress = TerminalStepProgress::new(&mut buffer, 15);

        progress.on_step_started(step.step_index, &step.label);
        progress.on_step_completed(&step);

        let text = String::from_utf8(buffer).expect("buffer should be utf8");
        assert!(text.contains("[progress] step 1/15 (Universe) started"));
        assert!(text.contains("[progress] step 1/15 (Universe) complete"));
        assert!(text.contains("nu=1"));
        assert!(text.contains("evaluated=1"));
    }
}
