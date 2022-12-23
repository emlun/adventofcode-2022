use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;

#[derive(Clone)]
struct State {
    map: HashSet<(isize, isize)>,
    first_dir: usize,
}

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

fn print_state(state: &State) {
    let (minx, maxx, miny, maxy) = state.map.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(minx, maxx, miny, maxy), (x, y)| {
            (
                std::cmp::min(minx, *x),
                std::cmp::max(maxx, *x),
                std::cmp::min(miny, *y),
                std::cmp::max(maxy, *y),
            )
        },
    );

    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            if state.map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".")
            };
        }
        println!();
    }
    println!();
}

fn rot((x, y): (isize, isize), r: usize) -> (isize, isize) {
    if r >= 1 {
        rot((-y, x), r - 1)
    } else {
        (x, y)
    }
}

fn step(state: &State) -> Option<State> {
    let (proposals, proposal_counts): (
        HashMap<(isize, isize), (isize, isize)>,
        HashMap<(isize, isize), usize>,
    ) = state
        .map
        .iter()
        .copied()
        .filter(|(x, y)| {
            (-1..=1)
                .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .filter(|(dx, dy)| (*dx, *dy) != (0, 0))
                .any(|(dx, dy)| state.map.contains(&(x + dx, y + dy)))
        })
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut props, mut prop_counts), (x, y): (isize, isize)| {
                // dbg!((x, y));
                if let Some((dx, dy)) = DIRECTIONS
                    .iter()
                    .cycle()
                    .skip(state.first_dir)
                    .take(4)
                    .copied()
                    .find(|(dx, dy)| {
                        let (ddx, ddy) = rot((*dx, *dy), 1);
                        // dbg!((dx, dy), (ddx, ddy));
                        (-1..=1).all(|k| {
                            !state
                                .map
                                // .contains(&dbg!((x + dx - k * ddx, y + dy - k * ddy)))
                                .contains(&(x + dx - k * ddx, y + dy - k * ddy))
                        })
                    })
                {
                    let prop = (x + dx, y + dy);
                    // dbg!(prop);
                    props.insert((x, y), prop);
                    prop_counts.entry(prop).and_modify(|m| *m += 1).or_insert(1);
                }
                (props, prop_counts)
            },
        );

    if proposals.is_empty() {
        None
    } else {
        Some(State {
            map: state
                .map
                .iter()
                .map(|orig| {
                    if let Some(prop) = proposals
                        .get(&orig)
                        .filter(|prop| proposal_counts[prop] == 1)
                    {
                        *prop
                    } else {
                        *orig
                    }
                })
                .collect(),
            first_dir: (state.first_dir + 1) % 4,
        })
    }
}

fn measure_size(state: &State) -> usize {
    let (minx, maxx, miny, maxy) = state.map.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(minx, maxx, miny, maxy), (x, y)| {
            (
                std::cmp::min(minx, *x),
                std::cmp::max(maxx, *x),
                std::cmp::min(miny, *y),
                std::cmp::max(maxy, *y),
            )
        },
    );
    ((maxx + 1 - minx) * (maxy + 1 - miny)) as usize - state.map.len()
}

fn solve_a(state: &State) -> usize {
    measure_size(
        &std::iter::successors(Some(state.clone()), step)
            .nth(10)
            .unwrap(),
    )
}

fn solve_b(state: &State) -> usize {
    std::iter::successors(Some(state.clone()), step).count()
}

pub fn solve(lines: &[String]) -> Solution {
    let map: HashSet<(isize, isize)> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (isize::try_from(x).unwrap(), -isize::try_from(y).unwrap()))
        })
        .collect();
    let state = State { map, first_dir: 0 };

    (solve_a(&state).to_string(), solve_b(&state).to_string())
}
