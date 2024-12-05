use history::map::Map;
use history::readfile;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Letter {
    #[default]
    Other,
    X,
    M,
    A,
    S,
}

impl From<char> for Letter {
    fn from(ch: char) -> Self {
        match ch {
            'X' => Self::X,
            'M' => Self::M,
            'A' => Self::A,
            'S' => Self::S,
            _ => Self::Other,
        }
    }
}

type Search = Map<Letter>;

fn check_dir(words: &Search, x: isize, y: isize, lr: isize, ud: isize) -> bool {
    words.read(x + lr, y + ud).is_some_and(|l| l == Letter::M)
        && words
            .read(x + lr + lr, y + ud + ud)
            .is_some_and(|l| l == Letter::A)
        && words
            .read(x + lr + lr + lr, y + ud + ud + ud)
            .is_some_and(|l| l == Letter::S)
}

fn check(words: &Search, x: isize, y: isize) -> usize {
    if words.read(x, y).is_some_and(|letter| letter != Letter::X) {
        return 0;
    }

    let mut count = 0;
    for (dx, dy) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        if check_dir(words, x, y, dx, dy) {
            count += 1;
        }
    }
    count
}

fn cross(words: &Search, x: isize, y: isize) -> (Letter, Letter, Letter, Letter) {
    let tl = words.read(x - 1, y - 1).unwrap_or(Letter::Other);
    let tr = words.read(x + 1, y - 1).unwrap_or(Letter::Other);
    let bl = words.read(x - 1, y + 1).unwrap_or(Letter::Other);
    let br = words.read(x + 1, y + 1).unwrap_or(Letter::Other);
    (tl, br, tr, bl)
}

fn xmas(words: &Search, x: isize, y: isize) -> bool {
    if words.read(x, y).is_some_and(|letter| letter != Letter::A) {
        return false;
    }

    matches!(
        cross(words, x, y),
        (Letter::M, Letter::S, Letter::M, Letter::S)
            | (Letter::S, Letter::M, Letter::M, Letter::S)
            | (Letter::M, Letter::S, Letter::S, Letter::M)
            | (Letter::S, Letter::M, Letter::S, Letter::M)
    )
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let words: Search = ctxt.value().parse().expect("should be a word search");
    let mut total = 0;
    for y in words.y() {
        for x in words.x() {
            total += check(&words, x, y);
        }
    }
    println!("{total} XMAS found");
}

pub fn b(filename: &str) {
    let ctxt = readfile(filename);
    let words: Search = ctxt.value().parse().expect("should be a word search");
    let mut count = 0;
    for y in words.y() {
        for x in words.x() {
            if xmas(&words, x, y) {
                count += 1;
            }
        }
    }
    println!("{count} X-MAS found");
}
