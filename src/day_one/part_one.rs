use regex::{Captures, Regex};

use crate::utils::{file, log::Logger};

#[allow(dead_code)]
pub fn run() {
    let input = file::read_file_lines("src/day_one/input.txt").unwrap();

    // need to grab first and last number from string, without any letters
    let pattern = Regex::new(r"(?<digits>[0-9])").unwrap();

    let result = input
        .iter()
        .map(|line| {
            let result_strs: Vec<Captures> = pattern.captures_iter(line).collect();
            let first = result_strs[0].get(0).unwrap().as_str();
            let last = result_strs[result_strs.len() - 1].get(0).unwrap().as_str();
            format!("{}{}", first, last).parse::<usize>().unwrap()
        })
        .collect::<Vec<usize>>();

    let mut answer_number: usize = 0;

    for num in result {
        answer_number += num;
    }

    Logger::numeric_answer(answer_number);

    // dump_to_file("src/day_1/part_one_answer.txt", str).unwrap();
}
