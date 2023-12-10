use std::vec;

use crate::day::Day;

struct Number {
    value: i32,
    row: i32,
    start_col: i32,
    end_col: i32,
}

struct Gear {
    row: i32,
    col: i32,
}

pub struct Day3 {
    pub lines: Vec<String>,
}

impl Day for Day3 {
    fn new() -> Self {
        Day3 {
            lines: Self::read_input(3),
        }
    }

    fn part1(&self) -> i32 {
        let mut char_matrix: Vec<Vec<u8>> = Vec::new();

        let _example_lines = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
            "..........",
            "......$42.",
            "..........",
        ];

        for line in &self.lines {
            let mut bytes = line.clone().into_bytes();
            bytes.push(b'\n');

            char_matrix.push(bytes);
        }

        let numbers = self.get_numbers(&char_matrix);

        let mut ans = 0;
        for num in &numbers {
            // println!("{} @ ({}, {} -> {})", num.value, num.row, num.start_col, num.end_col);
            for col in num.start_col..(num.end_col + 1) {
                if self.is_position_adjecent_to_symbol(&char_matrix, (num.row, col)) {
                    ans += num.value;
                    break;
                }
            }
        }

        ans
    }

    fn part2(&self) -> i32 {
        let mut char_matrix: Vec<Vec<u8>> = Vec::new();

        let _example_lines = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
            "..........",
            "......$42.",
            "..........",
        ];

        for line in &self.lines {
            let mut bytes = line.clone().into_bytes();
            bytes.push(b'\n');

            char_matrix.push(bytes);
        }

        let numbers = self.get_numbers(&char_matrix);
        let gears = self.get_gears(&char_matrix);

        let mut ans = 0;
        let mut adj_numbers: Vec<i32>;
        for gear in &gears {
            adj_numbers = vec![];
            for num in &numbers {
                if gear.row == num.row - 1 || gear.row == num.row || gear.row == num.row + 1 {
                    if gear.col >= num.start_col - 1 && gear.col <= num.end_col + 1 {
                        adj_numbers.push(num.value);
                    }
                }
            }

            if adj_numbers.len() == 2 {
                ans += adj_numbers[0] * adj_numbers[1];
            }
        }

        ans
    }
}

impl Day3 {
    fn get_gears(&self, char_matrix: &Vec<Vec<u8>>) -> Vec<Gear> {
        let mut gears: Vec<Gear> = Vec::new();

        let mut row_index: i32 = 0;
        let mut col_index: i32;
        for row in char_matrix {
            col_index = 0;
            for _col in row {
                let c = char_matrix[row_index as usize][col_index as usize] as char;

                if c == '*' {
                    gears.push(Gear {
                        row: row_index,
                        col: col_index,
                    });
                }

                col_index += 1;
            }

            row_index += 1;
        }

        gears
    }

    fn is_position_adjecent_to_symbol(
        &self,
        char_matrix: &Vec<Vec<u8>>,
        position: (i32, i32),
    ) -> bool {
        let offsets = vec![-1, 0, 1];

        for o_row in &offsets {
            for o_col in &offsets {
                let row = position.0 as i32 + o_row;
                let col = position.1 as i32 + o_col;

                if row < 0 || col < 0 {
                    continue;
                }

                let offseted = char_matrix.get(row as usize);
                if offseted.is_none() {
                    continue;
                }
                let offseted = offseted.unwrap().get(col as usize);
                if offseted.is_none() {
                    continue;
                }

                let c = *offseted.unwrap();

                if !c.is_ascii_digit() && c != b'.' && c != b'\n' {
                    return true;
                }
            }
        }

        false
    }

    fn get_numbers(&self, char_matrix: &Vec<Vec<u8>>) -> Vec<Number> {
        let mut numbers: Vec<Number> = Vec::new();

        let mut num: String = "".to_string();

        let mut row_index: i32 = 0;
        let mut col_index: i32;

        let mut start_col = 0;

        for row in char_matrix {
            col_index = 0;
            for c in row {
                let c = *c as char;
                if c.is_ascii_digit() && c != '\n' {
                    if num.is_empty() {
                        start_col = col_index;
                    }

                    num.push(c);
                } else if !num.is_empty() {
                    let number = Number {
                        value: num.parse::<i32>().unwrap(),
                        row: row_index,
                        start_col: start_col,
                        end_col: col_index - 1,
                    };
                    numbers.push(number);
                    num = "".to_string();
                } else {
                    num = "".to_string();
                }

                col_index += 1;
            }
            num = "".to_string();
            row_index += 1;
        }

        numbers
    }
}
