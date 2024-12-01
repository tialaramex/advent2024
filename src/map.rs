use std::ops::RangeInclusive;

#[derive(Copy, Clone, Debug)]
struct Plane {
    size: isize,
    offset: isize,
    start: isize, // Inclusive
    end: isize,   // Inclusive
}

impl Plane {
    fn from_to(from: isize, to: isize) -> Self {
        if to < from {
            panic!("{from} to {to} is not reasonable for defining a Plane");
        }
        let width = to - from;
        let mid = from + (width / 2);
        Self {
            size: width,
            offset: from,
            start: mid,
            end: mid,
        }
    }

    // Never shrink either end of the range, which might otherwise happen where Map::rect creates
    // large uninitialised Maps
    fn expand(&self) -> Self {
        const GROWTH: isize = 8;

        let offset = if self.offset < self.start - GROWTH {
            self.offset
        } else {
            self.start - GROWTH
        };
        let size = if self.size > self.end - offset + GROWTH {
            self.size
        } else {
            self.end - offset + GROWTH // When actually growing this ends up adding GROWTH at both edges
        };
        Self {
            size,
            offset,
            start: self.start,
            end: self.end,
        }
    }
}

#[derive(Clone)]
pub struct Map<T: Copy + Default> {
    data: Vec<T>,
    x: Plane,
    y: Plane,
}

