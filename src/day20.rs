use history::map::Map;
use history::readfile;

type Distance = u16;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Legend {
    #[default]
    Wall,
    Space,
    Start,
    End,
    Route(Distance),
}

impl history::map::Legend for Legend {
    fn from_char(ch: char) -> Self {
        match ch {
            '#' => Self::Wall,
            '.' => Self::Space,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!("Unexpected symbol on map"),
        }
    }

    fn to_char(self) -> char {
        match self {
            Self::Wall => '#',
            Self::Space => '.',
            Self::Start => 'S',
            Self::End => 'E',
            Self::Route(_) => '*',
        }
    }
}

type Maze = Map<Legend>;

fn obvious(map: &mut Maze) {
    let start = map.find(|p| p == Legend::Start)[0];
    let end = map.find(|p| p == Legend::End)[0];

    let mut d: Distance = 0;
    let mut pos = start;
    while pos != end {
        map.write(pos.0, pos.1, Legend::Route(d));
        let left = map.read(pos.0 - 1, pos.1).unwrap_or_default();
        let right = map.read(pos.0 + 1, pos.1).unwrap_or_default();
        let up = map.read(pos.0, pos.1 - 1).unwrap_or_default();
        let down = map.read(pos.0, pos.1 + 1).unwrap_or_default();
        d += 1;
        match (left, right, up, down) {
            (Legend::Space | Legend::End, _, _, _) => {
                pos.0 -= 1;
            }
            (_, Legend::Space | Legend::End, _, _) => {
                pos.0 += 1;
            }
            (_, _, Legend::Space | Legend::End, _) => {
                pos.1 -= 1;
            }
            (_, _, _, Legend::Space | Legend::End) => {
                pos.1 += 1;
            }
            _ => panic!("Ran out of maze without finding exit"),
        }
    }
    map.write(pos.0, pos.1, Legend::Route(d));
}

fn skip(a: Distance, b: Distance, v: &mut Vec<Distance>) {
    let d = a.abs_diff(b);
    if d > 2 {
        v.push(d - 2);
    }
}

fn cheats(map: &Maze) -> Vec<Distance> {
    let mut v: Vec<Distance> = Vec::new();
    for wall in map.find(|p| p == Legend::Wall) {
        let left = map.read(wall.0 - 1, wall.1).unwrap_or_default();
        let right = map.read(wall.0 + 1, wall.1).unwrap_or_default();
        let up = map.read(wall.0, wall.1 - 1).unwrap_or_default();
        let down = map.read(wall.0, wall.1 + 1).unwrap_or_default();
        if let Legend::Route(a) = left {
            if let Legend::Route(b) = right {
                skip(a, b, &mut v);
            }
            if let Legend::Route(b) = up {
                skip(a, b, &mut v);
            }
            if let Legend::Route(b) = down {
                skip(a, b, &mut v);
            }
        }
        if let Legend::Route(a) = right {
            if let Legend::Route(b) = up {
                skip(a, b, &mut v);
            }
            if let Legend::Route(b) = down {
                skip(a, b, &mut v);
            }
        }
        if let Legend::Route(a) = up {
            if let Legend::Route(b) = down {
                skip(a, b, &mut v);
            }
        }
    }
    v
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut map: Maze = ctxt.value().parse().unwrap();
    obvious(&mut map);
    let options = cheats(&map);
    let count = options.into_iter().filter(|&d| d >= 100).count();
    println!("{count} cheats would save at least 100 picoseconds");
}

fn longer_skip(a: Distance, b: Distance, d: Distance) -> Option<Distance> {
    if a > b + d {
        Some(a - b - d)
    } else {
        None
    }
}

fn longer_cheats(map: &Maze) -> Vec<Distance> {
    let mut v: Vec<Distance> = Vec::new();
    for from in map.find(|p| matches!(p, Legend::Route(_))) {
        let fx = from.0;
        let fy = from.1;
        let from = map.read(fx, fy).unwrap_or_default();
        let Legend::Route(a) = from else {
            panic!("How did we end up not on the route?");
        };
        for x in (fx - 20)..=(fx + 20) {
            let w = fx.abs_diff(x) as isize;
            for y in (fy + w - 20)..=(fy + 20 - w) {
                let to = map.read(x, y).unwrap_or_default();
                if let Legend::Route(b) = to {
                    if let Some(distance) =
                        longer_skip(b, a, (fx.abs_diff(x) + fy.abs_diff(y)) as Distance)
                    {
                        v.push(distance);
                    }
                }
            }
        }
    }
    v
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut map: Maze = ctxt.value().parse().unwrap();
    obvious(&mut map);
    let options = longer_cheats(&map);
    let count = options.into_iter().filter(|&d| d >= 100).count();
    println!("{count} of the (optionally longer) cheats would save at least 100 picoseconds");
}
