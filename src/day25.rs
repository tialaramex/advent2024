use history::readfile;

type Pins = [u8; 5];

#[derive(Clone, Debug)]
enum Schematic {
    Key(Pins),
    Lock(Pins),
}

fn parse<'t, T>(lines: &mut T) -> Schematic
where
    T: Iterator<Item = &'t str>,
{
    let Some(first) = lines.next() else {
        panic!("Should be a schematic");
    };
    let mut array: [u8; 5] = [0, 0, 0, 0, 0];
    let initial = match first {
        "#####" => '#',
        "....." => '.',
        _ => {
            panic!("Schematic should show a key ..... or a lock  ##### not {first}");
        }
    };
    for depth in 1..=5 {
        let Some(pins) = lines.next() else {
            panic!("Schematics should have seven lines in total");
        };
        for (n, pin) in pins.chars().enumerate() {
            if pin == initial {
                array[n] = depth;
            }
        }
    }
    assert!(lines.next().is_some());
    match initial {
        '#' => Schematic::Lock(array),
        '.' => {
            for pin in array.iter_mut() {
                *pin = 5 - *pin;
            }
            Schematic::Key(array)
        }
        _ => unreachable!("Should be either a lock or a key"),
    }
}

fn fit(key: &Pins, lock: &Pins) -> bool {
    lock.iter().zip(key).all(|(lock, key)| lock + key < 6)
}

pub fn a(filename: &str) {
    let ctxt = readfile(filename);
    let mut lines = ctxt.lines();

    let mut locks: Vec<Pins> = Vec::new();
    let mut keys: Vec<Pins> = Vec::new();

    loop {
        match parse(&mut lines) {
            Schematic::Lock(lock) => {
                locks.push(lock);
            }
            Schematic::Key(key) => {
                keys.push(key);
            }
        }
        if lines.next().is_none() {
            break;
        }
    }
    let mut count = 0;
    for key in keys {
        for lock in locks.iter() {
            if fit(&key, lock) {
                count += 1;
            }
        }
    }
    println!("{count} keys fit");
}

pub fn b(_filename: &str) {
    println!("Happy Christmas!");
}
