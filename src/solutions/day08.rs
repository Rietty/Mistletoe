// https://adventofcode.com/2023/day/08
use std::collections::HashMap;
use rayon::prelude::*;
use crate::library::utility;

pub fn get_path_size(
    instructions: &[char],
    map: &HashMap<String, (String, String)>,
    start: &str,
    end: &str,
) -> u64 {
    let mut curr_node = start;
    let mut i = 0;
    let mut path_size = 0;

    while !curr_node.ends_with(end) {
        let curr_char = instructions[i];
        let (left, right) = &map[curr_node];

        curr_node = if curr_char == 'L' { left } else { right };
        path_size += 1;

        i = (i + 1) % instructions.len();
    }

    path_size
}

pub fn solve(data: &(Vec<char>, HashMap<String, (String, String)>)) -> (u64, u64) {
    let p1 = get_path_size(&data.0, &data.1, "AAA", "ZZZ");

    let path_sizes: Vec<_> = data
        .1
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>()
        .par_iter()
        .map(|k| get_path_size(&data.0, &data.1, k, "Z"))
        .collect();

    let p2 = path_sizes
        .iter()
        .fold(1, |acc, &num| num::integer::lcm(acc, num));

    (p1, p2)
}

pub fn parse(data: &[String]) -> (Vec<char>, HashMap<String, (String, String)>) {
    let chars = data[0].chars().collect::<Vec<char>>();

    let mut map = HashMap::new();

    for line in &data[2..] {
        if let Some((key, value_part)) = line.split_once(" = ") {
            let values = value_part
                .trim_matches(|p| p == '(' || p == ')')
                .split(", ")
                .collect::<Vec<&str>>();
            if values.len() == 2 {
                map.insert(
                    key.to_string(),
                    (values[0].to_string(), values[1].to_string()),
                );
            }
        }
    }

    (chars, map)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&utility::files::read_file("data/day08.txt")));
    println!("Day 08:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day08.txt"));
    c.bench_function("Day 08 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 08 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day08.txt"));
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
        let expected = 6;
        let res = solve(&parse(&utility::files::read_file("testdata/day08.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 6;
        let res = solve(&parse(&utility::files::read_file("testdata/day08.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
