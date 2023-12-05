use std::{collections::HashMap, time::Instant};

use itertools::Itertools;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    drawn_numbers: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Self {
        let (card, numbers) = line.split_once(':').expect("Should be card!");
        let id = card
            .replace("Card", "")
            .trim()
            .parse::<u32>()
            .expect("Should be number!");
        let (winnings, draws) = numbers
            .split_once('|')
            .expect("Should be winnings and drawn!");
        let winning_numbers = winnings
            .split_whitespace()
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().expect("Should be number!"))
            .collect::<Vec<u32>>();
        let drawn_numbers = draws
            .split_whitespace()
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<u32>().expect("Should be number!"))
            .collect_vec();
        Self {
            id,
            winning_numbers,
            drawn_numbers,
        }
    }

    fn check_winning_numbers(&self) -> Vec<u32> {
        self.winning_numbers
            .iter()
            .filter(|n| self.drawn_numbers.contains(n))
            .cloned()
            .collect()
    }

    fn calculate_score(&self) -> u32 {
        let number_of_winnings = self.check_winning_numbers().len();
        if number_of_winnings == 0 {
            return 0;
        }
        2u32.pow(number_of_winnings as u32 - 1)
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(Card::new)
        .map(|card| card.calculate_score())
        .sum()
}

fn part2(input: &str) -> u32 {
    let cards = input.lines().map(Card::new).collect_vec();

    let mut score_cards: HashMap<u32, u32> = cards.iter().map(|card| (card.id, 1)).collect();

    for card in cards {
        let number_of_wins = card.check_winning_numbers().len() as u32;
        let amount_of_cards = *score_cards.get(&card.id).unwrap();
        for id in (card.id + 1)..=(card.id + number_of_wins) {
            if let Some(n) = score_cards.get_mut(&id) {
                *n += amount_of_cards;
            }
        }
    }

    score_cards.values().copied().sum()
}

fn main() {
    println!("Running day3");

    let input = include_str!("../tests/04_1.txt");
    let time = Instant::now();
    let result = part1(input);
    println!("Part1 result: {result} in {}μs", time.elapsed().as_micros());

    let input = include_str!("../tests/04_1.txt");
    let time = Instant::now();
    let result: u32 = part2(input);
    println!("Part2 result: {result} in {}μs", time.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests/04_1_test.txt");
        let result = part1(input);
        let expected = 13;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../tests/04_1_test.txt");
        let result = part2(input);
        let expected = 30;
        assert_eq!(result, expected);
    }
}
