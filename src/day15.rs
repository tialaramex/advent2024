use history::map::Map;
use history::readfile;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Legend {
    #[default]
    Wall,
    Space,
    Crate,
    Robot,
}

impl history::map::Legend for Legend {
    fn from_char(ch: char) -> Self {
        match ch {
            '#' => Self::Wall,
            '.' => Self::Space,
            'O' => Self::Crate,
            '@' => Self::Robot,
            _ => panic!("Unexpected symbol on map"),
        }
    }

    fn to_char(self) -> char {
        match self {
            Self::Wall => '#',
            Self::Space => '.',
            Self::Crate => 'O',
            Self::Robot => '@',
        }
    }
}

type Warehouse = Map<Legend>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn go(&self) -> (isize, isize) {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

use Direction::*;

fn check(map: &Warehouse, mut x: isize, mut y: isize, dir: Direction) -> Option<(isize, isize)> {
    use Legend::*;
    let (dx, dy) = dir.go();
    loop {
        x += dx;
        y += dy;
        match map.read(x, y).unwrap_or_default() {
            Wall => return None,
            Space => return Some((x, y)),
            Crate => (),
            Robot => panic!("No other Robots should be in the warehouse"),
        }
    }
}

fn shove(map: &mut Warehouse, mut x: isize, mut y: isize, dir: Direction) -> (isize, isize) {
    use Legend::*;
    debug_assert_eq!(map.read(x, y), Some(Robot));
    if let Some((fx, fy)) = check(map, x, y, dir) {
        let (dx, dy) = dir.go();
        map.write(x, y, Space);
        x += dx;
        y += dy;
        map.write(x, y, Robot);
        let robot = (x, y);
        while x != fx || y != fy {
            x += dx;
            y += dy;
            map.write(x, y, Crate);
        }
        return robot;
    }
    (x, y)
}

use std::ops::RangeInclusive;

fn gps(wide: &RangeInclusive<isize>, tall: &RangeInclusive<isize>, x: isize, y: isize) -> isize {
    let x = x - wide.start();
    let y = y - tall.start();
    y * 100 + x
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let (map, rest) = ctxt
        .value()
        .split_once("\n\n")
        .expect("should have a map and a list of instructions");
    let mut map: Warehouse = map.parse().unwrap();

    let (mut x, mut y) = map.find(|r| r == Legend::Robot)[0];
    for ch in rest.chars() {
        match ch {
            '^' => {
                (x, y) = shove(&mut map, x, y, Up);
            }
            'v' => {
                (x, y) = shove(&mut map, x, y, Down);
            }
            '<' => {
                (x, y) = shove(&mut map, x, y, Left);
            }
            '>' => {
                (x, y) = shove(&mut map, x, y, Right);
            }
            '\n' => (),
            _ => panic!("Unexpected {ch} in instruction stream"),
        }
    }
    let mut sum = 0;
    let wide = map.x();
    let tall = map.y();
    for (x, y) in map.find(|p| p == Legend::Crate) {
        sum += gps(&wide, &tall, x, y);
    }
    println!("Sum of all boxes GPS co-ordinates was: {sum}");
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum BigLegend {
    #[default]
    Wall,
    Space,
    LCrate,
    RCrate,
    Robot,
}

impl history::map::Legend for BigLegend {
    fn from_char(ch: char) -> Self {
        match ch {
            '#' => Self::Wall,
            '.' => Self::Space,
            '[' => Self::LCrate,
            ']' => Self::RCrate,
            '@' => Self::Robot,
            _ => panic!("Unexpected symbol on map"),
        }
    }

    fn to_char(self) -> char {
        match self {
            Self::Wall => '#',
            Self::Space => '.',
            Self::LCrate => '[',
            Self::RCrate => ']',
            Self::Robot => '@',
        }
    }
}

type BigWarehouse = Map<BigLegend>;

fn scale(from: Warehouse) -> BigWarehouse {
    let old_range = from.x();
    let lx = *old_range.start();
    let rx = *old_range.end();
    let mut to: BigWarehouse = Map::ranged(lx..=(rx * 3), from.y());
    for y in from.y() {
        for x in from.x() {
            match from.read(x, y) {
                Some(Legend::Wall) => {
                    to.write(x * 2, y, BigLegend::Wall);
                    to.write(x * 2 + 1, y, BigLegend::Wall);
                }
                Some(Legend::Space) => {
                    to.write(x * 2, y, BigLegend::Space);
                    to.write(x * 2 + 1, y, BigLegend::Space);
                }
                Some(Legend::Crate) => {
                    to.write(x * 2, y, BigLegend::LCrate);
                    to.write(x * 2 + 1, y, BigLegend::RCrate);
                }
                Some(Legend::Robot) => {
                    to.write(x * 2, y, BigLegend::Robot);
                    to.write(x * 2 + 1, y, BigLegend::Space);
                }
                None => panic!("Within range we should not see None in map reads"),
            }
        }
    }
    to
}

fn big_horiz_check(
    map: &BigWarehouse,
    mut x: isize,
    y: isize,
    dir: Direction,
) -> Option<(isize, isize)> {
    use BigLegend::*;
    let (dx, _) = dir.go();
    loop {
        x += dx;
        match map.read(x, y).unwrap_or_default() {
            Wall => return None,
            Space => return Some((x, y)),
            LCrate => (),
            RCrate => (),
            Robot => panic!("No other Robots should be in the warehouse"),
        }
    }
}

fn attempt_vert(map: &mut BigWarehouse, x: isize, oy: isize, dy: isize) -> bool {
    use BigLegend::*;
    if map.read(x, oy + dy).unwrap_or_default() == Space {
        map.write(x, oy, Space);
        map.write(x, oy + dy, Robot);
        return true;
    }

    // First, check if we can shove up
    let mut crates: Vec<isize> = Vec::new();
    let mut y = oy;
    crates.push(x);
    loop {
        y += dy;
        let mut next: Vec<isize> = Vec::with_capacity(crates.len());
        while let Some(x) = crates.pop() {
            match map.read(x, y).unwrap_or_default() {
                Wall => {
                    return false;
                }
                Space => {
                    // We can move the crate below into this space
                }
                LCrate => {
                    next.push(x);
                    // also push RCrate
                    next.push(x + 1);
                }
                RCrate => {
                    // also push LCrate
                    next.push(x - 1);
                    next.push(x);
                }
                Robot => panic!("No other Robots should be in the warehouse"),
            }
        }
        if next.is_empty() {
            break;
        }
        next.sort_unstable();
        next.dedup();
        crates = next;
    }

    // Then, shove
    let mut crates: Vec<(isize, BigLegend)> = Vec::new();
    let mut y = oy;
    crates.push((x, Robot));
    map.write(x, y, Space);
    loop {
        y += dy;
        let mut next: Vec<(isize, BigLegend)> = Vec::with_capacity(crates.len());
        for &(x, _) in crates.iter() {
            match map.read(x, y).unwrap_or_default() {
                Robot => panic!("No other Robots should be in the warehouse"),
                Wall => panic!("Should not find a Wall during vertical shove"),
                Space => {
                    // We can move the crate below into this space if was real
                }
                LCrate => {
                    next.push((x, LCrate));
                    // also push RCrate
                    debug_assert_eq!(map.read(x + 1, y), Some(RCrate));
                    next.push((x + 1, RCrate));
                    map.write(x + 1, y, Space);
                }
                RCrate => {
                    next.push((x, RCrate));
                    // also push LCrate
                    debug_assert_eq!(map.read(x - 1, y), Some(LCrate));
                    next.push((x - 1, LCrate));
                    map.write(x - 1, y, Space);
                }
            }
        }
        // Overwrite anything we're moving
        for &(x, thing) in crates.iter() {
            map.write(x, y, thing);
        }
        if next.is_empty() {
            break;
        }
        crates = next;
    }

    true
}

fn big_shove(map: &mut BigWarehouse, mut x: isize, y: isize, dir: Direction) -> (isize, isize) {
    use BigLegend::*;
    debug_assert_eq!(map.read(x, y), Some(Robot));
    match dir {
        Left => {
            if let Some((fx, _)) = big_horiz_check(map, x, y, dir) {
                map.write(x, y, Space);
                x -= 1;
                map.write(x, y, Robot);
                let robot = (x, y);
                while x != fx {
                    map.write(x - 2, y, LCrate);
                    map.write(x - 1, y, RCrate);
                    x -= 2;
                }
                robot
            } else {
                (x, y)
            }
        }
        Right => {
            if let Some((fx, _)) = big_horiz_check(map, x, y, dir) {
                map.write(x, y, Space);
                x += 1;
                map.write(x, y, Robot);
                let robot = (x, y);
                while x != fx {
                    map.write(x + 1, y, LCrate);
                    map.write(x + 2, y, RCrate);
                    x += 2;
                }
                robot
            } else {
                (x, y)
            }
        }
        Up => {
            if attempt_vert(map, x, y, -1) {
                (x, y - 1)
            } else {
                (x, y)
            }
        }
        Down => {
            if attempt_vert(map, x, y, 1) {
                (x, y + 1)
            } else {
                (x, y)
            }
        }
    }
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let (map, rest) = ctxt
        .value()
        .split_once("\n\n")
        .expect("should have a map and a list of instructions");
    let small: Warehouse = map.parse().unwrap();
    let mut map = scale(small);
    let (mut x, mut y) = map.find(|r| r == BigLegend::Robot)[0];
    for ch in rest.chars() {
        match ch {
            '^' => {
                (x, y) = big_shove(&mut map, x, y, Up);
            }
            'v' => {
                (x, y) = big_shove(&mut map, x, y, Down);
            }
            '<' => {
                (x, y) = big_shove(&mut map, x, y, Left);
            }
            '>' => {
                (x, y) = big_shove(&mut map, x, y, Right);
            }
            '\n' => (),
            _ => panic!("Unexpected {ch} in instruction stream"),
        }
    }

    let mut sum = 0;
    let wide = map.x();
    let tall = map.y();
    for (x, y) in map.find(|p| p == BigLegend::LCrate) {
        sum += gps(&wide, &tall, x, y);
    }
    println!("In wider warehouse, sum of all boxes GPS co-ordinates was: {sum}");
}
