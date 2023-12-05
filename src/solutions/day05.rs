// https://adventofcode.com/2023/day/05
use std::collections::BTreeMap;

fn process_maps(n: u64, maps: &[BTreeMap<u64, (u64, u64)>]) -> u64 {
    // Go through the maps array, one by one, so it processes seed-to-soil, then soil-to-fertilizer etc..
    maps.iter().fold(n, |acc, map| {
        // For each map, we need to keep the last processed one, so we can apply some new transformations to it.
        match map.range(..acc + 1).last() {
            // Get the highest key in the map that is less than our accumulator.
            Some((&src, &(dst, len))) => {
                // Calculate the offset, and figure out the new applied range and all that for the next map.
                let offset = acc - src;
                if offset < len {
                    dst + offset
                } else {
                    acc
                }
            }
            _ => acc, // Basically forces the 1 to 1 mapping to be applied instead.
        }
    })
}

pub fn solve(data: &(Vec<u64>, Vec<BTreeMap<u64, (u64, u64)>>)) -> (u64, u64) {
    // Process all the maps so we get the locations for each seed.
    let p1 = data
        .0
        .iter()
        .map(|&s| process_maps(s, &data.1))
        .min()
        .unwrap();

    // For part 2, we need to operate on a seed of values. That is the data.0 vector is actually a set of ranges...
    // So we just do the thing for all the seeds, and then find the minimum value.
    let p2 = (0..data.0.len())
        .step_by(2)
        .filter_map(|i| {
            let start = data.0.get(i)?;
            let end = start + data.0.get(i + 1)?;
            (*start..=end).map(|s| process_maps(s, &data.1)).min()
        })
        .min()
        .unwrap_or(0);

    (p1, p2)
}

pub fn parse(data: &[String]) -> (Vec<u64>, Vec<BTreeMap<u64, (u64, u64)>>) {
    // Need to get lines, and remove the empty lines, so we can split the data into sections for each mapping.
    let lines: Vec<_> = data.iter().map(|s| s.to_string()).collect();
    let mut sections = lines.split(|s| s.is_empty());

    // Seed is the first line only..
    let seeds = sections.next().unwrap()[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // Now parse the maps into a vector of hashmaps.
    // Each hash-map has format of destination -> (source, amount)
    let maps = sections
        .map(|section| {
            section
                .iter()
                .skip(1)
                .map(|line| {
                    let parts = line
                        .split_whitespace()
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    (parts[1], (parts[0], parts[2]))
                })
                .collect::<BTreeMap<_, _>>()
        })
        .collect::<Vec<_>>();

    (seeds, maps)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day05.txt")));
    println!("Day 05:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day05.txt"));
    c.bench_function("Day 05 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 05 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day05.txt"));
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
        let expected = 35;
        let res = solve(&parse(&crate::library::read_file("testdata/day05.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 46;
        let res = solve(&parse(&crate::library::read_file("testdata/day05.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
