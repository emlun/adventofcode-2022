use std::collections::VecDeque;

use crate::common::Solution;

type Point = (usize, usize);
const ASCII_A: u8 = 0x61;
const ASCII_Z: u8 = 0x7a;

fn step(
    r: usize,
    c: usize,
    rr: usize,
    cc: usize,
    map: &[Vec<u8>],
    search_map: &mut Vec<Vec<Option<usize>>>,
    poss: &mut VecDeque<Point>,
) {
    let next = search_map[r][c].unwrap() + 1;
    if map[rr][cc] <= map[r][c] + 1 && search_map[rr][cc].map(|s| s > next).unwrap_or(true) {
        search_map[rr][cc] = Some(next);
        poss.push_back((rr, cc));
    }
}

fn steps(
    r: usize,
    c: usize,
    map: &[Vec<u8>],
    search_map: &mut Vec<Vec<Option<usize>>>,
    poss: &mut VecDeque<Point>,
) {
    if r > 0 {
        step(r, c, r - 1, c, map, search_map, poss);
    }
    if r + 1 < map.len() {
        step(r, c, r + 1, c, map, search_map, poss);
    }
    if c > 0 {
        step(r, c, r, c - 1, map, search_map, poss);
    }
    if c + 1 < map[0].len() {
        step(r, c, r, c + 1, map, search_map, poss);
    }
}

fn solve_b(pos: Point, pos_b: &[Point], goal: Point, map: &[Vec<u8>]) -> (usize, usize) {
    let mut search_map = vec![vec![None; map[0].len()]; map.len()];
    let mut poss = VecDeque::with_capacity(map.len() * 2);
    search_map[pos.0][pos.1] = Some(0);
    poss.push_back(pos);

    while let Some((r, c)) = poss.pop_front() {
        steps(r, c, map, &mut search_map, &mut poss);

        if (r, c) == goal {
            break;
        }
    }

    let sol_a = search_map[goal.0][goal.1].unwrap();

    poss.clear();
    poss.extend(pos_b);
    for pos in pos_b {
        search_map[pos.0][pos.1] = Some(0);
    }

    while let Some((r, c)) = poss.pop_front() {
        steps(r, c, map, &mut search_map, &mut poss);

        if (r, c) == goal {
            break;
        }
    }

    (sol_a, search_map[goal.0][goal.1].unwrap())
}

pub fn solve(lines: &[String]) -> Solution {
    let (pos, goal, pos_b, map): (Point, Point, Vec<Point>, Vec<Vec<u8>>) =
        lines.iter().filter(|line| !line.is_empty()).fold(
            ((0, 0), (0, 0), vec![], Vec::with_capacity(lines.len())),
            |(pos, goal, mut pos_b, mut lines), line| {
                lines.push(line.as_bytes().to_vec());
                for (pos_b_c, _) in line.match_indices('a') {
                    pos_b.push((lines.len() - 1, pos_b_c));
                }
                match (line.find('S'), line.find('E')) {
                    (Some(pos_c), Some(goal_c)) => {
                        lines.last_mut().unwrap()[pos_c] = ASCII_A;
                        lines.last_mut().unwrap()[goal_c] = ASCII_Z;
                        (
                            (lines.len() - 1, pos_c),
                            (lines.len() - 1, goal_c),
                            pos_b,
                            lines,
                        )
                    }
                    (Some(pos_c), None) => {
                        lines.last_mut().unwrap()[pos_c] = ASCII_A;
                        ((lines.len() - 1, pos_c), goal, pos_b, lines)
                    }
                    (None, Some(goal_c)) => {
                        lines.last_mut().unwrap()[goal_c] = ASCII_Z;
                        (pos, (lines.len() - 1, goal_c), pos_b, lines)
                    }
                    (None, None) => (pos, goal, pos_b, lines),
                }
            },
        );

    let (sol_a, sol_b) = solve_b(pos, &pos_b, goal, &map);
    (sol_a.to_string(), sol_b.to_string())
}
