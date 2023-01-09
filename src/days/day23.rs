use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;
use crate::util::iter::Countable;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point(u32);

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self(((x + i32::from(u16::MAX / 2)) << 16) as u32 | (y + i32::from(u16::MAX / 2)) as u32)
    }
}

impl From<Point> for (i32, i32) {
    fn from(Point(xy): Point) -> Self {
        let x = (xy >> 16) & 0xffff;
        let y = xy & 0xffff;
        (
            x as i32 - i32::from(u16::MAX / 2),
            y as i32 - i32::from(u16::MAX / 2),
        )
    }
}

#[derive(Clone)]
struct State {
    map: HashSet<Point>,
    first_dir: usize,
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

fn step(state: &State) -> Option<State> {
    let proposals: HashMap<Point, Point> = state
        .map
        .iter()
        .copied()
        .map(|xy| xy.into())
        .filter(|(x, y)| {
            (-1..=1)
                .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
                .filter(|(dx, dy)| (*dx, *dy) != (0, 0))
                .any(|(dx, dy)| state.map.contains(&(x + dx, y + dy).into()))
        })
        .flat_map(|(x, y)| {
            DIRECTIONS
                .iter()
                .cycle()
                .skip(state.first_dir)
                .take(4)
                .copied()
                .find_map(|(dx, dy)| {
                    let (xx, yy) = (x + dx, y + dy);
                    if !(-1..=1).any(|k| state.map.contains(&(xx - k * dy, yy - k * dx).into())) {
                        Some(((x, y).into(), (xx, yy).into()))
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
    let (minx, maxx, miny, maxy) = state.map.iter().map(|xy| (*xy).into()).fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(minx, maxx, miny, maxy), (x, y)| {
            (
                std::cmp::min(minx, x),
                std::cmp::max(maxx, x),
                std::cmp::min(miny, y),
                std::cmp::max(maxy, y),
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
                .map(move |(x, _)| (i32::try_from(x).unwrap(), -i32::try_from(y).unwrap()))
        })
        .map(|xy| xy.into())
        .collect();
    let state = State { map, first_dir: 0 };

    (solve_a(&state).to_string(), solve_b(&state).to_string())
}
