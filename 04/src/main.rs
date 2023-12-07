use regex::Regex;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;
    let re = Regex::new(r":\s+(.+)\s+\|\s+(.+)").unwrap();
    for line in input.lines() {
        let mut wins: Option<i32> = None;
        let caps = re.captures(line).unwrap();
        let winners: HashSet<i32> = HashSet::from_iter(
            caps.get(1)
                .map_or("", |m| m.as_str())
                .split_whitespace()
                .map(|m| m.parse::<i32>().unwrap()),
        );
        let picks = caps
            .get(2)
            .map_or("", |m| m.as_str())
            .split_whitespace()
            .map(|m| m.parse::<i32>().unwrap());

        for pick in picks {
            if winners.contains(&pick) {
                wins = Some(wins.unwrap_or(-1) + 1);
            }
        }

        if wins.is_some() {
            sum += 2_i32.pow(wins.unwrap() as u32)
        }
    }
    sum
}

fn part2(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn check_part1() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        };
        let result = part1(input);
        assert_eq!(result, 13)
    }

    #[test]
    fn check_part2() {
        let input = indoc! {"
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let result = part2(input);
        assert_eq!(result, 30)
    }
}
