use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(&input));
}

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    let races: Vec<Vec<u32>> = input
        .trim()
        .split("\n")
        .map(|line| {
            line.split(":")
                .last()
                .unwrap()
                .split_whitespace()
                .map(|value| value.trim().parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    races[0]
        .iter()
        .zip(races[1].iter())
        .map(|e| (*e.0, *e.1))
        .collect()
}

fn part1(races: Vec<(u32, u32)>) -> usize {
    races
        .iter()
        .map(|&(time, distance)| {
            (0..time)
                .map(|speed| (time - speed) * speed)
                .filter(|d| *d > distance)
                .count()
        })
        .product()
}

fn part2(input: &str) -> usize {
    let race: [u64; 2] = input
        .trim()
        .split("\n")
        .map(|line| {
            line.split(":")
                .last()
                .unwrap()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect::<Vec<u64>>()
        .try_into()
        .unwrap();

    let [time, distance] = race;
    (0..time)
        .map(|speed| (time - speed) * speed)
        .filter(|d| *d > distance)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
Time:      7  15   30
Distance:  9  40  200
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 288);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 32076);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 71503);
        assert_eq!(part2(&get_input!(file!())), 34278221);
    }
}
