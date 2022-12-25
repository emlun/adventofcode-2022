use crate::common::Solution;

fn from_snafu(s: &str) -> i64 {
    s.chars().fold(0, |d, c| match c {
        '2' => d * 5 + 2,
        '1' => d * 5 + 1,
        '0' => d * 5 + 0,
        '-' => d * 5 - 1,
        '=' => d * 5 - 2,
        _ => unimplemented!(),
    })
}

fn to_snafu(mut d: i64) -> String {
    let mut snafu = vec![0];
    let mut i = 0;
    while d > 0 {
        snafu.resize(i + 2, 0);
        match d % 5 {
            4 => {
                snafu[i] -= 1;
                snafu[i + 1] += 1;
            }
            3 => {
                snafu[i] -= 2;
                snafu[i + 1] += 1;
            }
            2 => snafu[i] += 2,
            1 => snafu[i] += 1,
            0 => snafu[i] += 0,
            _ => unimplemented!(),
        }
        if snafu[i] > 2 {
            snafu[i] -= 5;
            snafu[i + 1] += 1;
        }
        d /= 5;
        i += 1;
    }
    snafu
        .into_iter()
        .rev()
        .skip_while(|i| *i == 0)
        .map(|d| match d {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => unimplemented!("{}", d),
        })
        .collect()
}

pub fn solve(lines: &[String]) -> Solution {
    (
        to_snafu(lines.iter().map(|s| from_snafu(s)).sum()).to_string(),
        "".to_string(),
    )
}
