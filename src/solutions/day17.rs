// https://adventofcode.com/2023/day/17
use crate::library::{
    containers::grid::{Grid, Point},
    utility,
};
use std::collections::{binary_heap::BinaryHeap, HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Spot {
    pos: Point,
    dir: i32, // The dir is actually the direction we came from, and thus can't go back.
    cost: i32,
}

// Implement Ord and PartialOrd for Spot, so we can use it in a BinaryHeap, we need to sort by cost.
impl Ord for Spot {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Spot {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Function that will return a Position that contains a delta on which way to move based off direction.
pub fn get_delta(dir: i32) -> Point {
    match dir {
        0 => Point { x: 0, y: 1 },
        1 => Point { x: 1, y: 0 },
        2 => Point { x: 0, y: -1 },
        3 => Point { x: -1, y: 0 },
        _ => unreachable!(),
    }
}

// This function will try to flow the lava according to the constraints about distance traveled.
pub fn flow(data: &Grid, min: usize, max: usize, sdir: i32) -> i32 {
    // Create a queue of spots to flow from, always starts from top-left.
    let mut queue = BinaryHeap::new();

    queue.push(Spot {
        pos: Point { x: 0, y: 0 },
        dir: sdir,
        cost: 0,
    });

    // Create a HashSet of visited/seen spots.
    let mut visited: HashSet<(Point, i32)> = HashSet::new();
    let mut costs: HashMap<(Point, i32), i32> = HashMap::new();

    // While the queue is not empty, keep flowing.
    while !queue.is_empty() {
        // Get the values from the queue.
        let spot = queue.pop().unwrap();

        // If we have reached our goal of the bottom-right, we can return the current cost of the spot.
        if spot.pos.x == data.width() as i32 - 1 && spot.pos.y == data.height() as i32 - 1 {
            return spot.cost;
        }

        // If we have already visited this spot, we can skip it.
        if visited.contains(&(spot.pos, spot.dir)) {
            continue;
        }

        // Add the spot to the visited set.
        visited.insert((spot.pos, spot.dir));

        // For each of the four directions, we will try to flow in that direction.
        for dir in 0..4 {
            // Amount of increase for the new cost.
            let mut increase = 0;

            // If the direction is the same as the direction we came from, we can't go back.
            if dir == spot.dir || dir == (spot.dir + 2) % 4 {
                continue;
            }

            // For the range of min to max, we will try to flow in that direction.
            for dist in 1..max + 1 {
                // Calculate the new amount of increase for X and Y for the current direction.
                let delta = get_delta(dir);
                let new_x = spot.pos.x + delta.x * dist as i32;
                let new_y = spot.pos.y + delta.y * dist as i32;

                // Check if we are in bounds.
                if new_x < 0
                    || new_x >= data.width() as i32
                    || new_y < 0
                    || new_y >= data.height() as i32
                {
                    break;
                } else {
                    // Increase the amount of increase, go from a char of 0 to 0, 1, 2, 3, etc.
                    increase += data
                        .get_point(Point { x: new_x, y: new_y })
                        .unwrap()
                        .to_digit(10)
                        .unwrap();

                    // If distance is less than min, we can't flow.
                    if dist < min {
                        continue;
                    }

                    let nc = spot.cost + increase as i32;

                    // Try to read from hashmap, if it doesn't exist, default to 1000000.
                    let c = *costs
                        .get(&(Point { x: new_x, y: new_y }, dir))
                        .unwrap_or(&1000000);

                    if c <= nc {
                        continue;
                    }

                    costs.insert((Point { x: new_x, y: new_y }, dir), nc);
                    queue.push(Spot {
                        pos: Point { x: new_x, y: new_y },
                        dir: dir,
                        cost: nc,
                    });
                }
            }
        }
    }

    0 // Should never happen.
}

pub fn solve(data: &Grid) -> (i32, i32) {
    let p1 = std::cmp::min(flow(data, 1, 3, 0), flow(data, 1, 3, 1));
    let p2 = std::cmp::min(flow(data, 4, 10, 0), flow(data, 4, 10, 1));
    (p1, p2)
}

pub fn parse(data: &[String]) -> Grid {
    let rows = data.iter().map(|s| s.to_string()).collect();
    Grid::from_rows(rows, data[0].len())
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&utility::files::read_file("data/day17.txt")));
    println!("Day 17:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day17.txt"));
    c.bench_function("Day 17 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 17 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day17.txt"));
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
        let expected = 102;
        let res = solve(&parse(&utility::files::read_file("testdata/day17.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 94;
        let res = solve(&parse(&utility::files::read_file("testdata/day17.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
