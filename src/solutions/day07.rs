// https://adventofcode.com/2023/day/07
use std::cmp::Ordering;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
pub struct Hand {
    cards: String,
    bid: i32,
}

// Get the card value for a given card.
fn card_value(card: char, p2: bool) -> i32 {
    // Card value goes from 2 to 14, where 14 is an Ace, however if p2 is true then 'J' is a value of 0.
    match card {
        '2'..='9' => card.to_digit(10).unwrap() as i32,
        'T' => 10,
        'J' => {
            if p2 {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}

pub fn rank(hand: &Hand, p2: bool) -> (HandType, Vec<i32>) {
    // Create a vector to store the count of each card, and the rank of each card. It will be of length of hand.cards.
    let mut count: Vec<i32> = vec![0; hand.cards.len()];
    let mut rank: Vec<i32> = vec![0; hand.cards.len()];

    // Loop through each card in the hand.
    for (i, card) in hand.cards.chars().enumerate() {
        // Set value of count to number of times the card appears in the hand.
        count[i] = hand.cards.matches(card).count() as i32;
        // Set value of rank to the value of the card.
        rank[i] = card_value(card, p2);
    }

    // Based on if we are in p2 or not, we need to calculate the HandType
    if p2 {
        // Get number of 'J' cards in the hand.
        let jokers = hand.cards.matches('J').count() as i32;
        match count {
            _ if count.contains(&(5 - jokers)) || jokers == 5 => (HandType::FiveOfAKind, rank),
            _ if (count.contains(&(4 - jokers)) && jokers != 2)
                || jokers == 3
                || (jokers == 2 && count.iter().filter(|&n| *n == 2).count() == 4) =>
            {
                (HandType::FourOfAKind, rank)
            }
            _ if (count.contains(&3) && count.contains(&2))
                || (jokers >= 1
                    && jokers < 3
                    && count.iter().filter(|&n| *n == 2).count() == 4) =>
            {
                (HandType::FullHouse, rank)
            }
            _ if count.contains(&(3 - jokers)) || jokers == 2 => (HandType::ThreeOfAKind, rank),
            _ if count.iter().filter(|&n| *n == 2).count() == 4
                || (jokers == 1 && count.contains(&2)) =>
            {
                (HandType::TwoPairs, rank)
            }
            _ if count.contains(&2) || jokers == 1 => (HandType::OnePair, rank),
            _ => (HandType::HighCard, rank),
        }
    } else {
        match count {
            _ if count.contains(&5) => (HandType::FiveOfAKind, rank),
            _ if count.contains(&4) => (HandType::FourOfAKind, rank),
            _ if count.contains(&3) && count.contains(&2) => (HandType::FullHouse, rank),
            _ if count.contains(&3) => (HandType::ThreeOfAKind, rank),
            _ if count.iter().filter(|&n| *n == 2).count() == 4 => (HandType::TwoPairs, rank),
            _ if count.contains(&2) => (HandType::OnePair, rank),
            _ => (HandType::HighCard, rank),
        }
    }
}

fn compute_sum(data: &[Hand], p2: bool) -> i32 {
    let mut hand_ranks: Vec<(HandType, Vec<i32>, i32)> = data
        .par_iter()
        .map(|hand| {
            let (hand_type, rank) = rank(hand, p2);
            (hand_type, rank, hand.bid)
        })
        .collect();

    hand_ranks.sort_by(|a, b| match a.0.cmp(&b.0) {
        Ordering::Equal => a.1.cmp(&b.1),
        other => other,
    });

    hand_ranks
        .par_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| bid * (i as i32 + 1))
        .sum()
}

pub fn solve(data: &[Hand]) -> (i32, i32) {
    let p1 = compute_sum(data, false);
    let p2 = compute_sum(data, true);

    (p1, p2)
}

pub fn parse(data: &[String]) -> Vec<Hand> {
    let mut hands = Vec::new();

    for line in data {
        // Split the string into two parts: the string key and the integer value
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let Ok(value) = parts[1].parse::<i32>() {
                // Insert the key and value into the map
                // hands.push((parts[0].to_string(), value));
                hands.push(Hand {
                    cards: parts[0].to_string(),
                    bid: value,
                });
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
