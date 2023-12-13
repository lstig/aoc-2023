use aho_corasick::AhoCorasick;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut value = String::from("");
        for c in line.chars() {
            if c.is_numeric() {
                value.push(c);
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_numeric() {
                value.push(c);
                break;
            }
        }
        sum += value.parse::<u32>().unwrap();
    }
    sum
}

const NUMS: [(&'static str, &'static str); 10] = [
    ("zero", "0"),
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let nummap: HashMap<_, _> = NUMS.into_iter().collect();
    let patterns = &[
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];
    let re = AhoCorasick::new(patterns).unwrap();
    for line in input.lines() {
        let mut value = String::from("");
        let matches: Vec<_> = re
            .find_overlapping_iter(line)
            .map(|m| m.pattern())
            .collect();

        for id in vec![matches[0], matches[matches.len() - 1]] {
            let m = patterns[id.as_usize()];
            if nummap.contains_key(m) {
                value.push_str(nummap[m]);
            } else {
                value.push_str(m);
            }
        }
        sum += value.parse::<u32>().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn check_part1() {
        let input = indoc! {"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"
        };
        let result = part1(input);
        assert_eq!(result, 142)
    }

    #[test]
    fn check_part2() {
        let input = indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen"
        };
        let result = part2(input);
        assert_eq!(result, 281)
    }
}
