use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input)));
}

#[derive(Debug)]
struct Map {
    seeds: Vec<u64>,
    transformations: Vec<Vec<[u64; 3]>>,
}

fn parse_input(input: &str) -> Map {
    let mut input = input.trim().split("\n\n");
    Map {
        seeds: input
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(" ")
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect(),
        transformations: input
            .map(|transormation| {
                transormation
                    .split("map:\n")
                    .last()
                    .unwrap()
                    .split("\n")
                    .map(|range| {
                        range
                            .split(" ")
                            .map(|n| n.parse().unwrap())
                            .collect::<Vec<u64>>()
                            .try_into()
                            .unwrap()
                    })
                    .collect()
            })
            .collect(),
    }
}

fn part1(map: Map) -> u64 {
    map.seeds
        .into_iter()
        .map(|mut seed| {
            for transformation in &map.transformations {
                for [destination, source, length] in transformation {
                    if seed >= *source && seed < source + length {
                        seed = destination + (seed - source);
                        break;
                    }
                }
            }
            seed
        })
        .min()
        .unwrap()
}

fn part2(map: Map) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 35);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 322500873);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 0);
        // assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    }
}
