use crate::common::Solution;
use crate::util::collections::GridCount;

fn solve_b(moves: &[(i32, i32)], parts: usize) -> (usize, usize) {
    let mut pos: Vec<(i32, i32)> = vec![(0, 0); parts];
    let mut visited_a: GridCount = GridCount::new();
    let mut visited_b: GridCount = GridCount::new();
    visited_a.insert((0, 0));
    visited_b.insert((0, 0));

    for (dx, dy) in moves {
        pos[0].0 += dx;
        pos[0].1 += dy;

        let mut any_changed = true;
        while any_changed {
            any_changed = false;
            for i in 1..pos.len() {
                let dhtx: i32 = pos[i - 1].0 - pos[i].0;
                let dhty: i32 = pos[i - 1].1 - pos[i].1;

                if dhtx.abs() >= 2 || dhty.abs() >= 2 {
                    if dhtx.abs() + dhty.abs() >= 3 {
                        pos[i].0 += dhtx.signum();
                        pos[i].1 += dhty.signum();
                    } else {
                        if dhtx.abs() >= 2 {
                            pos[i].0 += dhtx.signum();
                        }
                        if dhty.abs() >= 2 {
                            pos[i].1 += dhty.signum();
                        }
                    }

                    if i == 1 {
                        visited_a.insert(pos[i]);
                    }
                    if i == pos.len() - 1 {
                        visited_b.insert(pos[i]);
                    }
                    any_changed = true;
                } else {
                    break;
                }
            }
        }
    }

    (visited_a.len(), visited_b.len())
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
    let (sol_a, sol_b) = solve_b(&moves, 10);
    (sol_a.to_string(), sol_b.to_string())
}
