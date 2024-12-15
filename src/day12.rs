use history::map::Map;
use history::readfile;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Crop {
    #[default]
    Empty,
    Kind(u8),
}

impl history::map::Legend for Crop {
    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Crop::Empty,
            'A'..='Z' => Crop::Kind(ch as u8),
            _ => panic!("Unexpected symbol on map"),
        }
    }

    fn to_char(self) -> char {
        match self {
            Crop::Empty => '.',
            Crop::Kind(ch) => ch as char,
        }
    }
}

type Farm = Map<Crop>;
type Done = Map<bool>;

fn kind(farm: &Farm, done: &Done, x: isize, y: isize) -> (bool, Crop) {
    (
        done.read(x, y).unwrap_or_default(),
        farm.read(x, y).unwrap_or_default(),
    )
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let map: Farm = ctxt.value().parse().unwrap();

    let mut price = 0;
    let mut done: Done = Map::new();
    for y in map.y() {
        for x in map.x() {
            if done.read(x, y).unwrap_or_default() {
                continue;
            }
            let crop = map.read(x, y).unwrap_or_default();
            if crop == Crop::Empty {
                panic!("Somehow an empty Crop is in the farm at {x},{y}");
            }
            let mut edges: Vec<(isize, isize)> = Vec::new();
            let mut area = 0;
            let mut perimeter = 0;
            edges.push((x, y));
            while let Some((x, y)) = edges.pop() {
                if done.read(x, y).unwrap_or_default() {
                    continue;
                }
                area += 1;
                done.write(x, y, true);
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    match kind(&map, &done, x + dx, y + dy) {
                        (false, c) if c == crop => {
                            edges.push((x + dx, y + dy));
                        }
                        (true, c) if c == crop => (),
                        (_, _) => {
                            perimeter += 1;
                        }
                    }
                }
            }
            price += area * perimeter;
        }
    }
    println!("Total price of fencing all regions: {price}");
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let map: Farm = ctxt.value().parse().unwrap();

    let mut price = 0;
    let mut done: Done = Map::new();
    for y in map.y() {
        for x in map.x() {
            if done.read(x, y).unwrap_or_default() {
                continue;
            }
            let crop = map.read(x, y).unwrap_or_default();
            if crop == Crop::Empty {
                panic!("Somehow an empty Crop is in the farm at {x},{y}");
            }
            let mut edges: Vec<(isize, isize)> = Vec::new();
            let mut area = 0;
            let mut corners = 0;
            edges.push((x, y));
            while let Some((x, y)) = edges.pop() {
                if done.read(x, y).unwrap_or_default() {
                    continue;
                }
                area += 1;
                done.write(x, y, true);
                let w = kind(&map, &done, x - 1, y);
                let e = kind(&map, &done, x + 1, y);
                let n = kind(&map, &done, x, y - 1);
                let s = kind(&map, &done, x, y + 1);
                if !w.0 && w.1 == crop {
                    edges.push((x - 1, y));
                }
                if !e.0 && e.1 == crop {
                    edges.push((x + 1, y));
                }
                if !n.0 && n.1 == crop {
                    edges.push((x, y - 1));
                }
                if !s.0 && s.1 == crop {
                    edges.push((x, y + 1));
                }

                // If two adjacent sides touch our own crop but the diagonal does not, that's a
                // corner
                if w.1 == crop && n.1 == crop && map.read(x - 1, y - 1).unwrap_or_default() != crop
                {
                    corners += 1;
                }
                if n.1 == crop && e.1 == crop && map.read(x + 1, y - 1).unwrap_or_default() != crop
                {
                    corners += 1;
                }
                if e.1 == crop && s.1 == crop && map.read(x + 1, y + 1).unwrap_or_default() != crop
                {
                    corners += 1;
                }
                if s.1 == crop && w.1 == crop && map.read(x - 1, y + 1).unwrap_or_default() != crop
                {
                    corners += 1;
                }

                // If neither adjacent side is our own crop, that's a corner too
                if w.1 != crop && n.1 != crop {
                    corners += 1;
                }
                if n.1 != crop && e.1 != crop {
                    corners += 1;
                }
                if e.1 != crop && s.1 != crop {
                    corners += 1;
                }
                if s.1 != crop && w.1 != crop {
                    corners += 1;
                }
            }
            price += area * corners;
        }
    }
    println!("Bulk discounted price of fencing all regions: {price}");
}
