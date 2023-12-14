use std::{ops::Range, collections::{HashMap, VecDeque}, str::Lines, cmp::Ordering};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MAP_PATTERN: Regex = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
}

fn combine_ranges(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut result = Vec::new();
    ranges.sort_by(|a,b| {
        a.start.cmp(&b.start)
    });
    let mut current_range = Option::None::<Range<u64>>;
    for range in ranges {
        if let Some(ref current) = current_range {
            if current.end == range.start || current.contains(&range.start) {
                current_range = Some(current.start..range.end);
            } else {
                result.push(current.clone());
                current_range = Some(range);
            }

        } else {
            current_range = Some(range.clone());
        }
    }
    if let Some(ref current) = current_range {
        result.push(current.clone());
    }

    result
}

fn split_range(range: &Range<u64>, split_index: &u64) -> (Range<u64>, Range<u64>) {
    (range.start..*split_index, *split_index..range.end)
}

struct Almanac {
    maps: HashMap<String, RangeMap>,
}

impl Almanac {
    fn parse_new(lines: &mut Lines) -> Almanac {
        let mut maps = HashMap::new();
        let mut lines = lines.peekable();
        while lines.peek().is_some() {
            let map = RangeMap::parse_new(&mut lines);
            maps.insert(map.source_name.clone(), map);
        }
        Almanac { maps }
    }

    fn seed_to_location(&self, seed: &u64) -> u64 {
        let mut map = self.maps.get("seed").unwrap();
        let mut result = *seed;
        while map.dest_name != "location" {
            result = map.to_destination(&result);
            map = self.maps.get(&map.dest_name).unwrap();
        }
        map.to_destination(&result)
    }

    fn seed_range_to_location(&self, seed_ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut map = self.maps.get("seed").unwrap();
        let mut result = combine_ranges(seed_ranges);
        while map.dest_name != "location" {
            result = combine_ranges(map.ranges_to_destination(result));
            map = self.maps.get(&map.dest_name).unwrap();
        }
        map.ranges_to_destination(result)
    }
}

struct RangeMap {
    source_name: String,
    dest_name: String,
    pairs: Vec<RangePair>,
}

impl RangeMap {
    fn parse_new<'a>(lines: &mut impl Iterator<Item = &'a str>) -> RangeMap {
        let descriptions = MAP_PATTERN.captures(lines.next().unwrap()).unwrap();
        let source_name = &descriptions[1];
        let dest_name = &descriptions[2];
        let mut pairs = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            pairs.push(RangePair::parse_new(line));
        }
        pairs.sort_by(|a,b| {
            a.source.start.cmp(&b.source.start)
        });
        RangeMap {
            source_name: source_name.to_string(),
            dest_name: dest_name.to_string(),
            pairs
        }
    }

    fn to_destination(&self, input: &u64) -> u64 {
        match self.find_containing_range(&input) {
            Ok(index) => self.pairs[index].to_destination(input),
            Err(_) => *input,
        }
    }

    fn find_containing_range(&self, input: &u64) -> Result<usize, usize> {
        self.pairs.binary_search_by(|pair| {
            if pair.source.contains(input) {
                return Ordering::Equal;
            }
            pair.source.start.cmp(input)
        })
    }

    fn ranges_to_destination(&self, input: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut result: Vec<Range<u64>> = Vec::new();
        let mut current_ranges = VecDeque::new();
        for range in &input {
            let start_range = self.find_containing_range(&range.start);
            let mut current_index = match start_range {
                Ok(index) => {
                    current_ranges.push_back(range.clone());
                    index
                }
                Err(index) => {
                    if index == self.pairs.len() {
                        result.push(range.clone());
                        continue;
                    }
                    let (outside, inside) = split_range(range, &self.pairs[index].source.start);
                    result.push(outside);
                    current_ranges.push_back(inside);
                    index
                }
            };
            while !current_ranges.is_empty() {
                let next_range = current_ranges.pop_front().unwrap();
                if next_range.is_empty() {
                    break;
                }
                if current_index == self.pairs.len() {
                    result.push(next_range);
                    break;
                }
                let converted_range = self.pairs[current_index].range_to_destination(&next_range);
                match converted_range {
                    RangeConversion::Complete(range) => result.push(range),
                    RangeConversion::Remainder(converted, remainder) => {
                        result.push(converted);
                        current_ranges.push_back(remainder);
                        current_index += 1;
                    }
                }
            }
        }
        result
    }
}

