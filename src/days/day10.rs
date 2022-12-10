use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;
use crate::util::collections::GridCount;

pub fn solve(lines: &[String]) -> Solution {
    let targets = vec![20, 60, 100, 140, 180, 220];

    let (_, _, signal_a) = lines.iter().filter(|line| !line.is_empty()).fold(
        (1_i32, 1_i32, 0_i32),
        |(cycle, x, signal), line| {
            if let Some(xadd) = line.strip_prefix("addx ") {
                let new_signal = if targets.contains(&cycle) {
                    dbg!(cycle, x);
                    dbg!(signal + cycle * x)
                } else if targets.contains(&(cycle + 1)) {
                    dbg!(cycle, x);
                    dbg!(signal + (cycle + 1) * x)
                } else {
                    signal
                };

                (cycle + 2, x + xadd.parse::<i32>().unwrap(), new_signal)
            } else {
                let new_signal = if targets.contains(&cycle) {
                    dbg!(cycle, x);
                    dbg!(signal + cycle * x)
                } else {
                    signal
                };

                (cycle + 1, x, new_signal)
            }
        },
    );
    (signal_a.to_string(), "".to_string())
}
