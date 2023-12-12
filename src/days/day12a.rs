use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<(&str, Vec<usize>)> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut split = line.split(" ");
            let springs = split.next().unwrap();
            let blocks = split
                .next()
                .unwrap()
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect();
            (springs, blocks)
        })
        .collect()
}

fn part1(input: Vec<(&str, Vec<usize>)>) -> u32 {
    input
        .iter()
        .map(|(springs, blocks)| count_arrangements(springs, blocks, 0))
        .sum()
}

fn is_valid(springs: &str, blocks: &Vec<usize>) -> bool {
    let created_blocks: Vec<&str> = springs.split(".").filter(|v| v != &"").collect();
    if created_blocks.len() != blocks.len() {
        return false;
    }
    for i in 0..blocks.len() {
        if created_blocks[i].len() != blocks[i] {
            return false;
        }
    }
    true
}

fn count_arrangements(springs: &str, blocks: &Vec<usize>, i: usize) -> u32 {
    if i == springs.len() {
        if is_valid(springs, blocks) {
            return 1;
        } else {
            return 0;
        }
    }

    if springs.as_bytes()[i] == '?' as u8 {
        return count_arrangements(
            &format!("{}#{}", &springs[..i], &springs[i + 1..]),
            blocks,
            i + 1,
        ) + count_arrangements(
            &format!("{}.{}", &springs[..i], &springs[i + 1..]),
            blocks,
            i + 1,
        );
    }

    count_arrangements(springs, blocks, i + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
            ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 21);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 7843);
    }
}
