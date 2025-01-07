use history::readfile;

type Num = u32;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Block {
    id: Option<Num>,
}

impl Block {
    fn new(id: Num) -> Block {
        Block { id: Some(id) }
    }

    fn free() -> Block {
        Block { id: None }
    }

    fn id(&self) -> Option<Num> {
        self.id
    }
}

use std::fmt::{Debug, Formatter};

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.id {
            None => f.write_str("."),
            Some(id) => {
                if id < 10 {
                    write!(f, "{id}")
                } else {
                    f.write_str("X")
                }
            }
        }
    }
}

#[derive(Clone)]
struct Diskmap {
    v: Vec<Block>,
}

use std::str::FromStr;
impl FromStr for Diskmap {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v: Vec<Block> = Vec::new();
        let mut id: Num = 0;
        let mut free = false;
        for blocks in s.trim().chars().map(|c| c.to_digit(10).unwrap()) {
            if free {
                for _ in 0..blocks {
                    v.push(Block::free());
                }
                free = false;
            } else {
                for _ in 0..blocks {
                    v.push(Block::new(id));
                }
                id += 1;
                free = true;
            }
        }

        Ok(Diskmap { v })
    }
}

impl Debug for Diskmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for block in self.v.iter() {
            f.write_fmt(format_args!("{:?}", block))?
        }
        Ok(())
    }
}

impl Diskmap {
    fn checksum(&self) -> u64 {
        let mut sum = 0;
        for (n, block) in self.v.iter().enumerate() {
            if let Some(id) = block.id() {
                sum += (id as u64) * (n as u64);
            }
        }
        sum
    }

    fn crush(&mut self) {
        let mut first = 0;
        let mut last = self.v.len() - 1;
        loop {
            // 1. Move first forward to find an empty block
            while first < last && self.v[first].id().is_some() {
                first += 1;
            }

            // 2. Move last back to find a non-empty block
            while first < last && self.v[last].id().is_none() {
                last -= 1;
            }
            // 3. Swap

            if first >= last {
                return;
            }

            self.v.swap(first, last);
        }
    }

    // Assumes all files are contiguous, so, don't use after self.crush
    fn files(&self) -> Vec<(Num, usize, usize)> {
        let mut v = Vec::new();
        let mut last: Option<Num> = None;

        let mut length = 0;
        let mut n = 0;
        for block in self.v.iter() {
            match (block.id(), last) {
                (None, None) => (),
                (None, Some(id)) => {
                    v.push((id, n - length, length));
                    last = None;
                }
                (Some(id), None) => {
                    last = Some(id);
                    length = 1;
                }
                (Some(id1), Some(id2)) => {
                    if id1 != id2 {
                        v.push((id2, n - length, length));
                        last = Some(id1);
                        length = 1;
                    } else {
                        length += 1;
                    }
                }
            }
            n += 1;
        }
        if let Some(id) = last {
            v.push((id, n - length, length));
        }
        v
    }

    fn free_blocks(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();

        let mut length = 0;
        let mut n = 0;
        for block in self.v.iter() {
            if block.id().is_none() {
                length += 1;
            } else if length > 0 {
                v.push((n - length, length));
                length = 0;
            }
            n += 1;
        }
        if length > 0 {
            v.push((n - length, length));
        }
        v
    }

    fn file_move(&mut self, from: usize, to: usize, length: usize) {
        for k in 0..length {
            self.v.swap(from + k, to + k);
        }
    }

    fn defrag(&mut self) {
        // Free list will only grow after the part we're actively changing
        let mut free = self.free_blocks();
        free.reverse();
        let mut v = self.files();
        v.sort_unstable();

        use std::cmp::Ordering::*;

        let mut stack: Vec<(usize, usize)> = Vec::new();
        while let Some((_, from, need)) = v.pop() {
            while let Some((to, blocks)) = free.pop() {
                if to > from {
                    // No point moving later
                    break;
                }
                match blocks.cmp(&need) {
                    Greater => {
                        self.file_move(from, to, need);
                        free.push((to + need, blocks - need));
                        break;
                    }
                    Equal => {
                        self.file_move(from, to, need);
                        break;
                    }
                    Less => {
                        stack.push((to, blocks));
                    }
                }
            }
            while let Some((to, blocks)) = stack.pop() {
                free.push((to, blocks));
            }
        }
    }
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut disk: Diskmap = ctxt.value().parse().expect("input should be a diskmap");
    disk.crush();
    println!("Checksum is {}", disk.checksum());
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut disk: Diskmap = ctxt.value().parse().expect("input should be a diskmap");
    disk.defrag();
    println!("Defragged checksum is {}", disk.checksum());
}
