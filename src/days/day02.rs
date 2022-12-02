use std::str::FromStr;

use crate::common::Solution;

enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn play(&self, other: &Move) -> PlayResult {
        use Move::*;
        use PlayResult::*;
        match (self, other) {
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Scissors, Paper) => Win,
            (Rock, Paper) => Loss,
            (Paper, Scissors) => Loss,
            (Scissors, Rock) => Loss,
            _ => Draw,
        }
    }
}

enum PlayResult {
    Win,
    Loss,
    Draw,
}

impl PlayResult {
    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Draw => 3,
        }
    }
}

impl FromStr for Move {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn solve_a(moves: &Vec<(Move, Move)>) -> i32 {
    moves.iter().fold(0, |score, (opp, me)| {
        score + me.score() + me.play(opp).score()
    })
}

pub fn solve(lines: &[String]) -> Solution {
    let moves: Vec<(Move, Move)> = lines
        .iter()
        .map(|line| {
            let mut splits = line.split(" ");
            (
                splits.next().unwrap().parse().unwrap(),
                splits.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    (solve_a(&moves).to_string(), "".to_string())
}
