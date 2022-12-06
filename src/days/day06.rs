use crate::common::Solution;
use std::collections::HashMap;

fn solve_a(line: &str) -> usize {
    let chars: Vec<char> = line.chars().collect();
    for i in 3..line.len() {
        if chars[i - 3] != chars[i - 2]
            && chars[i - 3] != chars[i - 1]
            && chars[i - 3] != chars[i]
            && chars[i - 2] != chars[i - 1]
            && chars[i - 2] != chars[i]
            && chars[i - 1] != chars[i]
        {
            return i + 1;
        }
    }
    unimplemented!()
}

fn solve_b(line: &str) -> usize {
    let chars: Vec<char> = line.chars().collect();
    let mut counts: HashMap<char, usize> = HashMap::new();
    const N: usize = 14;

    for i in 0..N {
        *counts.entry(chars[i]).or_insert(0) += 1;
    }

    for i in N..line.len() {
        if counts.values().filter(|v| **v > 0).count() >= N {
            return i;
        }

        *counts.get_mut(&chars[i - N]).unwrap() -= 1;
        *counts.entry(chars[i]).or_insert(0) += 1;
    }
    unimplemented!()
}

pub fn solve(lines: &[String]) -> Solution {
    let line = &lines[0];
    let solution_a = solve_a(line);

    (solution_a.to_string(), solve_b(line).to_string())
}
