use crate::common::Solution;

fn solve_b(line: &str, n: usize) -> usize {
    let chars: &[u8] = line.as_bytes();
    let mut counts: [u8; 123] = [0; 123]; // ASCII 'z' = 122
    let mut num_nonzero = 0;

    for i in 0..n {
        if counts[usize::from(chars[i])] == 0 {
            num_nonzero += 1;
        }
        counts[usize::from(chars[i])] += 1;
    }

    for i in n..line.len() {
        if num_nonzero >= n {
            return i;
        }

        counts[usize::from(chars[i - n])] -= 1;
        if counts[usize::from(chars[i - n])] == 0 {
            num_nonzero -= 1;
        }

        if counts[usize::from(chars[i])] == 0 {
            num_nonzero += 1;
        }
        counts[usize::from(chars[i])] += 1;
    }
    unimplemented!()
}

pub fn solve(lines: &[String]) -> Solution {
    let line = &lines[0];
    (solve_b(line, 4).to_string(), solve_b(line, 14).to_string())
}
