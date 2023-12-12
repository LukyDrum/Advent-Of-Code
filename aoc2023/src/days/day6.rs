use crate::day::Day;

pub struct Day6 {
    pub lines: Vec<String>,
    times: Vec<i32>,
    distances: Vec<i32>,
}

impl Day for Day6 {
    fn new() -> Self {
        Day6 {
            lines: Self::read_input(6, true),
            times: Vec::new(),
            distances: Vec::new(),
        }
    }

    fn part1(&self) -> i32 {
        -1
    }

    fn part2(&self) -> i32 {
        -1
    }

    fn setup(&mut self) -> () {
        let ls: Vec<Vec<i32>> = self
            .lines
            .iter()
            .map(|line| {
                line.split_ascii_whitespace().collect::<Vec<&str>>()[1..]
                    .iter()
                    .map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>()
            })
            .collect();

        for n in &ls[0] {
            self.times.push(*n);
        }
        for n in &ls[1] {
            self.distances.push(*n);
        }
    }
}
