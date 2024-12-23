use history::readfile;

type Id = u16;

use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Network {
    names: HashSet<Id>,
    links: HashMap<Id, Vec<Id>>,
}

impl Network {
    fn new() -> Self {
        Network {
            names: HashSet::new(),
            links: HashMap::new(),
        }
    }

    fn name_to_id(&mut self, name: &str) -> Id {
        debug_assert!(name.len() == 2);
        let bytes = name.as_bytes();
        let id = bytes[0] as Id * 256 + bytes[1] as Id;
        self.names.insert(id);
        id
    }

    fn id_to_name(&self, id: Id) -> String {
        debug_assert!(id > 0x101 && id < 0x7f7f);
        let mut name = String::with_capacity(2);
        name.push(char::from_u32((id >> 8) as u32).unwrap());
        name.push(char::from_u32((id & 0xff) as u32).unwrap());
        name
    }

    fn threes(&self) -> Vec<(Id, Id, Id)> {
        let mut v = Vec::new();
        for a in self.names.iter() {
            let Some(ours) = self.links.get(a) else {
                continue;
            };
            for b in ours {
                if b < a {
                    continue;
                }
                let Some(theirs) = self.links.get(b) else {
                    continue;
                };
                for c in theirs {
                    if b < c && ours.contains(c) {
                        v.push((*a, *b, *c));
                    }
                }
            }
        }
        v
    }

    fn connect(&mut self, a: Id, b: Id) {
        self.links.entry(a).or_default().push(b);
        self.links.entry(b).or_default().push(a);
    }

    fn parse(filename: &str) -> Self {
        let mut net = Self::new();
        let ctxt = readfile(filename);
        for line in ctxt.lines() {
            let (a, b) = line
                .split_once('-')
                .expect("Each line should be in the form ab-cd");
            let a = net.name_to_id(a);
            let b = net.name_to_id(b);
            net.connect(a, b);
        }
        net
    }

    fn party(&self) -> Vec<Id> {
        let mut biggest = Vec::new();
        let mut remainder: Vec<Id> = self.names.iter().copied().collect();
        while !remainder.is_empty() {
            let mut next: Vec<Id> = Vec::with_capacity(remainder.len());
            let mut big: Vec<Id> = Vec::new();
            while let Some(node) = remainder.pop() {
                let ours = self
                    .links
                    .get(&node)
                    .expect("Every node should have at least one link");
                if big.iter().all(|node| ours.contains(node)) {
                    big.push(node);
                } else {
                    next.push(node);
                }
            }
            if big.len() > biggest.len() {
                biggest = big;
            }
            remainder = next;
        }
        biggest
    }
}

pub fn a(filename: &str) {
    let net = Network::parse(filename);
    let mut count = 0;
    let threes = net.threes();
    for (a, b, c) in threes {
        let a = net.id_to_name(a);
        let b = net.id_to_name(b);
        let c = net.id_to_name(c);
        if a.starts_with('t') || b.starts_with('t') || c.starts_with('t') {
            count += 1;
        }
    }
    println!(
        "{count} sets of three interconnected computers have at least one named starting with t"
    );
}

pub fn b(filename: &str) {
    let net = Network::parse(filename);
    let party = net.party();
    let mut party: Vec<_> = party.into_iter().map(|i| net.id_to_name(i)).collect();
    party.sort_unstable();
    let password: String = party.join(",");
    println!("The password is: {password}");
}
