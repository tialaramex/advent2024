use history::map::Map;
use history::readfile;
use history::State;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Byte {
    #[default]
    Corrupted,
    Empty,
}

impl history::map::Legend for Byte {
    fn from_char(ch: char) -> Self {
        match ch {
            '#' => Self::Corrupted,
            '.' => Self::Empty,
            _ => panic!("Unexpected '{ch}"),
        }
    }

    fn to_char(self) -> char {
        match self {
            Self::Corrupted => '#',
            Self::Empty => '.',
        }
    }
}

type Memory = Map<Byte>;

const EXIT: (isize, isize) = (70, 70);
const STOP: usize = 1024;

fn init() -> Memory {
    let mut map = Map::new();
    for y in 0..=EXIT.1 {
        for x in 0..=EXIT.0 {
            map.write(x, y, Byte::Empty);
        }
    }
    map
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Historians {
    x: isize,
    y: isize,
}

impl State<Memory> for Historians {
    fn describe(&self, _map: &Memory) -> String {
        format!("{x}.{y}", x = self.x, y = self.y)
    }

    fn next(&self, map: &Memory) -> Vec<Self> {
        let mut v = Vec::new();
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            if map.read(self.x + dx, self.y + dy).unwrap_or_default() == Byte::Empty {
                v.push(Historians {
                    x: self.x + dx,
                    y: self.y + dy,
                });
            }
        }
        v
    }
}

fn read_coords(line: &str) -> (isize, isize) {
    let (x, y) = line.split_once(',').expect("should be two co-ordinates");
    let x: isize = x.parse().expect("X should be a number");
    let y: isize = y.parse().expect("Y should be a number");
    if x > EXIT.0 || y > EXIT.1 {
        panic!("Inputs for real data maybe? Check EXIT and STOP constants");
    }
    (x, y)
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut lines = ctxt.lines();
    let mut map: Memory = init();
    for _ in 0..STOP {
        let line = lines.next().expect("should be a line of input");
        let (x, y) = read_coords(line);
        map.write(x, y, Byte::Corrupted);
    }
    let start = Historians { x: 0, y: 0 };
    let end = Historians {
        x: EXIT.0,
        y: EXIT.1,
    };

    if let Some(steps) = State::best(start, end, &map) {
        println!("At least {steps} steps to reach the exit");
    } else {
        println!("Impossible to reach end after {STOP} bytes");
    }
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut lines = ctxt.lines();
    let mut map: Memory = init();
    for _ in 0..STOP {
        let line = lines.next().expect("should be a line of input");
        let (x, y) = line.split_once(',').expect("should be two co-ordinates");
        let x: isize = x.parse().expect("X should be a number");
        let y: isize = y.parse().expect("Y should be a number");
        if x > EXIT.0 || y > EXIT.1 {
            panic!("Inputs for real data maybe? Check EXIT and STOP constants");
        }
        map.write(x, y, Byte::Corrupted);
    }
    for line in lines {
        let (x, y) = read_coords(line);
        map.write(x, y, Byte::Corrupted);

        let start = Historians { x: 0, y: 0 };
        let end = Historians {
            x: EXIT.0,
            y: EXIT.1,
        };

        if State::best(start, end, &map).is_none() {
            println!("Co-ordinates of the first byte to make the exit unreachable: {line}");
            return;
        }
    }
}
