use std::collections::BinaryHeap;

use crate::common::Solution;

fn solve_a(elves: &[Vec<i32>]) -> i32 {
    elves.iter().map(|elf| elf.into_iter().sum()).max().unwrap()
}

fn solve_b(elves: &[Vec<i32>]) -> i32 {
    let mut top_elves: BinaryHeap<i32> = elves.iter().map(|elf| elf.into_iter().sum()).collect();
    top_elves.pop().unwrap() + top_elves.pop().unwrap() + top_elves.pop().unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let elves: Vec<Vec<i32>> = lines.iter().fold(vec![Vec::new()], |mut elves, line| {
        match line.as_str() {
            "" => elves.push(Vec::new()),
            nonempty => elves.last_mut().unwrap().push(nonempty.parse().unwrap()),
        }
        elves
    });

    (solve_a(&elves).to_string(), solve_b(&elves).to_string())
}
