use aoc2023::get_input;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    str_repr: String,
    str_repr_for_cmp: String,
    bid: u64,
    cards: HashMap<u8, u8>,
}

impl Hand {
    fn from_str(hand: &str, bid: u64) -> Self {
        let mut parsed_hand = Hand {
            str_repr: hand.to_string(),
            str_repr_for_cmp: hand
                .replace("T", &(('9' as u8 + 1) as char).to_string())
                .replace("J", &(('9' as u8 + 2) as char).to_string())
                .replace("Q", &(('9' as u8 + 3) as char).to_string())
                .replace("K", &(('9' as u8 + 4) as char).to_string())
                .replace("A", &(('9' as u8 + 5) as char).to_string()),
            bid,
            cards: HashMap::new(),
        };
        for card in hand.chars() {
            let value: u8 = match card {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => card.to_digit(10).unwrap() as u8,
            };
            parsed_hand
                .cards
                .entry(value)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        parsed_hand
    }

    // it is returned as Vec<(number_of_cards, card_value)>
    fn sort(&self) -> Vec<(u8, u8)> {
        let mut hand: Vec<(u8, u8)> = self.cards.iter().map(|v| (*v.1, *v.0)).collect();
        hand.sort_by(|a, b| {
            if b.0 == a.0 {
                b.1.cmp(&a.1)
            } else {
                b.0.cmp(&a.0)
            }
        });
        hand
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_a = self.sort();
        let hand_b = other.sort();
        for i in 0..hand_a.len().max(hand_b.len()) {
            let &(a_number, _) = hand_a.get(i).unwrap();
            let &(b_number, _) = hand_b.get(i).unwrap();
            if a_number > b_number {
                return Ordering::Greater;
            } else if a_number < b_number {
                return Ordering::Less;
            }
        }
        self.str_repr_for_cmp.cmp(&other.str_repr_for_cmp)
    }
}

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<(&str, u64)> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut line = line.split(" ");
            (
                line.next().unwrap(),
                line.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect()
}

fn part1(hands: Vec<(&str, u64)>) -> u64 {
    let mut hands: Vec<Hand> = hands.iter().map(|h| Hand::from_str(h.0, h.1)).collect();
    hands.sort_by(|a, b| a.cmp(&b));
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as u64 + 1))
        .sum()
}

// fn part2(input: Vec<&str>) {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
        ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 6440);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 249748283);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(parse_input(EXAMPLE)), 0);
    //     assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    // }
}
