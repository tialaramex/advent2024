use history::readfile;

type Num = i32;

#[derive(Copy, Clone, Debug)]
struct Rule {
    before: Num,
    after: Num,
}

use std::str::FromStr;
impl FromStr for Rule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((before, after)) = s.split_once('|') else {
            return Err("Missing pipe");
        };
        let before: Num = before.parse().map_err(|_| "Should be a number")?;
        let after: Num = after.parse().map_err(|_| "Both should be numbers")?;
        Ok(Rule { before, after })
    }
}

impl Rule {
    fn obey(&self, nums: &[Num]) -> bool {
        let mut late = false;
        for n in nums {
            if n == &self.after {
                late = true;
            }
            if n == &self.before && late {
                return false;
            }
        }
        true
    }

    fn correct(&self, nums: &mut Vec<Num>) -> bool {
        if self.obey(nums) {
            return false;
        }
        // Both present, wrong order

        let before = nums
            .iter()
            .position(|&n| n == self.before)
            .expect("should be present");
        nums.remove(before);
        let after = nums
            .iter()
            .position(|&n| n == self.after)
            .expect("should also be present");
        nums.insert(after, self.before);
        true
    }
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut rules = Vec::new();
    let mut lines = ctxt.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let rule: Rule = line.parse().expect("should be a page rule");
        rules.push(rule);
    }
    let mut sum = 0;
    'outer: for line in lines {
        let nums: Vec<Num> = line.split(',').map(|s| s.parse().unwrap()).collect();
        for rule in rules.iter() {
            if !rule.obey(&nums) {
                continue 'outer;
            }
        }
        let middle = nums.len() / 2;
        sum += nums[middle];
    }
    println!("Middle page numbers of correct updates sum to {sum}");
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut rules = Vec::new();
    let mut lines = ctxt.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let rule: Rule = line.parse().expect("should be a page rule");
        rules.push(rule);
    }
    let mut sum = 0;
    for line in lines {
        let mut nums: Vec<Num> = line.split(',').map(|s| s.parse().unwrap()).collect();
        let mut corrected = false;
        loop {
            let mut this = false;
            for rule in rules.iter() {
                if rule.correct(&mut nums) {
                    this = true;
                }
            }
            if this {
                corrected = true;
            } else {
                break;
            }
        }
        if corrected {
            let middle = nums.len() / 2;
            sum += nums[middle];
        }
    }
    println!("Middle page numbers of corrected updates sum to {sum}");
}
