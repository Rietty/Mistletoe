// https://adventofcode.com/2023/day/14
use crate::library::{containers::grid::Grid, utility};
//use rayon::prelude::*;
//use std::collections::HashMap;

const _TOTAL_CYCLES: i32 = 1_000_000_000;

pub fn solve(_data: &Grid) -> (i32, i32) {
    (0, 0)
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
