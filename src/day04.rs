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

fn check(words: &Search, x: isize, y: isize) -> usize {
    if words.read(x, y).is_some_and(|letter| letter != Letter::X) {
        return 0;
    }

    let mut count = 0;
    if words
        .read(x + 1, y)
        .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x + 2, y)
            .is_some_and(|letter| letter == Letter::A)
        && words
            .read(x + 3, y)
            .is_some_and(|letter| letter == Letter::S)
    {
        count += 1;
    }
    if words
        .read(x - 1, y)
        .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x - 2, y)
            .is_some_and(|letter| letter == Letter::A)
        && words
            .read(x - 3, y)
            .is_some_and(|letter| letter == Letter::S)
    {
        count += 1;
    }
    if words
        .read(x, y + 1)
        .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x, y + 2)
            .is_some_and(|letter| letter == Letter::A)
        && words
            .read(x, y + 3)
            .is_some_and(|letter| letter == Letter::S)
    {
        count += 1;
    }
    if words
        .read(x, y - 1)
        .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x, y - 2)
            .is_some_and(|letter| letter == Letter::A)
        && words
            .read(x, y - 3)
            .is_some_and(|letter| letter == Letter::S)
    {
        count += 1;
    }
    if words
        .read(x - 1, y - 1)
        .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x - 2, y - 2)
            .is_some_and(|letter| letter == Letter::A)
        && words
            .read(x - 3, y - 3)
            .is_some_and(|letter| letter == Letter::S)
    {
        count += 1;
    }
    if words
        .read(x + 1, y - 1)
        .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x + 2, y - 2)
            .is_some_and(|letter| letter == Letter::A)
        && words
            .read(x + 3, y - 3)
            .is_some_and(|letter| letter == Letter::S)
    {
        count += 1;
    }
    if words
        .read(x - 1, y + 1)
        .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x - 2, y + 2)
            .is_some_and(|letter| letter == Letter::A)
        && words
            .read(x - 3, y + 3)
            .is_some_and(|letter| letter == Letter::S)
    {
        count += 1;
    }
    if words
        .read(x + 1, y + 1)
        .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x + 2, y + 2)
            .is_some_and(|letter| letter == Letter::A)
        && words
            .read(x + 3, y + 3)
            .is_some_and(|letter| letter == Letter::S)
    {
        count += 1;
    }

    count
}

fn xmas(words: &Search, x: isize, y: isize) -> usize {
    if words.read(x, y).is_some_and(|letter| letter != Letter::A) {
        return 0;
    }

    let mut count = 0;
    if words
        .read(x - 1, y - 1)
        .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x - 1, y + 1)
            .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x + 1, y - 1)
            .is_some_and(|letter| letter == Letter::S)
        && words
            .read(x + 1, y + 1)
            .is_some_and(|letter| letter == Letter::S)
    {
        count += 1;
    }
    if words
        .read(x - 1, y - 1)
        .is_some_and(|letter| letter == Letter::S)
        && words
            .read(x - 1, y + 1)
            .is_some_and(|letter| letter == Letter::S)
        && words
            .read(x + 1, y - 1)
            .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x + 1, y + 1)
            .is_some_and(|letter| letter == Letter::M)
    {
        count += 1;
    }
    if words
        .read(x - 1, y - 1)
        .is_some_and(|letter| letter == Letter::S)
        && words
            .read(x - 1, y + 1)
            .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x + 1, y - 1)
            .is_some_and(|letter| letter == Letter::S)
        && words
            .read(x + 1, y + 1)
            .is_some_and(|letter| letter == Letter::M)
    {
        count += 1;
    }
    if words
        .read(x - 1, y - 1)
        .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x - 1, y + 1)
            .is_some_and(|letter| letter == Letter::S)
        && words
            .read(x + 1, y - 1)
            .is_some_and(|letter| letter == Letter::M)
        && words
            .read(x + 1, y + 1)
            .is_some_and(|letter| letter == Letter::S)
    {
        count += 1;
    }

    count
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
    let mut total = 0;
    for y in words.y() {
        for x in words.x() {
            total += xmas(&words, x, y);
        }
    }
    println!("{total} X-MAS found");
}
