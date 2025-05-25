use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Should work");

    let safe_reports = file
        .lines()
        .filter_map(|line| {
            if line.trim().is_empty() { return None; }
            // Parse once, bail out on any parse error
            let nums = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().ok())
                .collect::<Option<Vec<_>>>()?;
            Some(is_safe_with_dampener(&nums))
        })
        .filter(|&safe| safe)
        .count();

    println!("safe = {}", safe_reports);
}

fn is_safe(levels: &[i64]) -> bool {
    if levels.len() < 2 {
        return false;
    }

    let mut diffs = levels.windows(2).map(|w| w[1] - w[0]);
    // Early exit: check direction on first diff
    let first_diff = match diffs.next() {
        Some(d) if d != 0 => d,
        _ => return false, // equal or missing
    };

    // Determine expected sign: +1 for increasing, -1 for decreasing
    let sign = first_diff.signum();

    // Check that the first diff is in range
    if !(1..=3).contains(&first_diff.abs()) {
        return false;
    }

    // Now verify the rest follow the same sign and range
    diffs.all(|d| d.signum() == sign && (1..=3).contains(&d.abs()))
}

fn is_safe_with_dampener(levels: &[i64]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut reduced = Vec::with_capacity(levels.len() - 1);

        reduced.extend_from_slice(&levels[..i]);
        reduced.extend_from_slice(&levels[i+1..]);

        if is_safe(&reduced) {
            return true;
        }

    }

    false
}
