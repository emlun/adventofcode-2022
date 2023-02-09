mod crate_info;

use clap::App;
use clap::Arg;
use std::path::Path;

use crate::crate_info::crate_author;
use crate::crate_info::crate_description;
use crate::crate_info::crate_name;
use crate::crate_info::crate_version;
use adventofcode_2022::common::day_input_filename;
use adventofcode_2022::common::get_file_lines;
use adventofcode_2022::days;

fn main() -> Result<(), std::io::Error> {
    let cli = App::new(crate_name())
        .version(crate_version())
        .about(crate_description())
        .author(crate_author())
        .arg(
            Arg::with_name("day")
                .takes_value(true)
                .help(r#"Day number (1 - 25) to run. If omitted, all days are run."#)
        )
        .arg(
            Arg::with_name("input-file")
                .takes_value(true)
                .help(r#"Path to a file containing input for the chosen day. Use "-" for standard input; omit to use "./inputs/day<day>.in"."#)
        );

    let matches = cli.get_matches();

    if let Some(day) = matches.value_of("day") {
        run_day(
            day.parse::<u8>()
                .unwrap_or_else(|_| panic!("Invalid day number: {day}")),
            matches.value_of("input-file").map(Path::new),
        )
    } else {
        run_all_days()
    }
}

fn run_day(day: u8, input_path: Option<&Path>) -> Result<(), std::io::Error> {
    println!();
    println!("=== Day {day: >2} ===");

    let day_func = days::get_solver(day).unwrap_or_else(|| panic!("Unknown day: {day}"));
    let lines = input_path
        .map(get_file_lines)
        .unwrap_or_else(|| get_file_lines(&day_input_filename(day)))?;
    let solution = day_func(&lines);

    println!("A: {}", solution.0);
    println!("B: {}", solution.1);

    Ok(())
}

fn run_all_days() -> Result<(), std::io::Error> {
    for day in days::all_numbers() {
        run_day(day, None)?
    }
    Ok(())
}
