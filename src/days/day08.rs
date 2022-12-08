use crate::common::Solution;

fn is_visible(map: &[Vec<u32>], r: usize, c: usize) -> bool {
    let W = map[0].len();
    let H = map.len();
    let height = map[r][c];

    (0..c).all(|cc| map[r][cc] < height)
        || ((c + 1)..W).all(|cc| map[r][cc] < height)
        || (0..r).all(|rr| map[rr][c] < height)
        || ((r + 1)..H).all(|rr| map[rr][c] < height)
}

fn scenic_score(map: &[Vec<u32>], r: usize, c: usize) -> usize {
    let W = map[0].len();
    let H = map.len();
    let height = map[r][c];

    let trees_l = (0..c).rev().take_while(|cc| map[r][*cc] < height).count();
    let trees_r = ((c + 1)..W).take_while(|cc| map[r][*cc] < height).count();
    let trees_u = (0..r).rev().take_while(|rr| map[*rr][c] < height).count();
    let trees_d = ((r + 1)..H).take_while(|rr| map[*rr][c] < height).count();

    let view_dist_l = trees_l + if trees_l < c { 1 } else { 0 };
    let view_dist_r = trees_r + if trees_r < W - 1 - c { 1 } else { 0 };
    let view_dist_u = trees_u + if trees_u < r { 1 } else { 0 };
    let view_dist_d = trees_d + if trees_d < H - 1 - r { 1 } else { 0 };

    view_dist_l * view_dist_r * view_dist_u * view_dist_d
}

fn solve_a(map: &[Vec<u32>]) -> usize {
    let W = map[0].len();
    let H = map.len();

    let mut num_visible = 0;
    for r in 0..W {
        for c in 0..H {
            if is_visible(map, r, c) {
                num_visible += 1;
            }
        }
    }

    num_visible
}

fn solve_b(map: &[Vec<u32>]) -> usize {
    let W = map[0].len();
    let H = map.len();

    (0..W)
        .flat_map(|c| (0..H).map(move |r| (r, c)))
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
