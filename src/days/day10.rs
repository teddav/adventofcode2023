use aoc2023::get_input;
use std::{
    collections::{HashSet, VecDeque},
    ops::Rem,
};

type Map = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(usize, usize);
impl Position {
    fn next(&self, map: &Map) -> Vec<Position> {
        let max = (map[0].len(), map.len());
        let mut _next = vec![];
        match map[self.1][self.0] {
            '|' => {
                if self.1 > 0 {
                    _next.push(Position(self.0, self.1 - 1));
                }
                if self.1 < max.1 {
                    _next.push(Position(self.0, self.1 + 1));
                }
            }
            '-' => {
                if self.0 > 0 {
                    _next.push(Position(self.0 - 1, self.1));
                }
                if self.0 < max.0 {
                    _next.push(Position(self.0 + 1, self.1));
                }
            }
            'L' => {
                if self.1 > 0 {
                    _next.push(Position(self.0, self.1 - 1));
                }
                if self.0 < max.0 {
                    _next.push(Position(self.0 + 1, self.1));
                }
            }
            'J' => {
                if self.1 > 0 {
                    _next.push(Position(self.0, self.1 - 1));
                }
                if self.0 > 0 {
                    _next.push(Position(self.0 - 1, self.1));
                }
            }
            '7' => {
                if self.1 < max.1 {
                    _next.push(Position(self.0, self.1 + 1));
                }
                if self.0 > 0 {
                    _next.push(Position(self.0 - 1, self.1));
                }
            }
            'F' => {
                if self.0 < max.0 {
                    _next.push(Position(self.0 + 1, self.1));
                }
                if self.1 < max.1 {
                    _next.push(Position(self.0, self.1 + 1));
                }
            }
            '.' => panic!("you're in the wrong place"),
            _ => panic!("unknown pipe {}", map[self.1][self.0]),
        }

        _next
    }
}

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> (Map, Position) {
    let mut start = Position(0, 0);
    let map = input
        .trim()
        .split("\n")
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = Position(x, y);
                    }
                    c
                })
                .collect()
        })
        .collect();
    (map, start)
}

fn get_start_pipe(map: &Map, start: &Position) -> char {
    let mut s: HashSet<char> = HashSet::from_iter(vec!['|', '-', 'L', 'J', '7', 'F']);
    if start.0 > 0 {
        if let Some(&left) = map.get(start.1).and_then(|line| line.get(start.0 - 1)) {
            if left == '-' || left == 'L' || left == 'F' {
                s = s
                    .intersection(&HashSet::from_iter(vec!['-', '7', 'J']))
                    .cloned()
                    .collect();
            }
        }
    }
    if start.1 > 0 {
        if let Some(&up) = map.get(start.1 - 1).and_then(|line| line.get(start.0)) {
            if up == '|' || up == '7' || up == 'F' {
                s = s
                    .intersection(&HashSet::from_iter(vec!['|', 'J', 'L']))
                    .cloned()
                    .collect();
            }
        }
    }
    if let Some(&right) = map.get(start.1).and_then(|line| line.get(start.0 + 1)) {
        if right == '-' || right == '7' || right == 'J' {
            s = s
                .intersection(&HashSet::from_iter(vec!['-', 'L', 'F']))
                .cloned()
                .collect();
        }
    }
    if let Some(&down) = map.get(start.1 + 1).and_then(|line| line.get(start.0)) {
        if down == '|' || down == 'L' || down == 'J' {
            s = s
                .intersection(&HashSet::from_iter(vec!['|', '7', 'F']))
                .cloned()
                .collect();
        }
    }

    s.into_iter().next().unwrap()
}

fn part1((mut map, start): (Map, Position)) -> usize {
    map[start.1][start.0] = get_start_pipe(&map, &start);

    let mut positions = VecDeque::from_iter(vec![(start, 0)]);
    let mut already_seen: HashSet<Position> = HashSet::new();
    let mut max = 0;

    while let Some((position, moves)) = positions.pop_front() {
        if moves > max {
            max = moves;
        }

        if already_seen.contains(&position) || map[position.1][position.0] == '.' {
            continue;
        }
        already_seen.insert(position);

        for next_position in position.next(&map) {
            if !already_seen.contains(&next_position) {
                positions.push_back((next_position, moves + 1));
            }
        }
    }

    assert_eq!(max, already_seen.len() / 2);
    // print_path(
    //     &already_seen.into_iter().collect(),
    //     (map[0].len(), map.len()),
    // );

    max
}

