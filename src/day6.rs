use itertools::Itertools;
use std::time::Instant;

fn calculate_distance(charge_time: u64, total_time: u64) -> u64 {
    if charge_time > total_time {
        return 0;
    }

    let velocity = charge_time;
    let remaining_time = total_time - charge_time;

    velocity * remaining_time
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace();
    let distances = lines.next().unwrap().split_whitespace();
    let races = times
        .zip(distances)
        .skip(1)
        .map(|(time, distance)| {
            (
                time.parse::<u64>().expect("should be a number"),
                distance.parse::<u64>().expect("should be a number"),
            )
        })
        .collect_vec();

    let mut all_possible_strategies = Vec::new();
    for (record_time, record_distance) in races {
        let mut possible_strategies = Vec::new();
        for charge_time in 1..record_time {
            let distance = calculate_distance(charge_time, record_time);
            if distance > record_distance {
                possible_strategies.push(charge_time);
            }
        }
        all_possible_strategies.push(possible_strategies.len() as u64);
    }

    all_possible_strategies
        .into_iter()
        .reduce(|acc, e| acc * e)
        .expect("should be at least 2 strategies")
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let record_time = lines
        .next()
        .unwrap()
        .replace(' ', "")
        .split_once(':')
        .expect("should be delimited")
        .1
        .parse::<u64>()
        .expect("should be a number");
    let record_distance = lines
        .next()
        .unwrap()
        .replace(' ', "")
        .split_once(':')
        .expect("should be delimited")
        .1
        .parse::<u64>()
        .expect("should be a number");

    let mut possible_strategies = 0;
    for charge_time in 1..record_time {
        let distance = calculate_distance(charge_time, record_time);
        if distance > record_distance {
            possible_strategies += 1;
        }
    }

    possible_strategies
}

fn main() {
    println!("Running AoC 2023...");

    let input = include_str!("../tests/06_1.txt");
    let time = Instant::now();
    let result = part1(input);
    println!("Part1 result: {result} in {}μs", time.elapsed().as_micros());

    let input = include_str!("../tests/06_1.txt");
    let time = Instant::now();
    let result = part2(input);
    println!("Part2 result: {result} in {}μs", time.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests/06_1_test.txt");
        let result = part1(input);
        let expected = 288;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../tests/06_1_test.txt");
        let result = part2(input);
        let expected = 71503;
        assert_eq!(result, expected);
    }
}
