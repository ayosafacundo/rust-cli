use crate::modules::utils::get_time;
use crate::modules::utils::pause;
use crate::modules::utils::select_from_options;

pub fn start() {
    welcome();
    pause();
    game();
}

fn welcome() {
    println!("Welcome! This is a plain Tic Tac Toe game, it's here just to make this project seem like it has more games and stuff than it really has.");
    println!("BUT! the game is working, although the AI won't understand the game completely.");
    println!("So, the game is simple, 3 by 3 grid, the AI and you will take turns occupying places in that grid, the first one to occupy a line of places wins.");
    println!("(so any entire column or row or a diagonal.)");
    println!("If you're ready press enter.");
}

fn game() {
    let mut grid: [[&str; 3]; 3] = [
        ["-", "-", "-"],
        ["-", "-", "-"],
        ["-", "-", "-"],
    ];
    let mut selected: usize; 
    let options = ["1A".to_owned(), "2A".to_owned(), "3A".to_owned(),"1B".to_owned(), "2B".to_owned(), "3B".to_owned(),"1C".to_owned(), "2C".to_owned(), "3C".to_owned(),];
    while check_grid(&grid) {
        print_grid(&grid);
        selected = select_from_options(&options);
        match selected {
            1 => choose(&mut grid, 0, 0),
            2 => choose(&mut grid, 0, 1),
            3 => choose(&mut grid, 0, 2),
            4 => choose(&mut grid, 1, 0),
            5 => choose(&mut grid, 1, 1),
            6 => choose(&mut grid, 1, 2),
            7 => choose(&mut grid, 2, 0),
            8 => choose(&mut grid, 2, 1),
            9 => choose(&mut grid, 2, 2),
            _ => panic!("Unexpected result"),
        };
        pc(&mut grid);
    }
}

fn pc(grid: &mut [[&str; 3]; 3]) {
    
}

fn choose(grid: &mut [[&str; 3]; 3], row: usize, col: usize) {
    grid[row][col] = "X";
}

fn print_grid(grid: &[[&str; 3]; 3]) {
    println!("   | A | B | C |");
    for i in 1..7 {
        if i % 2 == 1 {
            println!("----------------");
        } else {
            print!(" {} |", i/2);
            print!(" {} | {} | {} |", grid[i-1][0], grid[i-1][1], grid[i-1][2]);
        }
    }
}

fn check_grid(grid: &[[&str; 3]; 3]) -> bool {
    for i in 0..3 {
        if (grid[i][0] == "X" && grid[i][1] == "X" && grid[i][2] == "X") || (grid[i][0] == "O" && grid[i][1] == "O" && grid[i][2] == "O") {
            return false
        }
        if (grid[0][i] == "X" && grid[1][i] == "X" && grid[2][i] == "X") || (grid[0][i] == "O" && grid[1][i] == "O" && grid[2][i] == "O") {
            return false
        }
    }
    if (grid[0][0] == "X" && grid[1][1] == "X" && grid[2][2] == "X") || (grid[0][0] == "O" && grid[1][1] == "O" && grid[2][2] == "O") {
        return false
    }
    if (grid[2][0] == "X" && grid[1][1] == "X" && grid[0][2] == "X") || (grid[2][0] == "O" && grid[1][1] == "O" && grid[0][2] == "O") {
        return false
    }
    false
}