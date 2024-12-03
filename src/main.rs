mod day01;
mod day02;
mod day03;

use history::days;

fn main() {
    let mut args = std::env::args();
    args.next();

    if let Some(day) = args.next() {
        let fun = format!("day{:0>3}", day.to_ascii_lowercase());
        let day = day.trim_end_matches(char::is_alphabetic);
        let file = if let Some(extra) = args.next() {
            format!("{:0>2}{extra}", day)
        } else {
            format!("{:0>2}", day)
        };

        days!(fun.as_str(), file.as_str(), day01, day02, day03);
    } else {
        eprintln!("Provide a parameter specifying which day e.g. 1a means day 1, part A while 4b means day 4, part B.");
        eprintln!("You may also optionally specify a filename suffix e.g. 5b test will use the file 05test");
    }
}
