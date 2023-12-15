// https://adventofcode.com/2023/day/04
use std::collections::HashSet;
use crate::library::utility;

pub fn solve(data: &Vec<(HashSet<i32>, HashSet<i32>)>) -> (i32, i32) {
    let p1 = data
        .iter()
        .map(|(wnums, gnums)| {
            let overlap = wnums.intersection(gnums).count();
            if overlap > 0 {
                1 << (overlap - 1)
            } else {
                0
            }
        })
        .sum();

    let mut card_counter = vec![1; data.len()];

    // Iterate over current
    for i in 0..data.len() {
        let (wnums, gnums) = &data[i]; // Get winning numbers, and current numbers.
        let winners = wnums.intersection(&gnums).count(); // Get how many winning numbers we have.
        for j in (i + 1)..(i + 1 + winners) {
            // Increment the next n cards, multiple times if needed.
            card_counter[j] += card_counter[i];
        }
    }

    let p2: i32 = card_counter.iter().sum();

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<(HashSet<i32>, HashSet<i32>)> {
    let mut result = Vec::new();

    for line in data {
        let parts: Vec<&str> = line.split(':').collect();
        let card_numbers: Vec<&str> = parts[1].split('|').collect();

        let mut wnums = HashSet::new();
        let mut gnums = HashSet::new();

        for number in card_numbers[0].split_whitespace() {
            wnums.insert(number.parse::<i32>().unwrap());
        }

        for number in card_numbers[1].split_whitespace() {
            gnums.insert(number.parse::<i32>().unwrap());
        }

        result.push((wnums, gnums));
    }

    result
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&utility::files::read_file("data/day04.txt")));
    println!("Day 04:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day04.txt"));
    c.bench_function("Day 04 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 04 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day04.txt"));
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
        let expected = 13;
        let res = solve(&parse(&utility::files::read_file("testdata/day04.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 30;
        let res = solve(&parse(&utility::files::read_file("testdata/day04.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
