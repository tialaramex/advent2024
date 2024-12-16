use history::map::Map;
use history::readfile;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Legend {
    #[default]
    Wall,
    Space,
    Start,
    End,
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
        }
    }
}

type Points = u32;
type Maze = Map<Legend>;
type Scores = Map<Option<Points>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn go(&self) -> (isize, isize) {
        match self {
            North => (0, -1),
            South => (0, 1),
            West => (-1, 0),
            East => (1, 0),
        }
    }

    fn turn_clock(self) -> Self {
        match self {
            North => East,
            South => West,
            West => North,
            East => South,
        }
    }

    fn turn_anti(self) -> Self {
        match self {
            North => West,
            South => East,
            West => South,
            East => North,
        }
    }
}

use Direction::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Reindeer {
    x: isize,
    y: isize,
    head: Direction,
    score: Points,
}

impl Reindeer {
    fn start((x, y): (isize, isize)) -> Self {
        Self {
            x,
            y,
            head: East,
            score: 0,
        }
    }
}

fn flood(map: &Maze) -> Option<Points> {
    let start = map.find(|p| p == Legend::Start)[0];
    let end = map.find(|p| p == Legend::End)[0];
    let init = Reindeer::start(start);
    let mut best: Scores = Map::new();
    let mut this: Vec<Reindeer> = Vec::new();
    this.push(init);
    let mut lowest: Option<Points> = None;
    loop {
        let mut next: Vec<Reindeer> = Vec::with_capacity(this.len());
        while let Some(mut deer) = this.pop() {
            if lowest.is_some_and(|p| p <= deer.score) {
                // This deer can't be a lowest scorer
                continue;
            }
            // We always either move or turn & move, never just turn, so score the move here
            deer.score += 1;
            let (dx, dy) = deer.head.go();
            match map.read(deer.x + dx, deer.y + dy).unwrap_or_default() {
                Legend::Space | Legend::End => {
                    let prev = best.read(deer.x + dx, deer.y + dy).unwrap_or_default();
                    if prev.is_none_or(|p| p > deer.score) {
                        let mut new = deer;
                        new.x += dx;
                        new.y += dy;
                        next.push(new);
                        best.write(new.x, new.y, Some(new.score));
                    }
                }
                _ => (),
            }
            // Maybe turn 90°
            deer.score += 1000;
            let clock = deer.head.turn_clock();
            let anti = deer.head.turn_anti();
            for d in [clock, anti] {
                let (dx, dy) = d.go();
                match map.read(deer.x + dx, deer.y + dy).unwrap_or_default() {
                    Legend::Space | Legend::End => {
                        let prev = best.read(deer.x + dx, deer.y + dy).unwrap_or_default();
                        if prev.is_none_or(|p| p > deer.score) {
                            let mut new = deer;
                            new.head = d;
                            new.x += dx;
                            new.y += dy;
                            next.push(new);
                            best.write(new.x, new.y, Some(new.score));
                        }
                    }
                    _ => (),
                }
            }
        }
        lowest = best.read(end.0, end.1).unwrap_or_default();
        if next.is_empty() {
            break;
        }
        this = next;
    }
    lowest
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let map: Maze = ctxt.value().parse().unwrap();
    let lowest = flood(&map).expect("there should be a route to the end");
    println!("Lowest score a reindeer could get is: {lowest}");
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct TrackedReindeer {
    x: isize,
    y: isize,
    head: Direction,
    score: Points,
    been: Vec<(isize, isize)>,
}

impl TrackedReindeer {
    fn start((x, y): (isize, isize)) -> Self {
        let been: Vec<(isize, isize)> = vec![(x, y)];
        Self {
            x,
            y,
            head: East,
            score: 0,
            been,
        }
    }
}

fn tiles(map: &Maze) -> usize {
    let start = map.find(|p| p == Legend::Start)[0];
    let end = map.find(|p| p == Legend::End)[0];
    let init = TrackedReindeer::start(start);
    let mut best: Scores = Map::new();
    let mut this: Vec<TrackedReindeer> = Vec::new();
    this.push(init);
    let mut lowest: Option<Points> = None;
    let mut routes: Vec<TrackedReindeer> = Vec::new();
    loop {
        let mut next: Vec<TrackedReindeer> = Vec::with_capacity(this.len());
        while let Some(mut deer) = this.pop() {
            if deer.x == end.0 && deer.y == end.1 {
                routes.push(deer);
                continue;
            }
            if lowest.is_some_and(|p| p <= deer.score) {
                // This deer can't be a lowest scorer
                continue;
            }
            // We always either move or turn & move, never just turn, so score the move here
            deer.score += 1;
            let (dx, dy) = deer.head.go();
            match map.read(deer.x + dx, deer.y + dy).unwrap_or_default() {
                Legend::Space | Legend::End => {
                    let prev = best.read(deer.x + dx, deer.y + dy).unwrap_or_default();
                    if prev.is_none_or(|p| p >= deer.score) {
                        let mut new = deer.clone();
                        new.x += dx;
                        new.y += dy;
                        new.been.push((new.x, new.y));
                        best.write(new.x, new.y, Some(new.score));
                        next.push(new);
                    } else {
                        // Maybe we're not turning here but others did so their score was lower but
                        // soon it won't be?
                        let mut new = deer.clone();
                        new.x += dx;
                        new.y += dy;
                        new.been.push((new.x, new.y));
                        next.push(new);
                    }
                }
                _ => (),
            }
            // Maybe turn 90°
            deer.score += 1000;
            let clock = deer.head.turn_clock();
            let anti = deer.head.turn_anti();
            for d in [clock, anti] {
                let (dx, dy) = d.go();
                match map.read(deer.x + dx, deer.y + dy).unwrap_or_default() {
                    Legend::Space | Legend::End => {
                        let prev = best.read(deer.x + dx, deer.y + dy).unwrap_or_default();
                        if prev.is_none_or(|p| p >= deer.score) {
                            let mut new = deer.clone();
                            new.head = d;
                            new.x += dx;
                            new.y += dy;
                            new.been.push((new.x, new.y));
                            best.write(new.x, new.y, Some(new.score));
                            next.push(new);
                        }
                    }
                    _ => (),
                }
            }
        }
        lowest = best.read(end.0, end.1).unwrap_or_default();
        if next.is_empty() {
            break;
        }
        this = next;
    }
    if let Some(distance) = lowest {
        let mut good: Map<bool> = Map::new();
        for deer in routes {
            if deer.score == distance {
                for (x, y) in deer.been {
                    good.write(x, y, true);
                }
            }
        }
        good.count(|&&b| b)
    } else {
        // Too bad, no routes to the end
        0
    }
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let map: Maze = ctxt.value().parse().unwrap();
    let count = tiles(&map);
    println!("{count} tiles are part of at least one of the best paths");
}
