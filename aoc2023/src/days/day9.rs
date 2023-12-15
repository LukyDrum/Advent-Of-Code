use crate::day::Day;

pub struct Day9 {
    pub lines: Vec<String>,
    series: Vec<Vec<i32>>,
}

impl Day for Day9 {
    fn new() -> Self {
        Day9 {
            lines: Self::read_input(9, false),
            series: Vec::new(),
        }
    }

    fn part1(&self) -> i32 {
        self.series.iter().map(|serie| predict_next(serie)).sum()
    }

    fn part2(&self) -> i32 {
        self.series.iter().map(|serie| {
            let mut reversed = serie.clone();
            reversed.reverse();
            predict_next(&reversed)
        }).sum()
    }

    fn setup(&mut self) -> () {
        for line in &self.lines {
            self.series.push(
                line.split_ascii_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect(),
            );
        }
    }
}

fn predict_next(serie: &Vec<i32>) -> i32 {
    if serie.iter().all(|x| *x == 0) {
        return 0;
    }

    let mut diffs: Vec<i32> = Vec::new();
    for i in 0..serie.len() - 1 {
        diffs.push(serie[i + 1] - serie[i]);
    }

    return serie.last().unwrap() + predict_next(&diffs);
}
