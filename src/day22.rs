use history::readfile;

type Num = u64;

fn mix(secret: Num, other: Num) -> Num {
    secret ^ other
}

fn prune(secret: Num) -> Num {
    secret % 16777216
}

fn evolve(mut secret: Num) -> Num {
    secret = mix(secret, secret * 64);
    secret = prune(secret);
    secret = mix(secret, secret / 32);
    secret = prune(secret);
    secret = mix(secret, secret * 2048);
    secret = prune(secret);
    secret
}

fn future(mut secret: Num, generations: Num) -> Num {
    for _ in 0..generations {
        secret = evolve(secret);
    }
    secret
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut sum = 0;
    for line in ctxt.lines() {
        let secret: Num = line.parse().expect("Should be a number");
        let becomes = future(secret, 2000);
        sum += becomes;
    }
    println!("The sum of the 2000th secret numbers generated by each buyer was: {sum}");
}

fn price(secret: Num) -> i8 {
    (secret % 10) as i8
}

type Pattern = (i8, i8, i8, i8);
type Summary = HashMap<Pattern, Num>;

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};

fn summarise_into(mut secret: Num, summary: &mut Summary) {
    let mut done: HashSet<Pattern> = HashSet::new();
    let mut p = price(secret);
    secret = evolve(secret);
    let mut next = price(secret);
    let mut first = next - p;
    secret = evolve(secret);
    p = next;
    next = price(secret);
    let mut second = next - p;
    secret = evolve(secret);
    p = next;
    next = price(secret);
    let mut third = next - p;
    secret = evolve(secret);
    p = next;
    next = price(secret);
    let mut fourth = next - p;

    for _ in 0..1996 {
        if done.insert((first, second, third, fourth)) {
            match summary.entry((first, second, third, fourth)) {
                Vacant(vacant) => {
                    vacant.insert(next as Num);
                }
                Occupied(mut occupied) => {
                    *occupied.get_mut() += next as Num;
                }
            }
        }
        first = second;
        second = third;
        third = fourth;
        secret = evolve(secret);
        p = next;
        next = price(secret);
        fourth = next - p;
    }
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut summary = HashMap::with_capacity(2000);
    for line in ctxt.lines() {
        let secret: Num = line.parse().expect("Should be a number");
        summarise_into(secret, &mut summary);
    }
    let mut most: Option<Num> = None;
    for first in -9..=9 {
        for second in -9..=9 {
            for third in -9..=9 {
                for fourth in -9..=9 {
                    if let Some(&bananas) = summary.get(&(first, second, third, fourth)) {
                        if let Some(old) = most {
                            if old < bananas {
                                most = Some(bananas);
                            }
                        } else {
                            most = Some(bananas);
                        }
                    }
                }
            }
        }
    }
    if let Some(bananas) = most {
        println!("We can get at most {bananas} bananas");
    } else {
        println!("No bananas available");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixing() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn one_two_three() {
        let mut secret = 123;
        secret = evolve(secret);
        assert_eq!(secret, 15887950);
        secret = evolve(secret);
        assert_eq!(secret, 16495136);
        secret = evolve(secret);
        assert_eq!(secret, 527345);
        secret = evolve(secret);
        assert_eq!(secret, 704524);
        secret = evolve(secret);
        assert_eq!(secret, 1553684);
        secret = evolve(secret);
        assert_eq!(secret, 12683156);
        secret = evolve(secret);
        assert_eq!(secret, 11100544);
        secret = evolve(secret);
        assert_eq!(secret, 12249484);
        secret = evolve(secret);
        assert_eq!(secret, 7753432);
        secret = evolve(secret);
        assert_eq!(secret, 5908254);
    }

    #[test]
    fn generate_123() {
        let secret = 123;
        assert_eq!(future(secret, 1), 15887950);
        assert_eq!(future(secret, 2), 16495136);
        assert_eq!(future(secret, 3), 527345);
        assert_eq!(future(secret, 4), 704524);
        assert_eq!(future(secret, 5), 1553684);
        assert_eq!(future(secret, 6), 12683156);
        assert_eq!(future(secret, 7), 11100544);
        assert_eq!(future(secret, 8), 12249484);
        assert_eq!(future(secret, 9), 7753432);
        assert_eq!(future(secret, 10), 5908254);
    }
}