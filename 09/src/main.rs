fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(_input: &str) -> i32 {
    0
}

fn part2(_input: &str) -> i32 {
    0
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
        assert_eq!(result, 0)
    }
}
