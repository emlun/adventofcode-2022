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

pub fn solve(lines: &[String]) -> Solution {
    let map: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    (solve_a(&map).to_string(), "".to_string())
}
