use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    let part1 = day_one::part_one(&input);
    let part2 = day_one::part_two(&input);
    println!("Day 1, Part 1: {part1}");
    println!("Day 1, Part 2: {part2}");

    let input = fs::read_to_string("input2.txt").unwrap();
    let part1 = day_two::part_one(&input);
    let part2 = day_two::part_two(&input);
    println!("Day 2, Part 1: {part1}");
    println!("Day 2, Part 2: {part2}");    
}
