use std::{fs, env, collections::{BTreeMap, btree_map::Entry}};

fn day_one(input: &str) {
    let part1 = day_one::part_one(&input);
    let part2 = day_one::part_two(&input);
    println!("Day 1, Part 1: {part1}");
    println!("Day 1, Part 2: {part2}");
}

fn day_two(input: &str) {
    let part1 = day_two::part_one(&input);
    let part2 = day_two::part_two(&input);
    println!("Day 2, Part 1: {part1}");
    println!("Day 2, Part 2: {part2}");  
}

fn day_three(input: &str) {
    let part1 = day_three::part_one(&input);
    let part2 = day_three::part_two(&input);
    println!("Day 3, Part 1: {part1}");
    println!("Day 3, Part 2: {part2}");
}

fn day_four(input: &str) {
    let part1 = day_four::part_one(&input);
    let part2 = day_four::part_two(&input);
    println!("Day 4, Part 1: {part1}");
    println!("Day 4, Part 2: {part2}");
}

fn day_five(input: &str) {
    let part1 = day_five::part_one(&input);
    println!("Day 5, Part 1: {part1}");
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let day_number = match args.len() {
        0 | 1 => 0,
        _ => args[1].parse::<u8>().unwrap_or(0)
    };

    let mut days: BTreeMap<u8, fn(&str)> = BTreeMap::new();
    days.insert(1, day_one);
    days.insert(2, day_two);
    days.insert(3, day_three);
    days.insert(4, day_four);
    days.insert(5, day_five);

    match days.entry(day_number) {
        Entry::Occupied(day) => {
            let input_file = format!("input{day_number}.txt");
            let input = fs::read_to_string(input_file).unwrap();
            day.get()(&input);
        },
        Entry::Vacant(_) => {
            for (day_number, func) in days.iter() {
                let input_file = format!("input{day_number}.txt");
                let input = fs::read_to_string(input_file).unwrap();
                func(&input);
            }
        },
    } 
}
