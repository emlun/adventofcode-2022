use std::collections::VecDeque;

use crate::common::Solution;

type Point = (usize, usize);
const ASCII_A: u8 = 0x61;
const ASCII_Z: u8 = 0x7a;

fn solve_a(pos: (usize, usize), goal: (usize, usize), map: &[Vec<u8>]) -> Option<usize> {
    let mut search_map = vec![vec![None; map[0].len()]; map.len()];
    let mut poss = VecDeque::with_capacity(map.len() * 2);
    search_map[pos.0][pos.1] = Some(0);
    poss.push_back(pos);

    while let Some((r, c)) = poss.pop_front() {
        if (r, c) == goal {
            return Some(search_map[r][c].unwrap());
        } else {
            if r > 0 {
                let rr = r - 1;
                if map[rr][c] <= map[r][c] + 1 && search_map[rr][c].is_none() {
                    search_map[rr][c] = Some(search_map[r][c].unwrap() + 1);
                    poss.push_back((rr, c));
                }
            }
            if r < map.len() - 1 {
                let rr = r + 1;
                if map[rr][c] <= map[r][c] + 1 && search_map[rr][c].is_none() {
                    search_map[rr][c] = Some(search_map[r][c].unwrap() + 1);
                    poss.push_back((rr, c));
                }
            }
            if c > 0 {
                let cc = c - 1;
                if map[r][cc] <= map[r][c] + 1 && search_map[r][cc].is_none() {
                    search_map[r][cc] = Some(search_map[r][c].unwrap() + 1);
                    poss.push_back((r, cc));
                }
            }
            if c < map[0].len() - 1 {
                let cc = c + 1;
                if map[r][cc] <= map[r][c] + 1 && search_map[r][cc].is_none() {
                    search_map[r][cc] = Some(search_map[r][c].unwrap() + 1);
                    poss.push_back((r, cc));
                }
            }
        }
    }
    None
}

fn solve_b(poss: &[(usize, usize)], goal: (usize, usize), map: &[Vec<u8>]) -> usize {
    poss.iter()
        .flat_map(|pos| solve_a(*pos, goal, map))
        .min()
        .unwrap()
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
                        pos_b.push((lines.len() - 1, pos_c));
                        (
                            (lines.len() - 1, pos_c),
                            (lines.len() - 1, goal_c),
                            pos_b,
                            lines,
                        )
                    }
                    (Some(pos_c), None) => {
                        lines.last_mut().unwrap()[pos_c] = ASCII_A;
                        pos_b.push((lines.len() - 1, pos_c));
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
    (
        solve_a(pos, goal, &map).unwrap().to_string(),
        solve_b(&pos_b, goal, &map).to_string(),
    )
}
