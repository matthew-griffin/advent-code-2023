use std::fs;

use day_one::part_one;
use day_one::part_two;

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let part1 = part_one(&input);
    let part2 = part_two(&input);
    println!("Day 1, Part 1: {part1}");
    println!("Day 1, Part 2: {part2}");
}
