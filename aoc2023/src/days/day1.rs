use crate::day::Day;

/// I tried solving the Day 1 (both parts) using mostly iterators and functional programming style
pub struct Day1 {
    pub lines: Vec<String>,
}

impl Day for Day1 {
    fn new() -> Self {
        Day1 {
            lines: Self::read_input(1, false),
        }
    }

    fn part1(&self) -> i32 {
        self.lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let nums: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();

                let mut num = String::from("");
                num.push(*nums.first().unwrap());
                num.push(*nums.last().unwrap());

                num.parse::<i32>().unwrap()
            })
            .fold(0, |sum, num| sum + num)
    }

    fn part2(&self) -> i32 {
        let replacements = vec![
            ("one", "o1e"),
            ("two", "t2o"),
            ("three", "t3e"),
            ("four", "f4r"),
            ("five", "f5e"),
            ("six", "s6x"),
            ("seven", "s7n"),
            ("eight", "e8t"),
            ("nine", "n9n"),
        ];

        self.lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut new_line = line.clone();
                for rep in &replacements {
                    new_line = new_line.replace(rep.0, rep.1);
                }

                let nums: Vec<char> = new_line.chars().filter(|c| c.is_digit(10)).collect();

                let mut num = String::from("");
                num.push(*nums.first().unwrap());
                num.push(*nums.last().unwrap());

                num.parse::<i32>().unwrap()
            })
            .fold(0, |sum, num| sum + num)
    }
}
