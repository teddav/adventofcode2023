use regex::Regex;
use std::{
    fs::{read_dir, File},
    io::Write,
};

#[derive(Debug, Clone, Copy)]
enum Part {
    A,
    B,
    None,
}
impl Part {
    fn to_char(&self) -> String {
        match self {
            Part::A => "a".to_string(),
            Part::B => "b".to_string(),
            Part::None => "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Day {
    number: u8,
    part: Part,
}

fn main() {
    let re = Regex::new(r"day(\d+)(a|b)?").unwrap();

    let days: Vec<Day> = read_dir("src/days")
        .unwrap()
        .map(|f| String::from(f.unwrap().path().file_stem().unwrap().to_str().unwrap()))
        .filter(|f| f.starts_with("day"))
        .map(|filename| {
            let captures = re.captures(&filename).unwrap();
            Day {
                number: captures.get(1).unwrap().as_str().parse::<u8>().unwrap(),
                part: if let Some(p) = captures.get(2) {
                    match p.as_str() {
                        "a" => Part::A,
                        "b" => Part::B,
                        _ => panic!("unknown part"),
                    }
                } else {
                    Part::None
                },
            }
        })
        .collect();
    println!("{:?}", days);

    let latest_day = days.iter().fold(
        Day {
            number: 0,
            part: Part::None,
        },
        |latest, day| {
            println!("day: {day:?}");
            let n = day.number;
            if n < latest.number {
                return latest;
            }
            if n > latest.number {
                return *day;
            }

            // otherwise it's the same day
            if let Part::None = day.part {
                *day
            } else {
                Day {
                    number: n,
                    part: Part::B,
                }
            }
        },
    );
    println!("latest: {latest_day:?}");

    let mut days_mod = days
        .iter()
        .map(|d| format!("pub mod day{}{};", d.number, d.part.to_char()))
        .collect::<Vec<String>>()
        .join("\n");
    days_mod.push_str("\n\n");
    days_mod.push_str(&format!(
        r#"#[path = "day{}{}.rs"]"#,
        latest_day.number,
        latest_day.part.to_char()
    ));
    days_mod.push_str("\npub mod latest;");

    let mut days_main_functions = String::from("pub const days_main: &[(&str, fn())] = &[\n");
    days_main_functions.push_str(
        &days
            .into_iter()
            .map(|day| {
                println!("d: {day:?}");
                let number = format!("{}{}", day.number, day.part.to_char());
                format!("\t(\"{}\", day{}::main),", number, number)
            })
            .collect::<Vec<String>>()
            .join("\n"),
    );
    days_main_functions.push_str("\n];");
    println!("{days_main_functions:?}");

    let mut mod_file = File::create("src/days/mod.rs").unwrap();
    mod_file.write_all(days_mod.as_bytes()).unwrap();
    mod_file.write_all("\n\n".as_bytes()).unwrap();
    mod_file.write_all(days_main_functions.as_bytes()).unwrap();
}
