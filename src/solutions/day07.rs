// https://adventofcode.com/2023/day/07
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

// Need to figure out what type of hand we have..
fn get_hand_type(hand: &[i32]) -> HandType {
    let mut counts = HashMap::new();
    for &card in hand {
        *counts.entry(card).or_insert(0) += 1;
    }

    match counts.values().max() {
        Some(&5) => HandType::FiveOfAKind,
        Some(&4) => HandType::FourOfAKind,
        Some(&3) if counts.len() == 2 => HandType::FullHouse,
        Some(&3) => HandType::ThreeOfAKind,
        Some(&2) if counts.len() == 3 => HandType::TwoPair,
        Some(&2) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn get_hand_type_jokers(hand: &[i32], joker_count: usize) -> HandType {
    let mut counts = HashMap::new();
    for &card in hand.iter().filter(|&&c| c != 0) {
        *counts.entry(card).or_insert(0) += 1;
    }

    let max_count = counts.values().max().cloned().unwrap_or(0) + joker_count;

    match max_count {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if counts.len() + joker_count == 2 || counts.len() == 1 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if counts.len() + joker_count == 3 || counts.len() == 1 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        _ => HandType::HighCard,
    }
}

pub fn solve(data: &[(String, i32)]) -> (i32, i32) {
    let mut card_strength = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);

    let mut hands = data.to_vec();
    hands.sort_by(|a, b| {
        let a_hand: Vec<i32> =
            a.0.chars()
                .map(|c| *card_strength.get(&c).unwrap())
                .collect();
        let b_hand: Vec<i32> =
            b.0.chars()
                .map(|c| *card_strength.get(&c).unwrap())
                .collect();

        let a_type = get_hand_type(&a_hand);
        let b_type = get_hand_type(&b_hand);

        match a_type.cmp(&b_type) {
            std::cmp::Ordering::Equal => a_hand.cmp(&b_hand),
            other => other,
        }
    });

    // Part 1 get the sum of the product of each bid and the index + 1 of the hand.
    let p1 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as i32 * bid)
        .sum::<i32>();

    // Part 2
    // Need to set value of 'J' to 0 to represent a joker
    card_strength.insert('J', 0);

    // Do stuff here.
    let mut hands = data.to_vec();
    hands.sort_by(|a, b| {
        let a_hand: Vec<i32> =
            a.0.chars()
                .map(|c| *card_strength.get(&c).unwrap())
                .collect();
        let b_hand: Vec<i32> =
            b.0.chars()
                .map(|c| *card_strength.get(&c).unwrap())
                .collect();
        let a_joker_count = a.0.chars().filter(|&c| c == 'J').count();
        let b_joker_count = b.0.chars().filter(|&c| c == 'J').count();

        let a_type = get_hand_type_jokers(&a_hand, a_joker_count);
        let b_type = get_hand_type_jokers(&b_hand, b_joker_count);

        match a_type.cmp(&b_type) {
            std::cmp::Ordering::Equal => a_hand.cmp(&b_hand),
            other => other,
        }
    });

    // Part 2 get the sum of the product of each bid and the index + 1 of the hand.
    let p2 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as i32 * bid)
        .sum::<i32>();

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<(String, i32)> {
    let mut hands = Vec::new();

    for line in data {
        // Split the string into two parts: the string key and the integer value
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let Ok(value) = parts[1].parse::<i32>() {
                // Insert the key and value into the map
                hands.push((parts[0].to_string(), value));
            }
        }
    }

    hands
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&crate::library::read_file("data/day07.txt")));
    println!("Day 07:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&crate::library::read_file("data/day07.txt"));
    c.bench_function("Day 07 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 07 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&crate::library::read_file("data/day07.txt"));
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
        let expected = 6440;
        let res = solve(&parse(&crate::library::read_file("testdata/day07.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 5905;
        let res = solve(&parse(&crate::library::read_file("testdata/day07.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
