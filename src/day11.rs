use history::readfile;

type Num = u64;

fn maybe_split(num: Num) -> Option<(Num, Num)> {
    match num {
        0..=9 => None,
        10..=99 => Some((num / 10, num % 10)),
        100..=999 => None,
        1000..=9999 => Some((num / 100, num % 100)),
        10000..=99999 => None,
        100000..=999999 => Some((num / 1000, num % 1000)),
        1000000..=9999999 => None,
        10000000..=99999999 => Some((num / 10_000, num % 10_000)),
        100000000..=999999999 => None,
        1000000000..=9999999999 => Some((num / 100_000, num % 100_000)),
        10000000000..=99999999999 => None,
        100000000000..=999999999999 => Some((num / 1_000_000, num % 1_000_000)),
        1000000000000..=9999999999999 => None,
        10000000000000..=99999999999999 => Some((num / 10_000_000, num % 10_000_000)),
        _ => panic!("Number too large to maybe split"),
    }
}

fn blink(before: &[Num]) -> Vec<Num> {
    let mut after = Vec::with_capacity(before.len());
    for &num in before {
        if num == 0 {
            after.push(1);
            continue;
        }
        if let Some((first, second)) = maybe_split(num) {
            after.push(first);
            after.push(second);
        } else {
            after.push(num * 2024);
        }
    }

    after
}

use std::collections::HashMap;
type Nums = HashMap<Num, Num>;

fn faster(before: &Nums) -> Nums {
    let mut after = HashMap::with_capacity(before.len());
    for (&num, &count) in before {
        if num == 0 {
            *after.entry(1).or_default() += count;
            continue;
        }
        if let Some((first, second)) = maybe_split(num) {
            *after.entry(first).or_default() += count;
            *after.entry(second).or_default() += count;
        } else {
            *after.entry(num * 2024).or_default() += count;
        }
    }
    after
}

fn count(map: &Nums) -> Num {
    let mut total = 0;
    for &count in map.values() {
        total += count;
    }
    total
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut nums: Vec<Num> = ctxt
        .value()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    for _ in 1..=25 {
        nums = blink(&nums);
    }
    let stones = nums.len();
    println!("After 25 blinks I have {stones} stones");
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let nums: Vec<Num> = ctxt
        .value()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut map = HashMap::with_capacity(nums.len());
    for n in nums {
        *map.entry(n).or_default() += 1;
    }
    for _ in 1..=75 {
        map = faster(&map);
    }
    let stones = count(&map);
    println!("After 75 blinks I have {stones} stones");
}
