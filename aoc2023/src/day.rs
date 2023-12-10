use std::{fs::read_to_string, str::FromStr};

pub trait Day {
    fn part1(&self) -> i32;
    fn part2(&self) -> i32;
    fn new() -> Self where Self: Sized;
    fn new_box() -> Box<Self> where Self: Sized {
        Box::new(Self::new())
    }
    fn read_input(day_num: u8) -> Vec<String> where Self: Sized {
        read_to_string(format!("inputs/input{}", day_num))
            .unwrap()
            .split('\n')
            .map(|s| String::from_str(s).unwrap())
            .filter(|s| !s.is_empty())
            .collect()
    }
    fn setup(&self) -> () {}
}
