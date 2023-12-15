use crate::day::Day;

pub struct Day10 {
    pub lines: Vec<String>,
}

impl Day for Day10 {
    fn new() -> Self {
        Day10 {
            lines: Self::read_input(10, false),
        }
    }

    fn part1(&self) -> i32 {
        -1
    }

    fn part2(&self) -> i32 {
        -1
    }

    fn setup(&mut self) -> () {}
}
