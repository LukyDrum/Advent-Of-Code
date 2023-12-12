use std::iter::zip;

use crate::day::Day;

pub struct Day6 {
    pub lines: Vec<String>,
    times: Vec<i32>,
    distances: Vec<i32>,
}

impl Day for Day6 {
    fn new() -> Self {
        Day6 {
            lines: Self::read_input(6, false),
            times: Vec::new(),
            distances: Vec::new(),
        }
    }

    fn part1(&self) -> i32 {
        zip(&self.times, &self.distances)
            .map(|(time, distance)| number_of_ways(*time as i64, *distance as i64))
            .fold(1, |acc, x| acc * x)
    }

    fn part2(&self) -> i32 {
        let time = self
            .times
            .iter()
            .fold("".to_string(), |acc, num| {
                acc.to_string() + &num.to_string()
            })
            .parse::<i64>()
            .unwrap();
        let distance = self
            .distances
            .iter()
            .fold("".to_string(), |acc, num| {
                acc.to_string() + &num.to_string()
            })
            .parse::<i64>()
            .unwrap();

        number_of_ways(time, distance)
    }

    fn setup(&mut self) -> () {
        let ls: Vec<Vec<i32>> = self
            .lines
            .iter()
            .map(|line| {
                line.split_ascii_whitespace().collect::<Vec<&str>>()[1..]
                    .iter()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
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

fn solve_qudratic(b: i64, c: i64) -> (f64, f64) {
    let x1 = (b as f64 - f64::sqrt((b * b - 4 * c) as f64)) / 2.0;
    let x2 = (b as f64 + f64::sqrt((b * b - 4 * c) as f64)) / 2.0;

    (x1, x2)
}

fn number_of_ways(time: i64, distance: i64) -> i32 {
    let (x1, x2) = solve_qudratic(time, distance);
    let x1 = x1.floor() as i32;
    let x2 = x2.ceil() as i32;

    x2 - x1 - 1
}
