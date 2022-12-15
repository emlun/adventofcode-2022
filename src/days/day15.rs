use std::collections::HashSet;

use crate::common::Solution;

const CHECK_Y: i32 = 2000000;
// const CHECK_Y: i32 = 10;

fn solve_a(sensors: &[((i32, i32), (i32, i32))]) -> usize {
    sensors
        .iter()
        .flat_map(|((sx, sy), (bx, by))| {
            let r = sx.abs_diff(*bx) + sy.abs_diff(*by);
            let dy = CHECK_Y.abs_diff(*sy);
            let check_r = r.checked_sub(dy).unwrap_or(0);
            let exclusion_range =
                (*sx - i32::try_from(check_r).unwrap())..(*sx + i32::try_from(check_r).unwrap());
            exclusion_range
        })
        .collect::<HashSet<i32>>()
        .len()
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
    (solve_a(&sensors).to_string(), "".to_string())
}
