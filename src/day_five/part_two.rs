use crate::utils::file;
use crate::utils::log::Logger;

#[allow(dead_code)]
pub fn run() {
    let input = file::read("src/day_five/input.txt").unwrap();

    let maps: Vec<&str> = input.split("\n\n").collect();
    let seeds_parts = maps[0].split(':').collect::<Vec<&str>>();
    let seeds = seeds_parts[1]
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

    let mut current: &mut Vec<(u64, u64)> = &mut Vec::new();
    let mut next: &mut Vec<(u64, u64)> = &mut Vec::new();

    for (start, length) in seeds
        .iter()
        .zip(seeds.iter().skip(2))
        .map(|(a, b)| (a, a + b - 1))
    {
        current.push((*start, length));
    }

    for stage in stages {
        'outer: for (s1, e1) in current.iter().copied() {
            for stage_parts in stage.clone() {
                let mut parts = stage_parts.iter();
                let dest = parts.next().unwrap();
                let s2 = parts.next().unwrap();
                let length = parts.next().unwrap();
                let e2 = s2 + length - 1;

                if *s2 <= s1 && e1 <= e2 {
                    next.push((s1 - s2 + dest, e1 - s2 + dest));
                    continue 'outer;
                } else if s1 < *s2 && s2 <= &e1 && e1 <= *&e2 {
                    next.push((s1, s2 - 1));
                    next.push((*dest, e1 - s2 + dest));
                    continue 'outer;
                } else if *s2 <= s1 && s1 <= e2 && e2 < e1 {
                    next.push((s1 - s2 + dest, e2 - s2 + dest));
                    next.push((e2 + 1, e1));
                    continue 'outer;
                } else if s1 < *s2 && e2 < e1 {
                    next.push((s1, s2 - 1));
                    next.push((*dest, dest + length - 1));
                    next.push((e2 + 1, e1));
                    continue 'outer;
                }
            }
            next.push((s1, e1));
        }

        (current, next) = (next, current);
        next.clear();
    }

    let answer = current.iter().map(|r| r.0).min().unwrap();

    Logger::numeric_answer(answer);
}
