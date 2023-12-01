use regex::Regex;
use std::{
    fs::{read_dir, File},
    io::Write,
};

fn main() {
    let re = Regex::new(r"day(\d+)(a|b)?").unwrap();

    let day = "day3a";
    let captures = re.captures(day).unwrap();
    let n = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
    let part = captures.get(2);

    println!("n {n:?}");
    println!("part {part:?}");

    let days: Vec<_> = read_dir("src/days")
        .unwrap()
        .map(|f| String::from(f.unwrap().path().file_stem().unwrap().to_str().unwrap()))
        .filter(|f| f.starts_with("day"))
        .collect();
    println!("{:?}", days);

    let re = Regex::new(r"day(\d+)(a|b)?").unwrap();
    let latest_day = days.iter().fold((1, ""), |latest, filename| {
        let captures = re.captures(filename).unwrap();
        let n = captures.get(1).unwrap().as_str().parse::<u8>().unwrap();
        if n < latest.0 {
            return latest;
        }

        let part = if let Some(part) = captures.get(2) {
            part.as_str()
        } else {
            ""
        };
        if n > latest.0 {
            return (n, part);
        }
        if part == "" {
            (n, "")
        } else {
            (n, "b")
        }
    });
    println!("latet: {latest_day:?}");

    let mut days_mod = days
        .iter()
        .map(|d| format!("pub mod {};", d))
        .collect::<Vec<String>>()
        .join("\n");
    days_mod.push_str(&format!(
        r#"
    #[path = "day{}{}.rs"]
    pub mod latest;"#,
        latest_day.0, latest_day.1
    ));
    let mut mod_file = File::create("src/days/mod.rs").unwrap();
    mod_file.write_all(days_mod.as_bytes()).unwrap();
}
