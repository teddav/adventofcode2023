use aoc2023::get_input;
use std::collections::VecDeque;

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
                        let r: Vec<u64> = range.split(" ").map(|n| n.parse().unwrap()).collect();
                        [r[0], r[1], r[1] + r[2]]
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
                for &[destination, source_start, source_end] in transformation {
                    if seed >= source_start && seed < source_end {
                        seed = destination + seed - source_start;
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
    let mut seeds: VecDeque<[u64; 2]> = VecDeque::from_iter(map.seeds.chunks(2).map(|c| {
        let c: [u64; 2] = c.try_into().unwrap();
        [c[0], c[0] + c[1]]
    }));
    let mut next: Vec<[u64; 2]> = vec![];

    for transformation in &map.transformations {
        'currentloop: while let Some([start, end]) = seeds.pop_front() {
            for &[destination, source_start, source_end] in transformation {
                let overlap_start = start.max(source_start);
                let overlap_end = end.min(source_end);
                if overlap_start < overlap_end {
                    next.push([
                        overlap_start + destination - source_start,
                        overlap_end + destination - source_start,
                    ]);

                    if overlap_start > start {
                        seeds.push_back([start, overlap_start]);
                    }
                    if end > overlap_end {
                        seeds.push_back([overlap_end, end]);
                    }
                    continue 'currentloop;
                }

                // if start < source_start && end >= source_start {
                //     next.push([start, source_start]);
                //     next.push([destination, end - source_start + destination]);

                //     if end >= source_end {
                //         next.push([source_end, end]);
                //     }
                //     continue 'currentloop;
                // }
                // if start >= source_start && start < source_end {
                //     next.push([
                //         destination + start - source_start,
                //         source_end + destination - source_start,
                //     ]);
                //     if end >= source_end {
                //         next.push([source_end, end]);
                //     }
                //     continue 'currentloop;
                // }
            }

            next.push([start, end]);
        }
        seeds = next.clone().into();
        next.clear();
    }
    seeds.iter().map(|c| c[0]).min().unwrap()
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
        assert_eq!(part2(parse_input(EXAMPLE)), 46);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 108956227);
    }
}
