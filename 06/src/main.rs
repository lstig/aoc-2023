fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let time: Vec<_> = input
        .lines()
        .nth(0) // first line contains Time
        .unwrap()
        .split_whitespace()
        .skip(1) // skip the line label
        .map(|n| n.parse::<u32>().unwrap())
        .collect();
    let distance: Vec<_> = input
        .lines()
        .nth(1) // second line contains Distance
        .unwrap()
        .split_whitespace()
        .skip(1) // skip the line label
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    // return tuples of (time, distance)
    time.into_iter().zip(distance.into_iter()).collect()
}

fn part1(input: &str) -> u32 {
    let races = parse_input(input);
    let mut wins = Vec::new();
    for (i, (time, distance)) in races.into_iter().enumerate() {
        wins.push(0);
        for (hold, accelerate) in (0..=time).rev().enumerate() {
            if hold as u32 * accelerate > distance {
                wins[i] += 1;
            }
        }
    }
    wins.into_iter().reduce(|acc, e| acc * e).unwrap()
}

fn part2(input: &str) -> u32 {
    let time: Vec<_> = input
        .lines()
        .nth(0) // first line contains Time
        .unwrap()
        .split_whitespace()
        .skip(1) // skip the line label
        .collect();
    let time = time.join("").parse::<u64>().unwrap();
    let distance: Vec<_> = input
        .lines()
        .nth(1) // second line contains Distance
        .unwrap()
        .split_whitespace()
        .skip(1) // skip the line label
        .collect();
    let distance = distance.join("").parse::<u64>().unwrap();

    let mut wins = 0;
    for (hold, accelerate) in (0..=time).rev().enumerate() {
        if hold as u64 * accelerate > distance {
            wins += 1;
        }
    }
    wins
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static INPUT: &str = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200"
    };

    #[test]
    fn check_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 288)
    }

    #[test]
    fn check_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 71503)
    }
}
