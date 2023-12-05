use std::{collections::HashMap};

pub fn part_one(input: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn it_sums_ids_of_possible_games() {
        const INPUT: &str = indoc! {"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                                    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                                    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                                    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                                    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"};

        let result = part_one(INPUT);
        assert_eq!(result, 8);
    }
}
