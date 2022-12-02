use crate::common::Solution;

const ASCII_A: u8 = 0x41;
const ASCII_X: u8 = 0x58;

pub fn solve(lines: &[String]) -> Solution {
    let (solution_a, solution_b): (u32, u32) =
        lines.iter().fold((0, 0), |(score_a, score_b), line| {
            let mut splits = line.split(' ');
            let opp = splits.next().unwrap().as_bytes()[0] - ASCII_A;
            let r = splits.next().unwrap().as_bytes()[0] - ASCII_X;

            let my_move_a = r;
            let my_move_b = (r + opp + 2) % 3;
            let result_a = (my_move_a + (3 - opp) + 1) % 3;

            (
                score_a + u32::from(my_move_a + 1 + result_a * 3),
                score_b + u32::from(my_move_b + 1 + r * 3),
            )
        });

    (solution_a.to_string(), solution_b.to_string())
}
