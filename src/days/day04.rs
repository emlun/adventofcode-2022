use crate::common::Solution;

fn solve_a(elves: &[((usize, usize), (usize, usize))]) -> usize {
    elves
        .iter()
        .filter(|((a_low, a_high), (b_low, b_high))| {
            (a_low <= b_low && a_high >= b_high) || (b_low <= a_low && b_high >= a_high)
        })
        .count()
}

pub fn solve(lines: &[String]) -> Solution {
    let elves: Vec<((usize, usize), (usize, usize))> = lines
        .iter()
        .map(|line| {
            let mut elf_splits = line.split(',');
            let mut elf_a_parts = elf_splits.next().unwrap().split('-');
            let mut elf_b_parts = elf_splits.next().unwrap().split('-');

            (
                (
                    elf_a_parts.next().unwrap().parse().unwrap(),
                    elf_a_parts.next().unwrap().parse().unwrap(),
                ),
                (
                    elf_b_parts.next().unwrap().parse().unwrap(),
                    elf_b_parts.next().unwrap().parse().unwrap(),
                ),
            )
        })
        .collect();
    (solve_a(&elves).to_string(), "".to_string())
}
