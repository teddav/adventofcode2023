use aoc2023::get_input;

#[allow(dead_code)]
pub fn main() {
    let input = get_input!(file!());
    parse_input(&input);
}

fn parse_input(input: &str) {
    println!("{input}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_day_part1() {
        parse_input(EXAMPLE);
    }
}
