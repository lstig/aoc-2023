use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn score_card(input: &str) -> (u32, i32) {
    let mut wins: Option<i32> = None;
    let re = Regex::new(r":\s+(.+)\s+\|\s+(.+)").unwrap();
    let caps = re.captures(input).unwrap();
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
        (wins.unwrap() as u32 + 1, 2_i32.pow(wins.unwrap() as u32))
    } else {
        (0, 0)
    }
}

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let (_, score) = score_card(line);
        sum += score;
    }

    sum
}

fn count_cards(cards: &Vec<&str>, visited: &mut HashMap<usize, u32>, card: &str, i: usize) -> u32 {
    if let Some(count) = visited.get(&i) {
        return *count
    }

    let (count, _) = score_card(card);
    let mut sum = count;

    let start = i + 1;
    for j in start..start + count as usize {
        sum += count_cards(cards, visited, cards[j], j);
    }

    visited.insert(i, sum);

    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    let cards: Vec<&str> = input.lines().collect();
    let mut visited: HashMap<usize, u32> = HashMap::new();

    for (i, card) in cards.iter().enumerate() {
        sum += 1;
        sum += count_cards(&cards, &mut visited, card, i);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static INPUT: &str = indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    };

    #[test]
    fn check_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 13)
    }

    #[test]
    fn check_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 30)
    }
}
