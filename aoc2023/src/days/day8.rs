use std::collections::HashMap;

use crate::day::Day;

#[derive(PartialEq, Clone, Copy)]
enum Instruction {
    Left,
    Right
}

pub struct Day8 {
    pub lines: Vec<String>,
    instructions: Vec<Instruction>,
    nodes: HashMap<String, (String, String)>,
}

impl Day for Day8 {
    fn new() -> Self {
        Day8 {
            lines: Self::read_input(8, false),
            instructions: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    fn part1(&self) -> i32 {
        self.get_route_length("AAA".to_string(), false)
    }

    fn part2(&self) -> i32 {
        // Find nodes ending with A
        let mut cur_nodes: Vec<String> = Vec::new();
        for node in self.nodes.keys() {
            if node.ends_with("A") {
                cur_nodes.push(node.to_string());
            }
        }

        let steps: Vec<i32> = cur_nodes.iter().map(|node| self.get_route_length(node.to_string(), true)).collect();

        -1
    }

    fn setup(&mut self) -> () {
        let ins_line = &self.lines[0];
        for c in ins_line.bytes() {
            if c == b'L' {
                self.instructions.push(Instruction::Left);
            }
            else if c == b'R' {
                self.instructions.push(Instruction::Right);
            }
        }

        let nodes = &self.lines[1..];
        for node in nodes {
            let split: Vec<&str> = node.split(" = ").collect();
            let from = split[0].to_string();
            let split: Vec<&str> = split[1].split(", ").collect();
            let left = split[0].replace("(", "");
            let right = split[1].replace(")", "");

            self.nodes.insert(from, (left, right));
        }
    }
}

impl Day8 {
    fn get_route_length(&self, start_node: String, is_part2: bool) -> i32 {
        let mut num_of_steps = 0;
        let mut cur_node = start_node;
        let mut instruction_iter = self.instructions.iter().cycle();

        while (!is_part2 && cur_node != "ZZZ") || (is_part2 && !cur_node.ends_with("Z")) {
            let next_pair = self.nodes.get(&cur_node).unwrap();
            let next_instruction = *instruction_iter.next().unwrap();

            if next_instruction == Instruction::Left {
                cur_node = next_pair.0.to_string();
            }
            else {
                cur_node = next_pair.1.to_string();
            }

            num_of_steps += 1;
        }

        num_of_steps
    }
}
