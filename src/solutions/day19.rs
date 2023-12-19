// https://adventofcode.com/2023/day/19
use crate::library::utility;
use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Part {
    x: i128,
    m: i128,
    a: i128,
    s: i128,
}

// Reads a string and converts it into the respective values for each category and makes the part.
impl Part {
    fn from_string(input: &str) -> Self {
        let mut part = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for pair in input[1..input.len() - 1].split(',') {
            let mut kv = pair.split('=');
            let key = kv.next().unwrap();
            let value = kv.next().unwrap().parse::<i128>().unwrap();
            match key {
                "x" => part.x = value,
                "m" => part.m = value,
                "a" => part.a = value,
                "s" => part.s = value,
                _ => (),
            }
        }
        part
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Rule {
    category: Option<char>,
    comparator: Option<char>,
    value: Option<i128>,
    target: String,
}

impl Rule {
    fn from_string(input: &str) -> Self {
        if input.contains(':') {
            let parts: Vec<&str> = input.split(|c| c == '<' || c == '>').collect();
            let category = parts[0].chars().next();
            let comparator = input.chars().nth(parts[0].len());
            let value_target: Vec<&str> = parts[1].split(':').collect();
            let value = value_target[0].parse::<i128>().unwrap();
            let target = value_target[1].to_string();
            Rule {
                category,
                comparator,
                value: Some(value),
                target,
            }
        } else {
            Rule {
                category: None,
                comparator: None,
                value: None,
                target: input.to_string(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Workflow {
    label: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn from_string(input: &str) -> Self {
        let parts: Vec<&str> = input.split('{').collect();
        let label = parts[0].to_string();
        let rules_str = &parts[1][..parts[1].len() - 1]; // Remove the closing '}'
        let rules: Vec<Rule> = rules_str.split(',').map(|s| Rule::from_string(s)).collect();
        Workflow { label, rules }
    }
}

// This function will follow the flow of a part and return a bool for acceptance or rejection based on it.
pub fn flow(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    // A queue will allow us to process the workflows in order and based off the rules.
    let mut queue: VecDeque<String> = VecDeque::new();
    // First one in is always the "in".
    queue.push_back("in".to_string());
    // Keep going until queue is empty or we return early..?
    while !queue.is_empty() {
        let label = queue.pop_front();
        let rules = &workflows[&label.unwrap()].rules;

        // Based on the rules we can process it one of two ways, either understand the rule and then move to target.. or if the rule is simply a redirect, then return or add to queue.
        // Iterate thru the rules:
        for rule in rules {
            // If the rule has a category of None, it's a simple redirect or a return, else if there is a value we can process according to the value.
            if rule.category.is_some() {
                // If something exits.. we need to operate on the rule fully and follow thru, either with a return, or an addition to queue and break or simply do nothing and go to next rule.
                // The first section gets the category from the rule, and inserts the value from the part instead into variable.
                let category: i128 = match rule.category.unwrap() {
                    'x' => part.x,
                    'm' => part.m,
                    'a' => part.a,
                    's' => part.s,
                    _ => unreachable!("There is always a category here!"),
                };

                // The value is stored in rule.value, and based on the stored comparator we can trigger the flow statement and either return the correct value, or simply add to queue.
                match rule.comparator.unwrap() {
                    '>' => {
                        if category > rule.value.unwrap() {
                            match rule.target.as_ref() {
                                "R" => return false,
                                "A" => return true,
                                _ => {
                                    queue.push_back(rule.target.to_string());
                                    break;
                                }
                            }
                        }
                    }
                    '<' => {
                        if category < rule.value.unwrap() {
                            match rule.target.as_ref() {
                                "R" => return false,
                                "A" => return true,
                                _ => {
                                    queue.push_back(rule.target.to_string());
                                    break;
                                }
                            }
                        }
                    }
                    _ => unreachable!("Comparator should exist!"),
                }
            } else {
                // If there is no value, we simply match and as such return as needed.. or push something into the queue.
                match rule.target.as_ref() {
                    "R" => return false,
                    "A" => return true,
                    _ => {
                        queue.push_back(rule.target.to_string());
                        break;
                    }
                }
            }
        }
    }

    unreachable!("This function should always return well before this point!");
}

pub fn solve(data: &(HashMap<String, Workflow>, Vec<Part>)) -> (i128, i128) {
    let (workflows, parts) = data;

    let p1: i128 = parts
        .par_iter()
        .map(|p| {
            if flow(&workflows, &p) {
                p.x + p.m + p.a + p.s
            } else {
                0
            }
        })
        .sum();

    // For each combination of x, m, a, s, we need to check if it's valid. If it is, we count it up.
    // Need to do 1-4000 inclusive of x, m, a, s and it needs to be in parallel.
    let p2: i128 = (1..=4000)
        .into_par_iter()
        .map(|x| {
            (1..=4000)
                .into_par_iter()
                .map(|m| {
                    (1..=4000)
                        .into_par_iter()
                        .map(|a| {
                            (1..=4000)
                                .into_par_iter()
                                .map(|s| {
                                    let part = Part { x, m, a, s };
                                    if flow(&workflows, &part) {
                                        1
                                    } else {
                                        0
                                    }
                                })
                                .sum::<i128>()
                        })
                        .sum::<i128>()
                })
                .sum::<i128>()
        })
        .sum::<i128>();

    (p1, p2)
}

pub fn parse(data: &[String]) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();

    let split = data.iter().position(|x| x == "").unwrap();
    let (s1, s2) = data.split_at(split);

    // Iterate over the first part and assign workflows.
    for workflow in s1.to_vec().iter() {
        let w = Workflow::from_string(workflow);
        workflows.insert(w.label.to_string(), w);
    }

    // Iterate over the second part and assign parts.
    for part in s2[1..].to_vec().iter() {
        parts.push(Part::from_string(part));
    }

    (workflows, parts)
}

#[allow(dead_code)]
pub fn run() {
    let res = solve(&parse(&utility::files::read_file("data/day19.txt")));
    println!("Day 19:\nStar 1: {}\nStar 2: {}\n", res.0, res.1);
}

#[allow(dead_code)]
pub fn benchmark(c: &mut criterion::Criterion) {
    let data = parse(&utility::files::read_file("data/day19.txt"));
    c.bench_function("Day 19 - solve:", |b| b.iter(|| solve(&data)));
    c.bench_function("Day 19 - parse & solve:", |b| {
        b.iter(|| {
            let data = parse(&utility::files::read_file("data/day19.txt"));
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
        let expected = 19114;
        let res = solve(&parse(&utility::files::read_file("testdata/day19.txt")));
        assert_eq!(res.0, expected);
        println!("Part 1: Expected: {}, Actual: {}", expected, res.0);
    }

    #[test]
    fn part2() {
        let expected = 167409079868000;
        let res = solve(&parse(&utility::files::read_file("testdata/day19.txt")));
        assert_eq!(res.1, expected);
        println!("Part 2: Expected: {}, Actual: {}", expected, res.1);
    }
}
