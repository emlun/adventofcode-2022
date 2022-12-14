use std::collections::HashMap;
use std::collections::VecDeque;

use crate::common::Solution;
use crate::util::iter::WithSliding;

fn solve_a(mut map: Vec<Vec<bool>>) -> usize {
    const SPAWN_X: usize = 500;
    const SPAWN_Y: usize = 0;
    let abyss_y = map
        .iter()
        .enumerate()
        .rev()
        .find(|(_, row)| !row.is_empty())
        .map(|(i, _)| i)
        .unwrap();

    let mut resting = 0;

    loop {
        let (mut sandx, mut sandy) = (SPAWN_X, SPAWN_Y);

        loop {
            if sandy >= abyss_y {
                return resting;
            } else if !map
                .get(sandy + 1)
                .and_then(|row| row.get(sandx))
                .unwrap_or(&false)
            {
                sandy += 1;
            } else if !map
                .get(sandy + 1)
                .and_then(|row| row.get(sandx - 1))
                .unwrap_or(&false)
            {
                sandy += 1;
                sandx -= 1;
            } else if !map
                .get(sandy + 1)
                .and_then(|row| row.get(sandx + 1))
                .unwrap_or(&false)
            {
                sandy += 1;
                sandx += 1;
            } else {
                if map.len() <= sandy {
                    map.resize(sandy * 2, Vec::new());
                }
                if map[sandy].len() <= sandx {
                    map[sandy].resize(sandx * 2, false);
                }
                map[sandy][sandx] = true;
                resting += 1;
                break;
            }
        }
    }
}

pub fn solve(lines: &[String]) -> Solution {
    let map = lines
        .iter()
        .filter(|line| !line.is_empty())
        .fold(Vec::new(), |mut map, line| {
            for ((startx, starty), (endx, endy)) in line
                .split(" -> ")
                .map(|s| {
                    let mut halves = s.split(',');
                    (
                        halves.next().unwrap().parse().unwrap(),
                        halves.next().unwrap().parse().unwrap(),
                    )
                })
                .sliding2()
            {
                let maxy = std::cmp::max(starty, endy);
                let maxx = std::cmp::max(startx, endx);
                if map.len() <= maxy {
                    map.resize((maxy + 1) * 2, Vec::new());
                }
                for y in std::cmp::min(starty, endy)..=maxy {
                    if map[y].len() <= maxx {
                        map[y].resize((maxx + 1) * 2, false);
                    }
                    for x in std::cmp::min(startx, endx)..=maxx {
                        map[y][x] = true;
                    }
                }
            }

            map
        });

    (solve_a(map).to_string(), "".to_string())
}
