fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    todo!()
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
        let input = indoc! {""};
        let result = part1(input);
        assert_eq!(result, 4361)
    }

    #[test]
    fn check_part2() {
        let input = indoc! {""};
        let result = part2(input);
        assert_eq!(result, 467835)
    }
}
