use std::collections::HashMap;

use crate::utils::{file, log::Logger};

#[allow(dead_code)]
pub fn run() {
    let input = file::read_file_lines("src/day_four/input.txt").unwrap();
    // let input = file::read_file_lines("src/day_four/sample.txt").unwrap();

    let mut thing_hash: HashMap<usize, usize> = HashMap::new();

    for (idx, _line) in input.iter().enumerate() {
        thing_hash.insert(idx, 1);
    }

    for (idx, line) in input.iter().enumerate() {
        // we need to see how many of the numbers in the ticket are in the winning numbers
        let parts: Vec<&str> = line.split("|").collect();

        let mut winning_nums: Vec<&str> = parts[0]
            .split(":")
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .split(" ")
            .collect();

        winning_nums.retain(|&num| num != "");

        let mut ticket_nums: Vec<&str> = parts[1].split(" ").collect();

        ticket_nums.retain(|&num| num != "");

        // println!("{:?} {:?}", winning_nums, ticket_nums);

        let mut our_winners: Vec<&str> = Vec::new();

        for num in ticket_nums.iter() {
            if winning_nums.contains(num) {
                our_winners.push(num);
            }
        }

        for (w_idx, _) in our_winners.iter().enumerate() {
            let next_idx = idx + w_idx.clone() + 1;
            if thing_hash.contains_key(&next_idx.clone()) {
                let copies = thing_hash.get(&next_idx.clone()).unwrap().clone();
                thing_hash.insert(next_idx, copies + 1);
            } else {
                thing_hash.insert(next_idx, 1);
            }
        }

        let copies = thing_hash.get(&idx.clone()).unwrap().clone();
        if copies > 1 {
            for _ in 1..copies {
                for win_idx in 0..our_winners.len() {
                    let next_idx = idx + win_idx.clone() + 1;
                    if thing_hash.contains_key(&next_idx.clone()) {
                        let copies = thing_hash.get(&next_idx.clone()).unwrap().clone();
                        thing_hash.insert(next_idx, copies + 1);
                    } else {
                        thing_hash.insert(next_idx, 1);
                    }
                }
            }
        }

        // println!("{:?}", thing_hash);
    }

    let answer: usize = thing_hash.values().sum();

    Logger::numeric_answer(answer as u64);
}
