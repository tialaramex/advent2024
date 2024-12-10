use history::map::Map;
use history::readfile;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Elevation(Option<u8>);

impl From<char> for Elevation {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Elevation(None),
            '0'..='9' => Elevation(Some(ch.to_digit(10).unwrap().try_into().unwrap())),
            _ => panic!("Unexpected symbol on map"),
        }
    }
}

type Trails = Map<Elevation>;

fn score(map: &Trails, x: isize, y: isize, unique: bool) -> u32 {
    let mut camp: Vec<(isize, isize)> = Vec::new();
    camp.push((x, y));
    for digit in 1..=9 {
        let mut next = Vec::with_capacity(camp.len());
        while let Some((x, y)) = camp.pop() {
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if map.read(x + dx, y + dy) == Some(Elevation(Some(digit))) {
                    next.push((x + dx, y + dy));
                }
            }
        }
        if unique {
            next.sort_unstable();
            next.dedup();
        }
        camp = next;
    }

    camp.len() as u32
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let map: Trails = ctxt.value().parse().unwrap();
    let mut sum = 0;
    for (x, y) in map.find(|loc| loc == Elevation(Some(0))) {
        sum += score(&map, x, y, true);
    }
    println!("Sum of scores is: {sum}");
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let map: Trails = ctxt.value().parse().unwrap();
    let mut sum = 0;
    for (x, y) in map.find(|loc| loc == Elevation(Some(0))) {
        sum += score(&map, x, y, false);
    }
    println!("Sum of ratings is: {sum}");
}
