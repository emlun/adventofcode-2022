use crate::common::Solution;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

fn play_a(opp: &str, me: &str) -> i32 {
    match (opp, me) {
        ("A", "X") => ROCK + DRAW,
        ("B", "Y") => PAPER + DRAW,
        ("C", "Z") => SCISSORS + DRAW,

        ("A", "Y") => PAPER + WIN,
        ("B", "Z") => SCISSORS + WIN,
        ("C", "X") => ROCK + WIN,

        ("A", "Z") => SCISSORS + LOSS,
        ("B", "X") => ROCK + LOSS,
        ("C", "Y") => PAPER + LOSS,

        _ => unimplemented!(),
    }
}

fn play_b(opp: &str, result: &str) -> i32 {
    match (opp, result) {
        ("A", "X") => SCISSORS + LOSS,
        ("B", "X") => ROCK + LOSS,
        ("C", "X") => PAPER + LOSS,

        ("A", "Y") => ROCK + DRAW,
        ("B", "Y") => PAPER + DRAW,
        ("C", "Y") => SCISSORS + DRAW,

        ("A", "Z") => PAPER + WIN,
        ("B", "Z") => SCISSORS + WIN,
        ("C", "Z") => ROCK + WIN,

        _ => unimplemented!(),
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let (solution_a, solution_b) = lines.iter().fold((0, 0), |(score_a, score_b), line| {
        let mut splits = line.split(' ');
        let l = splits.next().unwrap();
        let r = splits.next().unwrap();
        (score_a + play_a(l, r), score_b + play_b(l, r))
    });

    (solution_a.to_string(), solution_b.to_string())
}
