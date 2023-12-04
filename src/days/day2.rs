use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    println!("Part1: {}", part1(parse_input(&input)));
    println!("Part2: {}", part2(parse_input(&input)));
}

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u8,
    sets: Vec<Set>,
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let mut split = line.split(":");
            let game_id = split
                .next()
                .unwrap()
                .split(" ")
                .last()
                .unwrap()
                .parse::<u8>()
                .unwrap();
            let sets: Vec<Set> = split
                .next()
                .unwrap()
                .split(";")
                .map(|set| {
                    let set = set
                        .split(",")
                        .map(|draw| {
                            let mut cubes = draw.trim().split(" ");
                            (
                                cubes.nth(0).unwrap().parse::<u32>().unwrap(),
                                cubes.nth(0).unwrap(),
                            )
                        })
                        .collect::<Vec<(u32, &str)>>();
                    Set {
                        red: set.iter().find(|s| s.1 == "red").map_or(0, |s| s.0),
                        green: set.iter().find(|s| s.1 == "green").map_or(0, |s| s.0),
                        blue: set.iter().find(|s| s.1 == "blue").map_or(0, |s| s.0),
                    }
                })
                .collect();
            Game { id: game_id, sets }
        })
        .collect()
}

fn part1(games: Vec<Game>) -> u32 {
    let mut result = 0;
    for game in games {
        if game
            .sets
            .iter()
            .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
        {
            result += game.id as u32;
        }
    }
    result
}

fn part2(games: Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| {
            let rgb_min = game.sets.iter().fold((0, 0, 0), |min, set| {
                (
                    if set.red > min.0 { set.red } else { min.0 },
                    if set.green > min.1 { set.green } else { min.1 },
                    if set.blue > min.2 { set.blue } else { min.2 },
                )
            });
            rgb_min.0 * rgb_min.1 * rgb_min.2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(EXAMPLE)), 8);
        assert_eq!(part1(parse_input(&get_input!(file!()))), 2716);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(EXAMPLE)), 2286);
        assert_eq!(part2(parse_input(&get_input!(file!()))), 72227);
    }
}
