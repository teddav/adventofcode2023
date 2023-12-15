use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(",").collect()
}

fn part1(input: Vec<&str>) -> u32 {
    input.iter().map(|v| hash(v)).sum()
}

fn hash(s: &str) -> u32 {
    let mut current_value = 0;
    for c in s.chars() {
        current_value += c as u32;
        current_value *= 17;
        current_value = current_value.rem_euclid(256);
    }
    current_value
}

// fn part2(input: Vec<&str>) {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 1320);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 509152);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(parse_input(EXAMPLE)), 0);
    //     assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    // }
}
