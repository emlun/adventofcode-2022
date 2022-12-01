use std::collections::BinaryHeap;

use crate::common::Solution;

fn solve_a(elves: &BinaryHeap<i32>) -> &i32 {
    elves.peek().unwrap()
}

fn solve_b(mut elves: BinaryHeap<i32>) -> i32 {
    elves.pop().unwrap() + elves.pop().unwrap() + elves.pop().unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let elves: BinaryHeap<i32> = lines
        .iter()
        .fold(vec![0], |mut elves, line| {
            match line.as_str() {
                "" => elves.push(0),
                nonempty => *elves.last_mut().unwrap() += nonempty.parse::<i32>().unwrap(),
            }
            elves
        })
        .into();

    (solve_a(&elves).to_string(), solve_b(elves).to_string())
}
