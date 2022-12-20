use crate::common::Solution;

fn mix(nums: &[isize], times: usize) -> Vec<isize> {
    let mut file: Vec<(usize, isize)> = nums.iter().copied().enumerate().collect();
    for _ in 0..times {
        for orig_i in 0..nums.len() {
            let i = file
                .iter()
                .enumerate()
                .find(|(_, (oi, _))| *oi == orig_i)
                .map(|(i, _)| i)
                .unwrap();
            let (orig_i, v) = file.remove(i);
            let new_i: isize = isize::try_from(i).unwrap() + v;
            if new_i == isize::try_from(file.len()).unwrap() {
                file.push((orig_i, v));
            } else {
                file.insert(
                    usize::try_from(new_i.rem_euclid(isize::try_from(file.len()).unwrap()))
                        .unwrap(),
                    (orig_i, v),
                )
            }
        }
    }

    file.into_iter().map(|(_, v)| v).collect()
}

fn solve_a(nums: &[isize]) -> isize {
    let mixed = mix(nums, 1);

    let mut it = mixed
        .into_iter()
        .cycle()
        .skip_while(|i| *i != 0)
        .step_by(1000)
        .skip(1);
    let first = it.next().unwrap();
    let second = it.next().unwrap();
    let third = it.next().unwrap();
    first + second + third
}

fn solve_b(nums: &[isize]) -> isize {
    const DECRYPTION_KEY: isize = 811589153;
    let mixed = mix(
        &nums
            .iter()
            .map(|i| i * DECRYPTION_KEY)
            .collect::<Vec<isize>>(),
        10,
    );

    let mut it = mixed
        .into_iter()
        .cycle()
        .skip_while(|i| *i != 0)
        .step_by(1000)
        .skip(1);
    let first = it.next().unwrap();
    let second = it.next().unwrap();
    let third = it.next().unwrap();
    first + second + third
}
pub fn solve(lines: &[String]) -> Solution {
    let nums: Vec<isize> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    (solve_a(&nums).to_string(), solve_b(&nums).to_string())
}
