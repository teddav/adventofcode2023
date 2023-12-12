use aoc2023::get_input;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<(String, Vec<usize>)> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut split = line.split(" ");
            let springs = vec![split.next().unwrap()].repeat(5).join("?");
            let blocks = vec![split.next().unwrap()]
                .repeat(5)
                .join(",")
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect();
            (springs, blocks)
        })
        .collect()
}

fn part2(input: Vec<(String, Vec<usize>)>) -> usize {
    input
        .iter()
        .map(|(springs, blocks)| count_arrangements(springs, blocks, 0, 0, 0, &mut HashMap::new()))
        .sum()
}

// https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/12.py
fn count_arrangements(
    springs: &str,
    blocks: &Vec<usize>,
    i: usize,
    bi: usize,
    pound_count: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    let key = (i, bi, pound_count);
    if let Some(&count) = cache.get(&key) {
        return count;
    }

    if i == springs.len() {
        if bi == blocks.len() && pound_count == 0 {
            return 1;
        } else if bi == blocks.len() - 1 && blocks[bi] == pound_count {
            return 1;
        } else {
            return 0;
        }
    }

    let mut result = 0;
    let current_char = springs.as_bytes()[i] as char;

    if current_char == '?' || current_char == '.' {
        if pound_count == 0 {
            result += count_arrangements(springs, blocks, i + 1, bi, 0, cache);
        } else if pound_count > 0 && bi < blocks.len() && blocks[bi] == pound_count {
            result += count_arrangements(springs, blocks, i + 1, bi + 1, 0, cache);
        }
    }
    if current_char == '?' || current_char == '#' {
        result += count_arrangements(springs, blocks, i + 1, bi, pound_count + 1, cache);
    }

    cache.insert(key, result);
    result
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
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 525152);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 10153896718999);
    }
}
