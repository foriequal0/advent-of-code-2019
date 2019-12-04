use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

type BoxedError = Box<dyn Error + 'static>;
type Result<T> = std::result::Result<T, BoxedError>;

enum Direction {
    R,
    L,
    D,
    U,
}

struct Segment {
    direction: Direction,
    distance: u32,
}

impl FromStr for Segment {
    type Err = BoxedError;

    fn from_str(s: &str) -> Result<Segment> {
        let direction = match s.chars().nth(0).ok_or("No direction")? {
            'R' => Direction::R,
            'L' => Direction::L,
            'D' => Direction::D,
            'U' => Direction::U,
            _ => unreachable!(),
        };
        let distance = s.get(1..).ok_or("No distance")?.parse::<u32>()?;
        Ok(Segment {
            direction,
            distance,
        })
    }
}

fn get_wires<R: BufRead>(read: R) -> Result<(Vec<Segment>, Vec<Segment>)> {
    fn get_wire(line: &str) -> Result<Vec<Segment>> {
        line.split(',').map(Segment::from_str).collect()
    }

    let mut lines = read.lines();
    let line1 = lines.next().expect("No first line")?;
    let line2 = lines.next().expect("No second line")?;

    Ok((get_wire(&line1)?, (get_wire(&line2)?)))
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Default)]
struct Cursor {
    x: i32,
    y: i32,
}

struct WireIterator<'a> {
    cursor: Cursor,
    segments: &'a [Segment],
    distance: u32,
}

impl<'a> WireIterator<'a> {
    fn new(segments: &'a [Segment]) -> Self {
        Self {
            segments,
            cursor: Default::default(),
            distance: 0,
        }
    }
}

impl<'a> Iterator for WireIterator<'a> {
    type Item = Cursor;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.segments.get(0)?;
        match current.direction {
            Direction::R => {
                self.cursor.x += 1;
            }
            Direction::L => {
                self.cursor.x -= 1;
            }
            Direction::D => {
                self.cursor.y += 1;
            }
            Direction::U => {
                self.cursor.y -= 1;
            }
        }
        self.distance += 1;
        if current.distance == self.distance {
            self.segments = &self.segments[1..];
            self.distance = 0;
        }
        Some(self.cursor)
    }
}

fn find_closest_intersection(wire1: Vec<Segment>, wire2: Vec<Segment>) -> Result<i32> {
    let mut buf = HashSet::new();
    for cursor in WireIterator::new(&wire1) {
        buf.insert(cursor);
    }

    let mut min_distance = std::i32::MAX;
    for cursor in WireIterator::new(&wire2) {
        if buf.contains(&cursor) {
            min_distance = min_distance.min(cursor.x.abs() + cursor.y.abs())
        }
    }
    Ok(min_distance)
}

fn run<R: BufRead>(read: R) -> Result<i32> {
    let (line1, line2) = get_wires(read)?;
    find_closest_intersection(line1, line2)
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let result = run(stdin.lock())?;
    println!("result: {}", result);
    Ok(())
}

#[test]
fn test() {
    let input = include_bytes!("../../input/03");
    assert_eq!(run(&input[..]).ok(), Some(375));
}
