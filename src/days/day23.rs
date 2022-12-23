use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;
use crate::util::iter::Countable;

type Point = (isize, isize);

#[derive(Clone)]
struct State {
    map: HashSet<Point>,
    first_dir: usize,
}

const DIRECTIONS: [Point; 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

fn rot((x, y): Point, r: usize) -> Point {
    if r >= 1 {
        rot((-y, x), r - 1)
    } else {
        (x, y)
    }
}

fn step(state: &State) -> Option<State> {
    let proposals: HashMap<Point, Point> = state
        .map
        .iter()
        .copied()
        .filter(|(x, y)| {
            (-1..=1)
                .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .filter(|(dx, dy)| (*dx, *dy) != (0, 0))
                .any(|(dx, dy)| state.map.contains(&(x + dx, y + dy)))
        })
        .flat_map(|(x, y): Point| {
            DIRECTIONS
                .iter()
                .cycle()
                .skip(state.first_dir)
                .take(4)
                .copied()
                .find_map(|(dx, dy)| {
                    let (ddx, ddy) = rot((dx, dy), 1);
                    let (xx, yy) = (x + dx, y + dy);
                    if (-1..=1).all(|k| !state.map.contains(&(xx - k * ddx, yy - k * ddy))) {
                        Some(((x, y), (xx, yy)))
                    } else {
                        None
                    }
                })
        })
        .collect();

    if proposals.is_empty() {
        None
    } else {
        let proposal_counts = proposals.values().counts();

        Some(State {
            map: state
                .map
                .iter()
                .map(|orig| {
                    if let Some(prop) = proposals
                        .get(orig)
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
    let map: HashSet<Point> = lines
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
