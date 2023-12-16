// https://adventofcode.com/2023/day/16
use crate::library::{
    containers::grid::{Grid, Position},
    utility,
};
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Beam {
    pub dir: Direction,
    pub pos: Position,
}

impl Beam {
    pub fn new(dir: Direction, pos: Position) -> Self {
        Self { dir, pos }
    }

    // Get the next position of the beam, make sure to check if it's out of bounds.
    pub fn next(&self, grid: &Grid) -> Option<Position> {
        let new_position = match self.dir {
            Direction::Up => {
                if self.pos.y > 0 {
                    Position::new(self.pos.x, self.pos.y - 1)
                } else {
                    return None;
                }
            }
            Direction::Down => Position::new(self.pos.x, self.pos.y + 1),
            Direction::Left => {
                if self.pos.x > 0 {
                    Position::new(self.pos.x - 1, self.pos.y)
                } else {
                    return None;
                }
            }
            Direction::Right => Position::new(self.pos.x + 1, self.pos.y),
        };

        if grid.is_in_bounds(new_position) {
            Some(new_position)
        } else {
            None
        }
    }
}

// This function will take a grid and a beam, and raytrace the beam until it hits a wall or goes out of bounds, then return the amount of tiles it passed through.
pub fn raytrace(grid: &Grid, starting_beam: &Beam) -> i32 {
    // We want to basically just follow each beam until it hits a point where it can't go any further, that is out of bounds.
    let mut beams: Vec<Beam> = Vec::new();

    // Get the first character in the grid, and based off what it is, we create a beam going in the correct direction.
    beams.push(starting_beam.clone());

    // Create a HashSet to store the beams we've already visited, and their points, since the path will be identical.
    let mut visited: HashSet<Beam> = HashSet::new();

    // Keep going until the Vec is empty.
    while !beams.is_empty() {
        // Get the next beam from the Vec.
        let beam = beams.pop().unwrap();

        // If we've already had a beam with this path and position, we can skip it.
        if visited.contains(&beam) {
            continue;
        } else {
            visited.insert(beam);
        }

        // Need to get the next character in the grid according to the beam's position, if we can.
        // Else we can skip this beam.
        if let Some(next_pos) = beam.next(grid) {
            let ch = grid.get_pos(next_pos);
            match ch {
                Some('.') => {
                    // If the next character is a '.', we can keep going in the same direction.
                    beams.push(Beam::new(beam.dir, next_pos));
                }
                Some('/') => {
                    // Change direction based on the current direction.
                    let dir = match beam.dir {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    beams.push(Beam::new(dir, next_pos));
                }
                Some('\\') => {
                    // Change direction based on the current direction.
                    let dir = match beam.dir {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    beams.push(Beam::new(dir, next_pos));
                }
                Some('|') => {
                    // This is a splitter, if the beam comes from left or right, we split it into two beams, one going up and one going down.
                    // If the beam comes from up or down we let it continue in the same direction.
                    match beam.dir {
                        Direction::Left => {
                            beams.push(Beam::new(Direction::Up, next_pos));
                            beams.push(Beam::new(Direction::Down, next_pos));
                        }
                        Direction::Right => {
                            beams.push(Beam::new(Direction::Up, next_pos));
                            beams.push(Beam::new(Direction::Down, next_pos));
                        }
                        _ => {
                            beams.push(Beam::new(beam.dir, next_pos));
                        }
                    }
                }
                Some('-') => {
                    // This is a splitter, if the beam comes from up or down, we split it into two beams, one going left and one going right.
                    // If the beam comes from left or right we let it continue in the same direction.
                    match beam.dir {
                        Direction::Up => {
                            beams.push(Beam::new(Direction::Left, next_pos));
                            beams.push(Beam::new(Direction::Right, next_pos));
                        }
                        Direction::Down => {
                            beams.push(Beam::new(Direction::Left, next_pos));
                            beams.push(Beam::new(Direction::Right, next_pos));
                        }
                        _ => {
                            beams.push(Beam::new(beam.dir, next_pos));
                        }
                    }
                }
                None => {
                    // If the next character is None, we've hit the end of the grid, so we can skip this beam.
                    continue;
                }
                _ => todo!("This will never be reached, but it's here to make the compiler happy."),
            }
        }
    }

    // Count the number of beams that are in the visited HashSet, with unique positions.
    visited
        .iter()
        .map(|b| b.pos)
        .collect::<HashSet<Position>>()
        .len() as i32
}

pub fn solve(data: &Grid) -> (i32, i32) {
    // Get the first character in the grid, and based off what it is, we create a beam going in the correct direction.
    let ch = data.get_pos(Position::new(0, 0)).unwrap();

    let beam = match ch {
        '|' | '\\' => Beam::new(Direction::Down, Position::new(0, 0)),
        '-' | '.' => Beam::new(Direction::Right, Position::new(0, 0)),
        '/' => Beam::new(Direction::Up, Position::new(0, 0)),
        _ => todo!("This will never be reached, but it's here to make the compiler happy."),
    };

    let p1 = raytrace(data, &beam);

    // For part 2 we need to find the maximum amount of tiles that can be reached by a beam, so we need to try all possible starting directions and positions from the edges of the grid.
    // So iterate thru all points on west edge, and go right, then iterate thru all points on the north edge, and go down, etc.

    // Create a vector to hold all the beams we want to raytrace.
    let mut beams: Vec<Beam> = Vec::new();

    // Iterating through the top and bottom rows
    for x in 0..data.width() {
        beams.push(Beam::new(Direction::Down, Position::new(x, 0)));
        beams.push(Beam::new(
            Direction::Up,
            Position::new(x, data.height() - 1),
        ));
    }

    // Iterating through the left and right columns
    for y in 0..data.height() {
        beams.push(Beam::new(Direction::Right, Position::new(0, y)));
        beams.push(Beam::new(
            Direction::Left,
            Position::new(data.width() - 1, y),
        ));
    }

    // Iterate through all the beams, and raytrace them, and keep track of the maximum amount of tiles we've seen.
    let p2 = beams
        .par_iter()
        .map(|beam| raytrace(data, beam))
        .reduce(|| 0, |max, tiles_seen| std::cmp::max(max, tiles_seen));

    (p1, p2)
}

pub fn parse(data: &[String]) -> Grid {
    let rows = data.iter().map(|s| s.to_string()).collect();
    Grid::from_rows(rows, data[0].len())
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&utility::files::read_file("data/day16.txt")));
    println!("Day 16:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day16.txt"));
    c.bench_function("Day 16 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 16 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day16.txt"));
            solve(&data)
        })
    });
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part1() {
        let expected = 46;
        let res = solve(&parse(&utility::files::read_file("testdata/day16.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 51;
        let res = solve(&parse(&utility::files::read_file("testdata/day16.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
