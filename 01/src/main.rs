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

fn part2(input: &str) -> u32 {
    todo!()
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