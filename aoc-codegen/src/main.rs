use chrono::Datelike;
use chrono::Local;
use clap::Parser;

mod data;

fn day_num_from_today() -> u8 {
    let today = Local::now();
    today.day() as u8
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The day to codegen
    #[arg(short, long, default_value_t=day_num_from_today())]
    day: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
