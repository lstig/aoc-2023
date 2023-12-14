use crate::HandKind::*;
use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;

static CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandKind {
    // all cards' labels are distinct: 23456
    HighCard,
    // two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair,
    // two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair,
    // three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind,
    // three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse,
    // four cards have the same label and one card has a different label: AA8AA
    FourOfAKind,
    // all five cards have the same label: AAAAA
    FiveOfAKind,
}

impl HandKind {
    fn from_string(string: &str) -> Self {
        let mut cards = HashMap::new();

        // count the occurrences of each card
        for card in string.chars() {
            let count = match cards.entry(card) {
                Vacant(entry) => entry.insert(0),
                Occupied(entry) => entry.into_mut(),
            };

            *count += 1;
        }

        match cards.len() {
            5 => return HighCard,
            4 => return OnePair,
            3 => {
                return if cards.values().into_iter().any(|card| *card == 3) {
                    ThreeOfAKind
                } else {
                    TwoPair
                }
            }
            2 => {
                return if cards.values().into_iter().any(|card| *card == 4) {
                    FourOfAKind
                } else {
                    FullHouse
                }
            }
            1 => return FiveOfAKind,
            _ => panic!(),
        }
    }
}

#[derive(Eq, Debug)]
struct Hand<'a> {
    cards: &'a str,
    bet: u32,
    kind: HandKind,
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            // If hand kinds are equal, compare their cards
            Ordering::Equal => {
                // build HashMap of cards and their values
                let cards: HashMap<_, _> =
                    CARDS.into_iter().enumerate().map(|(v, c)| (c, v)).collect();
                // compare each card in each hand
                for (i, card) in self.cards.chars().into_iter().enumerate() {
                    let other = other.cards.as_bytes()[i] as char;
                    if card != other {
                        let c = cards.get(&card).unwrap();
                        let o = cards.get(&other).unwrap();
                        return c.cmp(o);
                    }
                }
                Ordering::Equal
            }
            _ => self.kind.cmp(&other.kind),
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.cards == self.cards
    }
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let (cards, bet) = (
            iter.next().unwrap(),
            iter.next().unwrap().parse::<u32>().unwrap(),
        );
        let hand = Hand {
            cards,
            bet,
            kind: HandKind::from_string(cards),
        };
        match hands.binary_search(&hand) {
            Err(pos) => hands.insert(pos, hand),
            _ => {}
        }
    }
    hands
}

fn part1(input: &str) -> u32 {
    let hands = parse_input(input);
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (pos, hand)| acc + hand.bet * (pos as u32 + 1))
}

fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static INPUT: &str = indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"
    };

    #[test]
    fn check_part1() {
        let result = part1(INPUT);
        assert_eq!(result, 6440)
    }

    #[test]
    fn check_part2() {
        let result = part2(INPUT);
        assert_eq!(result, 0)
    }
}
