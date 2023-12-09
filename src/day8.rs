mod util;

use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

enum Instruction {
    Left,
    Right,
}
impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("instruction is not L or R"),
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut network: HashMap<_, _> = HashMap::new();

    let mut lines = input.lines();
    let instructions = lines
        .next()
        .expect("should be a first line")
        .chars()
        .map(Instruction::from)
        .cycle();
    for line in lines.skip(1) {
        let node = &line[..3];
        let left = &line[7..10];
        let right = &line[12..15];
        network.insert(node, (left, right));
    }

    let mut steps = 0;
    let mut current_node = "AAA";
    for instruction in instructions {
        let node_map = network.get(current_node).expect("should exist node");
        current_node = match instruction {
            Instruction::Left => node_map.0,
            Instruction::Right => node_map.1,
        };
        steps += 1;
        if current_node == "ZZZ" {
            break;
        }
    }

    steps
}

fn part2(input: &str) -> u64 {
    let mut network: HashMap<_, _> = HashMap::new();

    let mut lines = input.lines();
    let instructions = lines
        .next()
        .expect("should be a first line")
        .chars()
        .map(Instruction::from)
        .cycle();
    for line in lines.skip(1) {
        let node = &line[..3];
        let left = &line[7..10];
        let right = &line[12..15];
        network.insert(node, (left, right));
    }
    let starting_nodes = network
        .keys()
        .filter(|node| node.ends_with('A'))
        .cloned()
        .collect_vec();

    let mut steps: u64 = 0;
    let mut current_nodes = starting_nodes.clone();
    let mut stable_cycle = HashMap::new();
    for instruction in instructions {
        let mut updated_nodes = Vec::new();
        for node in current_nodes {
            let node_map = network.get(node).expect("should exist node");
            let current_node = match instruction {
                Instruction::Left => node_map.0,
                Instruction::Right => node_map.1,
            };
            if node.ends_with('Z') && !stable_cycle.contains_key(node) {
                stable_cycle.insert(node, steps);
            }
            updated_nodes.push(current_node);
        }

        current_nodes = updated_nodes.clone();
        updated_nodes.clear();
        steps += 1;

        // found all cycles
        if stable_cycle.len() == starting_nodes.len() {
            break;
        }
        // safety
        if steps > 10_000_000 {
            println!("could not find anything...");
            break;
        }
    }

    stable_cycle
        .values()
        .cloned()
        .reduce(util::lcm)
        .expect("should have a large enough vector")
}

fn main() {
    println!("Running AoC 2023...");

    let input = include_str!("../tests/08_1.txt");
    let time = Instant::now();
    let result = part1(input);
    println!("Part1 result: {result} in {}μs", time.elapsed().as_micros());

    let input = include_str!("../tests/08_1.txt");
    let time = Instant::now();
    let result = part2(input);
    println!("Part2 result: {result} in {}μs", time.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests/08_1_test.txt");
        let result = part1(input);
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../tests/08_2_test.txt");
        let result = part2(input);
        let expected = 6;
        assert_eq!(result, expected);
    }
}
