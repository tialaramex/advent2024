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
                    f.write_fmt(format_args!("{}", id))
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

    fn last_file_id(&self) -> Option<Num> {
        let mut last: Option<Num> = None;
        for block in self.v.iter() {
            match (block.id(), last) {
                (None, _) => (),
                (Some(id), None) => {
                    last = Some(id);
                }
                (Some(id1), Some(id2)) => {
                    if id1 > id2 {
                        last = Some(id1);
                    }
                }
            }
        }
        last
    }

    // Offset and length of file block
    fn find_by_id(&self, id: Num) -> (usize, usize) {
        let first = self
            .v
            .iter()
            .position(|block| block.id() == Some(id))
            .unwrap();

        let mut length = 0;
        while first + length < self.v.len() && self.v[first + length].id() == Some(id) {
            length += 1;
        }
        (first, length)
    }

    // Offset of free block at least blocks wide
    fn find_free(&self, blocks: usize) -> Option<usize> {
        let mut start = 0;
        let mut length = 0;
        let mut offset = 0;
        for block in self.v.iter() {
            offset += 1;
            match block.id() {
                None => {
                    length += 1;
                }
                Some(_) => {
                    start = offset;
                    length = 0;
                }
            }
            if length >= blocks {
                return Some(start);
            }
        }
        None
    }

    fn swap(&mut self, from: usize, to: usize, length: usize) {
        for k in 0..length {
            self.v.swap(from + k, to + k);
        }
    }

    fn defrag(&mut self) {
        let Some(mut id) = self.last_file_id() else {
            // No files, done
            return;
        };

        loop {
            let (from, len) = self.find_by_id(id);

            if let Some(to) = self.find_free(len) {
                if to < from {
                    self.swap(from, to, len);
                }
            }

            if id == 0 {
                break;
            }
            id -= 1;
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
