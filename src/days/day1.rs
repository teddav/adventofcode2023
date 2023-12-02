use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split("\n").collect()
}

fn part1(input: Vec<&str>) -> u32 {
    let is_digit = |d: char| d.is_digit(10);
    input
        .iter()
        .map(|line| {
            let matches: Vec<&str> = line.matches(is_digit).collect();
            let value = format!("{}{}", matches.first().unwrap(), matches.last().unwrap());
            value.parse::<u8>().unwrap()
        })
        .fold(0, |calibration, line| (line as u32) + calibration)
}

fn part2(input: Vec<&str>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
    ";

    const EXAMPLE2: &str = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
    ";

    #[test]
    fn test_day1_part1() {
        assert_eq!(part1(parse_input(EXAMPLE1)), 142);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(part2(parse_input(EXAMPLE2)), 0);
    }
}
