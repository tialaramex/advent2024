#[macro_export]
macro_rules! days {
    ($obj:expr, $filename:expr, $($day:ident,)+) => {
        days!($obj, $(days!($day a) => $day::a($filename), days!($day b) => $day::b($filename)),*, _ => { println!("{} not available yet", $obj); });
    };
    ($obj:expr, $filename:expr, $($day:ident),+) => {
        days!($obj, $filename, $($day,)+);
    };
    ($s:ident a) => {
        concat!(stringify!($s), 'a')
    };
    ($s:ident b) => {
        concat!(stringify!($s), 'b')
    };
    ($obj:expr, $($matcher:pat => $result:expr),*) => {
       match $obj {
           $($matcher => $result),*
       }
    }
}

pub struct Contents {
    pub text: String,
}

impl<'t> Contents {
    pub fn lines(&'t self) -> impl Iterator<Item = &str> + use<'t> {
        self.text.lines()
    }

    pub fn numbers(&self) -> impl Iterator<Item = isize> + use<'_> {
        self.lines().map(|l| l.parse::<isize>().unwrap_or(0))
    }

    pub fn binary(&self) -> impl Iterator<Item = isize> + use<'_> {
        self.lines()
            .filter_map(|l| isize::from_str_radix(l, 2).ok())
    }

    pub fn digits(&self) -> impl Iterator<Item = u32> + use<'_> {
        self.text.trim().chars().map(|c| c.to_digit(10).unwrap())
    }

    pub fn list(&'t self) -> impl Iterator<Item = &str> + use<'t> {
        self.text.trim().split(',')
    }

    pub fn list_numbers(&self) -> impl Iterator<Item = isize> + use<'_> {
        self.list().map(|n| n.parse().unwrap())
    }

    pub fn value(&self) -> &str {
        self.text.trim()
    }

    pub fn number(&self) -> isize {
        self.value().parse().unwrap()
    }
}

use std::fs;

pub fn readfile(filename: &str) -> Contents {
    match fs::read_to_string(filename) {
        Ok(text) => Contents { text },
        Err(e) => {
            panic!("When attempting to read \"{filename}\" - {e}");
        }
    }
}

use core::ops::ControlFlow;
use std::collections::hash_map;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// T is a type for an invariant, such as a map
pub trait State<T>: Copy + Eq + Hash {
    fn describe(&self, invariant: &T) -> String;
    fn next(&self, invariant: &T) -> Vec<Self>;

    /// Search novel states, report each novel state to a Report function
    /// the Report function is given the novel state, invariant and a count of steps
    /// if the Report function returns Continue this state will be used to
    /// generate future new states, if Break it is ignored
    fn search<R, B>(initial: Self, report: R, invariant: &T) -> Vec<B>
    where
        R: Fn(&Self, &T, usize) -> ControlFlow<B>,
    {
        let mut found: Vec<B> = Vec::new();
        let mut seen: HashSet<Self> = HashSet::new();
        seen.insert(initial);
        let mut current: Vec<Self> = vec![initial];
        let mut steps = 0;

        while !current.is_empty() {
            let mut next: Vec<Self> = Vec::new();
            for state in current {
                match report(&state, invariant, steps) {
                    ControlFlow::Continue(_) => {
                        let more = state.next(invariant);
                        for state in more {
                            if !seen.contains(&state) {
                                seen.insert(state);
                                next.push(state);
                            }
                        }
                    }
                    ControlFlow::Break(b) => {
                        found.push(b);
                    }
                }
            }
            steps += 1;
            current = next;
        }
        found
    }

    /// Minimum Steps from initial until predicate is true
    fn steps<P>(initial: Self, predicate: P, invariant: &T) -> usize
    where
        P: Fn(&Self) -> bool,
        Self: std::fmt::Debug,
    {
        if predicate(&initial) {
            return 0;
        }

        let mut seen: HashSet<Self> = HashSet::new();
        seen.insert(initial);
        let mut current: Vec<Self> = vec![initial];
        let mut steps = 0;

        loop {
            let mut next: Vec<Self> = Vec::new();
            for state in current {
                let more = state.next(invariant);
                for state in more {
                    if predicate(&state) {
                        return steps + 1;
                    }
                    if !seen.contains(&state) {
                        seen.insert(state);
                        next.push(state);
                    }
                }
            }
            if next.is_empty() {
                panic!("No route to achieve predicate on {initial:?}");
            }
            steps += 1;
            current = next;
        }
    }

    /// Best (fewest steps) number of State transitions from initial to goal
    fn best(initial: Self, goal: Self, invariant: &T) -> usize
    where
        Self: std::fmt::Debug,
    {
        Self::steps(initial, |&s| s == goal, invariant)
    }

