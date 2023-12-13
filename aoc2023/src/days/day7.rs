use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    iter::zip,
};

use crate::day::Day;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_string(string: String) -> HandType {
        let bytes = string.as_bytes();
        let mut set_bytes: HashSet<u8> = HashSet::new();
        for b in bytes {
            set_bytes.insert(*b);
        }

        let set_len = set_bytes.len();
        match set_len {
            1 => Self::FiveOfAKind,
            2 => {
                // Four of a kind OR Full House
                let char_map_values: Vec<i32> = map_chars(&string).into_values().collect();
                if char_map_values[0] == 4 || char_map_values[1] == 4 {
                    return Self::FourOfAKind;
                } else {
                    return Self::FullHouse;
                }
            }
            3 => {
                // Three of a kind or two pair
                let mut char_map_values: Vec<i32> = map_chars(&string).into_values().collect();
                char_map_values.sort();
                if char_map_values == vec![1, 1, 3] {
                    return Self::ThreeOfAKind;
                } else {
                    return Self::TwoPair;
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => panic!("Invalid Hand"),
        }
    }

    fn from_string_part2(string: String) -> HandType {
        let bytes = string.as_bytes();
        let mut set_bytes: HashSet<u8> = HashSet::new();
        for b in bytes {
            set_bytes.insert(*b);
        }

        let set_len = set_bytes.len();
        let char_map = map_chars(&string);
        let mut char_map_values: Vec<i32> = char_map.values().map(|x| *x).collect();
        let char_map_keys: Vec<u8> = char_map.keys().map(|x| *x).collect();

        match set_len {
            1 => Self::FiveOfAKind,
            2 => {
                // Four of a kind OR Full House
                if char_map_keys.contains(&b'J') {
                    return Self::FiveOfAKind;
                }

                if char_map_values[0] == 4 || char_map_values[1] == 4 {
                    return Self::FourOfAKind;
                } else {
                    return Self::FullHouse;
                }
            }
            3 => {
                // Three of a kind or two pair
                char_map_values.sort();
                if char_map_keys.contains(&b'J') {
                    if char_map_values == vec![1, 1, 3] {
                        return Self::FourOfAKind;
                    } else if char_map[&b'J'] == 2 {
                        return Self::FourOfAKind;
                    } else {
                        return Self::FullHouse;
                    }
                }

                if char_map_values == vec![1, 1, 3] {
                    return Self::ThreeOfAKind;
                } else {
                    return Self::TwoPair;
                }
            }
            4 => {
                if char_map_keys.contains(&b'J') {
                    return Self::ThreeOfAKind;
                }

                return Self::OnePair;
            }
            5 => {
                if char_map_keys.contains(&b'J') {
                    return Self::OnePair;
                }

                return Self::HighCard;
            }
            _ => panic!("Invalid Hand"),
        }
    }
}

fn map_chars(string: &String) -> HashMap<u8, i32> {
    let mut map: HashMap<u8, i32> = HashMap::new();
    for c in string.bytes() {
        if map.contains_key(&c) {
            let val = map.get_mut(&c).unwrap();
            *val += 1;
        } else {
            map.insert(c, 1);
        }
    }

    map
}

#[derive(Clone)]
struct Hand {
    hand_type: HandType,
    hand_string: String,
    bid: i32,
}

impl Hand {
    fn compare(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            // Comapre strings
            return compare_hand_strings(&self.hand_string, &other.hand_string);
        } else if self.hand_type < other.hand_type {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }

    fn compare_part2(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            // Comapre strings
            return compare_hand_strings_part2(&self.hand_string, &other.hand_string);
        } else if self.hand_type < other.hand_type {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    }
}

fn compare_hand_strings(left: &String, right: &String) -> Ordering {
    for (cl, cr) in zip(left.bytes(), right.bytes()).collect::<Vec<(u8, u8)>>() {
        let ord = compare_card(cl as char, cr as char);
        if ord == Ordering::Equal {
            continue;
        } else {
            return ord;
        }
    }

    Ordering::Equal
}

fn compare_hand_strings_part2(left: &String, right: &String) -> Ordering {
    for (cl, cr) in zip(left.bytes(), right.bytes()).collect::<Vec<(u8, u8)>>() {
        let ord = compare_card_part2(cl as char, cr as char);
        if ord == Ordering::Equal {
            continue;
        } else {
            return ord;
        }
    }

    Ordering::Equal
}

fn compare_card(left: char, right: char) -> Ordering {
    if left == right {
        return Ordering::Equal;
    }

    if get_card_strength(left) > get_card_strength(right) {
        return Ordering::Greater;
    }

    Ordering::Less
}

fn compare_card_part2(left: char, right: char) -> Ordering {
    if left == right {
        return Ordering::Equal;
    }

    if get_card_strength_part2(left) > get_card_strength_part2(right) {
        return Ordering::Greater;
    }

    Ordering::Less
}

fn get_card_strength(card: char) -> u8 {
    if card.is_numeric() {
        return card.to_string().parse::<u8>().unwrap();
    } else {
        let strength = match card {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card!"),
        };

        return strength;
    }
}
fn get_card_strength_part2(card: char) -> u8 {
    if card.is_numeric() {
        return card.to_string().parse::<u8>().unwrap();
    } else {
        let strength = match card {
            'T' => 10,
            'J' => 0,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid card!"),
        };

        return strength;
    }
}

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
        let mut ordered: Vec<Hand> = Vec::new();

        for line in &self.lines {
            let split: Vec<&str> = line.split_whitespace().collect();
            let hand_type = HandType::from_string(split[0].to_string());
            let bid = split[1].parse::<i32>().unwrap();
            ordered.push(Hand {
                hand_type: hand_type,
                hand_string: split[0].to_string(),
                bid: bid,
            });
        }

        ordered.sort_by(|a, b| a.compare(b));

        let mut ans = 0;
        let mut rank = 1;
        for hand in &ordered {
            ans += hand.bid * rank;

            rank += 1;
        }

        ans
    }

    fn part2(&self) -> i32 {
        let mut ordered: Vec<Hand> = Vec::new();

        for line in &self.lines {
            let split: Vec<&str> = line.split_whitespace().collect();
            let hand_type = HandType::from_string_part2(split[0].to_string());
            let bid = split[1].parse::<i32>().unwrap();
            ordered.push(Hand {
                hand_type: hand_type,
                hand_string: split[0].to_string(),
                bid: bid,
            });
        }

        ordered.sort_by(|a, b| a.compare_part2(b));

        let mut ans = 0;
        let mut rank = 1;
        for hand in &ordered {
            ans += hand.bid * rank;

            rank += 1;
        }

        ans
    }
}
