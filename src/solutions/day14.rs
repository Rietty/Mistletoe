// https://adventofcode.com/2023/day/14
use crate::library::{containers::grid::Grid, utility};
use std::collections::{hash_map::Entry, HashMap};

const TOTAL_CYCLES: i32 = 1_000_000_000;

// This function will tilt the grid to the left.
// Thereby letting all the rocks that can move, move to the left.
fn tilt_left(grid: &mut Grid) {
    // Go thru each row, and for every 'O' move it as far left as it can go.
    // It only stops when it hits a '#', the edge of the grid or another 'O'.
    let mut rows = grid.rows();
    for row in rows.iter_mut() {
        for i in 0..row.len() {
            if row[i] == 'O' {
                for j in (0..i).rev() {
                    if row[j] == '#' || row[j] == 'O' {
                        break;
                    } else {
                        row.swap(j, j + 1);
                    }
                }
            }
        }
    }
    // Put the rows back into the grid.
    // Need to go from Vec<Vec<char>> to Vec<String>.
    *grid = Grid::from_rows(
        rows.iter().map(|row| row.iter().collect()).collect(),
        grid.get_width(),
    );
}

fn tilt_right(grid: &mut Grid) {
    grid.flip_y();
    tilt_left(grid);
    grid.flip_y();
}

fn tilt_up(grid: &mut Grid) {
    grid.transpose();
    tilt_left(grid);
    grid.transpose();
}

fn tilt_down(grid: &mut Grid) {
    grid.transpose();
    tilt_right(grid);
    grid.transpose();
}

fn cycle(grid: &mut Grid) {
    // Tilt the grid in all directions.
    tilt_up(grid);
    tilt_left(grid);
    tilt_down(grid);
    tilt_right(grid);
}

fn load(grid: &Grid) -> i32 {
    grid.columns().iter().fold(0, |acc, s| {
        acc + s.iter().rev().enumerate().fold(0, |acc, (i, c)| {
            acc + if c == &'O' { i as i32 + 1 } else { 0 }
        })
    })
}

pub fn solve(data: &Grid) -> (i32, i32) {
    // Create a copy of the grid for part 1.
    let mut grid = data.clone();
    tilt_up(&mut grid);
    let p1 = load(&grid);

    // Create a copy of the grid for part 2.
    let mut grid = data.clone();

    // Next section was shamelessly stolen from Lesley Lai.
    // I used it when I was rewriting my Grid class into it's own thing so a lot of the ideas are the same.
    let mut table = HashMap::new();
    let mut i: usize = 0;

    let remaining = loop {
        match table.entry(grid.clone()) {
            Entry::Vacant(v) => {
                v.insert(i);
            }
            Entry::Occupied(ref o) => {
                let repetition_iterations = i - o.get();
                break (TOTAL_CYCLES - i as i32) % repetition_iterations as i32;
            }
        }

        cycle(&mut grid);
        i += 1;
    };

    for _ in 0..remaining {
        cycle(&mut grid);
    }

    let p2 = load(&grid);

    (p1, p2)
}

pub fn parse(data: &[String]) -> Grid {
    let width = data.first().map_or(0, |s| s.len());
    let grid = data.iter().flat_map(|s| s.chars()).collect();
    Grid::new(grid, width)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&utility::files::read_file("data/day14.txt")));
    println!("Day 14:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day14.txt"));
    c.bench_function("Day 14 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 14 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day14.txt"));
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
        let expected = 136;
        let res = solve(&parse(&utility::files::read_file("testdata/day14.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 64;
        let res = solve(&parse(&utility::files::read_file("testdata/day14.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
