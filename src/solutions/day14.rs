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
    let mut new_row = String::new();
    let mut found_rock = false;
    for c in mirrors.chars() {
        if c == 'O' {
            found_rock = true;
        }
        if found_rock && c == '.' {
            new_row.push('O');
        } else {
            new_row.push(c);
        }
    }
    new_row
}

pub fn solve(data: &[String]) -> (i32, i32) {

    
    (0, 0)
}

pub fn parse(data: &[String]) -> Vec<String> {
    data.iter().map(|s| s.to_string()).collect()
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
        let expected = 0;
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
