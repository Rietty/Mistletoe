// https://adventofcode.com/2023/day/06
use crate::library::concat_vec_to_num;

pub fn ways_to_win(t: u64, d: u64) -> u64 {
    // This is actually an equation of a quadratic form: -x^2 + tx - d = 0
    // Just need to solve for x and we have the number of ways to win. Since it's essentially bounding our two ends.
    let m = ((t.pow(2) - 4 * d) as f64).sqrt();
    ((t as f64 + m) / 2.0 - 1.0).ceil() as u64 - ((t as f64 - m) / 2.0 + 1.0).floor() as u64 + 1
}

pub fn solve(data: &(Vec<u64>, Vec<u64>)) -> (u64, u64) {
    // The p1 is operating on the vector as a set of pairs.
    let p1 = data
        .0
        .iter()
        .zip(data.1.iter())
        .map(|(t, d)| ways_to_win(*t, *d))
        .product();

    // For p2, instead of just going off a zip, we need to assume we have two indexes..
    let p2 = ways_to_win(
        concat_vec_to_num(&data.0).unwrap(),
        concat_vec_to_num(&data.1).unwrap(),
    );

    (p1, p2)
}

pub fn parse(data: &[String]) -> (Vec<u64>, Vec<u64>) {
    let times: Vec<u64> = data.get(0).map_or_else(Vec::new, |line| {
        line.split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse::<u64>().ok())
            .collect()
    });

    let distances: Vec<u64> = data.get(1).map_or_else(Vec::new, |line| {
        line.split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse::<u64>().ok())
            .collect()
    });

    (times, distances)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day06.txt")));
    println!("Day 06:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day06.txt"));
    c.bench_function("Day 06 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 06 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day06.txt"));
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
        let expected = 288;
        let res = solve(&parse(&crate::library::read_file("testdata/day06.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 71503;
        let res = solve(&parse(&crate::library::read_file("testdata/day06.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
