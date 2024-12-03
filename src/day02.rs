use history::readfile;

type Number = i32;

fn is_safe(mut n: impl Iterator<Item = Number>) -> bool {
    let mut a = n.next().expect("Each line should have at least one number");
    let mut b = n
        .next()
        .expect("Each line should have at least two numbers");
    if a > b {
        // Descending
        loop {
            let diff = a - b;
            if (1..=3).contains(&diff) {
                return false;
            }
            a = b;
            if let Some(num) = n.next() {
                b = num;
            } else {
                break;
            }
        }
    } else {
        // Ascending
        loop {
            let diff = b - a;
            if (1..=3).contains(&diff) {
                return false;
            }
            a = b;
            if let Some(num) = n.next() {
                b = num;
            } else {
                break;
            }
        }
    }
    true
}

fn basic(s: &str) -> bool {
    is_safe(
        s.split_ascii_whitespace()
            .map(|s| s.parse::<Number>().unwrap()),
    )
}

fn dampen(s: &str) -> bool {
    let n = s
        .split_ascii_whitespace()
        .map(|s| s.parse::<Number>().unwrap());
    let all: Vec<Number> = n.collect();
    if is_safe(all.iter().copied()) {
        return true;
    }
    for skip in 0..all.len() {
        let mut skipped = all.clone();
        skipped.remove(skip);
        if is_safe(skipped.into_iter()) {
            return true;
        }
    }
    false
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut count = 0;
    for line in ctxt.lines() {
        if basic(line) {
            count += 1;
        }
    }
    println!("{count} reports are safe");
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut count = 0;
    for line in ctxt.lines() {
        if dampen(line) {
            count += 1;
        }
    }
    println!("{count} reports are now safe");
}
