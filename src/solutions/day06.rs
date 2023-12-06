// https://adventofcode.com/2023/day/06

pub fn ways_to_win(t: u64, d: u64) -> u64 {
    // The amount of ways we can win, will be multipled into the p1 answer.
    let mut wins: u64 = 0;
    // Hold button down for 0 to t seconds, inclusive.
    for b in 0..(t+1) {
        // Speed will equal to the value of the hold, i.e. the b. And the distance traveled will be t - b * b.
        if ((t - b) * b) > d {
            wins += 1;
        }
    }

    wins
}

pub fn solve(data: &(Vec<u64>, Vec<u64>)) -> (u64, u64) {
    // The p1 is operating on the vector as a set of pairs.    
    let p1 = data.0.iter().zip(data.1.iter()).map(|(t, d)| ways_to_win(*t, *d)).product();

    // For p2, instead of just going off a zip, we need to assume we have two indexes..
    let p2 = ways_to_win(data.0.iter().map(|&n| n.to_string()).collect::<String>().parse().unwrap(), data.1.iter().map(|&n| n.to_string()).collect::<String>().parse().unwrap());

    (p1, p2)
}

pub fn parse(data: &[String]) -> (Vec<u64>, Vec<u64>) {
    let lines: Vec<_> = data.iter().map(|s| s.to_string()).collect();
    let times: Vec<_> = lines[0].split_whitespace().skip(1).map(|s| s.parse::<u64>().unwrap()).collect();
    let distances: Vec<_> = lines[1].split_whitespace().skip(1).map(|s| s.parse::<u64>().unwrap()).collect();

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
