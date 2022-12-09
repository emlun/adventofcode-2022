use crate::common::Solution;
use std::collections::HashSet;

fn solve_a(moves: &[(i32, i32)]) -> usize {
    let mut h_x = 0;
    let mut h_y = 0;
    let mut t_x = 0;
    let mut t_y = 0;
    let mut visited: HashSet<(i32, i32)> = vec![(0, 0)].into_iter().collect();
    for (dx, dy) in moves {
        h_x += dx;
        h_y += dy;

        let mut dhtx: i32 = h_x - t_x;
        let mut dhty: i32 = h_y - t_y;

        while dhtx.abs() >= 2 || dhty.abs() >= 2 {
            if dhtx.abs() + dhty.abs() >= 3 {
                t_x += dhtx.signum();
                t_y += dhty.signum();
            } else {
                if dhtx.abs() >= 2 {
                    t_x += dhtx.signum();
                }
                if dhty.abs() >= 2 {
                    t_y += dhty.signum();
                }
            }

            dhtx = h_x - t_x;
            dhty = h_y - t_y;
            visited.insert((t_x, t_y));
        }
    }
    visited.len()
}

pub fn solve(lines: &[String]) -> Solution {
    let moves: Vec<(i32, i32)> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut splits = line.split(' ');
            let l = splits.next().unwrap();
            let r = splits.next().unwrap();
            match l {
                "U" => (0, r.parse().unwrap()),
                "D" => (0, -r.parse::<i32>().unwrap()),
                "L" => (-r.parse::<i32>().unwrap(), 0),
                "R" => (r.parse().unwrap(), 0),
                _ => unimplemented!(),
            }
        })
        .collect();
    (solve_a(&moves).to_string(), "".to_string())
}
