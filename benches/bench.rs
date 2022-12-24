use adventofcode_2022::common::day_input_filename;
use adventofcode_2022::common::get_file_lines;
use adventofcode_2022::common::Solution;
use adventofcode_2022::days;

macro_rules! setup_benchmark {
    ($($day_name:ident),*) => {
        $(
            pub fn $day_name(c: &mut criterion::Criterion) {
                let day_name = stringify!($day_name);
                let day_num: u8 = day_name[3..].parse().unwrap();
                c.bench_function(&format!("Day {}", day_num), |bencher| {
                    let input_lines = get_file_lines(&day_input_filename(day_num)).unwrap();
                    bencher.iter(|| days::$day_name::solve(&input_lines));
                });
            }
        )*

        pub fn days_all(c: &mut criterion::Criterion) {
            let solvers_and_inputs: Vec<(fn(&[String]) -> Solution, Vec<String>)> = days::all_numbers()
                .into_iter()
                .map(|day| {
                    (
                        days::get_solver(day).unwrap(),
                        get_file_lines(&day_input_filename(day)).unwrap(),
                    )
                })
                .collect();

            c.bench_function("All days", |bencher| {
                bencher.iter(|| {
                    solvers_and_inputs
                        .iter()
                        .map(|(solver, input)| solver(&input))
                        .collect::<Vec<Solution>>()
                })
            });
        }

        criterion::criterion_group! {
            name = benches;
            config = criterion::Criterion::default()
                .significance_level(0.01)
                .noise_threshold(0.05)
                .warm_up_time(::std::time::Duration::from_millis(100))
                .measurement_time(::std::time::Duration::from_millis(400));
            targets = $($day_name,)* days_all
        }
        criterion::criterion_main!(benches);
    };
}

setup_benchmark!(
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day20, day21, day22, day23, day24
);
