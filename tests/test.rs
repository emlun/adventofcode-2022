use adventofcode_2022::common::day_input_filename;
use adventofcode_2022::common::get_file_lines;
use adventofcode_2022::days;

fn test_day(day: u8, correct_a: &str, correct_b: &str) -> Result<(), std::io::Error> {
    let solve = days::get_solver(day).unwrap();
    let input_lines = get_file_lines(&day_input_filename(day))?;
    let (solution_a, solution_b) = solve(&input_lines);
    assert_eq!(
        solution_a.as_str(),
        correct_a,
        "Incorrect solution for day {}a",
        day
    );
    assert_eq!(
        solution_b.as_str(),
        correct_b,
        "Incorrect solution for day {}b",
        day
    );

    Ok(())
}

macro_rules! test_day {
    ($name: ident, $sol_a: literal, $sol_b: literal) => {
        #[test]
        fn $name() -> Result<(), std::io::Error> {
            let day_name = stringify!($name);
            let day_num: u8 = day_name[3..].parse().unwrap();
            test_day(day_num, $sol_a, $sol_b)
        }
    };
}

test_day!(day01, "70296", "205381");
test_day!(day02, "11841", "13022");
test_day!(day03, "7785", "2633");
test_day!(day04, "507", "897");
test_day!(day05, "JRVNHHCSJ", "GNFBSBJLH");
test_day!(day06, "1804", "2508");
test_day!(day07, "1886043", "3842121");
test_day!(day08, "1827", "335580");
test_day!(day09, "5902", "2445");
test_day!(
    day10,
    "11220",
    "
###..####.###...##....##.####.#....#..#.
#..#....#.#..#.#..#....#.#....#....#.#..
###....#..#..#.#..#....#.###..#....##...
#..#..#...###..####....#.#....#....#.#..
#..#.#....#....#..#.#..#.#....#....#.#..
###..####.#....#..#..##..####.####.#..#."
);
test_day!(day11, "182293", "54832778815");
test_day!(day12, "394", "388");
test_day!(day13, "5625", "23111");
test_day!(day14, "618", "26358");
test_day!(day15, "5716881", "10852583132904");
test_day!(day16, "1896", "2576");
