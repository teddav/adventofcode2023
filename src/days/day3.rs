use std::collections::HashMap;

use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input)));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(usize, usize);

#[derive(Debug)]
struct Map {
    numbers: Vec<(u32, Position)>,
    symbols: Vec<(char, Position)>,
    max: Position,
}

fn parse_input(input: &str) -> Map {
    let mut map = Map {
        numbers: vec![],
        symbols: vec![],
        max: Position(0, 0),
    };
    for (y, line) in input.trim().split("\n").enumerate() {
        map.max = Position(line.len(), y);

        let mut n = String::new();
        let mut it = line.chars().enumerate().peekable();
        while let Some((x, c)) = it.next() {
            if c == '.' {
                continue;
            }
            if c.is_digit(10) {
                n.push(c);
            } else {
                map.symbols.push((c, Position(x, y)));
            }
            let next = it
                .peek()
                .map(|_it| if _it.1.is_digit(10) { _it.1 } else { '.' });
            if next == Some('.') || next.is_none() {
                if let Ok(_n) = n.parse::<u32>() {
                    map.numbers.push((_n, Position(x, y)));
                    n.clear();
                }
            }
        }
    }
    map
}

fn part1(map: Map) -> u32 {
    map.numbers
        .into_iter()
        .map(|(number, pos)| {
            let positions_to_check = get_positions_to_check(pos, number.to_string().len(), map.max);
            if map
                .symbols
                .iter()
                .any(|symbol| positions_to_check.iter().any(|p| *p == symbol.1))
            {
                number
            } else {
                0
            }
        })
        .sum()
}

fn get_positions_to_check(position: Position, mut len: usize, max: Position) -> Vec<Position> {
    let mut positions = vec![];
    if position.0 < max.0 {
        if position.1 > 0 {
            positions.push(Position(position.0 + 1, position.1 - 1));
        }
        positions.push(Position(position.0 + 1, position.1));
        positions.push(Position(position.0 + 1, position.1 + 1));
    }
    if position.0 >= len {
        positions.push(Position(position.0 - len, position.1));
    }
    #[allow(unused_comparisons)]
    while len >= 0 {
        if position.0 >= len {
            if position.1 > 0 {
                positions.push(Position(position.0 - len, position.1 - 1));
            }
            if position.1 < max.1 {
                positions.push(Position(position.0 - len, position.1 + 1));
            }
        }
        if len == 0 {
            break;
        }
        len -= 1;
    }

    positions
}

fn part2(map: Map) -> u32 {
    let mut gears: HashMap<Position, Vec<u32>> = HashMap::new();
    for (number, pos) in map.numbers {
        let positions_to_check = get_positions_to_check(pos, number.to_string().len(), map.max);
        for sym in &map.symbols {
            if sym.0 == '*' && positions_to_check.iter().any(|p| *p == sym.1) {
                // gears.push((number, sym.1));
                match gears.get_mut(&sym.1) {
                    Some(v) => {
                        v.push(number);
                    }
                    None => {
                        gears.insert(sym.1, vec![number]);
                    }
                }
                break;
            }
        }
    }
    gears
        .iter()
        .map(|(_, v)| if v.len() == 2 { v[0] * v[1] } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 4361);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 535235);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 467835);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 79844424);
    }
}
