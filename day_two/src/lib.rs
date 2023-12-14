use std::collections::HashMap;

pub struct Solution;

impl common::Solution for Solution {
    fn part_one(&self, input: &str) -> Box<dyn std::fmt::Display> {
        Box::new(part_one(input))
    }

    fn part_two(&self, input: &str) -> Box<dyn std::fmt::Display> {
        Box::new(part_two(input))
    }
}

fn part_one(input: &str) -> u32 {
    let limits: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    input.lines()
    .map(|line| {
        let parts: Vec<_> = line.split(":").collect();
        let game_id = parts[0].split_whitespace().collect::<Vec<_>>()[1].parse::<u32>().unwrap();
        let mut cube_counts = parts[1].split(&[';', ','][..]);
        if cube_counts.any(|cube_count| {
            let mut cube_info = cube_count.trim().split_whitespace();
            let cube_count = cube_info.next().unwrap().parse::<u32>().unwrap();
            let cube_colour = cube_info.next().unwrap();
            cube_count > *limits.get(cube_colour).unwrap()
        }) {
            return 0;
        }           
        game_id
    })
    .sum()
}

fn part_two(input: &str) -> u32 {
    input.lines()
    .map(|line| {
        let mut min_cubes: HashMap<&str, u32> = HashMap::new();

        let parts: Vec<_> = line.split(":").collect();
        let cube_counts = parts[1].split(&[';', ','][..]);
        cube_counts.for_each(|cube_count| {
            let mut cube_info = cube_count.trim().split_whitespace();
            let cube_count = cube_info.next().unwrap().parse::<u32>().unwrap();
            let cube_colour = cube_info.next().unwrap();
            min_cubes.entry(cube_colour).and_modify(|existing| {
              if cube_count > *existing {
                *existing = cube_count
              }  
            }).or_insert(cube_count);
        });
        min_cubes.values().cloned().reduce(|acc, e| acc * e).unwrap()
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    fn test_input() -> &'static str {
        indoc! {"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"}
    }

    #[test]
    fn it_sums_ids_of_possible_games() {
        let result = part_one(test_input());
        assert_eq!(result, 8);
    }

    #[test]
    fn it_sums_power_of_minimum_sets() {
        let result = part_two(test_input());
        assert_eq!(result, 2286);
    }
}
