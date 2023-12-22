use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

#[derive(Debug)]
enum Direction {
    x,
    y,
    z,
}

#[derive(Debug)]
struct Brick {
    length: u32,
    direction: Direction,
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let bounds: [[u32; 3]; 2] = line
                .split("~")
                .map(|bound| {
                    bound
                        .split(",")
                        .map(|i| i.parse().unwrap())
                        .collect::<Vec<u32>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[u32; 3]>>()
                .try_into()
                .unwrap();
            let mut b = Brick {
                length: 0,
                direction: Direction::x,
            };
            for i in 0..3 {
                if bounds[0][i] != bounds[1][i] {
                    b = Brick {
                        length: bounds[1][i] - bounds[0][i] + 1,
                        direction: match i {
                            0 => Direction::x,
                            1 => Direction::y,
                            2 => Direction::z,
                            _ => panic!("wrong direction"),
                        },
                    };
                }
            }

            b
        })
        .collect()
}

fn part1(input: Vec<Brick>) -> u32 {
    println!("{input:?}");
    0
}
// fn part2(input: Vec<&str>) {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
    ";

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
