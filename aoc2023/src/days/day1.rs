use crate::day::Day;

pub struct Day1 {
    pub lines: Vec<String>,
}

impl Day for Day1 {
    fn new() -> Self {
        Day1 {
            lines: Self::read_input(1),
        }
    }

    fn part1(&self) -> i32 {
        self.lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                
                let nums: Vec<char> = line.chars()
                    .filter(|c| c.is_digit(10))
                    .collect();

                let mut num = String::from("");
                num.push(*nums.first().unwrap());
                num.push(*nums.last().unwrap());

                num.parse::<i32>().unwrap()
            })
            .fold(0, |sum, num| sum + num)
    }

    fn part2(&self) -> i32 {
        -1
    }
}
