use crate::common::Solution;

fn solve_a(line: &str) -> usize {
    let chars: Vec<char> = line.chars().collect();
    for i in 3..line.len() {
        if chars[i - 3] != chars[i - 2]
            && chars[i - 3] != chars[i - 1]
            && chars[i - 3] != chars[i]
            && chars[i - 2] != chars[i - 1]
            && chars[i - 2] != chars[i]
            && chars[i - 1] != chars[i]
        {
            return i + 1;
        }
    }
    unimplemented!()
}

pub fn solve(lines: &[String]) -> Solution {
    let line = &lines[0];
    let solution_a = solve_a(line);

    (solution_a.to_string(), "".to_string())
}
