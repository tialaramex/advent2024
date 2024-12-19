use history::readfile;

fn brands(line: &str) -> Vec<&str> {
    line.split(", ").collect()
}

use std::collections::hash_set::HashSet;

fn attempt(pat: &str, from: &[&str]) -> bool {
    let mut futile: HashSet<usize> = HashSet::new();
    let mut stack: Vec<usize> = vec![0];

    while let Some(prefix) = stack.pop() {
        let mut next: HashSet<usize> = HashSet::new();
        let (_, right) = pat.split_at(prefix);
        for towel in from {
            if right.starts_with(towel) {
                if right.len() == towel.len() {
                    return true;
                }
                next.insert(prefix + towel.len());
            }
        }
        let mut new: Vec<usize> = next.difference(&futile).copied().collect();
        if new.is_empty() {
            futile.insert(prefix);
        } else {
            new.sort_unstable();
            stack.append(&mut new);
        }
    }
    false
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut lines = ctxt.lines();

    let towels = lines.next().expect("Should begin with a list of towels");
    let towels = brands(towels);
    lines.next().expect("Should then have a blank link");
    let possible = lines.filter(|line| attempt(line, &towels)).count();
    println!("{possible} designs are possible");
}

fn count_attempts(pat: &str, from: &[&str]) -> usize {
    let mut counts: Vec<usize> = Vec::with_capacity(pat.len() + 1);
    counts.push(1);
    counts.resize(pat.len() + 1, 0);

    for prefix in 0..pat.len() {
        let count = counts[prefix];
        let (_, right) = pat.split_at(prefix);
        for towel in from {
            if right.starts_with(towel) {
                counts[prefix + towel.len()] += count;
            }
        }
    }
    counts[pat.len()]
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut lines = ctxt.lines();

    let towels = lines.next().expect("Should begin with a list of towels");
    let towels = brands(towels);
    lines.next().expect("Should then have a blank link");
    let sum: usize = lines.map(|line| count_attempts(line, &towels)).sum();
    println!("The designs could be made in total {sum} ways");
}
