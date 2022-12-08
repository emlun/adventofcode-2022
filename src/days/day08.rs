use crate::common::Solution;

fn is_visible(map: &[Vec<u32>], r: usize, c: usize) -> bool {
    let w = map[0].len();
    let h = map.len();
    let height = map[r][c];

    (0..c).all(|cc| map[r][cc] < height)
        || ((c + 1)..w).all(|cc| map[r][cc] < height)
        || (0..r).all(|rr| map[rr][c] < height)
        || ((r + 1)..h).all(|rr| map[rr][c] < height)
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

    (0..w)
        .flat_map(|r| (0..h).map(move |c| (r, c)))
        .filter(|(r, c)| is_visible(map, *r, *c))
        .count()
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
