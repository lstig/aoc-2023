use std::collections::{HashMap, VecDeque};
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    // sections are separated by empty lines
    let re = Regex::new(r"(?m)\n\n").unwrap();
    let mut iter = re.split(input);
    let mut directions = VecDeque::from_iter(iter.next().unwrap().chars());

    let mut tree = HashMap::new();
    let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
    for line in iter.next().unwrap().lines() {
        let caps = re.captures(line).unwrap();
        let (key, (left, right)) = (
            caps.get(1).unwrap().as_str(),
            (
                caps.get(2).unwrap().as_str(),
                caps.get(3).unwrap().as_str(),
            ),
        );
        tree.insert(key, (left, right));
    }

    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        // get current direction
        let direction = directions.pop_front().unwrap();
        directions.push_back(direction);

        // increase steps taken
        steps += 1;

        // update current node with the next in the tree based on the directions
        current = match direction {
            'L' => { tree.get(&current).unwrap().0 },
            'R' => { tree.get(&current).unwrap().1 },
            _ => panic!(), // this should never happen
        };
    }
    steps
}

fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn check_part1() {
        let input = indoc! {"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)"
        };
        let result = part1(input);
        assert_eq!(result, 6)
    }

    #[test]
    fn check_part2() {
        let result = part2("");
        assert_eq!(result, 0)
    }
}