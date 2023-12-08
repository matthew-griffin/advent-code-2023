use std::collections::{HashSet, HashMap};

const MARKERS: [char; 2] = [':','|'];

pub fn part_one(input: &str) -> u32 {
    input.lines()
    .map(|line| {
        let results = count_winners(line);
        match results {
            0 => 0,
            _ => 2u32.pow((results-1).try_into().unwrap())
        }
    })
    .sum()
}

pub fn part_two(input: &str) -> u32 {
    let mut extra_cards: HashMap<usize, u32> = HashMap::new();    
    input.lines().enumerate()
    .map(|(index, line)| {
        let results = count_winners(line);
        let current_cards = match extra_cards.get(&index) {
            Some(extra) => extra + 1,
            None => 1
        };
        for i in 1..=results {
            *extra_cards.entry(index + i).or_insert(0) += current_cards;
        }
        current_cards
    })
    .sum()
}

fn count_winners(line: &str) -> usize {
    let parts: Vec<_> = line.split(MARKERS).collect();
    let winning_numbers: HashSet<u32> = HashSet::from_iter(parts[1].split_whitespace().map(str::parse::<u32>).map(Result::unwrap));
    parts[2].split_whitespace().map(str::parse::<u32>).filter(|num| {
        winning_numbers.contains(num.as_ref().unwrap())
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    fn test_input() -> &'static str {
        indoc! {"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"}
    }

    #[test]
    fn it_totals_the_winnings() {
        let result = part_one(test_input());
        assert_eq!(result, 13);
    }

    #[test]
    fn it_totals_the_amount_of_scratchcards() {
        let result = part_two(test_input());
        assert_eq!(result, 30);
    }
}
