use std::{fs, env, collections::{BTreeMap, btree_map::Entry}};

use common::Solution;

fn main() {
    let args: Vec<_> = env::args().collect();
    let day_number = match args.len() {
        0 | 1 => 0,
        _ => args[1].parse::<u8>().unwrap_or(0)
    };

    let mut days: BTreeMap<u8, &dyn Solution> = BTreeMap::new();
    days.insert(1, &day_one::Solution);
    days.insert(2, &day_two::Solution);
    days.insert(3, &day_three::Solution);
    days.insert(4, &day_four::Solution);
    days.insert(5, &day_five::Solution);

    match days.entry(day_number) {
        Entry::Occupied(day) => {
            run_day(&day_number, *day.get());
        },
        Entry::Vacant(_) => {
            for (day_number, solution) in days.iter() {
                run_day(day_number, *solution);
            }
        },
    } 
}

fn run_day(day_number: &u8, solution: &dyn Solution) {
    let input_file = format!("input{day_number}.txt");
    let input = fs::read_to_string(input_file).unwrap();
    println!("Day {}, Part 1: {}", day_number, solution.part_one(&input));
    println!("Day {}, Part 2: {}", day_number, solution.part_two(&input));
}
