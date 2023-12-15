// https://adventofcode.com/2023/day/03
use std::collections::HashMap;
use crate::library::utility;

pub fn solve(data: &HashMap<(i32, i32), char>) -> (i32, i32) {
    let mut p1 = 0;
    let mut gears: HashMap<(i32, i32), Vec<i32>> = HashMap::new();

    // Find the bounds of the grid, so we can iterate over it and not reach any ends.
    let max_x = data.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let max_y = data.keys().map(|(_, y)| y).max().unwrap_or(&0);

    // Go through each coordinate and get the number, and then get proximity to symbols.
    for x in 0..=*max_x {
        let mut num = String::new();
        let mut valid = false;
        let mut gear_pos: Option<(i32, i32)> = None;

        for y in 0..=*max_y {
            // Checks the character at the coordinate, and if it's a digit, we add it to the number.
            if let Some(&c) = data.get(&(x, y)) {
                if c.is_digit(10) {
                    num.push(c);

                    // Need to check neighbouring coordinates.
                    if !valid {
                        for x1 in -1..=1 {
                            for y1 in -1..=1 {
                                // Find any non-digit characters, and if they're not a dot, we can add the number.
                                if let Some(&c2) = data.get(&(x + x1, y + y1)) {
                                    if !c2.is_digit(10) && c2 != '.' {
                                        valid = true;
                                        if c2 == '*' {
                                            gear_pos = Some((x + x1, y + y1));
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Once we hit a non-digit, we can add the number to the total.
                    if valid {
                        let n: i32 = num.parse().unwrap_or(0);
                        p1 += n;

                        // Have to add number to the gear list
                        if let Some(pos) = gear_pos {
                            gears.entry(pos).or_insert_with(Vec::new).push(n);
                        }

                        // Need to reset flags
                        valid = false;
                        gear_pos = None;
                    }
                    num.clear();
                }
            }
        }

        // Check last character.
        if valid {
            let n: i32 = num.parse().unwrap_or(0);
            p1 += n;
            if let Some(pos) = gear_pos {
                gears.entry(pos).or_insert_with(Vec::new).push(n);
            }
        }
    }

    // Part 2 is about finding gear-ratios, essentially look for any gear with 2 numbers near it, and multiply them then add to total.
    let p2 = gears
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(_, v)| v[0] * v[1])
        .sum();

    (p1, p2)
}

pub fn parse(data: &[String]) -> HashMap<(i32, i32), char> {
    let mut map = HashMap::new();
    for (x, line) in data.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), c as char);
        }
    }

    map
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&utility::files::read_file("data/day03.txt")));
    println!("Day 03:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day03.txt"));
    c.bench_function("Day 03 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 03 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day03.txt"));
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
        let expected = 4361;
        let res = solve(&parse(&utility::files::read_file("testdata/day03.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 467835;
        let res = solve(&parse(&utility::files::read_file("testdata/day03.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
