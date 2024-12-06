use history::map::Map;
use history::readfile;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Legend {
    #[default]
    Edge,
    Empty,
    Guard,
    Obstacle,
    Path(u8),
}

impl From<char> for Legend {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Empty,
            '^' => Self::Guard,
            '#' => Self::Obstacle,
            _ => panic!("Unexpected symbol on map"),
        }
    }
}

type Lab = Map<Legend>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(self) -> Self {
        use Direction::*;

        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn go(&self) -> (isize, isize) {
        use Direction::*;

        match self {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
        }
    }
}

fn loops(lab: &mut Lab) -> bool {
    let guards = lab.find(|l| l == Legend::Guard);
    assert_eq!(guards.len(), 1);
    let (mut x, mut y) = guards[0];
    // Now that we know where the guard was, remove from map
    lab.write(x, y, Legend::Empty);

    use Direction::*;
    let mut d = North;

    loop {
        let mv = d.go();
        match lab.read(x + mv.0, y + mv.1) {
            None | Some(Legend::Edge) => {
                lab.write(x, y, Legend::Path(0));
                return false;
            }
            Some(Legend::Empty) => {
                lab.write(x, y, Legend::Path(1));
                x += mv.0;
                y += mv.1;
            }
            Some(Legend::Path(prev)) => {
                if prev >= 4 {
                    return true;
                }
                lab.write(x, y, Legend::Path(prev + 1));
                x += mv.0;
                y += mv.1;
            }
            Some(Legend::Obstacle) => {
                d = d.turn();
            }
            Some(item) => panic!("Unexpected item on map {item:?}"),
        }
    }
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut lab: Lab = ctxt
        .value()
        .parse()
        .expect("should be a map of the suit lab");
    assert!(!loops(&mut lab)); // The guard does not loop
    let count = lab.count(|&&l| matches!(l, Legend::Path(_)));
    println!("Guard visits {count} distinct locations before leaving");
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let lab: Lab = ctxt
        .value()
        .parse()
        .expect("should be a map of the suit lab");
    let guards = lab.find(|l| l == Legend::Guard);
    assert_eq!(guards.len(), 1);
    let (gx, gy) = guards[0];
    let mut preview = lab.clone();
    assert!(!loops(&mut preview)); // The guard does not loop
    let possible = preview.find(|l| matches!(l, Legend::Path(_)));

    let mut places = 0;
    for (x, y) in possible {
        if x != gx || y != gy {
            let mut attempt = lab.clone();
            attempt.write(x, y, Legend::Obstacle);
            if loops(&mut attempt) {
                places += 1;
            }
        }
    }
    println!("Obstruction could go in {places} different places");
}