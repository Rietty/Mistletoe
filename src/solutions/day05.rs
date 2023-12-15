// https://adventofcode.com/2023/day/05
use std::collections::BTreeMap;
use rayon::prelude::*;
use crate::library::utility;

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

fn process_maps_ranges(seeds: &[u64], maps: &[BTreeMap<u64, (u64, u64)>]) -> Vec<(u64, u64)> {
    let mut ranges = seeds.chunks(2).map(|v| (v[0], v[1])).collect::<Vec<_>>();

    for map in maps {
        let mut new_ranges = Vec::with_capacity(ranges.len());

        for (start, len) in ranges {
            let mut current_start = start;

            let mut iter = map.range(..start + len).peekable();

            while current_start < start + len {
                match iter.peek() {
                    Some((&src, &(dst, n))) if src <= current_start => {
                        let map_end = src + n;

                        // Unmapped section.
                        if current_start < src {
                            new_ranges.push((
                                current_start,
                                std::cmp::min(start + len, src) - current_start,
                            ));
                            current_start = src;
                        }

                        // Mapped section.
                        if current_start < map_end {
                            new_ranges.push((
                                dst + current_start - src,
                                std::cmp::min(start + len, map_end) - current_start,
                            ));
                            current_start = map_end;
                        }

                        iter.next();
                    }
                    _ => {
                        // No more mappings applicable, add the rest of the range
                        new_ranges.push((current_start, start + len - current_start));
                        break;
                    }
                }
            }
        }

        ranges = new_ranges;
    }

    ranges
}

pub fn solve(data: &(Vec<u64>, Vec<BTreeMap<u64, (u64, u64)>>)) -> (u64, u64) {
    // Process all the maps so we get the locations for each seed.
    let p1 = data
        .0
        .par_iter()
        .map(|&s| process_maps(s, &data.1))
        .min()
        .unwrap();

    // For part 2, we need to operate on a seed of values. That is the data.0 vector is actually a set of ranges...
    // So we just do the thing for all the seeds, and then find the minimum value.
    let p2 = process_maps_ranges(&data.0, &data.1)
        .par_iter()
        .map(|&(s, _)| s)
        .min()
        .unwrap();

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
    let res = solve(&parse(&utility::files::read_file("data/day05.txt")));
    println!("Day 05:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day05.txt"));
    c.bench_function("Day 05 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 05 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day05.txt"));
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
        let res = solve(&parse(&utility::files::read_file("testdata/day05.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 46;
        let res = solve(&parse(&utility::files::read_file("testdata/day05.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
