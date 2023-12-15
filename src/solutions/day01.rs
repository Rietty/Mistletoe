// https://adventofcode.com/2023/day/01
use std::collections::VecDeque;
use crate::library::utility;

pub fn solve(data: &[String]) -> (i32, i32) {
    let numbers = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let p1: u32 = data
        .iter()
        .filter_map(|s| {
            let f = s.chars().find(|c| c.is_digit(10));
            let l = s.chars().rev().find(|c| c.is_digit(10));
            match (f, l) {
                (Some(f), Some(l)) => Some(f.to_digit(10).unwrap() * 10 + l.to_digit(10).unwrap()),
                _ => None,
            }
        })
        .collect::<Vec<_>>()
        .iter()
        .sum();

    let mut p2 = 0;

    for line in data {
        let mut q = VecDeque::new();
        let mut i = 0;

        while i < line.len() {
            if let Some(c) = line[i..].chars().next() {
                if c.is_digit(10) {
                    q.push_back(c.to_digit(10).unwrap() as i64);
                    i += 1;
                } else {
                    // If we do not have a digit, check if we have a word that is a number in the numbers vector starting at the current index
                    // If it finds a match, push the index of the number to the queue and increment the index by one
                    for (j, n) in numbers.iter().enumerate() {
                        if line[i..].starts_with(n) {
                            q.push_back(j as i64);
                            i += 1;
                            break;
                        }
                    }
                    i += 1;
                }
            }
        }

        if let (Some(first), Some(last)) = (q.front(), q.back()) {
            p2 += first * 10 + last;
        }
    }

    // Return the result
    (p1 as i32, p2 as i32)
}

pub fn parse(data: &[String]) -> Vec<String> {
    data.iter().map(|s| s.to_string()).collect()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&utility::files::read_file("data/day01.txt")));
    println!("Day 01:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day01.txt"));
    c.bench_function("Day 01 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 01 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day01.txt"));
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
        let expected = 209;
        let res = solve(&parse(&utility::files::read_file("testdata/day01.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 281;
        let res = solve(&parse(&utility::files::read_file("testdata/day01.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
