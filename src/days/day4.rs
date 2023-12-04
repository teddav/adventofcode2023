use aoc2023::get_input;
use regex::Regex;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input)));
}

#[derive(Debug)]
struct Card {
    id: u8,
    winning_numbers: Vec<u8>,
    drawn_numbers: Vec<u8>,
}

fn parse_input(input: &str) -> Vec<Card> {
    let re = Regex::new(
        // r"Card (\d+): ((?: *\d+ *)+)\|((?: *\d+ *)+)",
        r"Card\s+(?P<id>\d+): (?P<winning_numbers>(?: *\d+ *)+)\|(?P<drawn_numbers>(?: *\d+ *)+)",
    )
    .unwrap();

    input
        .trim()
        .split("\n")
        .map(|card| {
            let captures = re.captures(card).unwrap();

            // let id = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
            let id = captures["id"].parse::<u8>().unwrap();
            let winning_numbers: Vec<u8> = captures["winning_numbers"]
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let drawn_numbers: Vec<u8> = captures["drawn_numbers"]
                .trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            Card {
                id,
                winning_numbers,
                drawn_numbers,
            }
        })
        .collect()
}

fn part1(cards: Vec<Card>) -> u32 {
    cards
        .iter()
        .map(|card| {
            card.drawn_numbers.iter().fold(0, |count, n| {
                if card.winning_numbers.contains(&n) {
                    if count == 0 {
                        1
                    } else {
                        count * 2
                    }
                } else {
                    count
                }
            })
        })
        .sum()
}

fn part2(cards: Vec<Card>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 13);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 21213);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 30);
        // assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    }
}
