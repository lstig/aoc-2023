use regex::Regex;
use std::cmp;
use std::ops::Range;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, Clone)]
struct MapperRange {
    source: Range<u64>,
    destination: Range<u64>,
}

#[derive(Debug)]
struct Mapper {
    ranges: Vec<MapperRange>,
}

impl Mapper {
    fn new() -> Mapper {
        Mapper { ranges: Vec::new() }
    }

    fn add_range(&mut self, range: &str) {
        let r: Vec<_> = range
            .split_whitespace()
            .map(|n| n.parse::<u64>().unwrap())
            .collect();
        let r = MapperRange {
            source: r[1]..r[1] + r[2],
            destination: r[0]..r[0] + r[2],
        };

        match self
            .ranges
            .binary_search_by(|m| m.source.start.cmp(&r.source.start))
        {
            Err(pos) => self.ranges.insert(pos, r),
            _ => {}
        }
    }

    // fill in gaps in the ranges created by the mapper
    fn fill(&mut self) {
        let mut current = 0;
        for range in self.ranges.clone().into_iter().map(|m| m.source.clone()) {
            if current < range.start {
                self.add_range(format!("{current} {current} {}", range.start).as_str());
                current = range.end
            }
        }
    }

    fn translate(&self, n: u64) -> u64 {
        for range in &self.ranges {
            if range.source.contains(&n) {
                let diff = n - range.source.start;
                // return the new number from the destination range
                return range.destination.start + diff;
            }
        }
        // no range contains, return the original number
        n
    }
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Mapper>) {
    // sections are separated by empty lines
    let re = Regex::new(r"(?m)\n\n").unwrap();
    let mut iter = re.split(input);

    // parse seeds from input
    let seeds: Vec<u64> = iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    // parse mappers and their ranges
    let mappers: Vec<_> = iter
        .map(|chunk| {
            let mut mapper = Mapper::new();
            chunk
                .lines()
                .skip(1)
                .for_each(|line| mapper.add_range(line));
            mapper.fill();
            mapper
        })
        .collect();

    (seeds, mappers)
}

fn part1(input: &str) -> u64 {
    let (seeds, mappers) = parse_input(input);

    // get location of each seed
    let mut lowest: Option<u64> = None;
    for seed in seeds {
        let location = mappers.iter().fold(seed, |acc, m| m.translate(acc));
        lowest = Some(cmp::min(location, lowest.unwrap_or(location)));
    }
    lowest.unwrap()
}

fn part2(input: &str) -> u64 {
    let (seeds, mappers) = parse_input(input);

    // get ranges of seeds
    let mut seeds: Vec<_> = seeds.chunks(2).map(|r| r[0]..r[0] + r[1]).collect();

    for mapper in mappers {
        let mut translated = Vec::new();
        while seeds.len() > 0 {
            let seed = seeds.pop().unwrap();
            let mut found = false;
            for range in &mapper.ranges {
                if range.source.contains(&seed.start) && range.source.contains(&seed.end) {
                    // current seed's range is entirely within the current range
                    translated.push(mapper.translate(seed.start)..mapper.translate(seed.end));
                    found = true;
                    break;
                } else if range.source.contains(&seed.start) {
                    // current seed's range starts in the current range, but overlaps into the next range
                    translated
                        .push(mapper.translate(seed.start)..mapper.translate(range.source.end - 1));
                    // push the remaining part of this seed's range onto the seed vector
                    seeds.push(range.source.end..seed.end);
                    found = true;
                    break;
                }
            }
            if !found {
                // seed wasn't in any ranges, just push it onto the translated vector
                translated.push(seed);
            }
        }
        seeds = translated;
    }

    seeds.iter().map(|r| r.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static INPUT: &str = indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4"
    };

    #[test]
    fn check_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 35)
    }

    #[test]
    fn check_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 46)
    }
}
