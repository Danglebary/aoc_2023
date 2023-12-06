mod day_five;
mod day_four;
mod day_one;
mod day_six;
mod day_three;
mod day_two;
mod utils;

use std::env;

use crate::utils::log::Logger;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = args[1].as_str();
    let part = args[2].as_str();

    Logger::clear_screen();
    Logger::divider();
    Logger::banner();
    Logger::divider();
    Logger::day(day, part);
    Logger::divider();

    match day {
        "1" => match part {
            "1" => day_one::part_one::run(),
            "2" => day_one::part_two::run(),
            _ => println!("Invalid part number: {}", part),
        },
        "2" => match part {
            "1" => day_two::part_one::run(),
            "2" => day_two::part_two::run(),
            _ => println!("Invalid part number: {}", part),
        },
        "3" => match part {
            "1" => day_three::part_one::run(),
            "2" => day_three::part_two::run(),
            _ => println!("Invalid part number: {}", part),
        },
        "4" => match part {
            "1" => day_four::part_one::run(),
            "2" => day_four::part_two::run(),
            _ => println!("Invalid part number: {}", part),
        },
        "5" => match part {
            "1" => day_five::part_one::run(),
            "2" => day_five::part_two::run(),
            _ => println!("Invalid part number: {}", part),
        },
        "6" => match part {
            "1" => day_six::part_one::run(),
            "2" => day_six::part_two::run(),
            _ => println!("Invalid part number: {}", part),
        },
        _ => println!("Invalid day number: {}", day),
    }

    Logger::divider();
}
