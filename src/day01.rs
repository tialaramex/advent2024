use history::readfile;

type Number = u32;

fn diff(nums: (Number, Number)) -> Number {
    if nums.0 > nums.1 {
        nums.0 - nums.1
    } else {
        nums.1 - nums.0
    }
}

pub fn a(filename: &str) {
    let mut left = Vec::new();
    let mut right = Vec::new();
    let ctxt = readfile(filename);
    for line in ctxt.lines() {
        let mut numbers = line.split_ascii_whitespace();
        let l = numbers.next().expect("There should be a left number on each line");
        let r = numbers.next().expect("There should also be a right number on each line");
        assert_eq!(numbers.next(), None);
        let l: Number = l.parse().expect("LHS should be a number");
        let r: Number = r.parse().expect("RHS should be a number");
        left.push(l);
        right.push(r);
    }
    left.sort_unstable();
    right.sort_unstable();

    let pairs = left.into_iter().zip(right.into_iter());
    let total: Number = pairs.map(diff).sum();
    println!("Total distance between lists is {total}");
}

use std::collections::HashMap;

pub fn b(filename: &str) {
    let mut left = Vec::new();
    let mut right: HashMap<Number, usize> = HashMap::new();
    let ctxt = readfile(filename);
    for line in ctxt.lines() {
        let mut numbers = line.split_ascii_whitespace();
        let l = numbers.next().expect("There should be a left number on each line");
        let r = numbers.next().expect("There should also be a right number on each line");
        assert_eq!(numbers.next(), None);
        let l: Number = l.parse().expect("LHS should be a number");
        let r: Number = r.parse().expect("RHS should be a number");
        left.push(l);
        *right.entry(r).or_default() += 1;
    }

    let mut score = 0;
    for n in left {
        if let Some(count) = right.get(&n) {
            score += (n as usize) * count;
        }
    }

    println!("Similarity score is {score}");
}
