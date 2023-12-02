use itertools::Itertools;
use regex::Regex;
use std::{sync::OnceLock, time::Instant};

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|char| char.is_ascii_digit())
                .filter_map(|d| d.to_digit(10))
                .collect_vec();

            let first = digits.first().expect("should be a digit");
            let last = digits.last().expect("should be a digit");
            first * 10 + last
        })
        .sum::<u32>()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .map(|(_i, line)| {
            static RE: OnceLock<Regex> = OnceLock::new();
            static REV_RE: OnceLock<Regex> = OnceLock::new();
            let first = RE
                .get_or_init(|| Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap())
                .find(line)
                .expect("should find a first value")
                .as_str();
            let reverse = line.chars().rev().collect::<String>();
            let last = REV_RE
                .get_or_init(|| Regex::new(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap())
                .find(&reverse)
                .expect("msg")
                .as_str();

            fn to_digit(pattern: &str) -> u32 {
                match pattern {
                    "one" | "eno" | "1" => 1,
                    "two" | "owt" | "2" => 2,
                    "three" | "eerht" | "3" => 3,
                    "four" | "ruof" | "4" => 4,
                    "five" | "evif" | "5" => 5,
                    "six" | "xis" | "6" => 6,
                    "seven" | "neves" | "7" => 7,
                    "eight" | "thgie" | "8" => 8,
                    "nine" | "enin" | "9" => 9,
                    _ => panic!("should never happen")
                }
            }

            let first = to_digit(first);
            let last = to_digit(last);
            first * 10 + last
        })
        .sum()
}

fn main() {
    println!("Running day1");

    let input = include_str!("../tests/01_1.txt");
    let time = Instant::now();
    let result = part1(input);
    println!("Part1 result: {result} in {}μs", time.elapsed().as_micros());

    let input = include_str!("../tests/01_1.txt");
    let time = Instant::now();
    let result: u32 = part2(input);
    println!("Part2 result: {result} in {}μs", time.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests/01_1_test.txt");
        let result = part1(input);
        let expected = 142;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../tests/01_2_test.txt");
        let result = part2(input);
        let expected = 281;
        assert_eq!(result, expected);
    }
}
