use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input)));
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
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .iter()
        .map(|line| {
            let mut matches: Vec<String> = vec![];
            for i in 0..line.len() {
                if (line.as_bytes()[i] as char).is_digit(10) {
                    matches.push(String::from(&line[i..i + 1]));
                    continue;
                }
                if let Some(v) = numbers.iter().position(|n| line[i..].starts_with(*n)) {
                    matches.push((v + 1).to_string());
                }
            }

            let value = format!("{}{}", matches.first().unwrap(), matches.last().unwrap());
            value.parse::<u32>().unwrap()
        })
        .fold(0, |calibration, line| calibration + line)
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
        assert_eq!(part1(parse_input(&get_input!(file!()))), 54916);
    }

    #[test]
    fn test_day1_part2() {
        assert_eq!(part2(parse_input(EXAMPLE2)), 281);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 54728);
    }
}
