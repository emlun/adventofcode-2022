use std::collections::HashSet;

use crate::common::Solution;

fn map_exclusion(sensors: &[((i32, i32), (i32, i32))], y: i32) -> Vec<std::ops::Range<i32>> {
    sensors
        .iter()
        .flat_map(|((sx, sy), (bx, by))| {
            let r = sx.abs_diff(*bx) + sy.abs_diff(*by);
            let dy = y.abs_diff(*sy);
            r.checked_sub(dy).map(|check_r| {
                let exclusion_range = (*sx - i32::try_from(check_r).unwrap())
                    ..(*sx + 1 + i32::try_from(check_r).unwrap());
                exclusion_range
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

fn solve_a(sensors: &[((i32, i32), (i32, i32))], check_y: i32) -> i32 {
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

fn solve_b(sensors: &[((i32, i32), (i32, i32))], max_coord: i32) -> i64 {
    let range_max = max_coord + 1;

    for y in 0..=max_coord {
        let exclusion = map_exclusion(sensors, y);
        let excluded: i32 = exclusion
            .iter()
            .map(|range| {
                std::cmp::max(
                    0,
                    std::cmp::min(range_max, range.end) - std::cmp::max(0, range.start),
                )
            })
            .sum();
        if excluded == max_coord {
            let x = if exclusion[0].start == 1 {
                0
            } else if exclusion.len() == 1 {
                max_coord
            } else {
                exclusion[0].end
            };
            return i64::from(x) * 4000000 + i64::from(y);
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
