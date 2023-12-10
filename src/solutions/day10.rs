// https://adventofcode.com/2023/day/10
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
struct PipeGraph {
    grid: Vec<char>,
    width: i32,
}

impl PipeGraph {
    fn new(grid: Vec<Vec<char>>) -> PipeGraph {
        // Convert the grid into a single vector by chaining the rows together.
        let width = grid[0].len() as i32;
        let grid = grid.into_iter().flatten().collect();
        PipeGraph { grid, width }
    }
}

pub fn solve(data: &Vec<Vec<char>>) -> (i32, i32) {
    let graph = PipeGraph::new(data.clone());

    // So we need to be able to know which directions are valid from a given point, can do so by looking up from a hash-map.
    let mut dir_map = HashMap::from([
        // Directions
        ('|', vec![graph.width, -graph.width]), // We can move up and down by the width of the grid.
        ('-', vec![-1, 1]),                     // We can move left and right by 1.
        ('.', vec![]),                          // We can't move from a dot.
        ('S', vec![]), // Starting point is special, we need to figure out what direction we can move from it later.
        ('7', vec![-1, graph.width]), // Left and Down
        ('F', vec![1, graph.width]), // Right and Down
        ('L', vec![1, -graph.width]), // Right and Up
        ('J', vec![-1, -graph.width]), // Left and Up
    ]);

    // Need to find the starting point, which is located at character 'S'.
    let start = graph.grid.iter().position(|&x| x == 'S').unwrap() as i32;
    // Create a hash-set to store path.
    let mut path = HashSet::new();
    path.insert(start);

    // Using the graph.grid, we need to store a vector of all the lookups based on each character instead.
    // Basically so going from a grid of characters, to a grid of directional offsets for each type of pipe instead.
    let mut vec_grid = Vec::new();
    for c in graph.grid.iter() {
        vec_grid.push(dir_map.get(c).unwrap().clone());
    }

    // From now we use our new vec_grid instead of the graph.grid.
    // Need to figure out exactly which of the pipes or junctions the value of 'S' is at and update it.
    for (i, offsets) in vec_grid.iter_mut().enumerate() {
        if offsets.iter().any(|&x| (i as i32 + x) == start) {
            // We found the starting point, update it to be a junction that is relevant for us.
            dir_map.get_mut(&'S').unwrap().push(i as i32 - start);
        }
    }

    // Using the value of start, update vec_grid to contain the value of 'S' from dir_map at that point.
    vec_grid[start as usize] = dir_map.get(&'S').unwrap().clone();

    // Now we can calculate the distance while looping through the grid.
    let mut p1 = 0;
    // A hashset will let us keep track of visited points and what not.
    // Since it is a 1D grid, each point is simply a numeric value.
    let mut seen = HashSet::new();
    loop {
        // Create a new hashset for this iteration, while storing the previous one.
        let prev = std::mem::replace(&mut seen, HashSet::new());
        // Loop through both the path and the previous seen points.
        for &p in path.iter().chain(prev.iter()) {
            for &offset in vec_grid[p as usize].iter() {
                // Get the new point.
                let point = p + offset;
                // Check if point is not in the path.
                if !path.contains(&point) {
                    // If it is not, add it to the seen hashset.
                    seen.insert(point);
                }
            }
        }

        // If seen is empty, we have reached the end.
        if seen.is_empty() {
            break;
        } else {
            // Chain the seen points to the path.
            path.extend(&seen);
            p1 += 1;
        }
    }

    let mut p2 = 0;
    for i in 0..vec_grid.len() {
        if path.contains(&(i as i32)) {
            continue;
        }

        let mut r = true;
        let mut l = true;
        let mut j: i32 = i as i32;

        while j > 0 {
            if path.contains(&j) && vec_grid[j as usize].contains(&1) {
                r = !r;
            }

            if path.contains(&j) && vec_grid[j as usize].contains(&-1) {
                l = !l;
            }

            j -= graph.width;
        }

        if !(r || l) {
            p2 += 1;
        }
    }

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<Vec<char>> {
    // Read in the data into a Vec<Vec<char>>
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in data {
        grid.push(line.chars().collect());
    }
    grid
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day10.txt")));
    println!("Day 10:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day10.txt"));
    c.bench_function("Day 10 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 10 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day10.txt"));
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
        let expected = 80;
        let res = solve(&parse(&crate::library::read_file("testdata/day10.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 10;
        let res = solve(&parse(&crate::library::read_file("testdata/day10.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
