use clap::Parser;
mod days;
use std::time::Instant;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,

    #[arg(short, long, conflicts_with("day"), default_missing_value("true"), num_args(0..=1), require_equals(true))]
    bench: Option<bool>,
}

fn main() {
    let args = Args::parse();

    if args.bench.is_some() {
        for day in days::days_main {
            let start = Instant::now();
            println!("Day {}", day.0);
            day.1();
            println!("{:?}\n", start.elapsed());
        }
        return;
    }

    if let Some(day) = args.day {
        let day_to_run = &format!("{day}");
        days::days_main
            .iter()
            .find(|d| d.0 == day_to_run)
            .expect("This day doesnt exist!")
            .1();
    } else {
        days::latest::main();
    }
}
