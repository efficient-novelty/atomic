pub const DEFAULT_WINDOW_DEPTH: usize = 2;

pub fn d_bonacci(window_depth: usize, len: usize) -> Vec<u64> {
    if len == 0 {
        return Vec::new();
    }

    if window_depth <= 1 {
        return vec![1; len];
    }

    let mut sequence = vec![1; window_depth.min(len)];
    while sequence.len() < len {
        let start = sequence.len().saturating_sub(window_depth);
        let next = sequence[start..].iter().copied().sum();
        sequence.push(next);
    }

    sequence
}

pub fn d_bonacci_delta(window_depth: usize, n: usize) -> u64 {
    if n < 1 {
        1
    } else {
        d_bonacci(window_depth, n)[n - 1]
    }
}

pub fn d_bonacci_tau(window_depth: usize, n: usize) -> u64 {
    d_bonacci(window_depth, n).into_iter().sum()
}

pub fn active_window(step_index: u32, window_depth: u16, library_size: u32) -> Option<(u32, u32)> {
    if library_size == 0 || step_index <= 1 {
        return None;
    }

    let end = library_size.min(step_index - 1);
    let depth = u32::from(window_depth.max(1));
    let start = end.saturating_sub(depth.saturating_sub(1)).max(1);
    Some((start, end))
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_WINDOW_DEPTH, active_window, d_bonacci, d_bonacci_delta, d_bonacci_tau};

    #[test]
    fn fibonacci_window_matches_reference_values() {
        assert_eq!(DEFAULT_WINDOW_DEPTH, 2);
        assert_eq!(d_bonacci(2, 6), vec![1, 1, 2, 3, 5, 8]);
        assert_eq!(d_bonacci_delta(2, 5), 5);
        assert_eq!(d_bonacci_tau(2, 4), 7);
    }

    #[test]
    fn active_window_tracks_last_visible_steps() {
        assert_eq!(active_window(10, 2, 9), Some((8, 9)));
        assert_eq!(active_window(2, 2, 1), Some((1, 1)));
        assert_eq!(active_window(1, 2, 0), None);
    }
}
