use aoc2023::get_input;
use pathfinding::prelude::astar;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    pos: Position,
    value: u32,
    dir: Direction,
    straight_inarow: u32,
    heat_loss: u32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value).reverse()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Position(usize, usize);
impl Position {
    fn next(
        &self,
        dir: Direction,
        max: (usize, usize),
        straight_in_a_row: u32,
    ) -> Vec<(Self, Direction, u32)> {
        let mut next_pos = vec![];

        use Direction::*;
        match dir {
            Left => {
                if self.0 > 0 && straight_in_a_row < 3 {
                    next_pos.push((Position(self.0 - 1, self.1), Left, straight_in_a_row + 1));
                }
                if self.1 > 0 {
                    next_pos.push((Position(self.0, self.1 - 1), Up, 1));
                }
                if self.1 < max.1 {
                    next_pos.push((Position(self.0, self.1 + 1), Down, 1));
                }
            }
            Right => {
                if self.0 < max.0 && straight_in_a_row < 3 {
                    next_pos.push((Position(self.0 + 1, self.1), Right, straight_in_a_row + 1));
                }
                if self.1 > 0 {
                    next_pos.push((Position(self.0, self.1 - 1), Up, 1));
                }
                if self.1 < max.1 {
                    next_pos.push((Position(self.0, self.1 + 1), Down, 1));
                }
            }
            Down => {
                if self.1 < max.1 && straight_in_a_row < 3 {
                    next_pos.push((Position(self.0, self.1 + 1), Down, straight_in_a_row + 1));
                }
                if self.0 > 0 {
                    next_pos.push((Position(self.0 - 1, self.1), Left, 1));
                }
                if self.0 < max.1 {
                    next_pos.push((Position(self.0 + 1, self.1), Right, 1));
                }
            }
            Up => {
                if self.1 > 0 && straight_in_a_row < 3 {
                    next_pos.push((Position(self.0, self.1 - 1), Up, straight_in_a_row + 1));
                }
                if self.0 > 0 {
                    next_pos.push((Position(self.0 - 1, self.1), Left, 1));
                }
                if self.0 < max.1 {
                    next_pos.push((Position(self.0 + 1, self.1), Right, 1));
                }
            }
        };

        next_pos
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .split("\n")
        .map(|line| line.chars().map(|c| (c as u32) - ('0' as u32)).collect())
        .collect()
}

fn part1(map: Vec<Vec<u32>>) -> u32 {
    let max = (map[0].len() - 1, map.len() - 1);
    astar(
        &Node {
            pos: Position(0, 0),
            value: map[0][0],
            dir: Direction::Right,
            straight_inarow: 0,
            heat_loss: 0,
        },
        |n| {
            n.pos
                .next(n.dir, max, n.straight_inarow)
                .iter()
                .map(|(pos, dir, straight_inarow)| {
                    let value = map[pos.1][pos.0];
                    (
                        Node {
                            pos: *pos,
                            value: value,
                            dir: *dir,
                            straight_inarow: *straight_inarow,
                            heat_loss: n.heat_loss + value,
                        },
                        value,
                    )
                })
                .collect::<Vec<(Node, u32)>>()
        },
        |p| p.value,
        |p| p.pos == Position(max.0, max.1),
    )
    .unwrap()
    .1
}

#[allow(dead_code)]
fn part1_almost(map: Vec<Vec<u32>>) -> u32 {
    let max = (map[0].len() - 1, map.len() - 1);

    let mut path = BinaryHeap::new();
    let mut seen = HashSet::new();

    path.push(Node {
        pos: Position(0, 0),
        value: map[0][0],
        dir: Direction::Right,
        straight_inarow: 0,
        heat_loss: 0,
    });

    while let Some(Node {
        pos,
        dir,
        straight_inarow,
        heat_loss,
        ..
    }) = path.pop()
    {
        seen.insert(pos);

        if pos == Position(max.0, max.1) {
            return heat_loss;
        }

        for (next_pos, next_dir, next_straight_inarow) in pos.next(dir, max, straight_inarow) {
            if seen.contains(&next_pos) {
                continue;
            }
            let next = Node {
                pos: next_pos,
                value: map[next_pos.1][next_pos.0],
                dir: next_dir,
                straight_inarow: next_straight_inarow,
                heat_loss: heat_loss + map[next_pos.1][next_pos.0],
            };
            path.push(next);
        }
    }
    0
}

// fn part2(input: Vec<&str>) {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 102);
        // assert_eq!(part1(parse_input(&get_input!(file!()))), 928);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(parse_input(EXAMPLE)), 0);
    //     assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    // }
}
