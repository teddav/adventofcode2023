use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

#[derive(Debug, Clone, Copy)]
struct Range {
    source: u64,
    destination: u64,
    length: u64,
}

#[derive(Debug, Clone)]
struct Transformation {
    tr_type: [String; 2],
    tr_map: Vec<Range>,
}

#[derive(Debug)]
struct Map {
    seeds: Vec<u64>,
    transformations: Vec<Transformation>,
}

fn parse_input(input: &str) -> Map {
    let mut input = input.trim().split("\n\n");
    let mut map = Map {
        seeds: input
            .next()
            .unwrap()
            .split(": ")
            .last()
            .unwrap()
            .split(" ")
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect(),
        transformations: vec![],
    };
    for transormation in input {
        let mut split = transormation.split("map:\n");

        let tr = Transformation {
            tr_type: split
                .next()
                .unwrap()
                .trim()
                .split("-to-")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
                .try_into()
                .unwrap(),
            tr_map: split
                .next()
                .unwrap()
                .split("\n")
                .map(|range| {
                    let range: Vec<u64> = range.split(" ").map(|n| n.parse().unwrap()).collect();
                    Range {
                        source: range[1],
                        destination: range[0],
                        length: range[2],
                    }
                })
                .collect(),
        };
        map.transformations.push(tr);
    }
    map
}

fn part1(map: Map) -> u64 {
    map.seeds
        .iter()
        .map(|seed| {
            let mut current_value = *seed;
            'trloop: for transformation in &map.transformations {
                for Range {
                    source,
                    destination,
                    length,
                } in &transformation.tr_map
                {
                    if current_value >= *source && current_value < source + length {
                        current_value = destination + (current_value - source);
                        continue 'trloop;
                    }
                }
            }
            current_value
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
