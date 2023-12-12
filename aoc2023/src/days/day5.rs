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

    fn reverse_contains(&self, value: i64) -> bool {
        if value >= self.destination_start && value < self.destination_start + self.length {
            return true;
        }
        false
    }

    fn translate(&self, value: i64) -> i64 {
        let offset = value - self.source_start;

        self.destination_start + offset
    }

    fn reverse_translate(&self, value: i64) -> i64 {
        let offset = value - self.destination_start;

        self.source_start + offset
    }

    fn source_last(&self) -> i64 {
        self.source_start + self.length - 1
    }

    fn destination_last(&self) -> i64 {
        self.destination_start + self.length - 1
    }
}

#[derive(Clone, Copy)]
struct Range {
    start: i64,
    length: i64,
}

impl Range {
    fn first(&self) -> i64 {
        self.start
    }
    fn last(&self) -> i64 {
        self.start + self.length - 1
    }

    fn from_start_end(start: i64, end: i64) -> Range {
        Range { start: start, length: end - start + 1 }
    }

    fn contains(&self, value: i64) -> bool {
        if value >= self.first() && value <= self.last() {
            return true;
        }
        false
    }

    fn print(&self) {
        print!("{} -> {}", self.first(), self.last());
    }
}

pub struct Day5 {
    pub lines: Vec<String>,
    seeds: Vec<i64>,
    sections: Vec<Vec<Map>>,
}

impl Day for Day5 {
    fn new() -> Self {
        let lines = Self::read_input(5, false);

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
        let mut ranges: Vec<Range> = self.get_seed_ranges();

        // println!("Reverse brute {}", self.part2_reverse_brute());

        for sec in &self.sections {
            let mut new_ranges: Vec<Range> = Vec::new();

            for range in &ranges {
                let mut splits: Vec<i64> = Vec::new();
                splits.push(range.first());

                for map in sec {
                    if range.contains(map.source_start) && range.first() != map.source_start {
                        splits.push(map.source_start);
                    }
                    if range.contains(map.source_last() + 1) {
                        splits.push(map.source_last() + 1);
                    }
                }

                if *splits.last().unwrap() != range.last() + 1 {
                    splits.push(range.last() + 1);
                }

                
                for i in 0..splits.len() - 1 {
                    let start = splits[i];
                    let end = splits[i + 1];
                    if start == end {
                        continue;
                    }

                    new_ranges.push(Range::from_start_end(start, end - 1));
                }
            }

            for range in &mut new_ranges {
                for map in sec {
                    if map.contains(range.first()) {
                        range.start = map.translate(range.start);
                    }
                }
            }

            ranges = new_ranges;
        }

        let min = ranges.iter().map(|range| range.start).min().unwrap();

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

    fn get_seed_ranges(&self) -> Vec<Range> {
        let mut ranges: Vec<Range> = Vec::new();
        let mut i = 0;
        while i < self.seeds.len() {
            ranges.push(Range { start: self.seeds[i], length: self.seeds[i + 1] });

            i += 2;
        }

        ranges
    }

    fn reverse_map_to_seed(&self, location: i64) -> i64 {
        let mut value = location;
        for sec in self.sections.iter().rev() {
            for map in sec {
                if map.reverse_contains(value) {
                    value = map.reverse_translate(value);
                    break;
                }
            }
        }

        value
    }

    fn part2_reverse_brute(&self) -> i32 {
        let seed_ranges = self.get_seed_ranges();
        let mut loc = 0;
        let mut flag = true;
        while flag {
            let seed = self.reverse_map_to_seed(loc);
            for range in &seed_ranges {
                if range.contains(seed) {
                    flag = false;
                    break;
                }
            }

            loc += 1;
        }

        (loc - 1) as i32
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