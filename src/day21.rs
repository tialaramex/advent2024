use history::readfile;

type Num = i32;

use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Starship {
    depth: u8,
    cache: HashMap<(u8, &'static str), usize>,
}

impl Starship {
    fn new(depth: u8) -> Starship {
        Starship {
            depth,
            cache: HashMap::new(),
        }
    }

    fn arrow(&mut self, level: u8, prev: char, this: char) -> usize {
        let patterns = match (prev, this) {
            ('A', 'A') | ('<', '<') | ('>', '>') | ('^', '^') | ('v', 'v') => ["A"].as_slice(),
            ('A', '^') => ["<A"].as_slice(),
            ('A', '>') => ["vA"].as_slice(),
            ('A', 'v') => ["v<A", "<vA"].as_slice(),
            ('A', '<') => ["v<<A"].as_slice(),

            ('<', 'A') => [">>^A"].as_slice(),
            ('<', '^') => [">^A"].as_slice(),
            ('<', 'v') => [">A"].as_slice(),

            ('>', 'A') => ["^A"].as_slice(),
            ('>', '^') => ["<^A", "^<A"].as_slice(),
            ('>', 'v') => ["<A"].as_slice(),

            ('^', 'A') => [">A"].as_slice(),
            ('^', '<') => ["v<A"].as_slice(),
            ('^', '>') => ["v>A", ">vA"].as_slice(),

            ('v', 'A') => ["^>A", ">^A"].as_slice(),
            ('v', '<') => ["<A"].as_slice(),
            ('v', '>') => [">A"].as_slice(),

            ('<', '>') | ('>', '<') | ('^', 'v') | ('v', '^') => {
                panic!("Contradictory movements '{prev}' and '{this}'")
            }
            _ => panic!("Can't handle arrow '{prev}' to '{this}' yet"),
        };
        let mut shortest: Option<usize> = None;
        for pattern in patterns {
            let length = self.shortest(level - 1, pattern);
            if let Some(s) = shortest {
                if s > length {
                    shortest = Some(length);
                }
            } else {
                shortest = Some(length);
            }
        }
        shortest.unwrap()
    }

    fn shortest(&mut self, level: u8, pattern: &'static str) -> usize {
        debug_assert!(pattern.ends_with('A'));
        // 1. Are we level 0? Use length of pattern
        if level == 0 {
            return pattern.len();
        }

        if let Some(&value) = self.cache.get(&(level, pattern)) {
            return value;
        }

        // Calculate length
        let mut length = 0;
        let mut prev = 'A';
        for ch in pattern.chars() {
            length += self.arrow(level, prev, ch);
            prev = ch;
        }
        // Write to cache
        self.cache.insert((level, pattern), length);
        length
    }

    const fn rowcol(d: char) -> (i8, i8) {
        match d {
            'A' => (4, 3),
            '0' => (4, 2),
            '1' => (3, 1),
            '2' => (3, 2),
            '3' => (3, 3),
            '4' => (2, 1),
            '5' => (2, 2),
            '6' => (2, 3),
            '7' => (1, 1),
            '8' => (1, 2),
            '9' => (1, 3),
            _ => panic!("Invalid digit position"),
        }
    }

    // Starting at digit prev (or A), move to digit this (or A) and press A to press the digit
    fn digit(&mut self, prev: char, this: char) -> usize {
        let from = Self::rowcol(prev);
        let to = Self::rowcol(this);

        let mut patterns = match (to.0 - from.0, to.1 - from.1) {
            (-3, -2) => ["^^^<<A"].as_slice(),
            (-3, -1) => ["^^^<A", "<^^^A"].as_slice(),
            (-3, 0) => ["^^^A"].as_slice(),
            (-3, 1) => ["^^^>A", ">^^^A"].as_slice(),

            (-2, -2) => ["^^<<A", "<<^^A"].as_slice(),
            (-2, -1) => ["^^<A", "<^^A"].as_slice(),
            (-2, 0) => ["^^A"].as_slice(),
            (-2, 1) => ["^^>A", ">^^A"].as_slice(),
            (-2, 2) => ["^^>>A", ">>^^A"].as_slice(),

            (-1, -2) => ["^<<A", "<<^A"].as_slice(),
            (-1, -1) => ["^<A", "<^A"].as_slice(),
            (-1, 0) => ["^A"].as_slice(),
            (-1, 1) => ["^>A", ">^A"].as_slice(),
            (-1, 2) => ["^>>A", ">>^A"].as_slice(),

            (0, -2) => ["<<A"].as_slice(),
            (0, -1) => ["<A"].as_slice(),
            (0, 0) => ["A"].as_slice(),
            (0, 1) => [">A"].as_slice(),
            (0, 2) => [">>A"].as_slice(),

            (1, -2) => ["v<<A", "<<vA"].as_slice(),
            (1, -1) => ["v<A", "<vA"].as_slice(),
            (1, 0) => ["vA"].as_slice(),
            (1, 1) => [">vA", "v>A"].as_slice(),
            (1, 2) => [">>vA", "v>>A"].as_slice(),

            (2, -2) => ["vv<<A", "<<vvA"].as_slice(),
            (2, -1) => ["vv<A", "<vvA"].as_slice(),
            (2, 0) => ["vvA"].as_slice(),
            (2, 1) => [">vvA", "vv>A"].as_slice(),
            (2, 2) => [">>vvA", "vv>>A"].as_slice(),

            (3, -1) => ["vvv<A", "<vvvA"].as_slice(),
            (3, 0) => ["vvvA"].as_slice(),
            (3, 1) => [">vvvA", "vvv>A"].as_slice(),
            (3, 2) => [">>vvvA"].as_slice(),
            _ => unreachable!("Should not be any cases where we don't match now"),
        };

        // Avoid bottom left corner
        if (from.0 == 4 && to.1 == 1) || (from.1 == 1 && to.0 == 4) {
            patterns = &patterns[..1];
        }

        let mut shortest: Option<usize> = None;
        for pattern in patterns {
            let length = self.shortest(self.depth, pattern);
            if let Some(s) = shortest {
                if s > length {
                    shortest = Some(length);
                }
            } else {
                shortest = Some(length);
            }
        }
        shortest.unwrap()
    }

    fn number(&mut self, pattern: &str) -> usize {
        debug_assert!(pattern.ends_with('A'));
        // For each digit (A) -> 1 -> 2 -> 3 -> A
        let mut length = 0;
        let mut prev = 'A';
        for this in pattern.chars() {
            length += self.digit(prev, this);
            prev = this;
        }
        length
    }
}

fn numeric(s: &str) -> Num {
    let front = s
        .split(|c: char| !c.is_ascii_digit())
        .next()
        .expect("splitn always gives at least one item");
    if front.is_empty() {
        0
    } else {
        front
            .parse()
            .expect("should be a number as we didn't include non-digits")
    }
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut ship = Starship::new(2);
    let sum: usize = ctxt
        .lines()
        .map(|line| numeric(line) as usize * ship.number(line))
        .sum();
    println!("The sum of complexities of the five codes is: {sum}");
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut ship = Starship::new(25);
    let sum: usize = ctxt
        .lines()
        .map(|line| numeric(line) as usize * ship.number(line))
        .sum();
    println!("The new sum of complexities of the five codes is: {sum}");
}
