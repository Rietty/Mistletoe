// https://adventofcode.com/2023/day/15
use crate::library::utility;
use core::panic;

// Custom hashfunction for a string.
pub fn hash(s: &String) -> i32 {
    s.chars().fold(0, |acc, c| (acc + c as i32) * 17 % 256)
}

pub fn split_step(s: &str) -> (String, char, i32) {
    let split_char = match s.find(['-', '=']) {
        Some(index) => s.chars().nth(index).unwrap(),
        None => panic!("Invalid string: {}", s),
    };

    let parts: Vec<&str> = s.split(split_char).collect();
    if parts.len() != 2 {
        panic!("Invalid string format: {}", s);
    }

    let label = parts[0].to_string();
    let value = if split_char == '-' {
        -1
    } else {
        parts[1].parse::<i32>().unwrap()
    };

    (label, split_char, value)
}

pub fn solve(data: &[String]) -> (i32, i32) {
    // For each string, calculate the hash and sum the result.
    let mut p1 = 0;

    // Create a vector of 256 elements, each containing a VecDeque that holds a given String.
    let mut buckets: Vec<Vec<(String, i32)>> = vec![vec![]; 256];

    // Iterate thru the data..
    for d in data {
        // Calculate the hash of the string and add it to the p1 part..
        p1 += hash(d);

        // Split the string into a label, a char and a value.
        let (label, ch, value) = split_step(d);
        let index = hash(&label);

        match ch {
            '-' => {
                // From the relevant bucket, we need to remove the pair that has the same label if it exists.
                if let Some(pos) = buckets[index as usize]
                    .iter()
                    .position(|(l, _)| l == &label)
                {
                    buckets[index as usize].remove(pos);
                }
            }

            '=' => {
                // From the relevant bucket, if the label already exits, update the value.
                // If the label does not exist, add it to the bucket at the end along with the value.
                if let Some(pos) = buckets[index as usize]
                    .iter()
                    .position(|(l, _)| l == &label)
                {
                    buckets[index as usize][pos].1 = value;
                } else {
                    buckets[index as usize].push((label, value));
                }
            }

            _ => panic!("Invalid string: {}", d),
        }
    }

    let p2 = buckets.iter().enumerate().fold(0, |acc, (i, b)| {
        if b.is_empty() {
            acc
        } else {
            acc + b.iter().enumerate().fold(0, |bucket_acc, (j, element)| {
                bucket_acc + element.1 * (i as i32 + 1) * (j as i32 + 1)
            })
        }
    });

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<String> {
    data[0].split(",").map(|s| s.to_string()).collect()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&utility::files::read_file("data/day15.txt")));
    println!("Day 15:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day15.txt"));
    c.bench_function("Day 15 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 15 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day15.txt"));
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
        let expected = 1320;
        let res = solve(&parse(&utility::files::read_file("testdata/day15.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 145;
        let res = solve(&parse(&utility::files::read_file("testdata/day15.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
