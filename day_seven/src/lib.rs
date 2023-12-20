use std::{collections::HashMap, cmp::Ordering, iter::zip};

use lazy_static::lazy_static;

lazy_static! {
    static ref CARD_MAP: HashMap<char, u64> = HashMap::from([
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('J', 10),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);
}

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

    fn order_by_sub_strength(&self, other: &Self) -> Ordering {
        for (l, r) in zip(self.cards.chars(), other.cards.chars()) {
            let l_sub = CARD_MAP.get(&l).unwrap();
            let r_sub = CARD_MAP.get(&r).unwrap();
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
    let mut hands: Vec<_> = input.lines()
    .map(Hand::parse_new)
    .collect();
    hands.sort_by(|l_hand, r_hand| {
        if l_hand.strength > r_hand.strength {
            Ordering::Greater
        } else if l_hand.strength < r_hand.strength {
            Ordering::Less
        } else {
            l_hand.order_by_sub_strength(r_hand)
        }
    });
    hands.iter().enumerate()
    .fold(0, |acc, (index, hand)| {
        acc + (index as u64 + 1) * hand.bid
    })
}

fn part_two(_input: &str) -> u64 {
    0
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
}
