use std::collections::HashSet;

use crate::common::Solution;

const ASCII_A: u8 = 0x41;
const ASCII_A_LOW: u8 = 0x61;

fn solve_a(rucksacks: &[(HashSet<u8>, HashSet<u8>)]) -> i32 {
    rucksacks
        .iter()
        .flat_map(|(l, r)| l.intersection(r))
        .map(|c| {
            i32::from(if *c >= ASCII_A_LOW {
                c - ASCII_A_LOW + 1
            } else {
                c - ASCII_A + 27
            })
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let rucksacks: Vec<(HashSet<u8>, HashSet<u8>)> = lines
        .iter()
        .map(|line| {
            let (l, r) = line.split_at(line.len() / 2);
            (
                l.as_bytes().into_iter().copied().collect(),
                r.as_bytes().into_iter().copied().collect(),
            )
        })
        .collect();

    (solve_a(&rucksacks).to_string(), "".to_string())
}
