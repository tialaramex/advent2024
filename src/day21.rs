use history::readfile;

type Num = i32;

fn numeric(s: &str) -> Num {
    let front = s
        .split(|c: char| !c.is_ascii_digit())
        .next()
        .expect("splitn always gives at least one item");
    if front.is_empty() {
        0
    } else {
        front
            .parse()
            .expect("should be a number as we didn't include non-digits")
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Corner {
    None,
    TopLeft,    // TopLeft caution: avoid dead corner, do D before L, R before U
    BottomLeft, // BottomLeft caution: avoid dead corner, do U before L, R before D
}

fn row_col_digit(digit: char) -> (u8, u8) {
    match digit {
        'A' => (3, 4),
        '0' => (2, 4),
        '1' => (1, 3),
        '2' => (2, 3),
        '3' => (3, 3),
        '4' => (1, 2),
        '5' => (2, 2),
        '6' => (3, 2),
        '7' => (1, 1),
        '8' => (2, 1),
        '9' => (3, 1),
        _ => panic!("{digit} is not on the keypad!"),
    }
}

type Move = (u8, u8, u8, u8, Corner);

fn move_to_strings(prefix: String, step: Move, answers: &mut Vec<String>) {
    let (u, d, l, r, corner) = step;
    match corner {
        Corner::None => {
            if l + r > 0 && u + d > 0 {
                let mut this = prefix.clone();
                match (l, r) {
                    (0, 0) => (),
                    (left, 0) => {
                        for _ in 0..left {
                            this.push('<');
                        }
                    }
                    (0, right) => {
                        for _ in 0..right {
                            this.push('>');
                        }
                    }
                    _ => panic!("Moving both left and right makes no sense"),
                }
                match (u, d) {
                    (0, 0) => (),
                    (up, 0) => {
                        for _ in 0..up {
                            this.push('^');
                        }
                    }
                    (0, down) => {
                        for _ in 0..down {
                            this.push('v');
                        }
                    }
                    _ => panic!("Moving both left and right makes no sense"),
                }
                this.push('A');
                answers.push(this);
            }
            let mut this = prefix;
            match (u, d) {
                (0, 0) => (),
                (up, 0) => {
                    for _ in 0..up {
                        this.push('^');
                    }
                }
                (0, down) => {
                    for _ in 0..down {
                        this.push('v');
                    }
                }
                _ => panic!("Moving both left and right makes no sense"),
            }
            match (l, r) {
                (0, 0) => (),
                (left, 0) => {
                    for _ in 0..left {
                        this.push('<');
                    }
                }
                (0, right) => {
                    for _ in 0..right {
                        this.push('>');
                    }
                }
                _ => panic!("Moving both left and right makes no sense"),
            }
            this.push('A');
            answers.push(this);
        }
        Corner::TopLeft => {
            let mut this = prefix;
            match (u, d, l, r) {
                (up, 0, 0, right) if up > 0 && right > 0 => {
                    for _ in 0..right {
                        this.push('>');
                    }
                    for _ in 0..up {
                        this.push('^');
                    }
                }
                (0, down, left, 0) if down > 0 && left > 0 => {
                    for _ in 0..down {
                        this.push('v');
                    }
                    for _ in 0..left {
                        this.push('<');
                    }
                }
                _ => unreachable!("To avoid TopLeft corner move R+U or D+L"),
            }
            this.push('A');
            answers.push(this);
        }
        Corner::BottomLeft => {
            let mut this = prefix;
            match (u, d, l, r) {
                (up, 0, left, 0) if up > 0 && left > 0 => {
                    for _ in 0..up {
                        this.push('^');
                    }
                    for _ in 0..left {
                        this.push('<');
                    }
                }
                (0, down, 0, right) if down > 0 && right > 0 => {
                    for _ in 0..right {
                        this.push('>');
                    }
                    for _ in 0..down {
                        this.push('v');
                    }
                }
                _ => unreachable!("To avoid BottomLeft corner move U+L or D+R"),
            }
            this.push('A');
            answers.push(this);
        }
    }
}

fn move_strings(moves: &[Move]) -> Vec<String> {
    let mut prefixes = vec![String::new()];
    for step in moves {
        let mut next = Vec::with_capacity(prefixes.len());
        for prefix in prefixes {
            move_to_strings(prefix, *step, &mut next);
        }
        prefixes = next;
    }
    prefixes
}

fn push_digits(s: &str) -> Vec<Move> {
    let mut v = Vec::with_capacity(s.len());
    let mut last = 'A';
    for c in s.chars() {
        v.push(push_digit(last, c));
        last = c;
    }
    v
}

// (u,d,l,r) then press A
// total moves always u + d + l + r + 1
fn push_digit(from: char, to: char) -> Move {
    let from = row_col_digit(from);
    let to = row_col_digit(to);
    let u = if from.1 > to.1 { from.1 - to.1 } else { 0 };
    let d = if from.1 < to.1 { to.1 - from.1 } else { 0 };
    let l = if from.0 > to.0 { from.0 - to.0 } else { 0 };

    let r = if from.0 < to.0 { to.0 - from.0 } else { 0 };

    let corner = match (from, to) {
        ((1, _), (_, 4)) => Corner::BottomLeft,
        ((_, 4), (1, _)) => Corner::BottomLeft,
        _ => Corner::None,
    };

    (u, d, l, r, corner)
}

fn push_dirs(s: &str) -> Vec<Move> {
    let mut v = Vec::with_capacity(s.len());
    let mut last = 'A';
    for c in s.chars() {
        v.push(push_dir(last, c));
        last = c;
    }
    v
}

// (u,d,l,r) then press A
// total moves always u + d + l + r + 1
fn push_dir(from: char, to: char) -> Move {
    match (from, to) {
        ('A', 'A') => (0, 0, 0, 0, Corner::None),
        ('A', '^') => (0, 0, 1, 0, Corner::None),
        ('A', '<') => (0, 1, 2, 0, Corner::TopLeft),
        ('A', 'v') => (0, 1, 1, 0, Corner::None),
        ('A', '>') => (0, 1, 0, 0, Corner::None),

        ('^', 'A') => (0, 0, 0, 1, Corner::None),
        ('^', '^') => (0, 0, 0, 0, Corner::None),
        ('^', '<') => (0, 1, 1, 0, Corner::TopLeft),
        ('^', 'v') => (0, 1, 0, 0, Corner::None),
        ('^', '>') => (0, 1, 0, 1, Corner::None),

        ('<', 'A') => (1, 0, 0, 2, Corner::TopLeft),
        ('<', '^') => (1, 0, 0, 1, Corner::TopLeft),
        ('<', '<') => (0, 0, 0, 0, Corner::None),
        ('<', 'v') => (0, 0, 0, 1, Corner::None),
        ('<', '>') => (0, 0, 0, 2, Corner::None),

        ('v', 'A') => (1, 0, 0, 1, Corner::None),
        ('v', '^') => (1, 0, 0, 0, Corner::None),
        ('v', '<') => (0, 0, 1, 0, Corner::None),
        ('v', 'v') => (0, 0, 0, 0, Corner::None),
        ('v', '>') => (0, 0, 0, 1, Corner::None),

        ('>', 'A') => (1, 0, 0, 0, Corner::None),
        ('>', '^') => (1, 0, 1, 0, Corner::None),
        ('>', '<') => (0, 0, 2, 0, Corner::None),
        ('>', 'v') => (0, 0, 1, 0, Corner::None),
        ('>', '>') => (0, 0, 0, 0, Corner::None),

        _ => panic!("Directional keypad doesn't have '{from}' and '{to}'"),
    }
}

fn shortest(s: &str) -> String {
    let mut best: Option<String> = None;
    let moves = push_digits(s);
    let strings = move_strings(&moves);
    for string in strings {
        let moves = push_dirs(&string);
        let strings = move_strings(&moves);
        for string in strings {
            let moves = push_dirs(&string);
            let strings = move_strings(&moves);
            for string in strings {
                if let Some(ref good) = best {
                    if good.len() > string.len() {
                        best = Some(string);
                    }
                } else {
                    best = Some(string);
                }
            }
        }
    }
    if let Some(input) = best {
        input
    } else {
        panic!("Somehow no possible inputs work");
    }
}

fn complexity(s: &str) -> Num {
    shortest(s)
        .len()
        .try_into()
        .expect("length of the shortest input should fit")
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let sum: Num = ctxt
        .lines()
        .map(|line| numeric(line) * complexity(line))
        .sum();
    println!("{sum}");
}

pub fn b(filename: &str) {
    let _ctxt = readfile(filename);
}
