use colored::Colorize;

use super::ascii_art;

type Color = colored::Color;

pub struct Logger {
    message: String,
}

#[allow(dead_code)]
impl Logger {
    pub fn new(message: &str) -> Logger {
        Logger {
            message: message.to_string(),
        }
    }

    pub fn banner() {
        ascii_art::banner();
    }

    pub fn divider() {
        ascii_art::divider();
    }

    pub fn day(day: &str, part: &str) {
        ascii_art::day_and_part(day, part)
    }

    pub fn numeric_answer(answer: usize) {
        ascii_art::numeric_answer(answer);
    }

    pub fn clear_screen() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    pub fn color(&mut self, color: Color) -> &mut Logger {
        self.message = self.message.color(color).to_string();
        self
    }

    pub fn bold(&mut self) -> &Logger {
        self.message = self.message.bold().to_string();
        self
    }

    pub fn underline(&mut self) -> &Logger {
        self.message = self.message.underline().to_string();
        self
    }

    pub fn print(&self) {
        println!("{}", self.message);
    }
}
