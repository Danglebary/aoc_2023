use crate::utils::{file, log::Logger};

#[derive(Debug, Clone)]
struct Set {
    color: String,
    count: usize,
}

#[derive(Debug, Clone)]
struct Game {
    id: usize,
}

#[allow(dead_code)]
pub fn run() {
    let content = file::read_file_lines("src/day_two/input.txt").unwrap();

    let mut games: Vec<Game> = Vec::new();

    // line structure: Game <id>: <Set>;

    for game in content {
        let game_parts = game.split(":").collect::<Vec<&str>>();
        let game_id = game_parts[0].split(" ").collect::<Vec<&str>>()[1];
        let raw_sets = game_parts[1]
            .split(";")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut sets: Vec<Set> = Vec::new();

        for raw_set in raw_sets {
            let set_parts = raw_set.split(",").map(|x| x.trim()).collect::<Vec<&str>>();

            for set_part in set_parts {
                let set_part_parts = set_part.split_whitespace().collect::<Vec<&str>>();
                let set_count = set_part_parts[0].parse::<usize>().unwrap();
                let set_color = set_part_parts[1];

                sets.push(Set {
                    color: set_color.to_string(),
                    count: set_count,
                });
            }
        }

        let mut possible: bool = true;

        for set in sets.clone() {
            match set.color.as_str() {
                "red" => {
                    if set.count > 12 {
                        possible = false;
                    }
                }
                "green" => {
                    if set.count > 13 {
                        possible = false;
                    }
                }
                "blue" => {
                    if set.count > 14 {
                        possible = false;
                    }
                }
                _ => panic!("Unknown color {}", set.color),
            }
        }

        if possible {
            games.push(Game {
                id: game_id.parse::<usize>().unwrap(),
            });
        }
    }

    let result = games.iter().map(|x| x.id).collect::<Vec<usize>>();
    let answer: usize = result.iter().sum();

    Logger::numeric_answer(answer as u64);
}
