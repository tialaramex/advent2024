use history::readfile;
use regex::Regex;

type Num = u64;

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let re = Regex::new(r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)").expect("This regex should compile");

    let mut total = 0;
    for line in ctxt.lines() {
        for capture in re.captures_iter(line) {
            let left = capture
                .get(1)
                .expect("Expression should match a left number")
                .as_str();
            let right = capture
                .get(2)
                .expect("Expression should match a right number")
                .as_str();
            let left: Num = left.parse().unwrap();
            let right: Num = right.parse().unwrap();
            total += left * right;
        }
    }
    println!("Adding up all the uncorrupted multiplications gives: {total}");
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let re = Regex::new(r"do\(\)|don\'t\(\)|mul\(([0-9]{1,3})\,([0-9]{1,3})\)")
        .expect("This regex should compile");

    let mut enabled = true;
    let mut total = 0;
    for line in ctxt.lines() {
        for capture in re.captures_iter(line) {
            let whole = capture.get(0).unwrap().as_str();
            if whole == "do()" {
                enabled = true;
                continue;
            }
            if whole == "don't()" {
                enabled = false;
                continue;
            }
            if enabled {
                let left = capture
                    .get(1)
                    .expect("Expression should match a left number")
                    .as_str();
                let right = capture
                    .get(2)
                    .expect("Expression should match a right number")
                    .as_str();
                let left: Num = left.parse().unwrap();
                let right: Num = right.parse().unwrap();
                total += left * right;
            }
        }
    }
    println!("Adding up only enabled multiplications gives: {total}");
}
