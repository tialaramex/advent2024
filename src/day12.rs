use history::map::Map;
use history::readfile;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Crop {
    #[default]
    Empty,
    Kind(u8),
}

impl From<char> for Crop {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Crop::Empty,
            'A'..='Z' => Crop::Kind(ch as u8),
            _ => panic!("Unexpected symbol on map"),
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

fn measure(fence: &[(isize, isize, isize, isize)]) -> usize {
    let mut length = 0;
    let mut prev: Option<(isize, isize, isize, isize)> = None;
    for (dx, dy, x, y) in fence {
        if let Some((odx, ody, ox, oy)) = prev {
            if odx != *dx || ody != *dy || ox != *x || oy + 1 != *y {
                length += 1;
            }
        } else {
            length += 1;
        }
        prev = Some((*dx, *dy, *x, *y));
    }
    length
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
            let mut fence: Vec<(isize, isize, isize, isize)> = Vec::new();
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
                            if dx == 0 {
                                fence.push((dx, dy, y, x));
                            } else {
                                fence.push((dx, dy, x, y));
                            }
                        }
                    }
                }
            }
            fence.sort_unstable();
            let perimeter = measure(&fence);
            price += area * perimeter;
        }
    }
    println!("Bulk discounted price of fencing all regions: {price}");
}
