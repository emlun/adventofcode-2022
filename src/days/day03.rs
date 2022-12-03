use crate::common::Solution;

const ASCII_A: u8 = 0x41;
const ASCII_A_LOW: u8 = 0x61;

fn char_to_flag(c: &u8) -> u64 {
    let idx = if *c >= ASCII_A_LOW {
        c - ASCII_A_LOW + 1
    } else {
        c - ASCII_A + 27
    };
    1 << idx
}

fn flags_to_priority_sum(flags: u64) -> i32 {
    (1..=52).filter(|i| flags & (1 << i) != 0).sum()
}

fn solve_a(rucksacks: &[(u64, u64)]) -> i32 {
    rucksacks
        .iter()
        .map(|(l, r)| l & r)
        .map(flags_to_priority_sum)
        .sum()
}

fn solve_b(rucksacks: &[(u64, u64)]) -> i32 {
    let mut sacks = rucksacks.iter();
    let mut result = 0;
    while let (Some((la, ra)), Some((lb, rb)), Some((lc, rc))) =
        (sacks.next(), sacks.next(), sacks.next())
    {
        result += flags_to_priority_sum((la | ra) & (lb | rb) & (lc | rc));
    }
    result
}

pub fn solve(lines: &[String]) -> Solution {
    let rucksacks: Vec<(u64, u64)> = lines
        .iter()
        .map(|line| {
            let (l, r) = line.split_at(line.len() / 2);
            (
                l.as_bytes().iter().fold(0, |acc, c| acc | char_to_flag(c)),
                r.as_bytes().iter().fold(0, |acc, c| acc | char_to_flag(c)),
            )
        })
        .collect();
    (
        solve_a(&rucksacks).to_string(),
        solve_b(&rucksacks).to_string(),
    )
}
