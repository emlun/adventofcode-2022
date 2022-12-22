use std::collections::HashMap;

use crate::common::Solution;

struct Map {
    minic: Vec<usize>,
    maxxc: Vec<usize>,
    minir: Vec<usize>,
    maxxr: Vec<usize>,
    walls: Vec<Vec<bool>>,
}

impl Map {
    fn new(h: usize) -> Self {
        Self {
            minic: Vec::with_capacity(h),
            maxxc: Vec::with_capacity(h),
            minir: Vec::with_capacity(h),
            maxxr: Vec::with_capacity(h),
            walls: Vec::with_capacity(h),
        }
    }
}

fn solve_a(map: &Map, path_len: &[usize], path_turn: &[bool]) -> usize {
    let mut c = map.minic[0];
    let mut r = map.minir[c];
    let mut dir = 0;

    for i in 0..path_len.len() {
        let h = map.maxxr[c] - map.minir[c];
        let w = map.maxxc[r] - map.minic[r];

        let (dr, dc) = match dir {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, w - 1),
            3 => (h - 1, 0),
            _ => unimplemented!(),
        };

        let dx = (1..=path_len[i])
            .take_while(|dx| {
                let nr = map.minir[c] + (r - map.minir[c] + dr * dx) % h;
                let nc = map.minic[r] + (c - map.minic[r] + dc * dx) % w;
                !map.walls[nr][nc]
            })
            .last()
            .unwrap_or(0);

        r = if dr == 0 {
            r
        } else {
            map.minir[c] + (r - map.minir[c] + dr * dx) % h
        };
        c = if dc == 0 {
            c
        } else {
            map.minic[r] + (c - map.minic[r] + dc * dx) % w
        };

        dir = match path_turn.get(i) {
            Some(true) => (dir + 1) % 4,
            Some(false) => (dir + 3) % 4,
            None => dir,
        }
    }

    (r + 1) * 1000 + (c + 1) * 4 + dir
}

pub fn solve(lines: &[String]) -> Solution {
    let mut map: Map = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .fold(Map::new(lines.len()), |mut map, (r, line)| {
            map.maxxc.push(0);
            map.minir
                .resize(std::cmp::max(map.minir.len(), line.len()), 0);
            map.maxxr
                .resize(std::cmp::max(map.maxxr.len(), line.len()), 0);
            map = line.chars().enumerate().fold(map, |mut map, (c, chr)| {
                if chr == '.' || chr == '#' {
                    if map.minic.len() <= r {
                        map.minic.push(c);
                    }
                    if map.minir[c] == 0 {
                        map.minir[c] = r + 1;
                    }
                    map.maxxc[r] = c + 1;
                    map.maxxr[c] = r + 1;
                }
                map
            });

            map.walls.push(line.chars().map(|chr| chr == '#').collect());

            map
        });
    map.minir = map.minir.iter().map(|minir| minir - 1).collect();

    let path_len: Vec<usize> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .last()
        .unwrap()
        .split(|chr| chr == 'L' || chr == 'R')
        .map(|s| s.parse().unwrap())
        .collect();
    let path_turn: Vec<bool> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .last()
        .unwrap()
        .chars()
        .filter(|chr| *chr == 'L' || *chr == 'R')
        .map(|chr| chr == 'R')
        .collect();

    (
        solve_a(&map, &path_len, &path_turn).to_string(),
        "".to_string(),
    )
}
