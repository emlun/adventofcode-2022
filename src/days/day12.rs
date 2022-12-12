use std::collections::VecDeque;

use crate::common::Solution;

fn solve_a(pos: (usize, usize), goal: (usize, usize), map: &[Vec<u8>]) -> usize {
    let mut search_map = vec![vec![None; map[0].len()]; map.len()];
    let mut poss = VecDeque::with_capacity(map.len() * 2);
    search_map[pos.0][pos.1] = Some(0);
    poss.push_back(pos);

    while let Some((r, c)) = poss.pop_front() {
        if (r, c) == goal {
            return search_map[r][c].unwrap();
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
    unimplemented!()
}

const ASCII_A: u8 = 0x61;
const ASCII_Z: u8 = 0x7a;

pub fn solve(lines: &[String]) -> Solution {
    let (pos, goal, map): ((usize, usize), (usize, usize), Vec<Vec<u8>>) =
        lines.iter().filter(|line| !line.is_empty()).fold(
            ((0, 0), (0, 0), Vec::with_capacity(lines.len())),
            |(pos, goal, mut lines), line| {
                lines.push(line.as_bytes().iter().copied().collect());
                match (line.find('S'), line.find('E')) {
                    (Some(pos_c), Some(goal_c)) => {
                        lines.last_mut().unwrap()[pos_c] = ASCII_A;
                        lines.last_mut().unwrap()[goal_c] = ASCII_Z;
                        ((lines.len() - 1, pos_c), (lines.len() - 1, goal_c), lines)
                    }
                    (Some(pos_c), None) => {
                        lines.last_mut().unwrap()[pos_c] = ASCII_A;
                        ((lines.len() - 1, pos_c), goal, lines)
                    }
                    (None, Some(goal_c)) => {
                        lines.last_mut().unwrap()[goal_c] = ASCII_Z;
                        (pos, (lines.len() - 1, goal_c), lines)
                    }
                    (None, None) => (pos, goal, lines),
                }
            },
        );
    (solve_a(pos, goal, &map).to_string(), "".to_string())
}
