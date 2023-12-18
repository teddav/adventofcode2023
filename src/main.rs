use clap::Parser;
mod days;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,
}

fn main() {
    let args = Args::parse();

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
