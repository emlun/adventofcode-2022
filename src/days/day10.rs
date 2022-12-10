use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;
use crate::util::collections::GridCount;

pub fn solve(lines: &[String]) -> Solution {
    let targets = vec![20, 60, 100, 140, 180, 220];

    let (_, _, signal_a, _, crt) = lines.iter().filter(|line| !line.is_empty()).fold(
        (1_i32, 1_i32, 0_i32, 0_i32, Vec::with_capacity(40 * 6)),
        |(cycle, x, signal, crt_pos, mut crt), line| {
            if crt_pos % 40 == 0 {
                crt.push('\n');
            }

            if (x - crt_pos).abs() <= 1 {
                crt.push('#');
            } else {
                crt.push('.');
            }

            if let Some(xadd) = line.strip_prefix("addx ") {
                if (x - ((crt_pos + 1) % 40)).abs() <= 1 {
                    crt.push('#');
                } else {
                    crt.push('.');
                }
                if (crt_pos + 1) % 40 == 0 {
                    crt.push('\n');
                }

                let new_signal = if targets.contains(&cycle) {
                    dbg!(cycle, x);
                    dbg!(signal + cycle * x)
                } else if targets.contains(&(cycle + 1)) {
                    dbg!(cycle, x);
                    dbg!(signal + (cycle + 1) * x)
                } else {
                    signal
                };

                (
                    cycle + 2,
                    x + xadd.parse::<i32>().unwrap(),
                    new_signal,
                    (crt_pos + 2) % 40,
                    crt,
                )
            } else {
                let new_signal = if targets.contains(&cycle) {
                    dbg!(cycle, x);
                    dbg!(signal + cycle * x)
                } else {
                    signal
                };

                (cycle + 1, x, new_signal, (crt_pos + 1) % 40, crt)
            }
        },
    );
    let sol_b: String = crt.into_iter().collect();
    (signal_a.to_string(), sol_b)
}
