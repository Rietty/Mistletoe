// https://adventofcode.com/2023/day/18
use crate::library::utility;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Point {
    pub x: i128,
    pub y: i128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    direction: char,
    steps: u32,
    colour: u32,
}

// Translate a direction into a coordinate delta.
pub fn get_delta(dir: char) -> Point {
    match dir {
        'D' => Point { x: 0, y: 1 },
        'R' => Point { x: 1, y: 0 },
        'U' => Point { x: 0, y: -1 },
        'L' => Point { x: -1, y: 0 },
        _ => unreachable!(),
    }
}

// Calculate interior for a given set of instructions.
pub fn calculate(data: &[Instruction]) -> i128 {
    let mut vertices: Vec<Point> = Vec::new();
    let mut current = Point { x: 0, y: 0 };
    let mut border: i128 = 0;

    // Move along the path, adding vertices and updating the border size.
    for instruction in data {
        let delta = get_delta(instruction.direction);
        current.x += delta.x * instruction.steps as i128;
        current.y += delta.y * instruction.steps as i128;
        border += instruction.steps as i128;
        vertices.push(current);
    }

    // Shoelace formula.
    let mut area = 0;
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        area += vertices[i].x * vertices[j].y;
        area -= vertices[j].x * vertices[i].y;
    }

    // Pick's theorem.
    (area - border + 2) / 2 + border
}

pub fn solve(data: &[Instruction]) -> (i128, i128) {
    let p1 = calculate(data);

    // Go thru the instructions, and update the direction and steps for each instruction.
    // The new direction is the colour % 16, and the new steps is the colour / 16.
    let mut instructions = Vec::new();

    for instruction in data {
        let direction = match instruction.colour % 16 {
            0 => 'R',
            1 => 'D',
            2 => 'L',
            3 => 'U',
            _ => unreachable!(),
        };
        let steps = instruction.colour / 16;
        instructions.push(Instruction {
            direction,
            steps,
            colour: instruction.colour,
        });
    }

    let p2 = calculate(&instructions);

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<Instruction> {
    // Each line is a single instruction of format: 'char, u32, (#u32)' where the last part is a hex colour. Read into a vector of instructions.
    let mut instructions = Vec::new();
    for line in data {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap().chars().next().unwrap();
        let steps = parts.next().unwrap().parse::<u32>().unwrap();
        let colour = parts.next().unwrap();
        match u32::from_str_radix(&colour[2..colour.len() - 1], 16) {
            Ok(num) => {
                instructions.push(Instruction {
                    direction,
                    steps,
                    colour: num,
                });
            }
            Err(e) => println!("Error: {}", e),
        }
    }
    instructions
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&utility::files::read_file("data/day18.txt")));
    println!("Day 18:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day18.txt"));
    c.bench_function("Day 18 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 18 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day18.txt"));
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
        let expected = 62;
        let res = solve(&parse(&utility::files::read_file("testdata/day18.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 952408144115;
        let res = solve(&parse(&utility::files::read_file("testdata/day18.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
