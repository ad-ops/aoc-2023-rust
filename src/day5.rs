use itertools::Itertools;
use rayon::prelude::*;
use std::{collections::HashMap, ops::Range, time::Instant};

#[derive(Debug)]
struct MapRange {
    source_range: Range<u32>,
    destination_start: u32,
}
impl MapRange {
    fn new(source: u32, destination: u32, range: u32) -> Self {
        Self {
            source_range: source..source + range,
            destination_start: destination,
        }
    }
    fn get_destination(&self, value: u32) -> Option<u32> {
        if self.source_range.contains(&value) {
            let offset = value - self.source_range.start;
            Some(self.destination_start + offset)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Map {
    _name: String,
    ranges: Vec<MapRange>,
}
impl Map {
    fn new(name: String) -> Self {
        Self {
            _name: name,
            ranges: Vec::new(),
        }
    }
    fn get_destination(&self, value: u32) -> u32 {
        for range in &self.ranges {
            if let Some(destination) = range.get_destination(value) {
                return destination;
            }
        }
        value
    }
}

fn part1(input: &str) -> u32 {
    let mut input = input.split("\n\n");
    let seeds = input
        .next()
        .expect("should be one line")
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|num| num.parse::<u32>().expect("should be numbers"))
        .collect_vec();

    let mut maps: HashMap<String, Map> = HashMap::new();
    for map in input {
        let mut lines = map.lines();
        let map_type = lines
            .next()
            .expect("should be map type")
            .replace(" map:", "");
        let mut map = Map::new(map_type.clone());
        for line in lines {
            let mut line = line.split_whitespace();
            let destination = line
                .next()
                .expect("should be symbol")
                .parse::<u32>()
                .expect("should be number");
            let source = line
                .next()
                .expect("should be number")
                .parse::<u32>()
                .expect("should be number");
            let range = line
                .next()
                .expect("should be symbol")
                .parse::<u32>()
                .expect("should be number");
            let map_range = MapRange::new(source, destination, range);
            map.ranges.push(map_range);
        }
        maps.insert(map_type.clone(), map);
    }

    let mut locations = Vec::new();
    for seed in seeds {
        let soil = maps
            .get("seed-to-soil")
            .expect("map should exist")
            .get_destination(seed);
        let fertilizer = maps
            .get("soil-to-fertilizer")
            .expect("map should exist")
            .get_destination(soil);
        let water = maps
            .get("fertilizer-to-water")
            .expect("map should exist")
            .get_destination(fertilizer);
        let light = maps
            .get("water-to-light")
            .expect("map should exist")
            .get_destination(water);
        let temperature = maps
            .get("light-to-temperature")
            .expect("map should exist")
            .get_destination(light);
        let humidity = maps
            .get("temperature-to-humidity")
            .expect("map should exist")
            .get_destination(temperature);
        let location = maps
            .get("humidity-to-location")
            .expect("map should exist")
            .get_destination(humidity);
        locations.push(location);
    }

    *locations.iter().min().unwrap()
}

fn part2(input: &str) -> u32 {
    let mut input = input.split("\n\n");
    let seeds = input
        .next()
        .expect("should be one line")
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|num| num.parse::<u32>().expect("should be numbers"))
        .collect_vec();
    let seeds = seeds.chunks_exact(2).map(|chunk| {
        let mut chunk_iter = chunk.iter();
        let start = chunk_iter.next().unwrap();
        let range = chunk_iter.next().unwrap();
        *start..(*start + *range - 1)
    });

    let mut maps: HashMap<String, Map> = HashMap::new();
    for map in input {
        let mut lines = map.lines();
        let map_type = lines
            .next()
            .expect("should be map type")
            .replace(" map:", "");
        let mut map = Map::new(map_type.clone());
        for line in lines {
            let mut line = line.split_whitespace();
            let destination = line
                .next()
                .expect("should be symbol")
                .parse::<u32>()
                .expect("should be number");
            let source = line
                .next()
                .expect("should be number")
                .parse::<u32>()
                .expect("should be number");
            let range = line
                .next()
                .expect("should be symbol")
                .parse::<u32>()
                .expect("should be number");
            let map_range = MapRange::new(source, destination, range);
            map.ranges.push(map_range);
        }
        maps.insert(map_type.clone(), map);
    }

    let mut processed_ranges = Vec::new();
    let mut locations = Vec::new();
    for seed_range in seeds {
        let time = Instant::now();
        let mut locs = seed_range
            .clone()
            .into_par_iter()
            .filter(|seed| {
                !processed_ranges
                    .iter()
                    .any(|range: &Range<u32>| range.contains(seed))
            })
            .map(|seed| {
                let soil = maps
                    .get("seed-to-soil")
                    .expect("map should exist")
                    .get_destination(seed);
                let fertilizer = maps
                    .get("soil-to-fertilizer")
                    .expect("map should exist")
                    .get_destination(soil);
                let water = maps
                    .get("fertilizer-to-water")
                    .expect("map should exist")
                    .get_destination(fertilizer);
                let light = maps
                    .get("water-to-light")
                    .expect("map should exist")
                    .get_destination(water);
                let temperature = maps
                    .get("light-to-temperature")
                    .expect("map should exist")
                    .get_destination(light);
                let humidity = maps
                    .get("temperature-to-humidity")
                    .expect("map should exist")
                    .get_destination(temperature);
                let location = maps
                    .get("humidity-to-location")
                    .expect("map should exist")
                    .get_destination(humidity);
                location
            })
            .collect();
        locations.append(&mut locs);
        processed_ranges.push(seed_range.clone());
        println!(
            "seed range: {:?} in {}s",
            seed_range,
            time.elapsed().as_secs_f32()
        );
    }

    *locations.par_iter().min().unwrap()
}

fn main() {
    println!("Running day3");

    let input = include_str!("../tests/05_1.txt");
    let time = Instant::now();
    let result = part1(input);
    println!("Part1 result: {result} in {}μs", time.elapsed().as_micros());

    let input = include_str!("../tests/05_1.txt");
    let time = Instant::now();
    let result: u32 = part2(input);
    println!("Part2 result: {result} in {}μs", time.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../tests/05_1_test.txt");
        let result = part1(input);
        let expected = 35;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../tests/05_1_test.txt");
        let result = part2(input);
        let expected = 46;
        assert_eq!(result, expected);
    }
}
