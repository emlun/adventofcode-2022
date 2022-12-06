use crate::common::Solution;

fn solve_b(chars: &[usize], n: usize) -> usize {
    let mut counts: [u8; 123] = [0; 123]; // ASCII 'z' = 122
    let mut num_nonzero = 0;

    for i in 0..n {
        if counts[chars[i]] == 0 {
            num_nonzero += 1;
        }
        counts[chars[i]] += 1;
    }

    for i in n..chars.len() {
        if num_nonzero >= n {
            return i;
        }

        let cin = chars[i - n];
        counts[cin] -= 1;
        if counts[cin] == 0 {
            num_nonzero -= 1;
        }

        if counts[chars[i]] == 0 {
            num_nonzero += 1;
        }
        counts[chars[i]] += 1;
    }
    unimplemented!()
}

pub fn solve(lines: &[String]) -> Solution {
    let line = &lines[0];
    let chars: Vec<usize> = line
        .as_bytes()
        .into_iter()
        .map(|c| usize::from(*c))
        .collect();
    (
        solve_b(&chars, 4).to_string(),
        solve_b(&chars, 14).to_string(),
    )
}
