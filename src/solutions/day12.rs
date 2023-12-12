// https://adventofcode.com/2023/day/12
use lru::LruCache;
use std::num::NonZeroUsize;

#[derive(Debug)]
pub struct Springs {
    chars: String,
    counts: Vec<usize>,
}

impl Springs {
    fn new(chars: String, counts: Vec<usize>) -> Springs {
        Springs { chars, counts }
    }
}

pub fn permute(springs: &Springs, si: usize, ci: usize, cache: &mut LruCache<(usize, usize), usize>) -> usize {
    if let Some(&cached_result) = cache.get(&(si, ci)) {
        return cached_result;
    }

    let mut result;

    if si == 0 && ci == 0 {
        result = 1;
    } else if si == 0 {
        result = 0;
    } else if ci == 0 {
        result = springs.chars[..si].chars().all(|c| c != '#') as usize;
    } else if springs.chars.chars().nth(si - 1).unwrap() == '.' {
        result = permute(springs, si - 1, ci, cache);
    } else {
        let curr_num = springs.counts[ci - 1];

        if curr_num > si || springs.chars[si - curr_num..si].chars().any(|c| c == '.') {
            result = 0;
        } else if si > curr_num && springs.chars.chars().nth(si - curr_num - 1).unwrap() == '#' {
            result = 0;
        } else {
            let new_si = if si - 1 >= curr_num {
                si - curr_num - 1
            } else {
                0
            };

            result = permute(springs, new_si, ci - 1, cache);
        }

        if springs.chars.chars().nth(si - 1).unwrap() == '?' {
            result += permute(springs, si - 1, ci, cache);
        }
    }

    cache.put((si, ci), result);

    result
}

pub fn solve(data: &Vec<Springs>) -> (u64, u64) {
    // Cache the results of the permutations.
    let mut cache: LruCache<(usize, usize), usize> = LruCache::new(NonZeroUsize::new(1024).unwrap());

    // Part 1: Count the number of permutations for base input.
    let p1 = data
        .iter()
        .map(|s| permute(s, s.chars.len(), s.counts.len(), &mut cache))
        .sum::<usize>();

    // Part 2: We need to make each input 5x the size, both the string and the counts.
    // Iterate over the data and resize/repeat the strings and counts by 5 for both.
    let data = data
        .iter()
        .map(|s| {
            let new_s = std::iter::repeat(s.chars.chars().collect::<String>())
                .take(5)
                .collect::<Vec<_>>()
                .join("?");

            let new_c = s
                .counts
                .iter()
                .cycle()
                .take(s.counts.len() * 5)
                .cloned()
                .collect::<Vec<_>>();

            Springs::new(new_s, new_c)
        })
        .collect::<Vec<Springs>>();

    // Count the number of permutations for the new data.
    let p2 = data
        .iter()
        .map(|s| permute(s, s.chars.len(), s.counts.len(), &mut cache))
        .sum::<usize>();

    (p1 as u64, p2 as u64)
}

pub fn parse(data: &[String]) -> Vec<Springs> {
    // Separate the data into two parts, the row and the conditions.
    // The two sections are separated by a space, and numbers in the conditions are separated by commas.
    let res = data
        .iter()
        .map(|row| {
            let mut split = row.split_whitespace();
            let row = split.next().unwrap().to_string();
            let conditions = split
                .next()
                .unwrap()
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Springs::new(row, conditions)
        })
        .collect();

    res
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day12.txt")));
    println!("Day 12:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day12.txt"));
    c.bench_function("Day 12 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 12 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day12.txt"));
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
        let expected = 21;
        let res = solve(&parse(&crate::library::read_file("testdata/day12.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 525152;
        let res = solve(&parse(&crate::library::read_file("testdata/day12.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
