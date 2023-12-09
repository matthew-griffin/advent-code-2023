use std::{ops::Range, collections::{BTreeSet, HashMap}, str::Lines, cmp::Ordering};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MAP_PATTERN: Regex = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
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
            result = map.to_destination(result);
            print!("{}={},", map.dest_name, result);
            map = self.maps.get(&map.dest_name).unwrap();
        }
        map.to_destination(result)
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
        println!("{}-{}", source_name, dest_name);
        println!("{:?}", pairs);
        RangeMap {
            source_name: source_name.to_string(),
            dest_name: dest_name.to_string(),
            pairs
        }
    }

    fn to_destination(&self, input: u64) -> u64 {
        let result = self.pairs.binary_search_by(|pair| {
            if pair.source.contains(&input) {
                return Ordering::Equal;
            }
            pair.source.start.cmp(&input)
        });
        match result {
            Ok(index) => self.pairs[index].to_destination(input),
            Err(_) => input,
        }
    }
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
        println!("dest:{}, source:{}, size:{}", destination, source, size);
        RangePair { source: source..source+size, destination: destination..destination+size }
    }

    fn to_destination(&self, input: u64) -> u64 {
        (input - self.source.start) + self.destination.start
    }
}

pub fn part_one(input: &str) -> u64 {
    let mut lines = input.lines();
    let initial_seeds: Vec<u64> = lines.next().unwrap()
        .split_whitespace()
        .skip(1)
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect();
    lines.next();
    let almanac = Almanac::parse_new(&mut lines);
    initial_seeds.iter().map(|seed| {
        almanac.seed_to_location(seed)
    })
    .min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

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
}
