// https://adventofcode.com/2023/day/09
use rayon::prelude::*;
use crate::library::utility;

// Generate differences between elements of a vector
pub fn differences(vec: &[i32]) -> Vec<i32> {
    vec.windows(2).map(|w| w[1] - w[0]).collect()
}

pub fn extrapolate(data: Vec<i32>) -> i32 {
    // Generate the chain of difference vectors
    let mut chain = vec![data];
    while chain
        .last()
        .unwrap()
        .par_iter()
        .all(|&x| x == chain.last().unwrap()[0])
        == false
    {
        chain.push(differences(chain.last().unwrap()));
    }

    // Backtrack and extrapolate the chain
    for i in (0..chain.len() - 1).rev() {
        let last_element = chain[i].last().unwrap() + chain[i + 1].last().unwrap();
        chain[i].push(last_element);
    }

    // Last element of first vector is the extrapolated value
    *chain.first().unwrap().last().unwrap()
}

pub fn solve(data: &Vec<Vec<i32>>) -> (i32, i32) {
    // Sum of all vectors called with extrapolate
    let p1 = data.par_iter().map(|v| extrapolate(v.to_vec())).sum();
    let p2 = data
        .par_iter()
        .map(|v| extrapolate(v.to_vec().into_iter().rev().collect()))
        .sum();

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<Vec<i32>> {
    // Read data into a vector of vectors of i32
    data.par_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&utility::files::read_file("data/day09.txt")));
    println!("Day 09:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day09.txt"));
    c.bench_function("Day 09 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 09 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day09.txt"));
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
        let expected = 114;
        let res = solve(&parse(&utility::files::read_file("testdata/day09.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 2;
        let res = solve(&parse(&utility::files::read_file("testdata/day09.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
