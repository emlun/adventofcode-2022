use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;

type Point = (usize, usize);

struct Rock<'a> {
    points: &'a [Point],
    width: usize,
}

const W: usize = 7;

const ROCKS: [Rock; 5] = [
    Rock {
        width: 4,
        points: &[(0, 0), (1, 0), (2, 0), (3, 0)],
    },
    Rock {
        width: 3,
        points: &[(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
    },
    Rock {
        width: 3,
        points: &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    },
    Rock {
        width: 1,
        points: &[(0, 0), (0, 1), (0, 2), (0, 3)],
    },
    Rock {
        width: 2,
        points: &[(0, 0), (1, 0), (0, 1), (1, 1)],
    },
];

fn solve_a<I>(mut jet: I) -> usize
where
    I: Iterator<Item = bool>,
{
    let mut rocks = ROCKS.iter().cycle();
    let mut num_settled: usize = 0;
    let mut settled: HashSet<Point> = HashSet::with_capacity(2022 * 5);
    let mut h = 0;

    while num_settled < 2022 {
        let rock = rocks.next().unwrap();
        let mut x = 2;
        let mut y = h + 3;

        while y > h {
            if jet.next().unwrap() {
                if x + rock.width < W {
                    x += 1;
                }
            } else {
                if x > 0 {
                    x -= 1;
                }
            }

            y -= 1;
        }

        let mut points: Vec<Point> = rock
            .points
            .iter()
            .map(|(xx, yy)| (*xx + x, *yy + y))
            .collect();

        loop {
            let jet_dir = jet.next().unwrap();
            if jet_dir {
                if points.iter().all(|(xx, _)| xx + 1 < W) {
                    for (xx, _) in points.iter_mut() {
                        *xx += 1;
                    }
                }
            } else {
                if points.iter().all(|(xx, _)| *xx > 0) {
                    for (xx, _) in points.iter_mut() {
                        *xx -= 1;
                    }
                }
            }

            if points.iter().any(|p| settled.contains(p)) {
                if jet_dir {
                    for (xx, _) in points.iter_mut() {
                        *xx -= 1;
                    }
                } else {
                    for (xx, _) in points.iter_mut() {
                        *xx += 1;
                    }
                }
            }

            if points.iter().all(|(_, yy)| *yy > 0) {
                for (_, yy) in points.iter_mut() {
                    *yy -= 1;
                }
            } else {
                break;
            }

            if points.iter().any(|p| settled.contains(p)) {
                for (_, yy) in points.iter_mut() {
                    *yy += 1;
                }
                break;
            }
        }

        h = std::cmp::max(h, 1 + points.iter().map(|(_, yy)| *yy).max().unwrap());
        settled.extend(points.into_iter());
        num_settled += 1;
    }

    h
}

fn solve_b(jet: &[bool]) -> usize {
    let mut num_settled: usize = 0;
    let mut settled: HashSet<Point> = HashSet::with_capacity(2022 * 5);
    let mut jet_i = 0;
    let mut rock_i = 0;
    let mut h = 0;
    let mut hs: Vec<usize> = Vec::new();
    let mut jet_is: Vec<usize> = Vec::new();
    let mut rock_is: Vec<usize> = Vec::new();
    let mut states: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    hs.push(0);
    jet_is.push(0);
    rock_is.push(0);
    states.insert((0, 0), vec![(0, 0)]);

    while num_settled < 1000000000000 {
        if let Some(st) = states.get(&(jet_i, rock_i)) {
            if st.len() > 2 {
                let diffs: Vec<(usize, usize)> = st[1..]
                    .iter()
                    .zip(st[..st.len() - 1].iter())
                    .map(|((ra, ha), (rb, hb))| (ra - rb, ha - hb))
                    .collect();
                if diffs[1..].iter().all(|dh| *dh == diffs[0]) {
                    let (drock, dh) = diffs[0];
                    if (1000000000000 - num_settled) % drock == 0 {
                        let n = (1000000000000 - num_settled) / drock;
                        return h + n * dh;
                    }
                }
            }
        }

        let mut x = 2;
        let mut y = h + 3;

        while y > h {
            if jet[jet_i] {
                if x + ROCKS[rock_i].width < W {
                    x += 1;
                }
            } else {
                if x > 0 {
                    x -= 1;
                }
            }

            y -= 1;
            jet_i = (jet_i + 1) % jet.len();
        }

        let mut points: Vec<Point> = ROCKS[rock_i]
            .points
            .iter()
            .map(|(xx, yy)| (*xx + x, *yy + y))
            .collect();

        loop {
            if jet[jet_i] {
                if points.iter().all(|(xx, _)| xx + 1 < W) {
                    for (xx, _) in points.iter_mut() {
                        *xx += 1;
                    }
                }
            } else {
                if points.iter().all(|(xx, _)| *xx > 0) {
                    for (xx, _) in points.iter_mut() {
                        *xx -= 1;
                    }
                }
            }

            if points.iter().any(|p| settled.contains(p)) {
                if jet[jet_i] {
                    for (xx, _) in points.iter_mut() {
                        *xx -= 1;
                    }
                } else {
                    for (xx, _) in points.iter_mut() {
                        *xx += 1;
                    }
                }
            }

            jet_i = (jet_i + 1) % jet.len();

            if points.iter().all(|(_, yy)| *yy > 0) {
                for (_, yy) in points.iter_mut() {
                    *yy -= 1;
                }
            } else {
                break;
            }

            if points.iter().any(|p| settled.contains(p)) {
                for (_, yy) in points.iter_mut() {
                    *yy += 1;
                }
                break;
            }
        }

        let new_h = std::cmp::max(h, 1 + points.iter().map(|(_, yy)| *yy).max().unwrap());
        h = new_h;
        settled.extend(points.into_iter());
        num_settled += 1;
        rock_i = (rock_i + 1) % ROCKS.len();
        hs.push(h);
        jet_is.push(jet_i);
        rock_is.push(rock_i);
        states
            .entry((jet_i, rock_i))
            .or_insert(vec![])
            .push((num_settled, h));
    }

    h
}

pub fn solve(lines: &[String]) -> Solution {
    let jet: Vec<bool> = lines[0]
        .trim()
        .chars()
        .map(|c| match c {
            '>' => true,
            '<' => false,
            _ => unimplemented!(),
        })
        .collect();

    (
        solve_a(jet.iter().copied().cycle()).to_string(),
        solve_b(&jet).to_string(),
    )
}
