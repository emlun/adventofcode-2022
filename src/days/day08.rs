use std::collections::HashSet;

use crate::common::Solution;

fn count_visible<R: Clone + Iterator<Item = usize>>(
    map: &[Vec<u32>],
    visible: &mut HashSet<usize>,
    mut maxes_horiz: Vec<u32>,
    mut maxes_vert: Vec<u32>,
    rs: R,
    cs: R,
) {
    for r in rs {
        for c in cs.clone() {
            if map[r][c] > maxes_horiz[c] {
                visible.insert((r << 16) | c);
                maxes_horiz[c] = map[r][c];
            }
            if map[r][c] > maxes_vert[r] {
                visible.insert((r << 16) | c);
                maxes_vert[r] = map[r][c];
            }
        }
    }
}

fn scenic_score(map: &[Vec<u32>], r: usize, c: usize) -> usize {
    let w = map[0].len();
    let h = map.len();
    let height = map[r][c];

    let trees_l = (0..c).rev().take_while(|cc| map[r][*cc] < height).count();
    let trees_r = ((c + 1)..w).take_while(|cc| map[r][*cc] < height).count();
    let trees_u = (0..r).rev().take_while(|rr| map[*rr][c] < height).count();
    let trees_d = ((r + 1)..h).take_while(|rr| map[*rr][c] < height).count();

    let view_dist_l = trees_l + usize::from(trees_l < c);
    let view_dist_r = trees_r + usize::from(trees_r < w - 1 - c);
    let view_dist_u = trees_u + usize::from(trees_u < r);
    let view_dist_d = trees_d + usize::from(trees_d < h - 1 - r);

    view_dist_l * view_dist_r * view_dist_u * view_dist_d
}

fn solve_a(map: &[Vec<u32>]) -> usize {
    let w = map[0].len();
    let h = map.len();

    let mut visible: HashSet<usize> = HashSet::new();
    count_visible(
        map,
        &mut visible,
        map[0].clone(),
        map.iter().map(|row| row[0]).collect(),
        1..(h - 1),
        1..(w - 1),
    );
    count_visible(
        map,
        &mut visible,
        map.last().unwrap().clone(),
        map.iter().map(|row| *row.last().unwrap()).collect(),
        (1..(h - 1)).rev(),
        (1..(w - 1)).rev(),
    );

    visible.len() + 2 * w + 2 * h - 4
}

fn solve_b(map: &[Vec<u32>]) -> usize {
    let w = map[0].len();
    let h = map.len();

    (0..w)
        .flat_map(|c| (0..h).map(move |r| (r, c)))
        .map(|(r, c)| scenic_score(map, r, c))
        .max()
        .unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    (solve_a(&map).to_string(), solve_b(&map).to_string())
}
