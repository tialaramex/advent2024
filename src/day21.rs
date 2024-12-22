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

    // Starting at digit prev (or A), move to digit this (or A) and press A to press the digit
    fn digit(&mut self, prev: char, this: char) -> usize {
        let patterns = match (prev, this) {
            ('A', '0') => ["<A"].as_slice(),
            ('A', '1') => ["^<<A"].as_slice(),
            ('A', '2') => ["^<A", "<^A"].as_slice(),
            ('A', '3') => ["^A"].as_slice(),
            ('A', '4') => ["^^<<A"].as_slice(),
            ('A', '8') => ["<^^^A", "^^^<A"].as_slice(),
            ('A', '9') => ["^^^A"].as_slice(),
            ('0', 'A') => [">A"].as_slice(),
            ('0', '2') => ["^A"].as_slice(),
            ('0', '5') => ["^^A"].as_slice(),
            ('0', '8') => ["^^^A"].as_slice(),
            ('1', '2') => [">A"].as_slice(),
            ('1', '6') => [">>^A", "^>>A"].as_slice(),
            ('1', '7') => ["^^A"].as_slice(),
            ('2', '0') => ["vA"].as_slice(),
            ('2', '9') => ["^^>A", ">^^A"].as_slice(),
            ('3', '7') => ["^^<<A", "<<^^A"].as_slice(),
            ('4', '5') => [">A"].as_slice(),
            ('5', 'A') => ["vv>A", ">vvA"].as_slice(),
            ('5', '6') => [">A"].as_slice(),
            ('6', 'A') => ["vvA"].as_slice(),
            ('6', '9') => ["^A"].as_slice(),
            ('7', '6') => [">>vA", "v>>A"].as_slice(),
            ('7', '9') => [">>A"].as_slice(),
            ('8', 'A') => ["vvv>A", ">vvvA"].as_slice(),
            ('8', '0') => ["vvvA"].as_slice(),
            ('9', 'A') => ["vvvA"].as_slice(),
            ('9', '8') => ["<A"].as_slice(),
            _ => panic!("Can't handle digit '{prev}' to '{this}' yet"),
        };
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
