use crate::utils::{file, log::Logger};

#[allow(dead_code)]
pub fn run() {
    let input = file::read_file_lines("src/day_six/input.txt").unwrap();
    // let input = file::read_file_lines("src/day_six/sample.txt").unwrap();

    let time_line = input.iter().nth(0).unwrap();
    let time_parts = time_line.split(":").collect::<Vec<&str>>();
    let time: u64 = time_parts[1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let dist_line = input.iter().nth(1).unwrap();
    let dist_parts = dist_line.split(":").collect::<Vec<&str>>();
    let dist: u64 = dist_parts[1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    let mut scores: Vec<u64> = Vec::new();

    for speed in 1..time + 1 {
        let time_left = time - speed;
        let our_dist = speed * time_left;
        if our_dist > dist {
            scores.push(dist);
        }
    }

    let answer = scores.len();
    Logger::numeric_answer(answer as u64);
}
