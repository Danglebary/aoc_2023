use crate::utils::{file, log::Logger};
use std::collections::HashSet;

#[allow(dead_code)]
pub fn run() {
    let grid = file::read_file_lines("src/day_three/input.txt").unwrap();
    let mut cords: HashSet<(usize, usize)> = HashSet::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, char) in row.chars().enumerate() {
            if char.is_ascii_digit() || char == '.' {
                continue;
            }

            for current_row in [row_idx - 1, row_idx, row_idx + 1] {
                for mut current_col in [col_idx - 1, col_idx, col_idx + 1] {
                    let col_chars = grid[current_row].chars().collect::<Vec<char>>();
                    let current_char = col_chars[current_col];

                    if current_row >= grid.len()
                        || current_col >= grid[current_row].len()
                        || !current_char.is_ascii_digit()
                    {
                        continue;
                    }

                    while current_col > 0 && col_chars[current_col - 1].is_ascii_digit() {
                        current_col -= 1;
                    }

                    cords.insert((current_row, current_col));
                }
            }
        }
    }

    let mut nums: Vec<usize> = Vec::new();

    for (row, mut col) in cords {
        let mut str = String::new();
        while col < grid[row].len() {
            let current_char = grid[row].chars().collect::<Vec<char>>()[col];

            if !current_char.is_ascii_digit() {
                break;
            }
            str.push(current_char);
            col += 1;
        }
        nums.push(str.parse::<usize>().unwrap());
    }

    let answer = nums.iter().sum::<usize>();

    Logger::numeric_answer(answer as u64);
}
