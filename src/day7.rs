use itertools::Itertools;
use std::time::Instant;

#[derive(Debug)]
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
            'a' => Self::Ace,
            'k' => Self::King,
            'q' => Self::Queen,
            'j' => Self::Jack,
            't' => Self::Number(10),
            c if c.is_ascii_digit() => Self::Number(c.to_digit(10).expect("should be a number")),
            _ => panic!("invalid card"),
        }
    }
}

impl Card {
    fn rating(&self) -> u32 {
        match self {
            Self::Number(n) => *n,
            Self::Jack => 11,
            Self::Queen => 12,
            Self::King => 13,
            Self::Ace => 14,
        }
    }
}

impl Into<u32> for Card {
    fn into(self) -> u32 {
        self.rating()
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rating().cmp(&other.rating())
    }
}

enum Hand {
    HighCard(Card),
    OnePair(Card),
    TwoPairs(Card, Card),
    ThreeOfAKind(Card),
    FullHouse(Card, Card),
    FourOfAKind(Card),
    FiveOfAKind(Card),
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::FiveOfAKind(c1), Self::FiveOfAKind(c2)) => c1.cmp(c2),
            (Self::FourOfAKind(c1), Self::FourOfAKind(c2)) => c1.cmp(c2),
            (Self::ThreeOfAKind(c1), Self::ThreeOfAKind(c2)) => c1.cmp(c2),
            (Self::TwoPairs(c1, c2), Self::TwoPairs(c3, c4)) => c1.cmp(c3).then(c2.cmp(c4)),
            (Self::OnePair(c1), Self::OnePair(c2)) => c1.cmp(c2),
            (Self::HighCard(c1), Self::HighCard(c2)) => c1.cmp(c2),
            _ => panic!("invalid hand"),
        }
    }
}

#[derive(Debug)]
struct Deck {
    hand: (Card, Card, Card, Card, Card),
    bid: u32,
}

impl From<&str> for Deck {
    fn from(input: &str) -> Self {
        let (hand, bid) = input
            .split_once(' ')
            .expect("should be delimited");
        let (hand, bid) = (
            hand.chars().map(Card::from).collect_tuple().expect("shoudl be 5 cards"),
            bid.parse::<u32>().expect("should be a number"),
        );

        Self { hand, bid }
    }
}

impl Deck {
    fn rating(&self) -> u32 {
        match self.hand { 
           (c, c, c, c, c) => c.into(),
           _ => 0,
        }
    }
}


fn part1(input: &str) -> u64 {
    let decks = input
        .lines()
        .map(Deck::from)
        .collect_vec();

    for deck in decks {
        println!("{:?} - {}", deck, deck.rating());
    }

    2
}

fn part2(input: &str) -> u64 {
    2
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
        let expected = 71503;
        assert_eq!(result, expected);
    }
}
