fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const COORDS: [(isize,isize);8] = [
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

fn find_parts(schematic: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> (bool, String) {
    let mut return_num = schematic[i][j].to_string();
    let mut adjacent = false;

    // get size for bounds checking
    let rows = schematic.len();
    let cols = schematic[i].len();

    // check the cell to the right, if it's numeric keep searching
    let (k, l) = (i, j + 1);
    if k < cols && l < rows && schematic[k][l].is_numeric() {
        let mut _s: String = String::new();
        (adjacent, _s) = find_parts(schematic, visited, k, l);
        return_num.push_str(&_s);

    }

    // set this position as visited so we can skip it in the calling function
    visited[i][j] = true;

    // no need to keep checking, return immediately
    if adjacent {
        return (adjacent, return_num.clone())
    }

    // check all the adjacent coordinates for special characters
    for (x, y) in COORDS {
        let (i_bound, j_bound) = (i as isize + x, j as isize + y);
        if i_bound >= rows as isize || j_bound >= cols as isize || i_bound < 0 || j_bound < 0 {
            continue
        }
        let (k, l) = (i_bound as usize, j_bound as usize);
        if !schematic[k][l].is_numeric() && schematic[k][l] != '.' {
            adjacent = true;
            break;
        }
    }
    (adjacent, return_num.clone())
}

fn part1(input: &str) -> i32 {
    let schematic = parse_schematic(input);
    let mut sum = 0;
    let mut visited = vec![vec![false; schematic[0].len()]; schematic.len()];
    for (i, row) in schematic.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if visited[i][j] {
                continue
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
    todo!()
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
