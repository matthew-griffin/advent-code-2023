use std::fs;

use day_one::part_one;

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let part1 = part_one(&input);
    println!("Day 1, Part 1: {part1}");
}
