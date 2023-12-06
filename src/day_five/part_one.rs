use crate::utils::file;
use crate::utils::log::Logger;

pub fn run() {
    let input = file::read("./src/day_five/input.txt").unwrap();

    let maps: Vec<&str> = input.split("\n\n").collect();
    let seeds_parts = maps[0].split(':').collect::<Vec<&str>>();
    let mut seeds = seeds_parts[1]
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let stages = maps[1..]
        .iter()
        .map(|stage| {
            let mut stage_rows: Vec<Vec<u64>> = Vec::new();

            let stage_parts = stage.split('\n').collect::<Vec<&str>>();

            for part in stage_parts {
                let pattern = regex::Regex::new(r"[0-9]").unwrap();
                if !pattern.is_match(part) {
                    continue;
                }

                let num_strs = part.split_whitespace().collect::<Vec<&str>>();

                let mut nums = num_strs.iter().map(|num| num.parse::<u64>().unwrap());

                let dest = nums.next().unwrap();
                let start = nums.next().unwrap();
                let length = nums.next().unwrap();

                stage_rows.push(vec![dest.clone(), start.clone(), (start + length)]);
            }

            stage_rows
        })
        .collect::<Vec<Vec<Vec<u64>>>>();

    for stage in stages {
        for seed in &mut seeds {
            let stage_parts = stage.iter();

            for stage_part in stage_parts {
                let mut part_parts = stage_part.iter();
                let dest = part_parts.next().unwrap();
                let start = part_parts.next().unwrap();
                let mut end = *part_parts.next().unwrap();

                if start <= seed && seed.lt(&&mut end) {
                    println!("{} = {} - {}", *seed, seed, end);
                    *seed = seed.clone() - start + dest;
                    break;
                }
            }
        }
    }

    let answer = *seeds.iter().min().unwrap();

    Logger::numeric_answer(answer);
}