enum RangeConversion {
    Complete(Range<u64>),
    Remainder(Range<u64>, Range<u64>),
}


#[derive(Debug, Eq, PartialEq)]
struct RangePair {
    source: Range<u64>,
    destination: Range<u64>
}

impl RangePair {
    fn parse_new(input: &str) -> RangePair {
        let mut numbers = input.split_whitespace().map(str::parse::<u64>).map(Result::unwrap);
        let destination = numbers.next().unwrap();
        let source = numbers.next().unwrap();
        let size = numbers.next().unwrap();
        RangePair { source: source..source+size, destination: destination..destination+size }
    }

    fn to_destination(&self, input: &u64) -> u64 {
        (input - self.source.start) + self.destination.start
    }

    fn range_to_destination(&self, input: &Range<u64>) -> RangeConversion {
        if input.start < self.source.start {
            panic!("cannot convert range that starts outside");
        }
        if input.end > self.source.end {
            let (to_convert, remainder) = split_range(input, &self.source.end);
            RangeConversion::Remainder(self.contained_range_to_destination(&&to_convert), remainder)
        } else {
            RangeConversion::Complete(self.contained_range_to_destination(&input))
        }
    }

    fn contained_range_to_destination(&self, input: &Range<u64>) -> Range<u64> {
        self.to_destination(&input.start)..self.to_destination(&input.end)
    }
}

fn extract_seeds(lines: &mut Lines<'_>) -> Vec<u64> {
    let initial_seeds: Vec<u64> = lines.next().unwrap()
        .split_whitespace()
        .skip(1)
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect();
    initial_seeds
}

fn parse_input(input: &str) -> (Vec<u64>, Almanac) {
    let mut lines = input.lines();
    let initial_seeds = extract_seeds(&mut lines);
    lines.next();
    let almanac = Almanac::parse_new(&mut lines);
    (initial_seeds, almanac)
}

pub fn part_one(input: &str) -> u64 {
    let (initial_seeds, almanac) = parse_input(input);
    initial_seeds.iter().map(|seed| {
        almanac.seed_to_location(seed)
    })
    .min().unwrap()
}

pub fn part_two(input: &str) -> u64 {
    let (initial_seeds, almanac) = parse_input(input);
    let seed_ranges: Vec<_> = initial_seeds.chunks(2).map(|seed_count| {
        seed_count[0]..(seed_count[0]+seed_count[1])
    })
    .collect();
    almanac.seed_range_to_location(seed_ranges).iter()
    .map(|range| range.start)
    .min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn it_combines_adjecent_ranges() {
        let result = combine_ranges(vec![0..2, 2..4]);
        assert_eq!(result, vec![0..4]);
    }

    #[test]
    fn it_leaves_ranges_with_gaps() {
        let result = combine_ranges(vec![0..2, 3..5]);
        assert_eq!(result, vec![0..2, 3..5]);
    }

    #[test]
    fn order_of_ranges_does_not_matter() {
        let result = combine_ranges(vec![2..4, 0..2, 3..5]);
        assert_eq!(result, vec![0..5]);
    }

    fn test_input() -> &'static str {
        indoc! {"seeds: 79 14 55 13

                seed-to-soil map:
                50 98 2
                52 50 48
                
                soil-to-fertilizer map:
                0 15 37
                37 52 2
                39 0 15
                
                fertilizer-to-water map:
                49 53 8
                0 11 42
                42 0 7
                57 7 4
                
                water-to-light map:
                88 18 7
                18 25 70
                
                light-to-temperature map:
                45 77 23
                81 45 19
                68 64 13
                
                temperature-to-humidity map:
                0 69 1
                1 0 69
                
                humidity-to-location map:
                60 56 37
                56 93 4"}
    }

    #[test]
    fn it_finds_the_lowest_location_number() {
        let result = part_one(test_input());
        assert_eq!(result, 35);
    }

    #[test]
    fn it_finds_the_lowest_location_in_ranges() {
        let result = part_two(test_input());
        assert_eq!(result, 46);
    }
}
