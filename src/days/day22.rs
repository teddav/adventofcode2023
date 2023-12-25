use std::iter::zip;

use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    x,
    y,
    z,
}

#[derive(Debug, PartialEq, Eq)]
struct Brick {
    x: (u32, u32),
    y: (u32, u32),
    z: (u32, u32),
    length: u32,
    direction: Direction,
}

impl Brick {
    fn is_blocked(&self, bricks: &Vec<Brick>) -> bool {
        if self.z.0 == 1 {
            return true;
        }
        let z = self.z.0 - 1;
        for b in bricks {
            if b == self {
                continue;
            }
            if z >= b.z.0 && z <= b.z.1 {
                if self.intersects(b) {
                    return true;
                }
            }
        }
        return false;
    }

    fn is_blocking(&self, bricks: &Vec<Brick>) -> bool {
        let z = self.z.1 + 1;
        for b in bricks {
            if b == self {
                continue;
            }
            if z >= b.z.0 && z <= b.z.1 {
                if self.intersects(b) {
                    return true;
                }
            }
        }
        return false;
    }

    fn intersects(&self, other: &Brick) -> bool {
        if self.x.0.max(other.x.0) <= self.x.1.min(other.x.1) {
            if self.y.0.max(other.y.0) <= self.y.1.min(other.y.1) {
                return true;
            }
        }
        return false;
    }
}

fn part1(mut bricks: Vec<Brick>) -> u32 {
    println!("{bricks:?}");
    bricks.sort_by(|a, b| a.z.0.cmp(&b.z.0));
    for i in 0..bricks.len() {
        while !bricks[i].is_blocked(&bricks) {
            bricks[i].z.0 -= 1;
            bricks[i].z.1 -= 1;
        }
    }

    for b in &bricks {
        if b.is_blocking(&bricks) {
            println!("blobk");
        }
    }
    0
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

            let bounds = zip(bounds[0], bounds[1]).collect::<Vec<(u32, u32)>>();
            let mut i = 0;
            let (length, direction) = loop {
                if i == bounds.len() {
                    break (1, Direction::x);
                }
                if bounds[i].0 != bounds[i].1 {
                    break (
                        bounds[i].1 - bounds[i].0 + 1,
                        match i {
                            0 => Direction::x,
                            1 => Direction::y,
                            2 => Direction::z,
                            _ => panic!("wrong direction"),
                        },
                    );
                }
                i += 1
            };
            Brick {
                x: (bounds[0].0, bounds[0].1),
                y: (bounds[1].0, bounds[1].1),
                z: (bounds[2].0, bounds[2].1),
                length,
                direction,
            }
        })
        .collect()
}

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
        assert_eq!(part1(parse_input(EXAMPLE)), 5);
        // assert_eq!(part1(parse_input(&get_input!(file!()))), 0);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(parse_input(EXAMPLE)), 0);
    //     assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    // }
}
