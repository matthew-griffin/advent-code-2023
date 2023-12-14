use std::fmt::Display;

pub trait Solution {
    fn part_one(&self, input: &str) -> Box<dyn Display>;
    fn part_two(&self, input: &str) -> Box<dyn Display>;
}