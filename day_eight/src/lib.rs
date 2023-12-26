use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ROUTE_PATTERN: Regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
}

pub struct Solution;

impl common::Solution for Solution {
    fn part_one(&self, input: &str) -> Box<dyn std::fmt::Display> {
        Box::new(part_one(input))
    }

    fn part_two(&self, input: &str) -> Box<dyn std::fmt::Display> {
        Box::new(part_two(input))
    }
}

fn part_one(input: &str) -> u64{
    let mut lines = input.lines();
    let pattern_chars: Vec<_> = lines.next().unwrap().chars().collect();
    let pattern_repeat = pattern_chars.len() as u64;
    lines.next();
    let mut routes = HashMap::new();
    for line in lines {
        let parts = ROUTE_PATTERN.captures(line).unwrap();
        routes.insert(parts[1].to_string(), [parts[2].to_string(), parts[3].to_string()]);
    }
    let mut current_point = "AAA".to_string();
    let mut current_steps = 0u64;
    while current_point != "ZZZ" {
        let pattern_index: usize = (current_steps % pattern_repeat).try_into().unwrap();
        let next_index: usize = if pattern_chars[pattern_index] == 'L' {
            0
        } else {
            1
        };
        current_point = routes.get(&current_point).unwrap()[next_index].clone();
        current_steps += 1;
    }
    current_steps
}

fn part_two(input: &str) -> u64{
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn test_input_one() -> &'static str {
        indoc! {"RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"}
    }

    fn test_input_two() -> &'static str {
        indoc! {"LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"}
    }

    #[test]
    fn it_passes_first_example() {
        let result = part_one(test_input_one());
        assert_eq!(result, 2);
    }

    #[test]
    fn it_passes_second_example() {
        let result = part_one(test_input_two());
        assert_eq!(result, 6);
    }    
}
