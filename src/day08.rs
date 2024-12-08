use history::map::Map;
use history::readfile;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Legend {
    #[default]
    Outside,
    Empty,
    Antenna(char),
}

impl From<char> for Legend {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Empty,
            '0'..='9' | 'a'..='z' | 'A'..='Z' => Self::Antenna(ch),
            _ => panic!("Unexpected symbol on map"),
        }
    }
}

type Bunny = Map<Legend>;
type Anti = Map<bool>;

fn antinode(map: &Bunny, anti: &mut Anti, x: isize, y: isize) {
    match map.read(x, y).unwrap_or_default() {
        Legend::Empty | Legend::Antenna(_) => {
            anti.write(x, y, true);
        }
        _ => (),
    }
}

fn check(map: &Bunny, anti: &mut Anti, ch: char) {
    let mut pos = map.find(|c| c == Legend::Antenna(ch));
    while let Some((ax, ay)) = pos.pop() {
        for (bx, by) in pos.iter() {
            antinode(map, anti, 2 * ax - bx, 2 * ay - by);
            antinode(map, anti, 2 * bx - ax, 2 * by - ay);
        }
    }
}

fn find_anti_nodes(map: &Bunny) -> Anti {
    let mut anti: Anti = Map::new();
    for symbol in ('0'..='9').chain('a'..='z').chain('A'..='Z') {
        check(map, &mut anti, symbol);
    }
    anti
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let map: Bunny = ctxt
        .value()
        .parse()
        .expect("should be a map of the antennas");
    let anti = find_anti_nodes(&map);
    let count = anti.count(|&&n| n);
    println!("{count} unique locations contain an antinode within the map");
}

fn resonant(map: &Bunny, anti: &mut Anti, diameter: isize, ch: char) {
    let mut pos = map.find(|c| c == Legend::Antenna(ch));
    while let Some((ax, ay)) = pos.pop() {
        for (bx, by) in pos.iter() {
            for n in 0..=diameter {
                antinode(map, anti, ax + n * (ax - bx), ay + n * (ay - by));
                antinode(map, anti, bx + n * (bx - ax), by + n * (by - ay));
            }
        }
    }
}
fn consider_resonance(map: &Bunny) -> Anti {
    let (ax, bx) = map.x().into_inner();
    let (ay, by) = map.y().into_inner();
    let width = 1 + bx - ax;
    let height = 1 + by - ay;
    let diameter = core::cmp::max(width, height);
    let mut anti: Anti = Map::new();
    for symbol in ('0'..='9').chain('a'..='z').chain('A'..='Z') {
        resonant(map, &mut anti, diameter, symbol);
    }
    anti
}
pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let map: Bunny = ctxt
        .value()
        .parse()
        .expect("should be a map of the antennas");
    let anti = consider_resonance(&map);
    let count = anti.count(|&&n| n);
    println!("With resonance, {count} unique locations contain an antinode within the map");
}