    /// Possible states after up to steps taken
    fn count(initial: Self, steps: usize, invariant: &T) -> usize {
        let mut seen: HashSet<Self> = HashSet::new();
        seen.insert(initial);
        let mut current: Vec<Self> = vec![initial];

        for _ in 0..steps {
            let mut next: Vec<Self> = Vec::new();
            for state in current {
                let more = state.next(invariant);
                for state in more {
                    if !seen.contains(&state) {
                        seen.insert(state);
                        next.push(state);
                    }
                }
            }
            current = next;
        }
        seen.len()
    }

    /// Report (if debug is true) the States from the goal back to the initial
    fn report(initial: Self, goal: Self, invariant: &T, debug: bool) -> Self {
        if initial == goal {
            return initial;
        }

        let mut seen: HashMap<Self, Self> = HashMap::new();
        seen.insert(initial, initial);
        let mut current: Vec<Self> = Vec::with_capacity(1);
        current.push(initial);

        loop {
            let mut next: Vec<Self> = Vec::new();
            for state in current {
                let more = state.next(invariant);
                for new in more {
                    if let hash_map::Entry::Vacant(e) = seen.entry(new) {
                        e.insert(state);
                        next.push(new);
                    }
                    if new == goal {
                        if debug {
                            let mut prev = &new;
                            while prev != &initial {
                                println!("D: {}", prev.describe(invariant));
                                prev = seen.get(prev).unwrap();
                            }
                        }
                        return new;
                    }
                }
            }
            if debug {
                println!("D: {} distinct states seen", seen.len());
                println!("D: {} new states this iteration", next.len());
            }
            current = next;
        }
    }
}

/// Number of permutations of n things is n!
pub const fn permutations(n: usize) -> usize {
    match n {
        1 => 1,
        2 => 2,
        3 => 6,
        4 => 24,
        5 => 120,
        6 => 720,
        7 => 5040,
        8 => 40320,
        9 => 362880,
        10 => 3628800,
        _ => {
            panic!("Unknown permutations");
        }
    }
}

/// Heap's Algorithm for permuting slices up to length 10
/// handle zero specially to do nothing
pub fn heap<T>(a: &mut [T], n: usize) {
    assert!(a.len() < 11);
    if n == 0 {
        //return;
    } else if n % 2 == 1 {
        a.swap(0, 1);
    } else if n % 6 > 0 {
        a.swap(0, 2);
    } else if n % 24 > 0 {
        let p = (n / 8) % 3;
        a.swap(p, 3);
    } else if n % 120 > 0 {
        a.swap(0, 4);
    } else if n % 720 > 0 {
        let p = (n / 144) % 5;
        a.swap(p, 5);
    } else if n % 5040 > 0 {
        a.swap(0, 6);
    } else if n % 40320 > 0 {
        let p = (n / 5760) % 7;
        a.swap(p, 7);
    } else if n % 362880 > 0 {
        a.swap(0, 8);
    } else {
        let p = (n / 403200) % 9;
        a.swap(p, 9);
    }
}

pub mod map;

#[cfg(test)]

mod tests {
    use crate::heap;

    #[test]
    fn heap_two() {
        let mut t = ['A', 'B'];
        heap(&mut t, 0);
        assert_eq!(t, ['A', 'B']);
        heap(&mut t, 1);
        assert_eq!(t, ['B', 'A']);
    }

    #[test]
    fn heap_three() {
        let mut t = [1, 2, 3];
        heap(&mut t, 0);
        assert_eq!(t, [1, 2, 3]);
        heap(&mut t, 1);
        assert_eq!(t, [2, 1, 3]);
        heap(&mut t, 2);
        assert_eq!(t, [3, 1, 2]);
        heap(&mut t, 3);
        assert_eq!(t, [1, 3, 2]);
        heap(&mut t, 4);
        assert_eq!(t, [2, 3, 1]);
        heap(&mut t, 5);
        assert_eq!(t, [3, 2, 1]);
    }

    #[test]
    fn heap_four() {
        let mut t = ['a', 'b', 'c', 'd'];
        heap(&mut t, 0);
        assert_eq!(t, ['a', 'b', 'c', 'd']);
        heap(&mut t, 1);
        assert_eq!(t, ['b', 'a', 'c', 'd']);
        for n in 2..23 {
            heap(&mut t, n);
        }
        assert_eq!(t, ['c', 'b', 'd', 'a']);
        heap(&mut t, 23);
        assert_eq!(t, ['b', 'c', 'd', 'a']);
    }

    #[test]
    fn heap_nine() {
        let mut t = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        heap(&mut t, 0);
        assert_eq!(t, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
        heap(&mut t, 1);
        assert_eq!(t, [2, 1, 3, 4, 5, 6, 7, 8, 9]);
        for n in 2..362879 {
            heap(&mut t, n);
        }
        assert_eq!(t, [2, 9, 3, 4, 5, 6, 7, 8, 1]);
        heap(&mut t, 362879);
        assert_eq!(t, [9, 2, 3, 4, 5, 6, 7, 8, 1]);
    }
}
