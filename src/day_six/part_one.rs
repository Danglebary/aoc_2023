use crate::utils::{file, log::Logger};

#[allow(dead_code)]
pub fn run() {
    let input = file::read_file_lines("src/day_six/input.txt").unwrap();
    // let input = file::read_file_lines("src/day_six/sample.txt").unwrap();

    let time_line = input.iter().nth(0).unwrap();
    let mut times = time_line.split_ascii_whitespace().collect::<Vec<&str>>();
    times.retain(|x| x != &" " && !x.contains("Time"));

    let dist_line = input.iter().nth(1).unwrap();
    let mut dists = dist_line.split_ascii_whitespace().collect::<Vec<&str>>();
    dists.retain(|x| x != &" " && !x.contains("Distance"));

    let mut scores: Vec<u64> = Vec::new();

    for (idx, time) in times.iter().enumerate() {
        println!("time: {}", *time);
        let time = time.parse::<u64>().unwrap();
        let dist = dists[idx].parse::<u64>().unwrap();

        let mut our_scores: Vec<u64> = Vec::new();

        for i in 1..time + 1 {
            let speed = i;
            let time_left = time - i;
            let our_dist = speed * time_left;
            if our_dist > dist {
                our_scores.push(dist);
            }
        }

        scores.push(our_scores.len() as u64);
    }

    // multiply each score together for the answer
    let answer = scores.iter().fold(1, |acc, x| acc * x);
    Logger::numeric_answer(answer);
}
