mod file_reader;
mod puzzles;

use puzzles::*;

fn main() {
    let day: String = std::env::args()
        .nth(1)
        .expect("Given day is not implemented yet");

    match day.as_str() {
        "01" => day01::run(),
        "02" => day02::run(),
        "03" => day03::run(),
        "04" => day04::run(),
        "05" => day05::run(),
        "06" => day06::run(),
        "07" => day07::run(),
        "08" => day08::run(),
        "09" => day09::run(),
        _ => println!("No valid day given"),
    };
}
