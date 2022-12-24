macro_rules! days {
    ($($day_mod:ident),*) => {
        $(pub mod $day_mod;)*

        pub fn get_solver(day: u8) -> Option<fn(&[String]) -> crate::common::Solution> {
            match format!("day{:02}", day).as_str() {
                $(stringify!($day_mod) => Some($day_mod::solve),)*
                _ => None,
            }
        }
    };
}

pub fn all_numbers() -> Vec<u8> {
    (1..=25).filter(|&day| get_solver(day).is_some()).collect()
}

days!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24
);
