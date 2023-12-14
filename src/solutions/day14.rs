// https://adventofcode.com/2023/day/14
use rayon::prelude::*;
use std::collections::HashMap;

const TOTAL_CYCLES: i32 = 1_000_000_000;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Grid {
    grid: Vec<char>,
    width: usize,
}

impl Grid {
    fn new(grid: Vec<char>, width: usize) -> Grid {
        Grid { grid, width }
    }

    fn new_from_row_vec(grid: Vec<String>, width: usize) -> Grid {
        Grid::new(grid.iter().flat_map(|s| s.chars()).collect(), width)
    }

    fn new_from_col_vec(grid: Vec<String>, width: usize) -> Grid {
        let mut chars = Vec::with_capacity(grid.len() * width);
        for i in 0..width {
            for j in 0..grid.len() {
                chars.push(grid[j].chars().nth(i).unwrap());
            }
        }
        Grid::new(chars, width)
    }

    fn rows(&self) -> Vec<String> {
        self.grid
            .chunks(self.width)
            .map(|chunk| chunk.iter().collect())
            .collect()
    }

    fn columns(&self) -> Vec<String> {
        (0..self.width)
            .map(|i| {
                self.grid[i..]
                    .chunks(self.width)
                    .map(|chunk| chunk.get(0).unwrap())
                    .collect()
            })
            .collect()
    }

    fn correct_orientation(&mut self) {
        // Correct orientation by reversing the order of the rows, and in each row, reverse the order of the characters.
        let rows = self.rows();
        self.grid = rows
            .iter()
            .rev()
            .flat_map(|s| s.chars().rev())
            .collect::<Vec<_>>();
    }

    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..self.grid.len() {
            print!("{}", self.grid[i]);
            if (i + 1) % self.width == 0 {
                println!();
            }
        }
    }
}

// Will move the rocks to the left-most edge of the given String and return the new String.
pub fn shift_rocks(mirrors: &String) -> String {
    // Each row is a String consisting of '.', '#' and 'O'. We want to move the 'O' to the left-most position it can go.
    // So going from left to right, we'll move the 'O' to the left as long as there is a '.' to the left of it.
    // If we reach a '#' or 'O' to its left, we'll stop moving it.
    let mut chars = mirrors.chars().collect::<Vec<_>>();
    let mut pos = 0;

    while pos < chars.len() {
        if chars[pos] == 'O' {
            let mut current = pos;
            while current > 0 && chars[current - 1] == '.' {
                chars.swap(current, current - 1);
                current -= 1;
            }
        }
        pos += 1;
    }

    chars.into_iter().collect()
}

// We need to cycle the rocks in the grid, that means they all shift north, then west, then south, then east.
// In order to do this we can go from grid -> columns -> back to grid -> rows -> back to grid -> columns (reversed) -> back to grid -> rows (reversed) -> back to grid.
pub fn cycle_rocks(grid: &mut Grid, count: usize) {
    for _ in 0..count {
        shift_and_update_grid(grid, true, false); // North
        shift_and_update_grid(grid, false, false); // West
        shift_and_update_grid(grid, true, true); // South
        shift_and_update_grid(grid, false, true); // East

        // Need to fix the orientation of the grid after each cycle.
        grid.correct_orientation();
    }
}

fn shift_and_update_grid(grid: &mut Grid, is_col: bool, reverse: bool) {
    let elements = if is_col { grid.columns() } else { grid.rows() };
    let shifted = elements
        .par_iter()
        .map(|e| {
            let e = if reverse {
                e.chars().rev().collect()
            } else {
                e.clone()
            };
            shift_rocks(&e)
        })
        .collect::<Vec<_>>();

    if is_col {
        *grid = Grid::new_from_col_vec(shifted, grid.width);
    } else {
        *grid = Grid::new_from_row_vec(shifted, grid.width);
    }
}

pub fn calculate_score(shifted: &Grid) -> i32 {
    // Score is calculated based off the length of the String and the positions of the 'O's.
    // For example if the String is "OOO....", the score is 7+6+5 = 18.
    // If the String is "O.O.O.O", the score is 7+5+3+1 = 16.
    shifted.columns().iter().fold(0, |acc, s| {
        acc + s.chars().rev().enumerate().fold(0, |acc, (i, c)| {
            acc + if c == 'O' { i as i32 + 1 } else { 0 }
        })
    })
}

pub fn solve(data: &Grid) -> (i32, i32) {
    // For the grid, get the columns, and then for each column, shift the rocks, then print them out.
    let cols = data.columns();

    // Shift the rocks in each column.
    let shifted = cols
        .par_iter()
        .map(|col| shift_rocks(col))
        .collect::<Vec<_>>();

    // Calculate the score for part 1.
    let p1 = calculate_score(&Grid::new_from_col_vec(shifted, data.width));

    // Part 2 we need to cycle, and save the score after each cycle, until we find a pattern in the scores.
    // Then we can calculate the score for the 1 billionth cycle.
    let mut data = data.clone();
    let mut p2 = 0;

    // Create a HashMap to store the iteration number, the score, and the grid, where the grid is the key.
    let mut scores: HashMap<Grid, (i32, i32)> = HashMap::new();
    let mut cycles: HashMap<i32, i32> = HashMap::new();

    let mut cycle_length = 0;
    let mut first_seen = 0;

    for i in 0..TOTAL_CYCLES {
        cycle_rocks(&mut data, 1);
        let score = calculate_score(&data);

        if let Some(&(first_iteration, _)) = scores.get(&data) {
            first_seen = first_iteration;
            cycle_length = i - first_iteration;
            break;
        } else {
            scores.insert(data.clone(), (i as i32, score));
            cycles.insert(i as i32, score);
        }
    }

    // The billionth cycle is the first seen + the remainder of the total cycles divided by the cycle length.
    let billionth_cycle = (first_seen + (TOTAL_CYCLES - first_seen) % cycle_length) - 1;

    // Get the score for the billionth cycle.
    if let Some(&score) = cycles.get(&billionth_cycle) {
        p2 = score;
    }

    // Realistically we should never get here, but if we do, just return the score for the 500th cycle.
    (p1, p2)
}

pub fn parse(data: &[String]) -> Grid {
    let width = data.first().map_or(0, |s| s.len());
    let grid = data.iter().flat_map(|s| s.chars()).collect();
    Grid::new(grid, width)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day14.txt")));
    println!("Day 14:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day14.txt"));
    c.bench_function("Day 14 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 14 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day14.txt"));
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
        let res = solve(&parse(&crate::library::read_file("testdata/day14.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 64;
        let res = solve(&parse(&crate::library::read_file("testdata/day14.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
