use regex::Regex;

use crate::utils::{self, log::Logger};

const NUMBER_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn run() {
    let input = utils::file::read_file_lines("src/day_one/input.txt").unwrap();

    // need to grab first and last number from string, without any letters
    let pattern = Regex::new(r"([0-9])").unwrap();

    let result: u64 = input
        .iter()
        .map(|line| {
            // alright, now we need to see if any of the NUMBER_STRINGS are found in the line
            // if so, we need to replace them with their number equivalent.
            // We also want to make sure we don't replace the full string, so we'll replace the middle character with the corresponding number
            // this will ensure we don't miss any letters that overlap with other string numbers
            let mut tmp_line = line.clone();

            for number_string in NUMBER_STRINGS.iter() {
                if tmp_line.contains(number_string) {
                    match *number_string {
                        "one" => tmp_line = tmp_line.replace(number_string, "o1e"),
                        "two" => tmp_line = tmp_line.replace(number_string, "t2o"),
                        "three" => tmp_line = tmp_line.replace(number_string, "t3hree"),
                        "four" => tmp_line = tmp_line.replace(number_string, "f4our"),
                        "five" => tmp_line = tmp_line.replace(number_string, "f5ive"),
                        "six" => tmp_line = tmp_line.replace(number_string, "s6ix"),
                        "seven" => tmp_line = tmp_line.replace(number_string, "s7even"),
                        "eight" => tmp_line = tmp_line.replace(number_string, "e8ight"),
                        "nine" => tmp_line = tmp_line.replace(number_string, "n9ine"),
                        _ => (),
                    }
                }
            }

            let result_strs: Vec<&str> = pattern
                .captures_iter(tmp_line.as_str())
                .map(|cap| cap.get(0).unwrap().as_str())
                .collect();

            let first = result_strs[0];
            let last = result_strs[result_strs.len() - 1];

            format!("{first}{last}").parse::<u64>().unwrap()
        })
        .sum();

    Logger::numeric_answer(result as u64);
}
