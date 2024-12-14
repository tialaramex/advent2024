use history::readfile;

type Num = i128;

#[derive(Copy, Clone, Debug)]
struct Button {
    x: Num,
    y: Num,
}

impl Button {
    fn read(prefix: &str, s: &str) -> Self {
        let s = s
            .strip_prefix(prefix)
            .expect("Buttons should begin with the agreed prefix");
        let (x, y) = s
            .split_once(", ")
            .expect("Buttons should have both X and Y");
        let x = x.strip_prefix("X+").expect("X should begin X+");
        let x: Num = x.parse().expect("Should be a number");
        let y = y.strip_prefix("Y+").expect("Y should begin Y+");
        let y: Num = y.parse().expect("Should be a number");
        Self { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
struct Prize {
    x: Num,
    y: Num,
}

use std::str::FromStr;
impl FromStr for Prize {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("Prize: ")
            .ok_or("Prizes should begin with Prize:")?;
        let (x, y) = s
            .split_once(", ")
            .ok_or("Prize should have two conditions")?;
        let x = x.strip_prefix("X=").ok_or("X should begin X=")?;
        let x: Num = x.parse().map_err(|_| "Should be a number")?;
        let y = y.strip_prefix("Y=").ok_or("Y should begin Y=")?;
        let y: Num = y.parse().map_err(|_| "Should be a number")?;
        Ok(Prize { x, y })
    }
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut lines = ctxt.lines();
    let mut tokens = 0;
    loop {
        let Some(first) = lines.next() else {
            panic!("Should not run out of lines mid-way through an arcade machine");
        };
        let a = Button::read("Button A: ", first);
        let Some(second) = lines.next() else {
            panic!("Should not run out of lines mid-way through an arcade machine");
        };
        let b = Button::read("Button B: ", second);
        let Some(third) = lines.next() else {
            panic!("Should not run out of lines mid-way through an arcade machine");
        };
        let prize: Prize = third.parse().expect("should be a Prize");

        let mut fewest: Option<Num> = None;
        for push_a in 0..=100 {
            let x = a.x * push_a;
            let y = a.y * push_a;
            if x > prize.x || y > prize.y {
                // No point trying pushing more often
                break;
            }
            let pushes = (prize.x - x) / b.x;
            if b.x * pushes != prize.x - x {
                continue;
            }
            if b.y * pushes != prize.y - y {
                continue;
            }
            let price = push_a * 3 + pushes;
            match fewest {
                None => {
                    fewest = Some(price);
                }
                Some(prev) => {
                    if prev > price {
                        fewest = Some(price);
                    }
                }
            }
        }
        if let Some(price) = fewest {
            tokens += price;
        }

        if lines.next().is_none() {
            break;
        }
    }
    println!("Spent {tokens} to win all possible prizes");
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut lines = ctxt.lines();
    let mut tokens = 0;
    loop {
        let Some(first) = lines.next() else {
            panic!("Should not run out of lines mid-way through an arcade machine");
        };
        let a = Button::read("Button A: ", first);
        let Some(second) = lines.next() else {
            panic!("Should not run out of lines mid-way through an arcade machine");
        };
        let b = Button::read("Button B: ", second);
        let Some(third) = lines.next() else {
            panic!("Should not run out of lines mid-way through an arcade machine");
        };
        let mut prize: Prize = third.parse().expect("should be a Prize");
        prize.x += 10000000000000;
        prize.y += 10000000000000;

        let push_a = ((b.y * prize.x) - (b.x * prize.y)) / ((b.y * a.x) - (b.x * a.y));
        let push_b = ((a.y * prize.x) - (a.x * prize.y)) / ((a.y * b.x) - (a.x * b.y));
        if push_a * a.x + push_b * b.x == prize.x && push_a * a.y + push_b * b.y == prize.y {
            tokens += push_a * 3 + push_b;
        }

        if lines.next().is_none() {
            break;
        }
    }
    println!("Spent {tokens} to win all possible prizes");
}
