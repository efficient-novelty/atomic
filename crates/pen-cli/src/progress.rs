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

#[cfg(test)]
mod tests {
    use super::{format_millis, render_goal_line, render_limit_line};

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
}
