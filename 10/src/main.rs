use std::ops::{Add, Sub};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const NORTH: Coord = Coord { x: 0, y: 1 };
const SOUTH: Coord = Coord { x: 0, y: -1 };
const EAST: Coord = Coord { x: 1, y: 0 };
const WEST: Coord = Coord { x: -1, y: 0 };

/// Structure for building a valid pipe
#[derive(Debug, Clone)]
struct Sketch<S> {
    sketch: Vec<Vec<S>>,
    pipe: Vec<Coord>,
}

/// X, Y coordinate tuple
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
/// Enumerates the types of pipes in a sketch.
enum Pipe {
    /// Vertical pipe connecting north and south.
    Vertical = b'|',
    /// Horizontal pipe connecting east and west.
    Horizontal = b'-',
    /// 90-degree bend connecting north and east.
    NE90 = b'L',
    /// 90-degree bend connecting north and west.
    NW90 = b'J',
    /// 90-degree bend connecting south and west.
    SW90 = b'7',
    /// 90-degree bend connecting south and east.
    SE90 = b'F',
    /// Ground; there is no pipe in this tile.
    Ground = b'.',
    /// Starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    Start = b'S',
}

impl Pipe {
    fn from(c: char) -> Result<Pipe, &'static str> {
        match c {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::NE90),
            'J' => Ok(Pipe::NW90),
            '7' => Ok(Pipe::SW90),
            'F' => Ok(Pipe::SE90),
            '.' => Ok(Pipe::Ground),
            'S' => Ok(Pipe::Start),
            _ => Err("invalid character"),
        }
    }

    fn next(&self, prev: Coord, current: Coord) -> Coord {
        match *self {
            Pipe::Vertical => match prev - current {
                NORTH => current + SOUTH,
                SOUTH => current + NORTH,
                _ => panic!("transition not defined"),
            },
            Pipe::Horizontal => match prev - current {
                EAST => current + WEST,
                WEST => current + EAST,
                _ => panic!("transition not defined"),
            },
            Pipe::NE90 => match prev - current {
                NORTH => current + EAST,
                EAST => current + NORTH,
                _ => panic!("transition not defined"),
            },
            Pipe::NW90 => match prev - current {
                NORTH => current + WEST,
                WEST => current + NORTH,
                _ => panic!("transition not defined"),
            },
            Pipe::SW90 => match prev - current {
                SOUTH => current + WEST,
                WEST => current + SOUTH,
                _ => panic!("transition not defined"),
            },
            Pipe::SE90 => match prev - current {
                SOUTH => current + EAST,
                EAST => current + SOUTH,
                _ => panic!("transition not defined"),
            },
            _ => panic!("transition not defined"),
        }
    }
}

impl Sketch<Pipe> {
    fn new(sketch: Vec<Vec<Pipe>>, start: Coord) -> Self {
        Sketch {
            sketch,
            pipe: vec![start],
        }
    }

    fn get(&self, coord: Coord) -> Pipe {
        self.sketch[coord.y as usize][coord.x as usize]
    }
    fn start(&self) -> Coord {
        self.pipe[0]
    }

    fn current_position(&self) -> Coord {
        self.pipe[self.pipe.len() - 1]
    }

    fn previous_position(&self) -> Coord {
        match self.pipe.len() {
            1 => self.pipe[0],
            _ => self.pipe[self.pipe.len() - 2],
        }
    }
}

/// parse_sketch returns the sketch and the starting position of the pipe
fn parse_sketch(input: &str) -> Sketch<Pipe> {
    let mut sketch: Vec<Vec<_>> = Vec::new();
    let mut start: Option<Coord> = None;
    for line in input.lines() {
        sketch.insert(0, line.chars().map(|c| Pipe::from(c).unwrap()).collect())
    }

    for (y, row) in sketch.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if *pipe == Pipe::Start {
                start = Some(Coord {
                    x: x as isize,
                    y: y as isize,
                });
                break;
            }
        }
    }

    match start {
        None => panic!("no start found"),
        _ => Sketch::new(sketch, start.unwrap()),
    }
}

fn part1(input: &str) -> usize {
    let mut sketch = parse_sketch(input);

    // Check every direction around the starting position and pick the first valid one
    for direction in vec![NORTH, SOUTH, EAST, WEST] {
        let pos = sketch.current_position() + direction;
        match (direction, sketch.get(pos)) {
            (NORTH, Pipe::Vertical | Pipe::SE90 | Pipe::SW90) => {
                sketch.pipe.push(pos);
                break;
            }
            (SOUTH, Pipe::Vertical | Pipe::NE90 | Pipe::NW90) => {
                sketch.pipe.push(pos);
                break;
            }
            (EAST, Pipe::Horizontal | Pipe::SW90 | Pipe::NW90) => {
                sketch.pipe.push(pos);
                break;
            }
            (WEST, Pipe::Horizontal | Pipe::SE90 | Pipe::NE90) => {
                sketch.pipe.push(pos);
                break;
            }
            _ => {}
        }
    }

    // Check to see if we found a valid connecting pipe.
    if sketch.current_position() == sketch.start() {
        panic!("no valid routes from the start")
    }

    // Follow the pipe until we reach the start.
    while sketch.current_position() != sketch.start() {
        let pipe = sketch.get(sketch.current_position());
        let next = pipe.next(sketch.previous_position(), sketch.current_position());
        sketch.pipe.push(next);
    }

    (sketch.pipe.len() - 1) / 2
}

fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static INPUT: &str = indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ"
    };

    #[test]
    fn check_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 8)
    }

    #[test]
    fn check_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 0)
    }
}
