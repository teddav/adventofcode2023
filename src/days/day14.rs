use aoc2023::get_input;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Pos(usize, usize);

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    // println!("Part2: {}", part2(parse_input(&input)));
}

fn parse_input(input: &str) -> (Vec<Pos>, Vec<Pos>, usize) {
    let mut rounds = vec![];
    let mut cubes = vec![];
    let mut height = 0;

    for (y, line) in input.trim().split("\n").enumerate() {
        for (x, c) in line.char_indices() {
            if c == 'O' {
                rounds.push(Pos(x, y));
            }
            if c == '#' {
                cubes.push(Pos(x, y));
            }
        }
        height = y + 1;
    }
    (rounds, cubes, height)
}

fn part1((mut rounds, cubes, height): (Vec<Pos>, Vec<Pos>, usize)) -> usize {
    tilt_north(&mut rounds, cubes);
    rounds.iter().map(|rock| height - rock.1).sum()
}

fn tilt_north(rounds: &mut Vec<Pos>, cubes: Vec<Pos>) {
    for i in 0..rounds.len() {
        let pos = rounds[i];
        let mut y = pos.1;
        while y > 0 {
            y -= 1;
            let north_pos = Pos(pos.0, y);

            // if it's a cube, we cannot move, we stop
            if cubes.contains(&north_pos) {
                break;
            }

            // if it's a round, we need to check higher, to see if there is an empty space
            if rounds.contains(&north_pos) {
                continue;
            }

            rounds[i].1 = y;
        }
    }
}

// fn part2(input: Vec<&str>) {}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 136);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 111979);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(parse_input(EXAMPLE)), 0);
    //     assert_eq!(part2(parse_input(&get_input!(file!()))), 0);
    // }
}
