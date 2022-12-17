use std::cmp::Reverse;
use std::collections::HashSet;

use crate::common::Solution;

type Point = (usize, usize);

struct Rock<'a> {
    points: &'a [Point],
    height: usize,
    width: usize,
}

const W: usize = 7;

const ROCKS: [Rock; 5] = [
    Rock {
        height: 1,
        width: 4,
        points: &[(0, 0), (1, 0), (2, 0), (3, 0)],
    },
    Rock {
        height: 3,
        width: 3,
        points: &[(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
    },
    Rock {
        height: 3,
        width: 3,
        points: &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    },
    Rock {
        height: 4,
        width: 1,
        points: &[(0, 0), (0, 1), (0, 2), (0, 3)],
    },
    Rock {
        height: 2,
        width: 2,
        points: &[(0, 0), (1, 0), (0, 1), (1, 1)],
    },
];

fn print_state(
    h: usize,
    settled: &HashSet<Point>,
    xyr: Option<(Point, &Rock)>,
    points: Option<&Vec<Point>>,
) {
    let rock_points: HashSet<Point> = xyr
        .map(|((x, y), rock)| {
            rock.points
                .iter()
                .map(|(xx, yy)| (*xx + x, *yy + y))
                .collect()
        })
        .or(points.map(|v| v.iter().copied().collect()))
        .unwrap();
    for print_y in
        (0..std::cmp::max(rock_points.iter().map(|(_, y)| *y).max().unwrap_or(0), h) + 2).rev()
    {
        print!("|");
        for x in 0..7 {
            if rock_points.contains(&(x, print_y)) {
                print!("@");
            } else if settled.contains(&(x, print_y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
}

fn solve_a<I>(mut jet: I) -> usize
where
    I: Iterator<Item = bool>,
{
    let mut rocks = ROCKS.iter().cycle();
    let mut num_settled: usize = 0;
    let mut settled: HashSet<Point> = HashSet::with_capacity(2022 * 5);
    let mut h = 0;

    while num_settled < 2022 {
        // println!("\n{} {}", num_settled, h);
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

            // println!("y={}", y);
            // print_state(h, &settled, Some(((x, y), rock)), None);

            y -= 1;
        }

        let mut points: Vec<Point> = rock
            .points
            .iter()
            .map(|(xx, yy)| (*xx + x, *yy + y))
            .collect();

        loop {
            // println!("y={}", y);

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

            // print_state(h, &settled, None, Some(&points));

            if points.iter().any(|p| settled.contains(p)) {
                // println!("Undo dx");
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

            // print_state(h, &settled, None, Some(&points));

            if points.iter().all(|(_, yy)| *yy > 0) {
                for (_, yy) in points.iter_mut() {
                    *yy -= 1;
                }
            } else {
                break;
            }

            // print_state(h, &settled, None, Some(&points));

            if points.iter().any(|p| settled.contains(p)) {
                // println!("Undo dy");
                for (_, yy) in points.iter_mut() {
                    *yy += 1;
                }
                break;
            }
        }

        // println!("y={}", y);
        // print_state(h, &settled, None, Some(&points));

        h = std::cmp::max(h, 1 + points.iter().map(|(_, yy)| *yy).max().unwrap());
        settled.extend(points.into_iter());
        num_settled += 1;
    }

    h
}

pub fn solve(lines: &[String]) -> Solution {
    let jet = lines[0]
        .trim()
        .chars()
        .map(|c| match c {
            '>' => true,
            '<' => false,
            _ => unimplemented!(),
        })
        .cycle();

    (solve_a(jet.clone()).to_string(), "".to_string())
}
