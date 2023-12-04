use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<&str> {
    println!("{input}");
    vec![]
}

fn part1(input: Vec<&str>) -> u32 {
    0
}
// fn part2(input: Vec<&str>) {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 0);
        // assert_eq!(part1(parse_input(&get_input!(file!()))), 0);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(parse_input(EXAMPLE)), 0);
    //     assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    // }
}
