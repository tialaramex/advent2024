mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

use history::days;

fn main() {
    let mut args = std::env::args();
    args.next();

    if let Some(day) = args.next() {
        let fun = format!("day{:0>3}", day.to_ascii_lowercase());
        let day = day.trim_end_matches(char::is_alphabetic);
        let file = if let Some(extra) = args.next() {
            format!("test-data/{:0>2}{extra}", day)
        } else {
            format!("{:0>2}", day)
        };

        days!(
            fun.as_str(),
            file.as_str(),
            day01,
            day02,
            day03,
            day04,
            day05,
            day06,
            day07,
            day08,
            day09,
            day10,
            day11,
            day12,
            day13,
            day14,
            day15,
            day16,
            day17,
            day18,
            day19,
        );
    } else {
        eprintln!("Provide a parameter specifying which day e.g. 1a means day 1, part A while 4b means day 4, part B.");
        eprintln!("You may also optionally specify a filename suffix e.g. 5b test will use the file 05test");
    }
}
