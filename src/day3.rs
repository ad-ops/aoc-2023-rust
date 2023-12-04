use std::{collections::HashMap, time::Instant};

type Coordinate = (usize, usize);

fn part1(input: &str) -> u32 {
    // create coordinates of all symbols
    let mut number_coordinates: HashMap<Coordinate, u32> = HashMap::new();
    let mut symbol_coordinates: HashMap<Coordinate, char> = HashMap::new();
    for (line_number, line) in input.lines().enumerate() {
        let mut number_pos = (0, 0);
        let mut number = String::new();
        for (char_number, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                number_pos = (line_number, char_number);
                number.push(char);
            } else {
                if char != '.' {
                    symbol_coordinates.insert((line_number, char_number), char);
                }
                if !number.is_empty() {
                    let length = number.len();
                    let parsed = number.parse::<u32>().expect("Should be number!");
                    let left_aligned_coord = (number_pos.0, number_pos.1 + 1 - length);
                    number_coordinates.insert(left_aligned_coord, parsed);
                    number.clear();
                }
            }
        }
        if !number.is_empty() {
            let length = number.len();
            let parsed = number.parse::<u32>().expect("Should be number!");
            let left_aligned_coord = (number_pos.0, number_pos.1 + 1 - length);
            number_coordinates.insert(left_aligned_coord, parsed);
            number.clear();
        }
    }

    // println!("{number_coordinates:?}");
    // println!("{symbol_coordinates:?}");
    // println!("numbers: {}, symbols: {}", number_coordinates.len(), symbol_coordinates.len());

    // find adjecent symbols
    let mut numbers: Vec<u32> = Vec::new();
    for (number_coordinate, number) in number_coordinates.iter() {
        let number_length = number.to_string().len();
        for (symbol_coordinate, _symbol) in symbol_coordinates.iter() {
            if is_adjecent(*number_coordinate, number_length, *symbol_coordinate) {
                numbers.push(*number);
                break;
            }
        }
    }

    numbers.iter().sum()
}

/// ....*.. (0,4)
/// ..123.. (1,2), 3
/// .......
fn is_adjecent(
    number_coordinate: Coordinate,
    number_length: usize,
    symbol_coordinate: Coordinate,
) -> bool {
    let on_adjecent_line = number_coordinate.0.abs_diff(symbol_coordinate.0) < 2;

    let column_range = number_coordinate.1..=(number_coordinate.1 + number_length);
    let on_adjecent_column = column_range.contains(&symbol_coordinate.1)
        || number_coordinate.1.abs_diff(symbol_coordinate.1) == 1; // could be before first column coordinate

    on_adjecent_line && on_adjecent_column
}

fn part2(input: &str) -> u32 {
    // create coordinates of all symbols
    let mut number_coordinates: HashMap<Coordinate, u32> = HashMap::new();
    let mut gear_coordinates: HashMap<Coordinate, char> = HashMap::new();
    for (line_number, line) in input.lines().enumerate() {
        let mut number_pos = (0, 0);
        let mut number = String::new();
        for (char_number, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                number_pos = (line_number, char_number);
                number.push(char);
            } else {
                if char == '*' {
                    gear_coordinates.insert((line_number, char_number), char);
                }
                if !number.is_empty() {
                    let length = number.len();
                    let parsed = number.parse::<u32>().expect("Should be number!");
                    let left_aligned_coord = (number_pos.0, number_pos.1 + 1 - length);
                    number_coordinates.insert(left_aligned_coord, parsed);
                    number.clear();
                }
            }
        }
        if !number.is_empty() {
            let length = number.len();
            let parsed = number.parse::<u32>().expect("Should be number!");
            let left_aligned_coord = (number_pos.0, number_pos.1 + 1 - length);
            number_coordinates.insert(left_aligned_coord, parsed);
            number.clear();
        }
    }

    // println!("{number_coordinates:?}");
    // println!("{gear_coordinates:?}");
    // println!("numbers: {}, gears: {}", number_coordinates.len(), gear_coordinates.len());

    // find adjecent symbols
    let mut numbers: Vec<u32> = Vec::new();
    let mut gear_numbers: Vec<u32> = Vec::new();
    for (gear_coordinate, _symbol) in gear_coordinates.iter() {
        for (number_coordinate, number) in number_coordinates.iter() {
            let number_length = number.to_string().len();
            if is_adjecent(*number_coordinate, number_length, *gear_coordinate) {
                gear_numbers.push(*number);
                if gear_numbers.len() > 2 {
                    break;
                }
            }
        }
        if gear_numbers.len() == 2 {
            numbers.push(gear_numbers[0] * gear_numbers[1]);
        }
        gear_numbers.clear();
    }

    numbers.iter().sum()
}

fn main() {
    println!("Running day3");

    let input = include_str!("../tests/03_1.txt");
    let time = Instant::now();
    let result = part1(input);
    println!("Part1 result: {result} in {}μs", time.elapsed().as_micros());

    let input = include_str!("../tests/03_1.txt");
    let time = Instant::now();
    let result: u32 = part2(input);
    println!("Part2 result: {result} in {}μs", time.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_adjecent_full_test() {
        let number_length = 3; //123;
        let number_coordinate = (1, 1);
        for line in 0..=2 {
            for column in 0..=(number_length + 1) {
                assert!(is_adjecent(
                    number_coordinate,
                    number_length,
                    (line, column)
                ));
            }
        }
    }

    #[test]
    fn is_adjecent_edge_test() {
        let number_length = 3; //123;
        let number_coordinate = (0, 0);
        for line in 0..=1 {
            for column in 0..=number_length {
                assert!(is_adjecent(
                    number_coordinate,
                    number_length,
                    (line, column)
                ));
            }
        }
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../tests/03_1_test.txt");
        let result = part1(input);
        let expected = 4361;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../tests/03_1_test.txt");
        let result = part2(input);
        let expected = 467835;
        assert_eq!(result, expected);
    }
}
