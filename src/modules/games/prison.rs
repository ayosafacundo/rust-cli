use crate::modules::utils::get_time;
use crate::modules::utils::pause;
use crate::modules::utils::select_from_options;

// Structs
struct Rounds {
    amount: u16,
    name: &'static str,
    enemy: Enemy 
}
struct RoundHistory {
    player: bool,
    pc: bool,
}
enum Enemy {
    Begginer,
    Easy,
    Normal,
    Hard,
    Inferno,
    Ranked
}

// Available Rounds
const ROUNDS: [Rounds; 6] = [
    Rounds { amount: 10, name: "Begginer: 10 rounds.", enemy: Enemy::Begginer},
    Rounds { amount: 25, name: "Easy: 25 rounds.", enemy: Enemy::Easy},
    Rounds { amount: 50, name: "Normal: 50 rounds.", enemy: Enemy::Normal},
    Rounds { amount: 75, name: "Hard: 75 rounds.", enemy: Enemy::Hard},
    Rounds { amount: 100, name: "Inferno: 100 rounds.", enemy: Enemy::Inferno},
    Rounds { amount: 200, name: "Ranked: 200 rounds.", enemy: Enemy::Ranked},
];

// Welcome Print statements, TL DR.
fn welcome() {
    print!("{}[2J", 27 as char);
    println!("Hello and welcome to the \"Iterated Prison's Dilemma\" game!");
    println!("The objective of this game is easy, you have to accumulate the most number of points from a number of rounds");
    println!("In each round you'll fight the PC by either splitting or stealing the points, and the PC will do the same, split or steal.");
    println!("The points will be given following these rules:");
    println!("If both you and the PC splits, you both obtain 3 points.");
    println!("If you steal and the PC splits, only you get 5 points.");
    println!("If you split and the PC steals, only the pc gets 5 points.");
    println!("If both you and the PC steals, you both obtain 1 point.");
    println!("translated to a table, this is what you'll be getting:");
    println!("------|  Split |  Steal  |");
    println!("------|--------|---------|");
    println!("Split | 3 each |    5    |");
    println!("------|--------|---------|");
    println!("Steal |   5    | 1 each  |");
    println!("Take your time to understand the rules, when you're ready to play press enter.");
}

// Setup
pub fn start() {
    let types = [String::from("Begginer"), String::from("Easy"), String::from("Normal"), String::from("Hard"), String::from("Inferno"), String::from("Ranked"), String::from("Back")];
    welcome();
    pause();
    loop {

        print!("\x1B[2J");
        println!("Ok, let's begin by asking: How many rounds?");
        for Rounds { amount: _, name, enemy: _ } in ROUNDS {
            println!("- {}", name);
        }
        println!("- Back");
        let selected:usize = select_from_options(&types);
        if selected < ROUNDS.len() {
            game(&ROUNDS[selected]);
        } else {
            crate::modules::games::main();
        }
    }
}

// Game Setup and point recap.
fn game(round_type: &Rounds) {
    print!("{}[2J", 27 as char);
    let pc: fn(&Vec<RoundHistory>) -> bool = match round_type.enemy {
        Enemy::Begginer => next_begginner,
        Enemy::Easy => next_easy,
        Enemy::Normal => next_normal,
        Enemy::Hard => next_hard,
        Enemy::Inferno => next_inferno,
        Enemy::Ranked => next_ranked,
    };
    let (pc_points, player_points, history) = play(round_type.amount, pc);
    println!("Game finished! Let's see the scores:");
    println!("You scored {}", player_points);
    println!("The PC scored {}", pc_points);
    if player_points > pc_points {
        println!("You Win!");
    } else {
        println!("You Lose!");
    }
    println!("This is your history:");
    for i in 0..5 {
        match i {
            0 | 2 | 4 => print!("--------"),
            1 => print!("Player |"),
            3 => print!("PC     |"),
            _ => ()
        }
        for item in &history {
            let player = item.player;
            let pc = item.pc;
            if i == 0 || i == 2 || i == 4 {
                print!("----");
            } else if i == 1 {
                if player {
                    print!(" O |");
                } else {
                    print!(" X |");
                }
            } else if i == 3 {
                if pc {
                    print!(" O |");
                } else {
                    print!(" X |");
                }
            }
        }
        println!();
    }
}

