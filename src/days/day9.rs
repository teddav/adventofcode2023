use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .split("\n")
        .map(|line| line.split(" ").map(|v| v.parse().unwrap()).collect())
        .collect()
}

fn part1(input: Vec<Vec<i64>>) -> i64 {
    input
        .into_iter()
        .map(|line| {
            let sequences = get_sequences(vec![line]);
            sequences.iter().rev().fold(0, |acc, seq| {
                let last = seq.last().unwrap();
                acc + last
            })
        })
        .sum()
}

fn get_sequences(mut sequences: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let current_line = sequences.last().unwrap();
    let mut next_line = vec![];
    for i in 1..current_line.len() {
        next_line.push(current_line[i] - current_line[i - 1]);
    }
    sequences.push(next_line.clone());
    if next_line.iter().all(|v| *v == 0) {
        return sequences;
    }
    return get_sequences(sequences);
}

fn part2(input: Vec<Vec<i64>>) -> i64 {
    input
        .into_iter()
        .map(|line| {
            let sequences = get_sequences(vec![line]);
            sequences.iter().rev().fold(0, |acc, seq| {
                let first = seq.first().unwrap();
                first - acc
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
            ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 114);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 1938800261);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 2);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 1112);
    }
}
