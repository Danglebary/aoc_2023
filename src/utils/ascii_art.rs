use colored::*;
use std::fmt::Display;

use super::file;

#[allow(dead_code)]
pub enum AsciiArtNumbers {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eightteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
}

impl Display for AsciiArtNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content: Vec<String> = vec![
            file::read("ascii_art/zero.txt").unwrap(),
            file::read("ascii_art/one.txt").unwrap(),
            file::read("ascii_art/two.txt").unwrap(),
            file::read("ascii_art/three.txt").unwrap(),
            file::read("ascii_art/four.txt").unwrap(),
            file::read("ascii_art/five.txt").unwrap(),
            file::read("ascii_art/six.txt").unwrap(),
            file::read("ascii_art/seven.txt").unwrap(),
            file::read("ascii_art/eight.txt").unwrap(),
            file::read("ascii_art/nine.txt").unwrap(),
        ];

        let message = match self {
            AsciiArtNumbers::Zero => content[0].clone(),
            AsciiArtNumbers::One => content[1].clone(),
            AsciiArtNumbers::Two => content[2].clone(),
            AsciiArtNumbers::Three => content[3].clone(),
            AsciiArtNumbers::Four => content[4].clone(),
            AsciiArtNumbers::Five => content[5].clone(),
            AsciiArtNumbers::Six => content[6].clone(),
            AsciiArtNumbers::Seven => content[7].clone(),
            AsciiArtNumbers::Eight => content[8].clone(),
            AsciiArtNumbers::Nine => content[9].clone(),
            AsciiArtNumbers::Ten => format!("{}{}", content[1], content[0]),
            AsciiArtNumbers::Eleven => format!("{}{}", content[1], content[1]),
            AsciiArtNumbers::Twelve => format!("{}{}", content[1], content[2]),
            AsciiArtNumbers::Thirteen => format!("{}{}", content[1], content[3]),
            AsciiArtNumbers::Fourteen => format!("{}{}", content[1], content[4]),
            AsciiArtNumbers::Fifteen => format!("{}{}", content[1], content[5]),
            AsciiArtNumbers::Sixteen => format!("{}{}", content[1], content[6]),
            AsciiArtNumbers::Seventeen => format!("{}{}", content[1], content[7]),
            AsciiArtNumbers::Eightteen => format!("{}{}", content[1], content[8]),
            AsciiArtNumbers::Nineteen => format!("{}{}", content[1], content[9]),
            AsciiArtNumbers::Twenty => format!("{}{}", content[2], content[0]),
            AsciiArtNumbers::TwentyOne => format!("{}{}", content[2], content[1]),
            AsciiArtNumbers::TwentyTwo => format!("{}{}", content[2], content[2]),
            AsciiArtNumbers::TwentyThree => format!("{}{}", content[2], content[3]),
            AsciiArtNumbers::TwentyFour => format!("{}{}", content[2], content[4]),
            AsciiArtNumbers::TwentyFive => format!("{}{}", content[2], content[5]),
        };

        write!(f, "{message}").expect("Failed to write ascii art number to stdout");
        Ok(())
    }
}

pub fn banner() {
    let message = file::read("ascii_art/banner.txt")
        .unwrap()
        .color(Color::Cyan);
    println!("{message}")
}

pub fn divider() {
    let message = file::read("ascii_art/divider.txt")
        .unwrap()
        .color(Color::Blue);
    println!("{message}")
}

