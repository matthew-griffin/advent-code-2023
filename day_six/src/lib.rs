use std::iter::zip;

pub struct Solution;

impl common::Solution for Solution {
    fn part_one(&self, input: &str) -> Box<dyn std::fmt::Display> {
        Box::new(part_one(input))
    }

    fn part_two(&self, input: &str) -> Box<dyn std::fmt::Display> {
        Box::new(part_two(input))
    }
}

fn part_one(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = extract_numbers(lines.next().unwrap());
    let distances = extract_numbers(lines.next().unwrap());
    zip(times, distances)
    .map(|(time, distance)| {
        count_winning_methods(&time, &distance)
    })
    .fold(1u64, |acc, x| acc * x)
}

fn extract_numbers(input: &str) -> Vec<u64> {
    input
    .split_whitespace()
    .skip(1)
    .map(str::parse::<u64>)
    .map(Result::unwrap)
    .collect()
}

fn count_winning_methods(time: &u64, distance: &u64) -> u64 {
    let mut beaten_distance = false;
    let mut winning_methods = 0u64;
    for x in 0..*time {
        let beats_distance = calculate_distance(time, x) > *distance;
        if beaten_distance && beats_distance {
            winning_methods += 1
        } else if beaten_distance && !beats_distance {
            return winning_methods;
        } else if !beaten_distance && beats_distance {
            beaten_distance = true;
            winning_methods += 1;
        }
    }
    winning_methods
}

fn calculate_distance(time: &u64, x: u64) -> u64 {
    time * x - x.pow(2)
}

fn part_two(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn test_input() -> &'static str {
        indoc! {"Time:      7  15   30
                Distance:  9  40  200"}
    }

    #[test]
    fn it_multiplies_ways_of_beating_record() {
        let result = part_one(test_input());
        assert_eq!(result, 288);
    }
}
