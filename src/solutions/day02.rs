// https://adventofcode.com/2023/day/02
use std::collections::HashMap;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

pub fn solve(data: &[(i32, i32, i32, i32)]) -> (i32, i32) {
    let mut p1 = 0;
    let mut p2 = 0;

    for &(game_id, red, green, blue) in data.iter() {
        if red <= MAX_RED && green <= MAX_GREEN && blue <= MAX_BLUE {
            p1 += game_id;
        }
        p2 += red * green * blue;
    }

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<(i32, i32, i32, i32)> {
    let mut result = Vec::new();

    for line in data {
        let mut max_counts = HashMap::new();
        max_counts.insert("red", 0);
        max_counts.insert("green", 0);
        max_counts.insert("blue", 0);

        // Replace the commas and colons with spaces.
        let line = line.replace(",", " ").replace(";", " ").replace(":", " ");
        // Get the game id, which is 2nd element in the line
        let game_id = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        // Remove the first 2 elements from the line, then process the rest of the line, it will always be # word.
        // Based on what the word is (red, green, blue), update the max value for that category.
        let pairs = line.split_whitespace().skip(2).collect::<Vec<&str>>();
        
        for pair in pairs.chunks(2) {
            // Print the pair
            if pair.len() == 2 {
                if let Ok(count) = pair[0].parse::<i32>() {
                    let color = pair[1];
                    let entry = max_counts.entry(color).or_insert(0);
                    *entry = (*entry).max(count);
                }
            }
        }

        let max_red = *max_counts.get("red").unwrap_or(&0);
        let max_green = *max_counts.get("green").unwrap_or(&0);
        let max_blue = *max_counts.get("blue").unwrap_or(&0);

        result.push((game_id, max_red, max_green, max_blue));
    }

    result
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day02.txt")));
    println!("Day 02:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day02.txt"));
    c.bench_function("Day 02 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 02 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day02.txt"));
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
        let expected = 8;
        let res = solve(&parse(&crate::library::read_file("testdata/day02.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 2286;
        let res = solve(&parse(&crate::library::read_file("testdata/day02.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
