use history::readfile;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Id(u32);

impl Id {
    fn name_to_id(name: &str) -> Self {
        debug_assert!(name.len() == 3);
        let bytes = name.as_bytes();
        let id = (bytes[0] as u32 * 65536) + (bytes[1] as u32 * 256) + (bytes[2] as u32);
        Self(id)
    }

    #[cfg(test)]
    fn id_to_name(self) -> String {
        debug_assert!(self.0 > 0x10101 && self.0 < 0x7f7f7f);
        let mut name = String::with_capacity(2);
        name.push(char::from_u32(self.0 >> 16).unwrap());
        name.push(char::from_u32((self.0 >> 8) & 0xff).unwrap());
        name.push(char::from_u32(self.0 & 0xff).unwrap());
        name
    }

    #[cfg(test)]
    fn prefix(self, byte: u8) -> bool {
        self.0 >> 16 == byte as u32
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Kind {
    And,
    Or,
    Xor,
}

#[derive(Copy, Clone, Debug)]
struct Gate {
    left: Id,
    right: Id,
    out: Id,
    kind: Kind,
}

impl Gate {
    fn operate(&self, wires: &mut HashMap<Id, bool>) -> bool {
        use Kind::*;
        let Some(left) = wires.get(&self.left) else {
            return true;
        };
        let Some(right) = wires.get(&self.right) else {
            return true;
        };
        let out = match self.kind {
            And => left & right,
            Or => left | right,
            Xor => left ^ right,
        };
        wires.insert(self.out, out);
        false
    }

    fn parse(s: &str, out: Id) -> Option<Self> {
        if let Some((left, right)) = s.split_once(" AND ") {
            let left = Id::name_to_id(left);
            let right = Id::name_to_id(right);
            return Some(Gate {
                left,
                right,
                out,
                kind: Kind::And,
            });
        }
        if let Some((left, right)) = s.split_once(" OR ") {
            let left = Id::name_to_id(left);
            let right = Id::name_to_id(right);
            return Some(Gate {
                left,
                right,
                out,
                kind: Kind::Or,
            });
        }
        if let Some((left, right)) = s.split_once(" XOR ") {
            let left = Id::name_to_id(left);
            let right = Id::name_to_id(right);
            return Some(Gate {
                left,
                right,
                out,
                kind: Kind::Xor,
            });
        }
        None
    }

    #[cfg(test)]
    fn is_add_bit(&self) -> bool {
        if self.kind != Kind::Xor {
            false
        } else if (self.left.prefix(b'x') && self.right.prefix(b'y'))
            || (self.left.prefix(b'y') && self.right.prefix(b'x'))
        {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Debug)]
struct Device {
    gates: Vec<Gate>,
    wires: HashMap<Id, bool>,
}

impl Device {
    fn new() -> Self {
        Self {
            gates: Vec::new(),
            wires: HashMap::new(),
        }
    }

    fn swap(&mut self, a: &str, b: &str) {
        let a = Id::name_to_id(a);
        let b = Id::name_to_id(b);
        let mut done = 0;
        for gate in self.gates.iter_mut() {
            if gate.out == a {
                done += 1;
                gate.out = b;
            } else if gate.out == b {
                done += 1;
                gate.out = a;
            }
        }
        debug_assert!(done == 2);
    }

    fn set_wire(&mut self, id: Id, level: bool) {
        if let Some(old) = self.wires.insert(id, level) {
            panic!("Wire changed from {old} to {level}");
        }
    }

    #[cfg(test)]
    // Currentlu sets all 64 bits
    fn set_number(&mut self, prefix: &str, n: u64) {
        for d in 0..64 {
            let wire = format!("{prefix}{d:02}");
            let wire = Id::name_to_id(&wire);
            if (n & (1 << d)) == 0 {
                self.wires.insert(wire, false);
            } else {
                self.wires.insert(wire, true);
            }
        }
    }

    fn number(&self, prefix: &str) -> u64 {
        let mut n = 0;
        for d in 0..64 {
            let wire = format!("{prefix}{d:02}");
            let wire = Id::name_to_id(&wire);
            if let Some(&level) = self.wires.get(&wire) {
                if level {
                    n += 1 << d;
                }
                continue;
            }
        }
        n
    }

    fn settle(&mut self) {
        let mut remaining = self.gates.clone();
        while !remaining.is_empty() {
            remaining.retain(|gate| gate.operate(&mut self.wires));
        }
    }

    #[cfg(test)]
    fn wire_backtrace(&self, wire: &str) {
        let wire_id = Id::name_to_id(wire);
        for gate in &self.gates {
            if gate.out == wire_id {
                let left = gate.left.id_to_name();
                let right = gate.right.id_to_name();
                let kind = gate.kind;
                if gate.is_add_bit() {
                    println!("{wire}: ADD BIT {left} {kind:?} {right}");
                } else {
                    println!("{wire}: {left} {kind:?} {right}");
                    self.wire_backtrace(&left);
                    self.wire_backtrace(&right);
                }
            }
        }
    }

    #[cfg(test)]
    fn wire_dump(&self) {
        for (id, val) in self.wires.iter() {
            let name = id.id_to_name();
            let val = if *val { 1 } else { 0 };
            println!("{name}: {val}");
        }
    }

    fn parse(filename: &str) -> Self {
        let ctxt = readfile(filename);
        let mut dev = Self::new();
        let mut lines = ctxt.lines();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (wire, init) = line.split_once(": ").expect("Wires should go wr5: 0");
            let wire = Id::name_to_id(wire);
            let init = match init {
                "0" => false,
                "1" => true,
                _ => panic!("Initial wire level should not be {init}"),
            };
            dev.set_wire(wire, init);
        }
        for line in lines {
            let (gate, output) = line.split_once(" -> ").expect("Gates should have output");
            let output = Id::name_to_id(output);
            if let Some(gate) = Gate::parse(gate, output) {
                dev.gates.push(gate);
            } else {
                panic!("Unexpected gate: {line}");
            }
        }
        dev
    }
}

pub fn a(filename: &str) {
    let mut dev = Device::parse(filename);
    dev.settle();
    let z = dev.number("z");
    println!("The decimal number on wires starting z was: {z}");
}

pub fn b(filename: &str) {
    let mut dev = Device::parse(filename);

    let mut swaps = ["z36", "nwq", "z18", "fvw", "z22", "mdb", "grf", "wpq"];
    dev.swap(swaps[0], swaps[1]);
    dev.swap(swaps[2], swaps[3]);
    dev.swap(swaps[4], swaps[5]);
    dev.swap(swaps[6], swaps[7]);

    let x = dev.number("x");
    let y = dev.number("y");
    dev.settle();
    let diff = (x + y) ^ dev.number("z");
    if diff == 0 {
        swaps.sort_unstable();
        let answer: String = swaps.join(",");
        println!("Eight wires to swap are: {answer}");
    } else {
        println!(
            "Swaps chosen didn't work for test input, {} differences\n{diff:050b}",
            diff.count_ones()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids() {
        assert_eq!(Id::name_to_id("AAA"), Id(0x414141));
        assert_eq!(Id::name_to_id("x01"), Id(0x783031));
    }

    #[test]
    fn prefix() {
        assert!(Id::name_to_id("AAA").prefix(b'A'));
        assert!(Id::name_to_id("z01").prefix(b'z'));
        assert!(Id::name_to_id("x18").prefix(b'x'));
    }
}
