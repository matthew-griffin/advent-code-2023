use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

pub fn part_one(input: &str) -> u32 {
    input.lines()
    .map(|line| {
        let first_digit = line.chars().find(char::is_ascii_digit).unwrap();
        let last_digit = line.chars().rev().find(char::is_ascii_digit).unwrap();
        let number = format!("{first_digit}{last_digit}");
        number.parse::<u32>().unwrap()
    })
    .sum()
}

lazy_static! {
    static ref LEADING_NUMBER: Regex = Regex::new(r"^(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    static ref NUMBER_MAP: HashMap<&'static str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);
}

fn extract_numbers(input: &str) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    for index in 0..input.len() {
        let remainder = &input[index..];
        let first_char = remainder.chars().next().unwrap();
        if first_char.is_ascii_digit() {
            result.push(first_char.to_digit(10).unwrap());
            continue;
        }
        let Some(string_number) = LEADING_NUMBER.captures(remainder) else {
            continue;
        };
        let number = NUMBER_MAP.get(&string_number[0]).unwrap();
        result.push(*number);
    } 
    result
}

pub fn part_two(input: &str) -> u32 {
    input.lines()
    .map(|line| {
        let numbers = extract_numbers(line);
        numbers.first().unwrap() * 10 + numbers.last().unwrap()
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn part1_input() -> &'static str {
        indoc! {"1abc2
                pqr3stu8vwx
                a1b2c3d4e5f
                treb7uchet"}
    }

    #[test]
    fn test_part_one() {
        let result = part_one(part1_input());
        assert_eq!(result, 142);
    }

    #[test]
    fn test_extract_numbers() {
        let result = extract_numbers("0onetwothreefourfivesixseveneightnine");
        assert_eq!(result, vec![0,1,2,3,4,5,6,7,8,9]);
    }

    fn part2_input() -> &'static str {
        indoc! {"two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen"}
    }

    #[test]
    fn test_part_two() {
        let result = part_two(part2_input());
        assert_eq!(result, 281);
    }
}
