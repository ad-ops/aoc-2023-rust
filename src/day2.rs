use std::time::Instant;

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (game, bag) = line.split_once(": ").unwrap();
            let game_number = game.split_once(' ').unwrap().1.parse::<u32>().unwrap();

            let mut is_possible = true;
            for set in bag.split("; ") {
                for cubes in set.split(", ") {
                    match cubes.split_once(' ').unwrap() {
                        (num, "red") => {
                            if num.parse::<u32>().unwrap() > 12 {
                                is_possible = false;
                                break;
                            }
                        }
                        (num, "green") => {
                            if num.parse::<u32>().unwrap() > 13 {
                                is_possible = false;
                                break;
                            }
                        }
                        (num, "blue") => {
                            if num.parse::<u32>().unwrap() > 14 {
                                is_possible = false;
                                break;
                            }
                        }
                        _ => panic!("should not exist!"),
                    }
                }
            }
            if is_possible {
                game_number
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, bag) = line.split_once(": ").unwrap();

            let (mut min_red, mut min_green, mut min_blue) = (0, 0, 0);
            for set in bag.split("; ") {
                for cubes in set.split(", ") {
                    let (num, color) = cubes.split_once(' ').unwrap();
                    match (num.parse::<u32>().unwrap(), color) {
                        (num, "red") => {
                            min_red = num.max(min_red);
                        }
                        (num, "green") => {
                            min_green = num.max(min_green);
                        }
                        (num, "blue") => {
                            min_blue = num.max(min_blue);
                        }
                        _ => panic!("should not exist!"),
                    }
                }
            }

            min_red * min_green * min_blue
        })
        .sum()
}

fn main() {
    println!("Running day2");

    let input = include_str!("../tests/02_1.txt");
    let time = Instant::now();
    let result = part1(input);
    println!("Part1 result: {result} in {}μs", time.elapsed().as_micros());

    let input = include_str!("../tests/02_1.txt");
    let time = Instant::now();
    let result: u32 = part2(input);
    println!("Part2 result: {result} in {}μs", time.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests/02_1_test.txt");
        let result = part1(input);
        let expected = 8;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../tests/02_1_test.txt");
        let result = part2(input);
        let expected = 2286;
        assert_eq!(result, expected);
    }
}
