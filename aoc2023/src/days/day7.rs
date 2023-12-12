use crate::day::Day;

pub struct Day7 {
    pub lines: Vec<String>,
}

impl Day for Day7 {
    fn new() -> Self {
        Day7 {
            lines: Self::read_input(7, false),
        }
    }

    fn part1(&self) -> i32 {
        -1
    }

    fn part2(&self) -> i32 {
        -2
    }

    fn setup(&mut self) -> () {
        
    }
}
