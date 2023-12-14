// https://adventofcode.com/2023/day/14
use rayon::prelude::*;

#[derive(Debug)]
pub struct Grid {
    grid: Vec<char>,
    width: usize,
}

impl Grid {
    fn new(grid: Vec<char>, width: usize) -> Grid {
        Grid { grid, width }
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

pub fn calculate_score(shifted: Vec<String>) -> i32 {
    // Score is calculated based off the length of the String and the positions of the 'O's.
    // For example if the String is "OOO....", the score is 7+6+5 = 18.
    // If the String is "O.O.O.O", the score is 7+5+3+1 = 16.
    shifted.iter().fold(0, |acc, s| {
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
    let p1 = calculate_score(shifted);

    (p1, 0)
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
        let expected = 0;
        let res = solve(&parse(&crate::library::read_file("testdata/day14.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
