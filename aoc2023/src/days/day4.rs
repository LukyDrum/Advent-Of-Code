use crate::day::Day;

struct Scratchcard {
    winning: Vec<i32>,
    my_numbers: Vec<i32>,
    copies: i32,
}

pub struct Day4 {
    pub lines: Vec<String>,
}

impl Day for Day4 {
    fn new() -> Self {
        Day4 {
            lines: Self::read_input(4),
        }
    }

    fn part1(&self) -> i32 {
        self.lines
            .iter()
            .map(|line| {
                let scratchcard = scratchcard_from_line(line);

                let mut value = 0;
                for wnum in &scratchcard.winning {
                    for mnum in &scratchcard.my_numbers {
                        if mnum > wnum {
                            break;
                        }

                        if wnum == mnum {
                            if value == 0 {
                                value = 1;
                            } else {
                                value *= 2;
                            }
                        }
                    }
                }

                value
            })
            .sum::<i32>()
    }

    fn part2(&self) -> i32 {
        let mut cards: Vec<Scratchcard> = self
            .lines
            .iter()
            .map(|line| scratchcard_from_line(line))
            .collect();

        for cur in 0..cards.len() {
            let matches = number_of_matches_on_card(&cards[cur]);
            for i in (cur + 1)..(cur + matches as usize + 1) {
                cards[i].copies += cards[cur].copies;
            }
        }

        cards.iter().fold(0, |acc, card| acc + card.copies)
    }
}

fn scratchcard_from_line(line: &String) -> Scratchcard {
    let split: Vec<&str> = line.split(":").collect();
    let numbers: Vec<&str> = split[1].split(" | ").filter(|n| !n.is_empty()).collect();

    let mut scratchcard: Scratchcard = Scratchcard {
        winning: vec![],
        my_numbers: vec![],
        copies: 1,
    };

    scratchcard.winning = numbers[0]
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    scratchcard.my_numbers = numbers[1]
        .split_ascii_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    scratchcard.winning.sort_unstable();
    scratchcard.my_numbers.sort_unstable();

    scratchcard
}

fn number_of_matches_on_card(scratchcard: &Scratchcard) -> i32 {
    let mut matches = 0;
    for wnum in &scratchcard.winning {
        for mnum in &scratchcard.my_numbers {
            if mnum > wnum {
                break;
            }

            if wnum == mnum {
                matches += 1;
            }
        }
    }

    matches
}
