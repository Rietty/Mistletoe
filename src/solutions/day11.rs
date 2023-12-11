// https://adventofcode.com/2023/day/11
use std::collections::HashSet;
use itertools::Itertools;

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
pub fn get_real_coords(p: &Coordinates, scale: i64, x_set: &HashSet<i64>, y_set: &HashSet<i64>) -> Coordinates {
    // Calculate the offset, that is the amount of numbers by which to shift our coordinate due to any empty row or columns.
    let x_offset = (1..=p.x).filter(|&i| !x_set.contains(&i)).count() as i64;
    let y_offset = (1..=p.y).filter(|&i| !y_set.contains(&i)).count() as i64;

    // We return the new coordinate, but we have to multiply the offset by scale, and then subtract the offset for the original row or column from the scaled up version.
    Coordinates::new(p.x + (x_offset * scale) - x_offset, p.y + (y_offset * scale) - y_offset)
}
 
pub fn solve(data: &[Coordinates]) -> (i64, i64) {
    // Generate hashsets once of the x and y coordinate directions which are needed to get actual coordinates.
    let x_set: HashSet<_> = data.iter().map(|key| key.x).collect();
    let y_set: HashSet<_> = data.iter().map(|key| key.y).collect();

    // Calculate scaled coordinates before hand for everything.
    let p1_data: Vec<Coordinates> = data.iter().map(|c| get_real_coords(&c, P1_SCALE, &x_set, &y_set)).collect();
    let p2_data: Vec<Coordinates> = data.iter().map(|c| get_real_coords(&c, P2_SCALE, &x_set, &y_set)).collect();

    let p1: i64 = p1_data.iter().combinations(2).map(|c| {
            distance(*c[0], *c[1])
        }
    ).sum();

    let p2: i64 = p2_data.iter().combinations(2).map(|c| {
            distance(*c[0], *c[1])
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
    c.bench_function("Day 11 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 11 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day11.txt"));
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