// Round loop and game logic
fn play(amount:u16,pc: fn(&Vec<RoundHistory>) -> bool) -> (u32, u32, Vec<RoundHistory>) {
    let mut history: Vec<RoundHistory> = vec![];
    let mut player_points:u32 = 0;
    let mut pc_points:u32 = 0;
    let options: [String; 2] = ["Split".to_owned(), "Steal".to_owned()];
    let mut selected;
    let mut chosen:RoundHistory;
    for round in 1..amount {
        println!("Your points: {} | - | PC Points: {}", player_points, pc_points);
        println!("Round: {}", round);
        for opt in &options {
            print!("{}  ", opt);
        }
        println!("");
        selected = select_from_options(&options);
        match selected {
            0 => chosen = RoundHistory {player: true, pc: false}, // Split
            1 => chosen = RoundHistory {player: false, pc: false}, // Steal
            _ => panic!("Wrong selection"), // Error
        }
        if chosen.player {
            println!("You've chosen \"Split\"");
        } else {
            println!("You've chosen \"Steal\"");
        }
        chosen.pc = pc(&history);
        if chosen.pc {
            println!("PC chose \"Split\"");
        } else {
            println!("PC chose \"Steal\"");
        }
        if chosen.player && chosen.pc {
            player_points += 3;
            pc_points += 3;
        } else if !chosen.player && !chosen.pc {
            player_points += 1;
            pc_points += 1;
        } else if chosen.player && !chosen.pc {
            player_points += 0;
            pc_points += 5;
        } else if !chosen.player && chosen.pc {
            player_points += 5;
            pc_points += 0;
        }
        history.push(chosen);
    }
    return (pc_points, player_points, history);
}

/* Bots Logic */

fn next_begginner(history:&Vec<RoundHistory>) -> bool {
    if history.len() == 0 || history.len() % 2 == 0 {
        return true;
    }
    false
}

fn next_easy(history:&Vec<RoundHistory>) -> bool {
    if history.len() == 0 || history.len() % 15 == 0 {
        return false;
    }
    true
}

fn next_normal(history:&Vec<RoundHistory>) -> bool {
    let prev: Option<&RoundHistory> = history.get(history.len()-1);
    match prev{
        None => false,
        Some(a) => !a.pc, 
    }
}

fn next_hard(history:&Vec<RoundHistory>) -> bool {
    let prev: Option<&RoundHistory> = history.get(history.len()-1);
    match prev{
        None => true,
        Some(a) => a.pc, 
    }
}

fn next_inferno(history:&Vec<RoundHistory>) -> bool {
    let prev: Option<&RoundHistory> = history.get(history.len()-1);
    match prev{
        None => true,
        Some(a) => {
            let now = get_time();
            let last_digit = now % 10;
            match last_digit {
                0 | 1 | 3 | 4 | 5 => !a.pc,
                2 | 6 | 7 | 8 | 9 => a.pc,
                _ => {
                    println!("Unexpected unknown number found");
                    false
                }
            }
        }, 
    }
}

fn next_ranked(history:&Vec<RoundHistory>) -> bool {
    let now = get_time();
    let last_digit = now % 10;
    match last_digit {
        0 | 1 => next_begginner(history),
        2 | 3 => next_easy(history),
        4 | 5 => next_normal(history),
        6 | 7 => next_hard(history),
        8 | 9 => next_inferno(history),
        _ => {
            println!("Unexpected unknown number found");
            false
        }
    }
}