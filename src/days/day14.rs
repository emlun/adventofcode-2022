use crate::common::Solution;
use crate::util::iter::WithSliding;

const SPAWN_X: usize = 500;
const SPAWN_Y: usize = 0;

fn solve_a(mut map: Vec<Vec<bool>>, abyss_y: usize, floor_y: usize) -> usize {
    let mut resting = 0;

    while !map
        .get(SPAWN_Y)
        .and_then(|row| row.get(SPAWN_X))
        .unwrap_or(&false)
    {
        // print_map(&orig_map, &map, 480, abyss_y);

        let (mut sandx, mut sandy) = (SPAWN_X, SPAWN_Y);

        loop {
            // dbg!((sandx, sandy));
            if sandy >= abyss_y {
                // dbg!("abyss");
                return resting;
            } else if sandy < floor_y - 1 {
                if !map
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
                    break;
                }
            } else {
                break;
            }
        }

        if map.len() <= sandy {
            map.resize(sandy * 2, Vec::new());
        }
        if map[sandy].len() <= sandx {
            map[sandy].resize(sandx * 2, false);
        }
        map[sandy][sandx] = true;
        resting += 1;
        // dbg!(resting);
    }
    // dbg!("block");
    resting
}

fn solve_b(map: Vec<Vec<bool>>, maxy: usize) -> usize {
    solve_a(map, maxy + 4, maxy + 2)
}

pub fn solve(lines: &[String]) -> Solution {
    let (map, maxy) = lines.iter().filter(|line| !line.is_empty()).fold(
        (Vec::new(), 0),
        |(mut map, mut maxy), line| {
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
                let lmaxy = std::cmp::max(starty, endy);
                let lmaxx = std::cmp::max(startx, endx);
                maxy = std::cmp::max(maxy, lmaxy);
                if map.len() <= lmaxy {
                    map.resize((lmaxy + 1) * 2, Vec::new());
                }
                for row in map
                    .iter_mut()
                    .take(lmaxy + 1)
                    .skip(std::cmp::min(starty, endy))
                {
                    if row.len() <= lmaxx {
                        row.resize((lmaxx + 1) * 2, false);
                    }
                    for cell in row
                        .iter_mut()
                        .take(lmaxx + 1)
                        .skip(std::cmp::min(startx, endx))
                    {
                        *cell = true;
                    }
                }
            }

            (map, maxy)
        },
    );

    (
        solve_a(map.clone(), maxy + 1, maxy + 10).to_string(),
        solve_b(map, maxy).to_string(),
    )
}
