fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

// Given a vector of numbers, returns the next number in the current line
fn next_number(numbers: Vec<i32>) -> i32 {
    // If the line is all zeros, we've hit the bottom and should return.
    if numbers.iter().all(|e| *e == 0) {
        return 0;
    }

    // Get the next number from the line after ours so we can calculate our next number
    let next_child = next_number(numbers.windows(2).map(|e| e[1] - e[0]).collect());

    return numbers[numbers.len() - 1] + next_child;
}

// Given a vector of numbers, returns the next number in the current line
fn previous_number(numbers: Vec<i32>) -> i32 {
    // If the line is all zeros, we've hit the bottom and should return.
    if numbers.iter().all(|e| *e == 0) {
        return 0;
    }

    // Get the previous number from the line after ours so we can calculate our next number
    let previous_child = previous_number(numbers.windows(2).map(|e| e[1] - e[0]).collect());

    return numbers[0] - previous_child;
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += next_number(
            line.split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect(),
        )
    }
    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        sum += previous_number(
            line.split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect(),
        );
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    static INPUT: &str = indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"
    };

    #[test]
    fn check_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 114)
    }

    #[test]
    fn check_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 2)
    }
}
