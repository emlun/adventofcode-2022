use crate::common::Solution;

fn mix(nums: &[isize], times: usize) -> Vec<isize> {
    let mut file: Vec<(usize, isize)> = nums.iter().copied().enumerate().collect();
    let n = isize::try_from(file.len() - 1).unwrap();
    for _ in 0..times {
        for orig_i in 0..nums.len() {
            let i = file
                .iter()
                .enumerate()
                .find(|(_, (oi, _))| *oi == orig_i)
                .map(|(i, _)| i)
                .unwrap();
            let (orig_i, v) = file.remove(i);
            let new_i: usize =
                usize::try_from((isize::try_from(i).unwrap() + v).rem_euclid(n)).unwrap();
            if new_i == 0 {
                file.push((orig_i, v));
            } else {
                file.insert(new_i, (orig_i, v))
            }
        }
    }

    file.into_iter().map(|(_, v)| v).collect()
}

fn solve_b(nums: &[isize], key: isize, times: usize) -> isize {
    mix(&nums.iter().map(|i| i * key).collect::<Vec<isize>>(), times)
        .into_iter()
        .cycle()
        .skip_while(|i| *i != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

pub fn solve(lines: &[String]) -> Solution {
    let nums: Vec<isize> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    (
        solve_b(&nums, 1, 1).to_string(),
        solve_b(&nums, 811589153, 10).to_string(),
    )
}
