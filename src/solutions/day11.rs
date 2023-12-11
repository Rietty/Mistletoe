// https://adventofcode.com/2023/day/11
use itertools::Itertools;
use rayon::prelude::*;

const P1_SCALE: i64 = 2;
const P2_SCALE: i64 = 1_000_000;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Coordinates {
    x: i64,
    y: i64
}

impl Coordinates {
    fn new(x: i64, y: i64) -> Coordinates {
        Coordinates {
            x,
            y
        }
    }
}

// Calculates taxicab distance between any two given coordinate.
pub fn distance(p1: Coordinates, p2: Coordinates) -> i64 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

// We need to be able to calculate the real coordinates based off the scaling of rows and columns without any galaxies in them.
pub fn get_real_coords(vec: &Vec<Coordinates>, p: &Coordinates, scale: i64) -> Coordinates {
    // Calculate the offset, that is the amount of numbers by which to shift our coordinate due to any empty row or columns.
    let x_offset = (1..=p.x).filter(|&i| !vec.iter().map(|key| key.x).collect::<Vec<_>>().contains(&i)).count() as i64;
    let y_offset = (1..=p.y).filter(|&i| !vec.iter().map(|key| key.y).collect::<Vec<_>>().contains(&i)).count() as i64;

    // We return the new coordinate, but we have to multiply the offset by scale, and then subtract the offset for the original row or column from the scaled up version.
    Coordinates::new(p.x + (x_offset * scale) - x_offset, p.y + (y_offset * scale) - y_offset)
}

pub fn solve(data: &Vec<Coordinates>) -> (i64, i64) {
    let p1: i64 = data.iter().combinations(2).par_bridge().map(|c| {
            let (p1, p2) = (c.clone().into_iter().nth(0).unwrap(), c.clone().into_iter().nth(1).unwrap());
            let r_p1 = get_real_coords(data, p1, P1_SCALE);
            let r_p2 = get_real_coords(data, p2, P1_SCALE);
            distance(r_p1, r_p2)
        }
    ).sum();

    let p2: i64 = data.iter().combinations(2).par_bridge().map(|c| {
            let (p1, p2) = (c.clone().into_iter().nth(0).unwrap(), c.clone().into_iter().nth(1).unwrap());
            let r_p1 = get_real_coords(data, p1, P2_SCALE);
            let r_p2 = get_real_coords(data, p2, P2_SCALE);
            distance(r_p1, r_p2)
        }
    ).sum();

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<Coordinates> {
    let mut vec = Vec::new();
    for (x, line) in data.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            // We only need to insert the '#' since the rest can be extrapolated from the HashMap itself.
            if c == '#' {
                vec.push(Coordinates::new(x as i64, y as i64));
            }
        }
    }
    vec
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day11.txt")));
    println!("Day 11:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day11.txt"));
    c.bench_function("Day 11", |b| b.iter(|| solve(&data)));
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn part1() {
        let expected = 374;
        let res = solve(&parse(&crate::library::read_file("testdata/day11.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 82000210;
        let res = solve(&parse(&crate::library::read_file("testdata/day11.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
