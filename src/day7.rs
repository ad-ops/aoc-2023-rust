use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}
impl From<char> for Card {
    fn from(input: char) -> Self {
        match input {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Number(10),
            '2'..='9' => Self::Number(input.to_digit(10).expect("should be a number")),
            _ => panic!("invalid card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum JokerCard {
    Joker,
    Number(u32),
    Queen,
    King,
    Ace,
}
impl From<char> for JokerCard {
    fn from(input: char) -> Self {
        match input {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Joker,
            'T' => Self::Number(10),
            '2'..='9' => Self::Number(input.to_digit(10).expect("should be a number")),
            _ => panic!("invalid card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Hand {
    HighCard([Card; 5]),
    OnePair([Card; 5]),
    TwoPairs([Card; 5]),
    ThreeOfAKind([Card; 5]),
    FullHouse([Card; 5]),
    FourOfAKind([Card; 5]),
    FiveOfAKind([Card; 5]),
}
impl From<[Card; 5]> for Hand {
    fn from(value: [Card; 5]) -> Self {
        let mut counts: HashMap<Card, u32> = HashMap::new();
        for card in value {
            counts
                .entry(card)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let sorted_counts = counts.values().sorted().rev().collect_vec();

        match sorted_counts[..] {
            [5, ..] => Self::FiveOfAKind(value),
            [4, ..] => Self::FourOfAKind(value),
            [3, 2, ..] => Self::FullHouse(value),
            [3, ..] => Self::ThreeOfAKind(value),
            [2, 2, ..] => Self::TwoPairs(value),
            [2, ..] => Self::OnePair(value),
            [1, ..] => Self::HighCard(value),
            _ => panic!("should not be able to be empty or more than 5"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum JokerHand {
    HighCard([JokerCard; 5]),
    OnePair([JokerCard; 5]),
    TwoPairs([JokerCard; 5]),
    ThreeOfAKind([JokerCard; 5]),
    FullHouse([JokerCard; 5]),
    FourOfAKind([JokerCard; 5]),
    FiveOfAKind([JokerCard; 5]),
}
impl From<[JokerCard; 5]> for JokerHand {
    fn from(value: [JokerCard; 5]) -> Self {
        let mut counts: HashMap<JokerCard, u32> = HashMap::new();
        for card in value.into_iter().filter(|card| card != &JokerCard::Joker) {
            counts
                .entry(card)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let jokers = value
            .iter()
            .filter(|card| card == &&JokerCard::Joker)
            .count() as u32;
        let mut sorted_counts = counts.values().sorted().rev().cloned().collect_vec();
        if sorted_counts.is_empty() {
            sorted_counts.push(jokers);
        } else {
            sorted_counts[0] += jokers;
        }

        match sorted_counts[..] {
            [5, ..] => Self::FiveOfAKind(value),
            [4, ..] => Self::FourOfAKind(value),
            [3, 2, ..] => Self::FullHouse(value),
            [3, ..] => Self::ThreeOfAKind(value),
            [2, 2, ..] => Self::TwoPairs(value),
            [2, ..] => Self::OnePair(value),
            [1, ..] => Self::HighCard(value),
            _ => panic!("should not be able to be empty or more than 5"),
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|input| {
            let (hand, bid) = input.split_once(' ').expect("should be delimited");
            let (hand, bid): ([Card; 5], u32) = (
                hand.chars()
                    .map(Card::from)
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Vec does not contain 5 cards"),
                bid.parse::<u32>().expect("should be a number"),
            );
            (hand, bid)
        })
        .map(|(hand, bid)| (hand.into(), bid))
        .collect_vec();
    hands.sort_by_key(|hand| hand.0);

    let mut total_winnings = 0;
    for (placing, (_hand, bid)) in hands.iter().enumerate() {
        let winning = bid * (placing + 1) as u32;
        total_winnings += winning;
    }

    total_winnings
}

fn part2(input: &str) -> u32 {
    let mut hands: Vec<(JokerHand, u32)> = input
        .lines()
        .map(|input| {
            let (hand, bid) = input.split_once(' ').expect("should be delimited");
            let (hand, bid): ([JokerCard; 5], u32) = (
                hand.chars()
                    .map(JokerCard::from)
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("Vec does not contain 5 cards"),
                bid.parse::<u32>().expect("should be a number"),
            );
            (hand, bid)
        })
        .map(|(hand, bid)| (hand.into(), bid))
        .collect_vec();
    hands.sort_by_key(|hand| hand.0);

    let mut total_winnings = 0;
    for (placing, (_hand, bid)) in hands.iter().enumerate() {
        let winning = bid * (placing + 1) as u32;
        total_winnings += winning;
    }

    total_winnings
}

fn main() {
    println!("Running AoC 2023...");

    let input = include_str!("../tests/07_1.txt");
    let time = Instant::now();
    let result = part1(input);
    println!("Part1 result: {result} in {}μs", time.elapsed().as_micros());

    let input = include_str!("../tests/07_1.txt");
    let time = Instant::now();
    let result = part2(input);
    println!("Part2 result: {result} in {}μs", time.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests/07_1_test.txt");
        let result = part1(input);
        let expected = 6440;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../tests/07_1_test.txt");
        let result = part2(input);
        let expected = 5905;
        assert_eq!(result, expected);
    }
}
