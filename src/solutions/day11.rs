// https://adventofcode.com/2023/day/11

pub fn solve(_data: &[String]) -> (i32, i32) {
    (0, 0)
}

pub fn parse(data: &[String]) -> Vec<String> {
    data.iter().map(|s| s.to_string()).collect()
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day11.txt")));
    println!("Day 11:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day11.txt"));
    c.bench_function("Day 11 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 11 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day11.txt"));
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
        let expected = 0;
        let res = solve(&parse(&crate::library::read_file("testdata/day11.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 0;
        let res = solve(&parse(&crate::library::read_file("testdata/day11.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
