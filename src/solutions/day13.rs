// https://adventofcode.com/2023/day/13
use rayon::prelude::*;

#[derive(Debug)]
pub struct Grid {
    grid: Vec<char>,
    width: usize,
}

impl Grid {
    fn new(grid: Vec<char>, width: usize) -> Grid {
        Grid { grid, width }
    }

    // Finds the line of reflection and counts the number of rows above it, or columns to the left, depending on type of reflection.
    // Then calculates the score for said reflective value based on if horizontal or vertical was used and multiplication was needed or not.
    fn calculate_reflection_score(&self, target: usize) -> usize {
        // Generate both the columns and rows as a Vec<String> so we can calculate the score for each type as needed.
        let (rows, cols) = (self.rows(), self.columns());

        // Essentially the way this will work is that we go through the lines, either of the rows or the columns.
        // By zipping up the lines in reverse iterator order, that is lines are zipped together from outside to inside.
        // Even amount: A B C D E F is zipped in: AF, BE, CD etc...
        // Odd amount: A B C D E F G is zipped in: BG CF DE and A is ignored (as per examples..)
        // We can then see if lines match, and if not exactly how much they differ by.
        // If the lines don't differ at all we can use that to get our index needed to calculate the score.
        // If the lines differ at exactly one character, then for part two we have found the character we need to flip and thus the new line of reflection.
        let calc_score = |lines: &Vec<String>| -> usize {
            (1..lines.len())
                .filter(|&i| {
                    let nm = lines[..i]
                        .iter()
                        .rev()
                        .zip(lines[i..].iter())
                        .flat_map(|(l, r)| l.chars().zip(r.chars()))
                        .filter(|(a, b)| a != b)
                        .count();
                    nm == target
                })
                .map(|i| i * if lines == &rows { 100 } else { 1 })
                .sum()
        };

        calc_score(&rows) + calc_score(&cols)
    }

    fn rows(&self) -> Vec<String> {
        self.grid
            .chunks(self.width)
            .map(|chunk| chunk.iter().collect())
            .collect()
    }

    fn columns(&self) -> Vec<String> {
        (0..self.width)
            .map(|i| {
                self.grid[i..]
                    .chunks(self.width)
                    .map(|chunk| chunk.get(0).unwrap())
                    .collect()
            })
            .collect()
    }
}

pub fn solve(data: &[Grid]) -> (usize, usize) {
    let p1 = data
        .par_iter()
        .map(|g| g.calculate_reflection_score(0))
        .sum();
    let p2 = data
        .par_iter()
        .map(|g| g.calculate_reflection_score(1))
        .sum();

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<Grid> {
    let mut grids = Vec::new();
    let mut group = Vec::new();
    let mut width = 0;

    for line in data {
        if line.is_empty() {
            if !group.is_empty() {
                grids.push(Grid::new(group, width));
                group = Vec::new();
            }
        } else {
            if group.is_empty() {
                width = line.len();
            }
            group.extend(line.chars());
        }
    }

    if !group.is_empty() {
        grids.push(Grid::new(group, width));
    }

    grids
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day13.txt")));
    println!("Day 13:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day13.txt"));
    c.bench_function("Day 13 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 13 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day13.txt"));
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
        let expected = 405;
        let res = solve(&parse(&crate::library::read_file("testdata/day13.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 400;
        let res = solve(&parse(&crate::library::read_file("testdata/day13.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
