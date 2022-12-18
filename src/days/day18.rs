use std::collections::HashSet;

use crate::common::Solution;

type Point = (i16, i16, i16);

const DXYZ: [Point; 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn solve_a(points: &HashSet<u64>, minx: i16, miny: i16, minz: i16) -> usize {
    points
        .iter()
        .map(|h| unkey(*h, minx, miny, minz))
        .map(|(x, y, z)| {
            6 - DXYZ
                .iter()
                .map(|(dx, dy, dz)| (x + dx, y + dy, z + dz))
                .filter(|p| points.contains(&tokey(*p, minx, miny, minz)))
                .count()
        })
        .sum()
}

fn tokey((x, y, z): Point, minx: i16, miny: i16, minz: i16) -> u64 {
    (((x - minx) as u64) << 32) | (((y - miny) as u64) << 16) | ((z - minz) as u64)
}

fn unkey(h: u64, minx: i16, miny: i16, minz: i16) -> Point {
    let x = ((h >> 32) & 0xffff) as i16 + minx;
    let y = ((h >> 16) & 0xffff) as i16 + miny;
    let z = (h & 0xffff) as i16 + minz;
    (x, y, z)
}

fn solve_b(
    points: &HashSet<u64>,
    minx: i16,
    maxx: i16,
    miny: i16,
    maxy: i16,
    minz: i16,
    maxz: i16,
) -> usize {
    let lx = maxx - minx + 1;
    let ly = maxy - miny + 1;
    let lz = maxz - minz + 1;
    let outer_surface = 2 * usize::try_from(lx * ly + lx * lz + ly * lz).unwrap();

    let cap = usize::try_from(lx * ly * lz).unwrap();
    let mut frontier: Vec<Point> = Vec::with_capacity(cap);
    let mut outer_points: HashSet<u64> = HashSet::with_capacity(cap);

    frontier.push((minx, miny, minz));
    while let Some((x, y, z)) = frontier.pop() {
        for (dx, dy, dz) in DXYZ {
            let (nx, ny, nz) = (x + dx, y + dy, z + dz);
            let next = (nx, ny, nz);
            if nx >= minx && nx <= maxx && ny >= miny && ny <= maxy && nz >= minz && nz <= maxz {
                let h = tokey(next, minx, miny, minz);
                if !points.contains(&h) && !outer_points.contains(&h) {
                    frontier.push(next);
                    outer_points.insert(h);
                }
            }
        }
    }

    outer_points
        .iter()
        .map(|h| {
            let (x, y, z) = unkey(*h, minx, miny, minz);
            6 - DXYZ
                .iter()
                .map(|(dx, dy, dz)| (x + dx, y + dy, z + dz))
                .filter(|p| outer_points.contains(&tokey(*p, minx, miny, minz)))
                .count()
        })
        .sum::<usize>()
        - outer_surface
}

pub fn solve(lines: &[String]) -> Solution {
    let points: Vec<Point> = lines
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

    let minx = points.iter().map(|(x, _, _)| x).min().unwrap() - 1;
    let miny = points.iter().map(|(_, y, _)| y).min().unwrap() - 1;
    let minz = points.iter().map(|(_, _, z)| z).min().unwrap() - 1;

    let maxx = points.iter().map(|(x, _, _)| x).max().unwrap() + 1;
    let maxy = points.iter().map(|(_, y, _)| y).max().unwrap() + 1;
    let maxz = points.iter().map(|(_, _, z)| z).max().unwrap() + 1;

    let droplet: HashSet<u64> = points
        .into_iter()
        .map(|p| tokey(p, minx, miny, minz))
        .collect();

    (
        solve_a(&droplet, minx, miny, minz).to_string(),
        solve_b(&droplet, minx, maxx, miny, maxy, minz, maxz).to_string(),
    )
}
