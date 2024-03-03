
// Crates
use core::fmt;
use crate::modules::utils::{print_banner, select_from_options};
// Games modules.
mod poker;
mod prison;

// Game struct
struct Game {
    name: &'static str,
    status: Status 
}
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

const GAMES: [Game; 2] = [
    Game {name: "Poker", status: Status::ToDo},
    Game {name: "Prison Dillema", status: Status::Finished},
];

pub fn main() {
    loop {
        print!("{}[2J", 27 as char);
        let gamenames = ["Poker".to_owned(), "Prison".to_owned(), "Back".to_owned()];
        print_banner(crate::modules::utils::Banners::Games);
        println!("Welcome! Here are the available games:");
        for Game {name, status} in GAMES {
            println!("- {} ({})", name, status);
        }
        println!("- Go Back");
        let selected = select_from_options(&gamenames);
        match selected {
            0 => poker::start(),
            1 => prison::start(),
            2 => break,
            _ => panic!(),
        }
    }
}