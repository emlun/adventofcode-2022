use std::str::FromStr;

use crate::common::Solution;

#[derive(Clone, Copy)]
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

    fn from_opponent(&self, opp: &Move) -> Move {
        use Move::*;
        use PlayResult::*;
        match (self, opp) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Loss, Rock) => Scissors,
            (Loss, Paper) => Rock,
            (Loss, Scissors) => Paper,
            (Draw, opp) => *opp,
        }
    }
}

impl FromStr for PlayResult {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

fn solve_a(moves: &Vec<(Move, Move, PlayResult)>) -> i32 {
    moves.iter().fold(0, |score, (opp, me, _)| {
        score + me.score() + me.play(opp).score()
    })
}

fn solve_b(moves: &Vec<(Move, Move, PlayResult)>) -> i32 {
    moves.iter().fold(0, |score, (opp, _, result)| {
        let me = result.from_opponent(opp);
        score + me.score() + result.score()
    })
}

pub fn solve(lines: &[String]) -> Solution {
    let moves: Vec<(Move, Move, PlayResult)> = lines
        .iter()
        .map(|line| {
            let mut splits = line.split(" ");
            let l = splits.next().unwrap();
            let r = splits.next().unwrap();
            (l.parse().unwrap(), r.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    (solve_a(&moves).to_string(), solve_b(&moves).to_string())
}