pub fn day_and_part(day: &str, part: &str) {
    let ascii_day = file::read("ascii_art/day.txt").unwrap();
    let ascii_part = file::read("ascii_art/part.txt").unwrap();
    let ascii_dash = file::read("ascii_art/dash.txt").unwrap();

    let day_digits = match day {
        "1" => AsciiArtNumbers::One.to_string(),
        "2" => AsciiArtNumbers::Two.to_string(),
        "3" => AsciiArtNumbers::Three.to_string(),
        "4" => AsciiArtNumbers::Four.to_string(),
        "5" => AsciiArtNumbers::Five.to_string(),
        "6" => AsciiArtNumbers::Six.to_string(),
        "7" => AsciiArtNumbers::Seven.to_string(),
        "8" => AsciiArtNumbers::Eight.to_string(),
        "9" => AsciiArtNumbers::Nine.to_string(),
        "10" => AsciiArtNumbers::Ten.to_string(),
        "11" => AsciiArtNumbers::Eleven.to_string(),
        "12" => AsciiArtNumbers::Twelve.to_string(),
        "13" => AsciiArtNumbers::Thirteen.to_string(),
        "14" => AsciiArtNumbers::Fourteen.to_string(),
        "15" => AsciiArtNumbers::Fifteen.to_string(),
        "16" => AsciiArtNumbers::Sixteen.to_string(),
        "17" => AsciiArtNumbers::Seventeen.to_string(),
        "18" => AsciiArtNumbers::Eightteen.to_string(),
        "19" => AsciiArtNumbers::Nineteen.to_string(),
        "20" => AsciiArtNumbers::Twenty.to_string(),
        "21" => AsciiArtNumbers::TwentyOne.to_string(),
        "22" => AsciiArtNumbers::TwentyTwo.to_string(),
        "23" => AsciiArtNumbers::TwentyThree.to_string(),
        "24" => AsciiArtNumbers::TwentyFour.to_string(),
        "25" => AsciiArtNumbers::TwentyFive.to_string(),
        _ => panic!("Invalid day number: {}", day),
    };

    let part_digits = match part {
        "1" => AsciiArtNumbers::One.to_string(),
        "2" => AsciiArtNumbers::Two.to_string(),
        _ => panic!("Invalid part number: {}", part),
    };

    let ascii_day_lines: Vec<&str> = ascii_day.split('\n').collect();
    let ascii_part_lines: Vec<&str> = ascii_part.split('\n').collect();
    let ascii_dash_lines: Vec<&str> = ascii_dash.split('\n').collect();
    let ascii_day_digits_lines: Vec<&str> = day_digits.split('\n').collect();
    let ascii_part_digits_lines: Vec<&str> = part_digits.split('\n').collect();

    let mut message = String::new();
    // we want to build the message line by line vertically
    for i in 0..ascii_day_lines.len() {
        message.push_str(&format!(
            "{}{}{}{}{}\n",
            ascii_day_lines[i],
            ascii_day_digits_lines[i].color(Color::TrueColor {
                r: 222,
                g: 91,
                b: 217
            }),
            ascii_dash_lines[i],
            ascii_part_lines[i],
            ascii_part_digits_lines[i].color(Color::TrueColor {
                r: 186,
                g: 122,
                b: 235
            })
        ));
    }

    print!("{message}");
}

pub fn number_to_ascii(number: u64) -> String {
    let number_str = number.to_string();

    let ascii_nums: Vec<String> = number_str
        .chars()
        .map(|c| match c {
            '0' => AsciiArtNumbers::Zero.to_string(),
            '1' => AsciiArtNumbers::One.to_string(),
            '2' => AsciiArtNumbers::Two.to_string(),
            '3' => AsciiArtNumbers::Three.to_string(),
            '4' => AsciiArtNumbers::Four.to_string(),
            '5' => AsciiArtNumbers::Five.to_string(),
            '6' => AsciiArtNumbers::Six.to_string(),
            '7' => AsciiArtNumbers::Seven.to_string(),
            '8' => AsciiArtNumbers::Eight.to_string(),
            '9' => AsciiArtNumbers::Nine.to_string(),
            _ => panic!("Invalid number: {}", number),
        })
        .collect();

    let num_lines = ascii_nums[0].lines().collect::<Vec<&str>>().len();

    let mut result = String::new();

    for i in 0..num_lines {
        let mut line = String::new();
        for num in &ascii_nums {
            line.push_str(num.lines().collect::<Vec<&str>>()[i].to_string().as_str());
        }
        result.push_str(&format!("{}\n", line));
    }

    result
}

pub fn numeric_answer(numeric_answer: u64) {
    let ascii_answer = file::read("ascii_art/answer.txt").unwrap();
    let ascii_colon = file::read("ascii_art/colon.txt").unwrap();

    let ascii_numeric_answer = number_to_ascii(numeric_answer);

    let ascii_answer_lines: Vec<&str> = ascii_answer.split('\n').collect();
    let ascii_colon_lines: Vec<&str> = ascii_colon.split('\n').collect();
    let ascii_numeric_answer_lines: Vec<&str> = ascii_numeric_answer.split('\n').collect();

    let mut message = String::new();

    for i in 0..ascii_answer_lines.len() {
        message.push_str(&format!(
            "{}{}{}\n",
            ascii_answer_lines[i],
            ascii_colon_lines[i],
            ascii_numeric_answer_lines[i].color(Color::Green),
        ));
    }

    print!("{message}");
}
