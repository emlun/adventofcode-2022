use crate::common::Solution;

const TARGETS: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn run_cycle(
    cycle: i32,
    x: i32,
    signal: i32,
    crt_pos: i32,
    crt: &mut Vec<char>,
    addx: Option<i32>,
) -> (i32, i32, i32, i32) {
    if crt_pos == 0 {
        crt.push('\n');
    }

    if (x - crt_pos).abs() <= 1 {
        crt.push('#');
    } else {
        crt.push('.');
    }

    let new_signal = if TARGETS.contains(&cycle) {
        signal + cycle * x
    } else {
        signal
    };

    (
        cycle + 1,
        x + addx.unwrap_or(0),
        new_signal,
        (crt_pos + 1) % 40,
    )
}

pub fn solve(lines: &[String]) -> Solution {
    let (_, _, signal_a, _, crt) = lines.iter().filter(|line| !line.is_empty()).fold(
        (1_i32, 1_i32, 0_i32, 0_i32, Vec::with_capacity(40 * 6)),
        |(mut cycle, mut x, mut signal, mut crt_pos, mut crt), line| {
            (cycle, x, signal, crt_pos) = run_cycle(cycle, x, signal, crt_pos, &mut crt, None);
            if let Some(xadd) = line.strip_prefix("addx ").map(|s| s.parse().unwrap()) {
                (cycle, x, signal, crt_pos) =
                    run_cycle(cycle, x, signal, crt_pos, &mut crt, Some(xadd));
            }
            (cycle, x, signal, crt_pos, crt)
        },
    );
    let sol_b: String = crt.into_iter().collect();
    (signal_a.to_string(), sol_b)
}