impl<T: Copy + Default> Default for Map<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A type for 2D maps of unknown expanses, the backing store automatically grows as necessary
/// Map<T> implements Debug and/or Display if they are implemented for T to conveniently show the
/// map
impl<T: Copy + Default> Map<T> {
    /// Map a Rectangle initially from (x1, y1) to (x2, y2) but it will grow automatically as
    /// necessary
    pub fn rect((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> Self {
        if x2 < x1 {
            panic!("{x1} must be less than or equal to {x2}");
        }
        let x = Plane::from_to(x1, x2);

        if y2 < y1 {
            panic!("{y1} must be less than or equal to {y2}");
        }
        let y = Plane::from_to(y1, y2);

        let size = (x.size * y.size) as usize;
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, Default::default);
        Self { data, x, y }
    }

    pub fn ranged(x: RangeInclusive<isize>, y: RangeInclusive<isize>) -> Self {
        let (x1, x2) = x.into_inner();
        let (y1, y2) = y.into_inner();
        Self::rect((x1, y1), (x2, y2))
    }

    pub fn new() -> Self {
        Self::rect((-8, -8), (8, 8))
    }

    fn inbound(&self, x: isize, y: isize) -> bool {
        x >= self.x.offset
            && x < (self.x.offset + self.x.size)
            && y >= self.y.offset
            && y < (self.y.offset + self.y.size)
    }

    /// Grow Map by suitably expanding both planes and re-allocating, then copying
    fn grow(&mut self) {
        let new_x = self.x.expand();
        let new_y = self.y.expand();

        let size = (new_x.size * new_y.size) as usize;
        let mut new_data = Vec::with_capacity(size);
        new_data.resize_with(size, Default::default);

        let off_x = self.x.offset - new_x.offset;
        let off_y = self.y.offset - new_y.offset;

        // Perform copy
        for y in 0..self.y.size {
            for x in 0..self.x.size {
                let from = y * self.x.size + x;
                let dest = (y + off_y) * new_x.size + (x + off_x);
                new_data[dest as usize] = self.data[from as usize];
            }
        }

        self.data = new_data;
        self.x = new_x;
        self.y = new_y;
    }

    fn include(&mut self, x: isize, y: isize) {
        if x < self.x.start {
            self.x.start = x;
        } else if x > self.x.end {
            self.x.end = x;
        }
        if y < self.y.start {
            self.y.start = y;
        } else if y > self.y.end {
            self.y.end = y;
        }
        if !self.inbound(x, y) {
            self.grow();
        }
    }

    /// Range of X values, it is possible that this range includes some "dead" space
    /// but if the Map was built by parsing a string, this will be the exact size
    pub fn x(&self) -> RangeInclusive<isize> {
        self.x.start..=self.x.end
    }

    /// Range of Y values, it is possible that this range includes some "dead" space
    /// but if the Map was built by parsing a string, this will be the exact size
    pub fn y(&self) -> RangeInclusive<isize> {
        self.y.start..=self.y.end
    }

    fn noitisop(&self, i: usize) -> (isize, isize) {
        let y = self.y.offset + (i as isize / self.x.size);
        let x = self.x.offset + (i as isize % self.x.size);
        (x, y)
    }

    fn position(&self, x: isize, y: isize) -> usize {
        let posn = (y - self.y.offset) * self.x.size + (x - self.x.offset);
        posn as usize
    }

    /// Write to (x, y) in the Map, this will grow the map automatically
    pub fn write(&mut self, x: isize, y: isize, value: T) {
        self.include(x, y);
        let posn = self.position(x, y);
        self.data[posn] = value;
    }

    /// Reads an (x, y) position on the Map, but can be None if that position wasn't yet mapped
    /// Use or(value) or or_else(function) if appropriate
    pub fn read(&self, x: isize, y: isize) -> Option<T> {
        if self.inbound(x, y) {
            Some(self.data[self.position(x, y)])
        } else {
            None
        }
    }

    /// Count how many of the mapped positions match the predicate
    pub fn count<P>(&self, predicate: P) -> usize
    where
        P: FnMut(&&T) -> bool,
    {
        self.data.iter().filter(predicate).count()
    }

    /// Obtain a Vec of (x, y) positions matching the predicate
    pub fn find<P>(&self, predicate: P) -> Vec<(isize, isize)>
    where
        P: Fn(T) -> bool,
    {
        let mut v = Vec::new();

        for (i, &value) in self.data.iter().enumerate() {
            if predicate(value) {
                let (x, y) = self.noitisop(i);
                v.push((x, y));
            }
        }
        v
    }
}

use std::fmt;
impl<T: fmt::Debug + Copy + Default> fmt::Debug for Map<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "x: [ {} {}..={} {} ] ",
            self.x.offset,
            self.x.start,
            self.x.end,
            self.x.offset + self.x.size
        ))?;
        f.write_fmt(format_args!(
            "y: [ {} {}..={} {} ]\n",
            self.y.offset,
            self.y.start,
            self.y.end,
            self.y.offset + self.y.size
        ))?;
        let from_y = self.y.start - self.y.offset;
        let from_x = self.x.start - self.x.offset;
        let to_y = self.y.size - (self.y.offset + self.y.size - self.y.end);
        let to_x = self.x.size - (self.x.offset + self.x.size - self.x.end);
        if self.data.is_empty() {
            return Ok(());
        }
        for row in from_y..=to_y {
            for col in from_x..=to_x {
                let posn = row * self.x.size + col;
                let s = format!("{:?}", self.data[posn as usize]);
                f.write_str(&s)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl<T: fmt::Display + Copy + Default> fmt::Display for Map<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from_y = self.y.start - self.y.offset;
        let from_x = self.x.start - self.x.offset;
        let to_y = self.y.size - (self.y.offset + self.y.size - self.y.end);
        let to_x = self.x.size - (self.x.offset + self.x.size - self.x.end);
        if self.data.is_empty() {
            return Ok(());
        }
        for row in from_y..=to_y {
            for col in from_x..=to_x {
                let posn = row * self.x.size + col;
                let s = format!("{}", self.data[posn as usize]);
                f.write_str(&s)?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

use std::convert::Infallible;
use std::str::FromStr;
impl<T> FromStr for Map<T>
where
    char: Into<T>,
    T: Copy + Default,
{
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Self::rect((0, 0), (0, 0));
        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let item = ch.into();
                map.write(col as isize, row as isize, item);
            }
        }
        Ok(map)
    }
}

#[cfg(test)]

mod tests {
    use crate::map::Map;

    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    enum Maze {
        #[default]
        Wall,
        Space,
        Us,
    }

    const MAZE: &str = include_str!("test-map.txt");

    impl From<char> for Maze {
        fn from(ch: char) -> Self {
            match ch {
                '#' => Maze::Wall,
                ' ' => Maze::Space,
                _ => panic!("Impossible"),
            }
        }
    }

    #[test]
    fn default_map() {
        let map: Map<u8> = Default::default();
        assert_eq!(map.count(|&i| i == &1), 0);
    }

    #[test]
    fn maze_size() {
        let map: Map<Maze> = MAZE.parse().unwrap();
        assert_eq!(map.x(), 0..=8);
        assert_eq!(map.y(), 0..=6);
    }

    #[test]
    fn maze_write() {
        let mut map: Map<Maze> = MAZE.parse().unwrap();
        assert_eq!(map.count(|&m| m == &Maze::Space), 23);
        map.write(1, 1, Maze::Us);
        assert_eq!(map.count(|&m| m == &Maze::Space), 22);
    }
}
