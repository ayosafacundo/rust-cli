
// Crates
use core::fmt;
use crate::modules::utils::print_banner;
// Games modules.
mod poker;
mod prison;

enum Status {
    Finished,
    ToDo,
    WIP
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Finished => write!(f, "Finished"),
            Status::ToDo => write!(f, "To Do"),
            Status::WIP => write!(f, "Work In Progress"),
        }
    }
}

struct Game {
    name: &'static str,
    status: Status 
}

const GAMES: [Game; 2] = [
    Game {name: "Poker", status: Status::ToDo},
    Game {name: "Prison Dillema", status: Status::ToDo},
];

pub fn main() {
    print_banner(crate::modules::utils::Banners::Games);
    println!("Welcome! Here are the available games:");
    for Game {name, status} in GAMES {
        println!("- {} ({})", name, status);
    }
}