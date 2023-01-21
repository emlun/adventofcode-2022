use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;
use crate::util::iter::Countable;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Point(i32, i32);

impl std::hash::Hash for Point {
    fn hash<H>(&self, h: &mut H)
    where
        H: std::hash::Hasher,
    {
        std::hash::Hash::hash(&pack((self.0, self.1)), h)
    }
}

fn pack((x, y): (i32, i32)) -> u32 {
    ((x + i32::from(u16::MAX / 2)) << 16) as u32 | (y + i32::from(u16::MAX / 2)) as u32
}

#[derive(Clone)]
struct State {
    map: HashSet<Point>,
    first_dir: usize,
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

fn step(state: State) -> Option<State> {
    let proposals: HashMap<Point, Point> = state
        .map
        .iter()
        .copied()
        .filter(|Point(x, y)| {
            (-1..=1)
                .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .filter(|(dx, dy)| (*dx, *dy) != (0, 0))
                .any(|(dx, dy)| state.map.contains(&Point(x + dx, y + dy)))
        })
        .flat_map(|Point(x, y)| {
            DIRECTIONS
                .iter()
                .cycle()
                .skip(state.first_dir)
                .take(4)
                .copied()
                .find_map(|(dx, dy)| {
                    let (xx, yy) = (x + dx, y + dy);
                    if !(-1..=1).any(|k| state.map.contains(&Point(xx - k * dy, yy - k * dx))) {
                        Some((Point(x, y), Point(xx, yy)))
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
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(minx, maxx, miny, maxy), Point(x, y)| {
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

fn solve_a(mut state: State) -> usize {
    for _ in 0..10 {
        state = step(state).unwrap();
    }
    measure_size(&state)
}

fn solve_b(mut state: State) -> usize {
    let mut i = 1;
    while let Some(s) = step(state) {
        state = s;
        i += 1;
    }
    i
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
                .map(move |(x, _)| Point(i32::try_from(x).unwrap(), -i32::try_from(y).unwrap()))
        })
        .collect();
    let state = State { map, first_dir: 0 };

    (
        solve_a(state.clone()).to_string(),
        solve_b(state).to_string(),
    )
}
