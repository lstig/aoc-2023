fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const COORDS: [(isize, isize); 8] = [
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
];

fn parse_schematic(input: &str) -> Vec<Vec<char>> {
    let mut schematic: Vec<Vec<_>> = Vec::new();
    for line in input.lines() {
        schematic.push(line.chars().collect());
    }
    schematic
}

fn check_coord(schematic: &Vec<Vec<char>>, i: isize, j: isize) -> Option<(usize, usize)> {
    // get size for bounds checking
    let rows = schematic.len() as isize;
    let cols = schematic[0].len() as isize;

    // check bounds
    if i >= rows || j >= cols || i < 0 || j < 0 {
        return None;
    }

    Some((i as usize, j as usize))
}

fn find_parts(
    schematic: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
) -> (bool, String) {
    let mut return_num = schematic[i][j].to_string();
    let mut adjacent = false;

    // check the cell to the right, if it's numeric keep searching
    if let Some((i, j)) = check_coord(schematic, i as isize, j as isize + 1) {
        if schematic[i][j].is_numeric() {
            let mut _s: String = String::new();
            (adjacent, _s) = find_parts(schematic, visited, i, j);
            return_num.push_str(&_s);
        }
    }

    // set this position as visited so we can skip it in the calling function
    visited[i][j] = true;

    // no need to keep checking, return immediately
    if adjacent {
        return (adjacent, return_num.clone());
    }

    // check all the adjacent coordinates for special characters
    for (x, y) in COORDS {
        if let Some((i, j)) = check_coord(schematic, i as isize + x, j as isize + y) {
            if !schematic[i][j].is_numeric() && schematic[i][j] != '.' {
                adjacent = true;
                break;
            }
        }
    }
    (adjacent, return_num.clone())
}

fn gear_ratio(schematic: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    let mut ratio = 1;
    let mut visited = vec![vec![false; schematic[0].len()]; schematic.len()];

    // check all the adjacent coordinates for numeric characters
    for (x, y) in COORDS {
        if let Some((i, j)) = check_coord(schematic, i as isize + x, j as isize + y) {
            if schematic[i][j].is_numeric() && !visited[i][j] {
                count += 1;
                let mut num = String::new();

                // look for numbers to the right
                for (k, c) in schematic[i][j..].iter().enumerate() {
                    if c.is_numeric() {
                        visited[i][j + k] = true;
                        num.push(*c);
                    } else {
                        break;
                    }
                }

                // look for numbers to the left
                for (k, c) in schematic[i][..j].iter().rev().enumerate() {
                    if c.is_numeric() {
                        visited[i][j - 1 - k] = true;
                        let mut s = c.to_string();
                        s.push_str(num.as_str());
                        num = s;
                    } else {
                        break;
                    }
                }

                ratio *= num.parse::<i32>().unwrap();
            }
        }
    }

    // invalid gear, set the ration to zero
    if count != 2 {
        ratio = 0
    }

    ratio
}

fn part1(input: &str) -> i32 {
    let schematic = parse_schematic(input);
    let mut sum = 0;
    let mut visited = vec![vec![false; schematic[0].len()]; schematic.len()];
    for (i, row) in schematic.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if visited[i][j] {
                continue;
            } else if c.is_numeric() {
                let (is_part, num) = find_parts(&schematic, &mut visited, i, j);
                if is_part {
                    sum += num.parse::<i32>().unwrap();
                }
            }
        }
    }
    sum
}

fn part2(input: &str) -> i32 {
    let schematic = parse_schematic(input);
    let mut sum = 0;
    for (i, row) in schematic.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '*' {
                sum += gear_ratio(&schematic, i, j);
            }
        }
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
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
        };
        let result = part1(input);
        assert_eq!(result, 4361)
    }

    #[test]
    fn check_part2() {
        let input = indoc! {"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
        };
        let result = part2(input);
        assert_eq!(result, 467835)
    }
}
