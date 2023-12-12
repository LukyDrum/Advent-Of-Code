use crate::day::Day;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub struct Day2 {
    pub lines: Vec<String>,
}

impl Day for Day2 {
    fn new() -> Self {
        Day2 {
            lines: Self::read_input(2, false),
        }
    }

    fn part1(&self) -> i32 {
        let mut ans = 0;
        let mut game_num = 1;
        for line in &self.lines {
            let max_of_line = self.get_max_of_line(line.clone());
            if max_of_line.0 <= MAX_RED && max_of_line.1 <= MAX_GREEN && max_of_line.2 <= MAX_BLUE {
                ans += game_num;
            }

            game_num += 1;
        }

        ans
    }

    fn part2(&self) -> i32 {
        let mut ans: i32 = 0;
        
        for line in &self.lines {
            let max_of_line = self.get_max_of_line(line.clone());
            ans += (max_of_line.0 * max_of_line.1 * max_of_line.2) as i32;
        }

        ans
    }
}

impl Day2 {
    fn get_max_of_line(&self, line: String) -> (u32, u32, u32) {
        let split: Vec<&str> = line.split(":").collect();

        let cubes = split[1];
        let cubes = cubes.replace(",", "");
        let cubes = cubes.replace(";", "");
        let cubes = cubes.replace(" red", "R");
        let cubes = cubes.replace(" green", "G");
        let cubes = cubes.replace(" blue", "B");

        let cubes: Vec<&str> = cubes.split_whitespace().collect();

        let (mut red, mut green, mut blue) = (0, 0, 0);

        for part in cubes {
            let mut part = part.to_string();
            let color = part.pop().unwrap();
            let count = part.parse::<u32>().unwrap();
            match color {
                'R' => {
                    if count > red {
                        red = count;
                    }
                }
                'G' => {
                    if count > green {
                        green = count;
                    }
                }
                'B' => {
                    if count > blue {
                        blue = count;
                    }
                }
                _ => {}
            }
        }

        (red, green, blue)
    }
}
