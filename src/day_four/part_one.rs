use crate::utils::{file, log::Logger};

#[allow(dead_code)]
pub fn run() {
    let input = file::read_file_lines("src/day_four/input.txt").unwrap();
    // let input = file::read_file_lines("src/day_four/sample.txt").unwrap();

    let mut score: usize = 0;

    for line in input.iter() {
        let parts: Vec<&str> = line.split("|").collect();
        let mut winning_nums_raw: Vec<&str> = parts[0]
            .split(":")
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>();

        winning_nums_raw.retain(|&num| num != "");

        let winning_nums: Vec<u8> = winning_nums_raw
            .iter()
            .map(|num| num.parse::<u8>().unwrap())
            .collect();

        let mut ticket_nums_raw: Vec<&str> = parts[1].split(" ").collect::<Vec<&str>>();

        ticket_nums_raw.retain(|&num| num != "");

        let ticket_nums: Vec<u8> = ticket_nums_raw
            .iter()
            .map(|num| num.parse::<u8>().unwrap())
            .collect();

        let mut our_winners: Vec<&u8> = Vec::new();

        for num in ticket_nums.iter() {
            if winning_nums.contains(num) {
                our_winners.push(num);
            }
        }

        let mut our_score: usize = 0;
        if our_winners.len() > 0 {
            our_score = 1;
            for _ in 1..our_winners.len() {
                our_score *= 2;
            }
        }

        score += our_score;
    }

    Logger::numeric_answer(score);
}
