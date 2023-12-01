use std::fs;
use std::io;
use std::path::Path;

pub fn read_file(day: &str) -> io::Result<String> {
    let path = Path::new("inputs").join(format!("day{day}"));
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

#[macro_export]
macro_rules! get_input {
    ($file:expr) => {{
        let day = regex::Regex::new(r".*day(\d+).*")
            .unwrap()
            .captures($file)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();
        aoc2023::read_file(day).unwrap()
    }};
}

// pub fn get_input() {
//     let f = file!();

//     // let file = read_file(day).unwrap();
//     // let game = parse_input(&file);
// }
