use crate::common::Solution;
use crate::util::collections::GridCount;

fn solve_b(moves: &[(i32, i32)], parts: usize) -> usize {
    let mut pos: Vec<(i32, i32)> = vec![(0, 0); parts];
    let mut visited: GridCount = GridCount::new();
    visited.insert((0, 0));

    for (dx, dy) in moves {
        pos[0].0 += dx;
        pos[0].1 += dy;

        let mut done_index = 0;
        while (done_index + 1) < pos.len() {
            for i in (done_index + 1)..pos.len() {
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

                    if i == pos.len() - 1 {
                        visited.insert(pos[i]);
                    }
                } else if i == done_index + 1 {
                    done_index = i;
                } else {
                    break;
                }
            }
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
    (
        solve_b(&moves, 2).to_string(),
        solve_b(&moves, 10).to_string(),
    )
}
