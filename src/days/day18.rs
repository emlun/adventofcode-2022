use std::collections::HashSet;

use crate::common::Solution;

type Point = (i32, i32, i32);

const dxyz: [Point; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn solve_a(points: &HashSet<Point>) -> usize {
    points
        .iter()
        .map(|(x, y, z)| {
            6 - dxyz
                .iter()
                .map(|(dx, dy, dz)| (x + dx, y + dy, z + dz))
                .filter(|p| points.contains(p))
                .count()
        })
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let droplet: HashSet<Point> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    (solve_a(&droplet).to_string(), "".to_string())
}
