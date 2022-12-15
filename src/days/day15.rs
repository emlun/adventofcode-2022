use std::collections::HashSet;

use crate::common::Solution;

type Point = (i32, i32);

fn map_exclusion(sensors: &[(Point, Point)], y: i32) -> Vec<std::ops::Range<i32>> {
    sensors
        .iter()
        .flat_map(|((sx, sy), (bx, by))| {
            let r = sx.abs_diff(*bx) + sy.abs_diff(*by);
            let dy = y.abs_diff(*sy);
            r.checked_sub(dy).map(|check_r| {
                (*sx - i32::try_from(check_r).unwrap())..(*sx + 1 + i32::try_from(check_r).unwrap())
            })
        })
        .fold(Vec::new(), |mut ranges, new_range| {
            for i in 0..ranges.len() {
                if new_range.end <= ranges[i].start {
                    ranges.insert(i, new_range);
                    return ranges;
                } else if (ranges[i].start <= new_range.start && ranges[i].end >= new_range.start)
                    || (new_range.start <= ranges[i].start && new_range.end >= ranges[i].start)
                {
                    ranges[i].start = std::cmp::min(ranges[i].start, new_range.start);
                    ranges[i].end = std::cmp::max(ranges[i].end, new_range.end);

                    while i + 1 < ranges.len() && ranges[i].end >= ranges[i + 1].start {
                        ranges[i].end = std::cmp::max(ranges[i].end, ranges.remove(i + 1).end);
                    }
                    return ranges;
                }
            }

            ranges.push(new_range);
            ranges
        })
}

fn solve_a(sensors: &[(Point, Point)], check_y: i32) -> i32 {
    map_exclusion(sensors, check_y)
        .into_iter()
        .map(|range| range.end - range.start)
        .sum::<i32>()
        - i32::try_from(
            sensors
                .iter()
                .map(|(_, (_, by))| *by)
                .filter(|by| *by == check_y)
                .collect::<HashSet<i32>>()
                .len(),
        )
        .unwrap()
}

fn solve_b(sensors: &[(Point, Point)], max_coord: i32) -> i64 {
    // Sensor range:
    // |x - sx| + |y - sy| <= r

    // Outside range:
    // x - sx + y - sy > r     if x >= sx, y >= sy
    // sx - x + y - sy > r     if x <  sx, y >= sy
    // x - sx + sy - y > r     if x >= sx, y <  sy
    // sx - x + sy - y > r     if x <  sx, y <  sy

    // Boundaries:
    //  x + y = r + sx + sy + 1
    // -x + y = r - sx + sy + 1
    //  x - y = r + sx - sy + 1
    // -x - y = r - sx - sy + 1

    // Pair each [x+y] eqn with each [x-y] eqn
    // from 2 different sensors
    // Gives 4 linear equation systems:
    // x + y =  r2 + sx2 + sy2 + 1 = b1
    // x - y = -r1 + sx1 - sy1 - 1 = b2

    // x + y = -r2 + sx2 + sy2 - 1 = b1
    // x - y = -r1 + sx1 - sy1 - 1 = b2

    // x + y =  r2 + sx2 + sy2 + 1 = b1
    // x - y =  r1 + sx1 - sy1 + 1 = b2

    // x + y = -r2 + sx2 + sy2 - 1 = b1
    // x - y =  r1 + sx1 - sy1 + 1 = b2

    // Each eqn. system gives 1 candidate point:
    // x + y = b1
    // x - y = b2
    // x = (b1 + b2) / 2
    // y = (b1 - b2) / 2

    for (i1, ((sx1, sy1), (bx1, by1))) in sensors.iter().enumerate() {
        let r1: i32 = i32::try_from(sx1.abs_diff(*bx1) + sy1.abs_diff(*by1)).unwrap();

        for ((sx2, sy2), (bx2, by2)) in sensors[i1 + 1..].iter() {
            let r2: i32 = i32::try_from(sx2.abs_diff(*bx2) + sy2.abs_diff(*by2)).unwrap();
            let b1s: [i32; 4] = [
                r2 + sx2 + sy2 + 1,
                -r2 + sx2 + sy2 - 1,
                r2 + sx2 + sy2 + 1,
                -r2 + sx2 + sy2 - 1,
            ];
            let b2s: [i32; 4] = [
                -r1 + sx1 - sy1 - 1,
                -r1 + sx1 - sy1 - 1,
                r1 + sx1 - sy1 + 1,
                r1 + sx1 - sy1 + 1,
            ];

            for (b1, b2) in b1s.iter().zip(b2s) {
                let x = (b1 + b2) / 2;
                let y = (b1 - b2) / 2;

                if (0..=max_coord).contains(&x)
                    && (0..=max_coord).contains(&y)
                    && sensors.iter().all(|((sx, sy), (bx, by))| {
                        let r = sx.abs_diff(*bx) + sy.abs_diff(*by);
                        let d = sx.abs_diff(x) + sy.abs_diff(y);
                        d > r
                    })
                {
                    return i64::from(x) * 4000000 + i64::from(y);
                }
            }
        }
    }

    // If solution is not on the boundary of two sensors,
    // it must be in a corner of the permitted region.
    for x in [0, max_coord] {
        for y in [0, max_coord] {
            if sensors.iter().all(|((sx, sy), (bx, by))| {
                let r = sx.abs_diff(*bx) + sy.abs_diff(*by);
                let d = sx.abs_diff(x) + sy.abs_diff(y);
                d > r
            }) {
                return i64::from(x) * 4000000 + i64::from(y);
            }
        }
    }

    unimplemented!()
}

pub fn solve(lines: &[String]) -> Solution {
    let sensors: Vec<((i32, i32), (i32, i32))> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut sbsplit = line.split(": closest beacon is at x=");
            let sensor = sbsplit.next().unwrap();
            let beacon = sbsplit.next().unwrap();

            let mut ssplit = sensor.split(", y=");
            let mut bsplit = beacon.split(", y=");

            (
                (
                    ssplit
                        .next()
                        .and_then(|s| s.strip_prefix("Sensor at x="))
                        .unwrap()
                        .parse()
                        .unwrap(),
                    ssplit.next().unwrap().parse().unwrap(),
                ),
                (
                    bsplit.next().unwrap().parse().unwrap(),
                    bsplit.next().unwrap().parse().unwrap(),
                ),
            )
        })
        .collect();

    (
        solve_a(&sensors, 2000000).to_string(),
        solve_b(&sensors, 4000000).to_string(),
    )
}
