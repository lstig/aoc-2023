use regex::Regex;
use std::cmp;
use std::collections::{HashMap, VecDeque};

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
            (caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str()),
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
            'L' => tree.get(&current).unwrap().0,
            'R' => tree.get(&current).unwrap().1,
            _ => panic!(), // this should never happen
        };
    }
    steps
}

// least common multiple
fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

// greatest common divisor
fn gcd(first: u64, second: u64) -> u64 {
    let mut max = cmp::max(first, second);
    let mut min = cmp::min(first, second);

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn part2(input: &str) -> u64 {
    // sections are separated by empty lines
    let re = Regex::new(r"(?m)\n\n").unwrap();
    let mut iter = re.split(input);
    let mut directions = VecDeque::from_iter(iter.next().unwrap().chars());

    let mut tree = HashMap::new();
    let mut current = Vec::new();
    let re = Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();
    for line in iter.next().unwrap().lines() {
        let caps = re.captures(line).unwrap();
        let (key, (left, right)) = (
            caps.get(1).unwrap().as_str(),
            (caps.get(2).unwrap().as_str(), caps.get(3).unwrap().as_str()),
        );
        tree.insert(key, (left, right));
        if key.ends_with('A') {
            current.push(key);
        }
    }

    let mut mutiples = vec![];
    let mut total_steps = 0;
    while current.len() > 0 {
        // Get current direction.
        let direction = directions.pop_front().unwrap();
        directions.push_back(direction);

        // Increase steps taken.
        total_steps += 1;

        // Map each current node to their next positions, removing any that end with 'Z'
        // If any end with 'Z', we make a note of the current
        // step so we can determine the least common multiple later
        let (next, found_z): (Vec<_>, Vec<_>) = current
            .clone()
            .into_iter()
            .map(|e| match direction {
                'L' => tree.get(e).unwrap().0,
                'R' => tree.get(e).unwrap().1,
                _ => panic!(), // this should never happen
            })
            .partition(|&e| !e.ends_with('Z'));

        // Assign the filtered nodes to current.
        current = next;

        // If any nodes are found that end with 'Z' we make a note of the current step so we can determine the least common multiple later.
        if found_z.len() > 0 {
            mutiples.push(total_steps);
        }
    }

    mutiples.into_iter().reduce(|acc, e| lcm(acc, e)).unwrap()
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
        let input = indoc! {"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)"
        };
        let result = part2(input);
        assert_eq!(result, 6)
    }
}
