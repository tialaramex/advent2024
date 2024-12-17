use history::readfile;

type Reg = u64;

#[derive(Clone, Debug)]
struct Device {
    a: Reg,
    b: Reg,
    c: Reg,
    ip: usize,
    prog: Vec<u8>,
}

use std::str::FromStr;
impl FromStr for Device {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let Some(a) = lines.next() else {
            return Err("Should define register A");
        };
        let a = a
            .strip_prefix("Register A: ")
            .ok_or("Should define register A")?;
        let a: Reg = a.parse().map_err(|_| "Should be a number")?;

        let Some(b) = lines.next() else {
            return Err("Should define register B");
        };
        let b = b
            .strip_prefix("Register B: ")
            .ok_or("Should define register A")?;
        let b: Reg = b.parse().map_err(|_| "Should be a number")?;

        let Some(c) = lines.next() else {
            return Err("Should define register C");
        };
        let c = c
            .strip_prefix("Register C: ")
            .ok_or("Should define register C")?;
        let c: Reg = c.parse().map_err(|_| "Should be a number")?;

        if lines.next().is_none_or(|line| !line.is_empty()) {
            return Err("Should have a blank separator line");
        }

        let Some(prog) = lines.next() else {
            return Err("Should define the program");
        };

        let prog = prog
            .strip_prefix("Program: ")
            .ok_or("Should prefix the program properly")?;
        let prog: Vec<u8> = prog.split(',').map(|s| s.parse().unwrap()).collect();

        Ok(Device {
            a,
            b,
            c,
            ip: 0,
            prog,
        })
    }
}

impl Device {
    fn reset_with(&mut self, a: Reg) {
        self.ip = 0;
        self.a = a;
        self.b = 0;
        self.c = 0;
    }

    fn combo(&self, n: u8) -> Reg {
        match n {
            0..=3 => n as Reg,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Reserved combo"),
        }
    }

    // either Some(output) or None
    fn step(&mut self) -> Option<u8> {
        let instruction = self.prog[self.ip];
        let operand = self.prog[self.ip + 1];
        // Doing this early means if we JNZ we'll overwrite this
        self.ip += 2;
        match instruction {
            0 => {
                self.a >>= self.combo(operand);
                None
            }
            1 => {
                self.b ^= operand as Reg;
                None
            }
            2 => {
                self.b = self.combo(operand) % 8;
                None
            }
            3 => {
                if self.a != 0 {
                    self.ip = operand as usize;
                }
                None
            }
            4 => {
                self.b ^= self.c;
                None
            }
            5 => Some((self.combo(operand) % 8) as u8),
            6 => {
                self.b = self.a >> self.combo(operand);
                None
            }
            7 => {
                self.c = self.a >> self.combo(operand);
                None
            }
            _ => panic!("Unexpected instruction {instruction}"),
        }
    }

    fn run(&mut self) -> Vec<u8> {
        let mut v = Vec::new();
        while self.ip < self.prog.len() {
            if let Some(value) = self.step() {
                v.push(value);
            }
        }
        v
    }
}

fn list(nums: &[u8]) -> String {
    let mut output: String = nums.iter().map(|n| n.to_string() + ",").collect();
    output.pop();
    output
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut device: Device = ctxt.value().parse().expect("Should define the device");
    let output = list(&device.run());
    println!("Output joined by commas: {output}");
}

fn check(a: &[u8], b: &[u8], steps: usize) -> bool {
    if a.len() > b.len() {
        return false;
    }
    if a.len() <= steps {
        return false;
    }
    if b.len() <= steps {
        return false;
    }
    for k in 0..=steps {
        if a[k] != b[k] {
            return false;
        }
    }
    true
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut device: Device = ctxt.value().parse().expect("Should define the device");
    let desired = device.prog.clone();

    let mut possible: Vec<Reg> = vec![0];
    for step in 0..desired.len() {
        let mut next: Vec<Reg> = Vec::with_capacity(1024);
        while let Some(input) = possible.pop() {
            for bits in 0..1024 {
                let n = input ^ (bits << (step * 3));
                device.reset_with(n);
                let output = device.run();
                if check(&output, &desired, step) {
                    next.push(n % (1 << ((step + 1) * 3)));
                }
            }
        }
        if next.is_empty() {
            panic!("Impossible to find a plausible input");
        }
        next.sort_unstable();
        next.dedup();
        possible = next;
    }
    let first = possible[0];
    println!("Lowest possible initial value of register A is {first}");
}
