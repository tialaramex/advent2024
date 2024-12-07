use history::readfile;

type Num = u64;

fn handle(line: &str) -> (Num, Num, Vec<Num>) {
    let (goal, rest) = line
        .split_once(": ")
        .expect("lines should have : separator");
    let goal: Num = goal.parse().expect("goal should be a number");
    let (first, rest) = rest
        .split_once(' ')
        .expect("lines should then have at least two numbers");
    let first: Num = first.parse().expect("should all be numbers");
    let nums: Vec<Num> = rest.split(' ').map(|s| s.parse().unwrap()).collect();
    (goal, first, nums)
}

fn check(line: &str) -> Num {
    let (goal, first, mut nums) = handle(line);
    // Use nums as a stack
    nums.reverse();
    let mut sums: Vec<Num> = Vec::new();
    sums.push(first);
    while let Some(b) = nums.pop() {
        let mut next: Vec<Num> = Vec::with_capacity(sums.len() * 3);
        while let Some(a) = sums.pop() {
            // We only grow, so if we're too big we're done
            if a > goal {
                continue;
            }
            next.push(a + b);
            next.push(a * b);
        }
        sums = next;
    }
    if sums.contains(&goal) {
        goal
    } else {
        0
    }
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut total = 0;
    for line in ctxt.lines() {
        total += check(line);
    }
    println!("Calibration result is {total}");
}

fn three(line: &str) -> Num {
    let (goal, first, mut nums) = handle(line);
    // Use nums as a stack
    nums.reverse();
    let mut sums: Vec<Num> = Vec::new();
    sums.push(first);
    while let Some(b) = nums.pop() {
        let mut next: Vec<Num> = Vec::with_capacity(sums.len() * 3);
        while let Some(a) = sums.pop() {
            // We only grow, so if we're too big we're done
            if a > goal {
                continue;
            }
            next.push(a + b);
            next.push(a * b);
            let s = format!("{a}{b}");
            let n: Num = s
                .parse()
                .expect("concatenating two numbers results in a number");
            next.push(n);
        }
        sums = next;
    }
    if sums.contains(&goal) {
        goal
    } else {
        0
    }
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut total = 0;
    for line in ctxt.lines() {
        total += three(line);
    }
    println!("Revised calibration result is {total}");
}
