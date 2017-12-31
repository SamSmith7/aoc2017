use std::env;

mod day1;
mod day2;
mod day3;

fn main() {

    let args: Vec<String> = env::args().collect();
    let day: &str = &args[1];
    let input: &str = &args[2];

    match day {
        "day1" => day1::run(input),
        "day2" => day2::run(input),
        "day3" => day3::run(input),
        _ => println!("No Matching Day")
    }
}
