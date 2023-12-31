use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const BAG: [(&'static str, i32); 3] = [("red", 12), ("green", 13), ("blue", 14)];

fn parse_game(input: &str) -> (u32, Vec<Vec<(i32, &str)>>) {
    let mut return_vec = Vec::new();

    let re = Regex::new(r"^Game (\d+): (.+)$").unwrap();
    let caps = re.captures(input).unwrap();

    // parse the game information
    let id = caps
        .get(1)
        .map_or("0", |m| m.as_str())
        .parse::<u32>()
        .unwrap();
    let rounds = caps.get(2).map_or("", |m| m.as_str());

    // parse the rounds of each game
    for round in rounds.split(";") {
        return_vec.push(
            round
                .split(",")
                .map(|m| {
                    let this: Vec<_> = m.split_whitespace().collect();
                    // (number, color)
                    (this[0].parse::<i32>().unwrap(), this[1])
                })
                .collect::<Vec<_>>(),
        );
    }

    (id, return_vec)
}

fn part1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    'game: for line in input.lines() {
        // parse the line to get the game ID and rounds
        let (id, rounds) = parse_game(line);

        // iterate over the rounds of this game
        for round in rounds {
            // create a bag for this round with all the cubes
            let mut bag: HashMap<_, _> = BAG.into_iter().collect();
            for (num, color) in round {
                // get the color or the block drawn and decrement the blocks in the bag
                let count = bag.get_mut(color).unwrap();
                *count -= num;
                // if the blocks be come less than zero, this game is invalid
                if bag[color] < 0 {
                    continue 'game;
                }
            }
        }
        // this game was valid!
        sum += id;
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut game: HashMap<&str, i32> = HashMap::new();
        let (_, rounds) = parse_game(line);
        for round in rounds {
            // create a bag for this round
            let mut bag: HashMap<&str, i32> = HashMap::new();

            // pull out blocks and count the number of each color
            for (num, block) in round {
                if bag.contains_key(block) {
                    let count = bag.get_mut(block).unwrap();
                    *count += num;
                } else {
                    bag.insert(block, num);
                }
            }

            // check the current round result against the game totals
            // assign whichever value is higher
            for (block, num) in bag {
                if game.contains_key(block) {
                    let current = game.get_mut(block).unwrap();
                    *current = max(num, *current);
                } else {
                    game.insert(block, num);
                }
            }
        }
        // multiply the final count of blocks together and add it to the total
        sum += game.values().copied().reduce(|acc, e| acc * e).unwrap() as u32;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static INPUT: &str = indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    };

    #[test]
    fn check_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 8)
    }

    #[test]
    fn check_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 2286)
    }
}
