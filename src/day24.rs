use history::readfile;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Id(u32);

impl Id {
    const fn name_to_id(name: &str) -> Self {
        debug_assert!(name.len() == 3);
        let bytes = name.as_bytes();
        let id = (bytes[0] as u32 * 65536) + (bytes[1] as u32 * 256) + (bytes[2] as u32);
        Self(id)
    }

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

    fn has_inputs(&self, a: Id, b: Id) -> bool {
        (self.left == a && self.right == b) | (self.right == a && self.left == b)
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

    fn find_gate(&self, a: Id, b: Id, kind: Kind) -> Option<Gate> {
        for gate in self.gates.iter() {
            if gate.kind == kind && gate.has_inputs(a, b) {
                return Some(*gate);
            }
        }
        None
    }

    // Either Ok(()) OR
    // Err((a,b)) where a, b are wires to try swapping
    fn check(&self) -> Result<(), (String, String)> {
        let x0 = Id::name_to_id("x00");
        let y0 = Id::name_to_id("y00");
        let z0 = Id::name_to_id("z00");
        // One, check half adder x0 XOR y0 -> z0
        let first_half = self
            .find_gate(x0, y0, Kind::Xor)
            .expect("This should be a half adder");
        if first_half.out != z0 {
            return Err((first_half.out.id_to_name(), "z00".to_owned()));
        }

        // Two, loop checking adders
        let mut carry = self
            .find_gate(x0, y0, Kind::And)
            .expect("Even Half Adders need carry bits");
        for bit in 1..64 {
            let x = Id::name_to_id(&format!("x{bit:02}"));
            let y = Id::name_to_id(&format!("y{bit:02}"));
            let z = Id::name_to_id(&format!("z{bit:02}"));

            // A: find sum xN XOR yN -> sN
            if let Some(sum) = self.find_gate(x, y, Kind::Xor) {
                // B: find new carry xN AND yN -> cN
                let new_carry = self
                    .find_gate(x, y, Kind::And)
                    .expect("If there's a sum there should be a new carry");
                match self.find_gate(sum.out, carry.out, Kind::Xor) {
                    Some(new_bit) => {
                        if new_bit.out != z {
                            // Try swapping new_bit with zN
                            return Err((new_bit.out.id_to_name(), format!("z{bit:02}")));
                        }
                    }
                    None => {
                        // Try swapping new_carry with sum
                        return Err((sum.out.id_to_name(), new_carry.out.id_to_name()));
                    }
                }
                let step = self
                    .find_gate(sum.out, carry.out, Kind::And)
                    .expect("sum.out AND carry.out should exist too");
                carry = self
                    .find_gate(step.out, new_carry.out, Kind::Or)
                    .expect("Hmmmm?");
            } else {
                // Last bit, just the carry? No problems here in my input
                return Ok(());
            }
        }
        panic!("Checking not yet ready for large circuits");
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

    let mut swaps: Vec<String> = Vec::new();
    while let Err((a, b)) = dev.check() {
        dev.swap(&a, &b);
        swaps.push(a);
        swaps.push(b);
    }

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
