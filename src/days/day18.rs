use std::collections::HashSet;

use crate::common::Solution;

type Point = (i32, i32, i32);

const DXYZ: [Point; 6] = [
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
            6 - DXYZ
                .iter()
                .map(|(dx, dy, dz)| (x + dx, y + dy, z + dz))
                .filter(|p| points.contains(p))
                .count()
        })
        .sum()
}

fn solve_b(points: &HashSet<Point>) -> usize {
    let minx = points.iter().map(|(x, _, _)| x).min().unwrap() - 1;
    let miny = points.iter().map(|(_, y, _)| y).min().unwrap() - 1;
    let minz = points.iter().map(|(_, _, z)| z).min().unwrap() - 1;

    let maxx = points.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let maxy = points.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let maxz = points.iter().map(|(_, _, z)| z).max().unwrap() + 1;

    let lx = maxx - minx + 1;
    let ly = maxy - miny + 1;
    let lz = maxz - minz + 1;
    let outer_surface = 2 * usize::try_from(lx * ly + lx * lz + ly * lz).unwrap();

    let cap = usize::try_from(lx * ly * lz).unwrap();
    let mut frontier: Vec<Point> = Vec::with_capacity(cap);
    let mut outer_points: HashSet<Point> = HashSet::with_capacity(cap);

    frontier.push((minx, miny, minz));
    while let Some((x, y, z)) = frontier.pop() {
        for (dx, dy, dz) in DXYZ {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            let next = (nx, ny, nz);
            if nx >= minx
                && nx <= maxx
                && ny >= miny
                && ny <= maxy
                && nz >= minz
                && nz <= maxz
                && !points.contains(&next)
                && !outer_points.contains(&next)
            {
                frontier.push(next);
                outer_points.insert(next);
            }
        }
    }

    outer_points
        .iter()
        .map(|(x, y, z)| {
            6 - DXYZ
                .iter()
                .map(|(dx, dy, dz)| (x + dx, y + dy, z + dz))
                .filter(|p| outer_points.contains(p))
                .count()
        })
        .sum::<usize>()
        - outer_surface
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

    (solve_a(&droplet).to_string(), solve_b(&droplet).to_string())
}
