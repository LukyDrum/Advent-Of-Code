use crate::day::Day;


struct Map {
    destination_start: i64,
    source_start: i64,
    length: i64
}

impl Map {
    fn contains(&self, value: i64) -> bool {
        if value >= self.source_start && value < self.source_start + self.length {
            return true
        }
        false
    }

    fn translate(&self, value: i64) -> i64 {
        let offset = value - self.source_start;

        self.destination_start + offset
    }
}

pub struct Day5 {
    pub lines: Vec<String>,
    seeds: Vec<i64>,
    sections: Vec<Vec<Map>>,
}

impl Day for Day5 {
    fn new() -> Self {
        let lines = Self::read_input(5);

        Day5 {
            lines: lines,
            seeds: Vec::new(),
            sections: Vec::new(),
        }
    }

    fn setup(&mut self) -> () {
        self.seeds = self.lines[0].split(":").collect::<Vec<&str>>()[1].split_ascii_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
        self.sections = lines_to_sections(&self.lines);
    }

    fn part1(&self) -> i32 {
        self.seeds.iter().map(|seed| self.get_location_of_seed(*seed)).min().unwrap() as i32
    }

    fn part2(&self) -> i32 {
        -1
    }
}

impl Day5 {
    fn get_location_of_seed(&self, seed: i64) -> i64 {
        let mut value = seed;
        for sec in &self.sections {
            for map in sec {
                if map.source_start > value {
                    break;
                }

                if map.contains(value) {
                    value = map.translate(value);
                    break;
                }
            }
        }

        value
    }
}

fn line_to_map(line: &str) -> Map {
    let split: Vec<i64> = line.split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();

    Map { destination_start: split[0], source_start: split[1], length: split[2] }
}

fn lines_to_sections(lines: &Vec<String>) -> Vec<Vec<Map>> {
    let mut sections: Vec<Vec<Map>> = Vec::new();
    for _ in 0..7 {
        sections.push(Vec::new());
    }
    let mut cur_section = 0;

    for line in &lines[1..] {
        if line.ends_with(":") {
            if !sections[0].is_empty() {
                cur_section += 1;
            }
        }
        else {
            sections[cur_section].push(
                line_to_map(&line)
            );
        }
    }

    for sec in &mut sections {
        sec.sort_unstable_by(|a, b| a.source_start.partial_cmp(&b.source_start).unwrap());
    }

    sections
}