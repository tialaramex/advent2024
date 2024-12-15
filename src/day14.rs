use history::readfile;

type Num = i32;

#[derive(Copy, Clone, Debug)]
struct Robot {
    x: Num,
    y: Num,
    vx: Num,
    vy: Num,
}

use std::str::FromStr;
impl FromStr for Robot {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s
            .split_once(' ')
            .ok_or("Robots should have position and velocity")?;
        let p = p.strip_prefix("p=").ok_or("Positions should begin p=")?;
        let (x, y) = p
            .split_once(',')
            .ok_or("Positions should be x,y separated by a comma")?;
        let x: Num = x
            .parse()
            .map_err(|_| "Numbers should fit in the agreed type")?;
        let y: Num = y
            .parse()
            .map_err(|_| "Numbers should fit in the agreed type")?;
        let v = v.strip_prefix("v=").ok_or("Velocities should begin v=")?;
        let (vx, vy) = v
            .split_once(',')
            .ok_or("Velocities should be x,y separated by a comma")?;
        let vx: Num = vx
            .parse()
            .map_err(|_| "Numbers should fit in the agreed type")?;
        let vy: Num = vy
            .parse()
            .map_err(|_| "Numbers should fit in the agreed type")?;
        Ok(Robot { x, y, vx, vy })
    }
}

const WIDTH: Num = 101;
const LENGTH: Num = 103;

#[derive(Copy, Clone, Debug)]
enum Quadrant {
    Middle,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Robot {
    fn apply(z: Num, vz: Num, max: Num, seconds: Num) -> Num {
        let mut p = z;
        for _ in 0..seconds {
            p += vz;
            if p < 0 {
                p += max;
            } else if p >= max {
                p -= max;
            }
        }
        p
    }

    fn simulate(&mut self, seconds: Num) {
        self.x = Robot::apply(self.x, self.vx, WIDTH, seconds);
        self.y = Robot::apply(self.y, self.vy, LENGTH, seconds);
    }

    fn quadrant(&self) -> Quadrant {
        use std::cmp::Ordering::*;
        use Quadrant::*;
        match (self.x.cmp(&(WIDTH / 2)), self.y.cmp(&(LENGTH / 2))) {
            (Less, Less) => TopLeft,
            (Less, Greater) => TopRight,
            (Greater, Less) => BottomLeft,
            (Greater, Greater) => BottomRight,
            (_, _) => Middle,
        }
    }
}

fn safety(robots: &[Robot]) -> Num {
    use Quadrant::*;

    let mut tl = 0;
    let mut tr = 0;
    let mut bl = 0;
    let mut br = 0;

    for r in robots {
        match r.quadrant() {
            TopLeft => tl += 1,
            TopRight => tr += 1,
            BottomLeft => bl += 1,
            BottomRight => br += 1,
            _ => (),
        }
    }

    tl * tr * bl * br
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut v: Vec<Robot> = Vec::new();
    for line in ctxt.lines() {
        let mut r: Robot = line.parse().expect("Should be a robot");
        r.simulate(100);
        v.push(r);
    }
    let safe = safety(&v);
    println!("Safety factor after 100 seconds is: {safe}");
}

use history::map::Map;

#[allow(dead_code)] // Just for diagnostics
fn display(robots: &[Robot]) {
    let mut grid: Map<char> = Map::new();
    for y in 0..LENGTH {
        for x in 0..WIDTH {
            grid.write(x as isize, y as isize, '.');
        }
    }
    for r in robots {
        grid.write(r.x as isize, r.y as isize, '*');
    }
    println!("{grid}");
}

fn entropy(robots: &[Robot]) -> Num {
    let mx = if robots.is_empty() {
        0
    } else {
        let tx: Num = robots.iter().map(|r| r.x).sum();
        tx / robots.len() as Num
    };
    let my = if robots.is_empty() {
        0
    } else {
        let ty: Num = robots.iter().map(|r| r.y).sum();
        ty / robots.len() as Num
    };
    let dx: Num = robots.iter().map(|r| (r.x - mx).abs()).sum();
    let dy: Num = robots.iter().map(|r| (r.y - my).abs()).sum();
    dx + dy
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let mut v: Vec<Robot> = Vec::new();
    for line in ctxt.lines() {
        let r: Robot = line.parse().expect("Should be a robot");
        v.push(r);
    }

    // try WIDTH * LENGTH simulations
    let mut best: Option<(Num, Num)> = None;
    for step in 1..=(WIDTH * LENGTH) {
        for r in v.iter_mut() {
            r.simulate(1);
        }
        let entropy = entropy(&v);
        if let Some((best_entropy, _)) = best {
            if best_entropy > entropy {
                best = Some((entropy, step));
            }
        } else {
            best = Some((entropy, step));
        }
    }
    if let Some((_, step)) = best {
        println!("Robots display the Xmas tree after {step} seconds");
    }
}