fn part2((mut map, start): (Map, Position)) -> usize {
    map[start.1][start.0] = get_start_pipe(&map, &start);

    let mut positions = VecDeque::from_iter(vec![start]);
    let mut already_seen: HashSet<Position> = HashSet::new();

    while let Some(current_position) = positions.pop_front() {
        if already_seen.contains(&current_position)
            || map[current_position.1][current_position.0] == '.'
        {
            continue;
        }
        already_seen.insert(current_position);

        for next_position in current_position.next(&map) {
            if !already_seen.contains(&next_position) {
                positions.push_back(next_position);
            }
        }
    }

    // print_path(
    //     &already_seen.clone().into_iter().collect(),
    //     (map[0].len(), map.len()),
    // );

    let max_x = map[0].len();
    let max_y = map.len();
    let mut enclosed_points = vec![];
    for y in 0..max_y {
        for x in 0..max_x {
            if already_seen.contains(&Position(x, y)) {
                continue;
            }

            let left = (0..x).fold(0, |acc, i| {
                if !already_seen.contains(&Position(i, y)) {
                    return acc;
                }
                let pipe = map[y][i];
                if pipe == '|' {
                    return acc + 1;
                }

                for j in i + 1..x {
                    let next_pipe = map[y][j];
                    if next_pipe == '-' {
                        continue;
                    }
                    if (pipe == 'L' && next_pipe == '7') || (pipe == 'F' && next_pipe == 'J') {
                        return acc + 1;
                    }
                    break;
                }

                acc
            });
            if left == 0 || left.rem(2) == 0 {
                continue;
            }

            let right = (x + 1..max_x).fold(0, |acc, i| {
                if !already_seen.contains(&Position(i, y)) {
                    return acc;
                }
                let pipe = map[y][i];
                if pipe == '|' {
                    return acc + 1;
                }

                for j in i + 1..max_x {
                    let next_pipe = map[y][j];
                    if next_pipe == '-' {
                        continue;
                    }
                    if (pipe == 'L' && next_pipe == '7') || (pipe == 'F' && next_pipe == 'J') {
                        return acc + 1;
                    }
                    break;
                }

                acc
            });
            if right == 0 || right.rem(2) == 0 {
                continue;
            }

            let up = (0..y).fold(0, |acc, i| {
                if !already_seen.contains(&Position(x, i)) {
                    return acc;
                }
                let pipe = map[i][x];
                if pipe == '-' {
                    return acc + 1;
                }

                for j in i + 1..y {
                    let next_pipe = map[j][x];
                    if next_pipe == '|' {
                        continue;
                    }
                    if (pipe == '7' && next_pipe == 'L') || (pipe == 'F' && next_pipe == 'J') {
                        return acc + 1;
                    }
                    break;
                }

                acc
            });
            if up == 0 || up.rem(2) == 0 {
                continue;
            }

            let down = (y + 1..max_y).fold(0, |acc, i| {
                if !already_seen.contains(&Position(x, i)) {
                    return acc;
                }
                let pipe = map[i][x];
                if pipe == '-' {
                    return acc + 1;
                }

                for j in i + 1..max_y {
                    let next_pipe = map[j][x];
                    if next_pipe == '|' {
                        continue;
                    }
                    if (pipe == '7' && next_pipe == 'L') || (pipe == 'F' && next_pipe == 'J') {
                        return acc + 1;
                    }
                    break;
                }

                acc
            });
            if down == 0 || down.rem(2) == 0 {
                continue;
            }

            enclosed_points.push(Position(x, y));
        }
    }

    // print_path(&enclosed_points, (map[0].len(), map.len()));
    enclosed_points.len()
}

#[allow(dead_code)]
fn print_path(path: &Vec<Position>, max: (usize, usize)) {
    let mut map: Vec<String> = (0..max.1).map(|_| ".".repeat(max.0)).collect();
    for pos in path {
        map[pos.1].replace_range(pos.0..pos.0 + 1, "x");
    }
    println!("{map:#?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "
.....
.S-7.
.|.|.
.L-J.
.....
    ";
    const EXAMPLE2: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE1)), 4);
        assert_eq!(part1(parse_input(EXAMPLE2)), 8);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 6864);
    }

    const EXAMPLE3: &str = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
    ";
    const EXAMPLE4: &str = "
..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
    ";
    const EXAMPLE5: &str = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
    ";
    const EXAMPLE6: &str = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
    ";

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE3)), 4);
        assert_eq!(part2(parse_input(EXAMPLE4)), 4);
        assert_eq!(part2(parse_input(EXAMPLE5)), 8);
        assert_eq!(part2(parse_input(EXAMPLE6)), 10);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 349);
    }
}
