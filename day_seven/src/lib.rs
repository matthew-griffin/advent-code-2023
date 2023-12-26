use std::{collections::HashMap, cmp::Ordering, iter::zip};

pub struct Solution;

impl common::Solution for Solution {
    fn part_one(&self, input: &str) -> Box<dyn std::fmt::Display> {
        Box::new(part_one(input))
    }

    fn part_two(&self, input: &str) -> Box<dyn std::fmt::Display> {
        Box::new(part_two(input))
    }
}

struct Hand<'a> {
    cards: &'a str,
    bid: u64,
    strength: u64,
}

impl<'a> Hand<'a> {
    fn parse_new(input: &'a str) -> Hand<'a> {
        let mut parts = input.split_whitespace();
        let cards = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<u64>().unwrap();
        let strength = Hand::calc_hand_strength(cards);
        Hand {
            cards,
            bid,
            strength
        }
    }

    fn parse_part_two(input: &'a str) -> Hand<'a> {
        let mut parts = input.split_whitespace();
        let cards = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<u64>().unwrap();
        let strength = Hand::calc_part_two_hand_strength(cards);
        Hand {
            cards,
            bid,
            strength
        }
    }

    fn calc_hand_strength(input: &str) -> u64 {
        let mut card_map: HashMap<char, u8> = HashMap::new();
        for card in input.chars() {
            *card_map.entry(card).or_insert(0) += 1;
        }
        match card_map.len() {
            1 => 6,
            2 => {
                if card_map.iter().any(|(_, count)| *count > 3) {
                    5
                } else {
                    4
                }
            },
            3 => {
                if card_map.iter().any(|(_, count)| *count > 2) {
                    3
                } else {
                    2
                }
            },
            4 => 1,
            5 => 0,
            x => panic!("Should not have {x} cards!"),
        }
    }

    fn calc_part_two_hand_strength(input: &str) -> u64 {
        let mut card_map: HashMap<char, u8> = HashMap::new();
        for card in input.chars() {
            *card_map.entry(card).or_insert(0) += 1;
        }
        let joker_count = card_map.remove(&'J').unwrap_or(0);
        let max_key = card_map
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k);
        match max_key {
            Some(key) => {*card_map.entry(*key).or_default() += joker_count;},
            None => return 6,
        }
        match card_map.len() {
            1 => 6,
            2 => {
                if card_map.iter().any(|(_, count)| *count > 3) {
                    5
                } else {
                    4
                }
            },
            3 => {
                if card_map.iter().any(|(_, count)| *count > 2) {
                    3
                } else {
                    2
                }
            },
            4 => 1,
            5 => 0,
            x => panic!("Should not have {x} cards!"),
        }
 
    }

    fn order_by_sub_strength(&self, other: &Self, card_map: &HashMap<char, u8>) -> Ordering {
        for (l, r) in zip(self.cards.chars(), other.cards.chars()) {
            let l_sub = card_map.get(&l).unwrap();
            let r_sub = card_map.get(&r).unwrap();
            if l_sub > r_sub {
                return Ordering::Greater;
            } else if r_sub > l_sub {
                return Ordering::Less;
            } 
        }
        Ordering::Equal
    }
}

fn part_one(input: &str) -> u64 {
    let card_map = create_card_map(&[
        '2','3','4','5','6','7','8','9','T','J','Q','K','A'
    ]);
    let mut hands: Vec<_> = input.lines()
    .map(Hand::parse_new)
    .collect();
    hands.sort_by(|l_hand, r_hand| {
        if l_hand.strength > r_hand.strength {
            Ordering::Greater
        } else if l_hand.strength < r_hand.strength {
            Ordering::Less
        } else {
            l_hand.order_by_sub_strength(r_hand, &card_map)
        }
    });
    hands.iter().enumerate()
    .fold(0, |acc, (index, hand)| {
        acc + (index as u64 + 1) * hand.bid
    })
}

fn part_two(input: &str) -> u64 {
    let card_map = create_card_map(&[
        'J','2','3','4','5','6','7','8','9','T','Q','K','A'
    ]);
    let mut hands: Vec<_> = input.lines()
    .map(Hand::parse_part_two)
    .collect();
    hands.sort_by(|l_hand, r_hand| {
        if l_hand.strength > r_hand.strength {
            Ordering::Greater
        } else if l_hand.strength < r_hand.strength {
            Ordering::Less
        } else {
            l_hand.order_by_sub_strength(r_hand, &card_map)
        }
    });
    hands.iter().enumerate()
    .fold(0, |acc, (index, hand)| {
        acc + (index as u64 + 1) * hand.bid
    })
}

fn create_card_map(char_order: &[char]) -> HashMap<char, u8> {
    let mut result = HashMap::new();
    for (index, character) in char_order.iter().enumerate() {
        result.insert(*character, index as u8);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn test_input() -> &'static str {
        indoc! {"32T3K 765
                T55J5 684
                KK677 28
                KTJJT 220
                QQQJA 483"}
    }

    #[test]
    fn it_multiplies_bid_by_rank() {
        let result = part_one(test_input());
        assert_eq!(result, 6440);
    }

    #[test]
    fn it_multiplies_bid_by_rank_using_jokers() {
        let result = part_two(test_input());
        assert_eq!(result, 5905);
    }
}
