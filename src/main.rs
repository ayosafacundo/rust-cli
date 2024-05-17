use crate::modules::utils::print_banner;

#[allow(dead_code)]
mod modules;

fn main() {
    let modules:[String; 4] = [String::from("Games"), String::from("Challenges"), String::from("Other"), String::from("Leave")];
    let mut selected:usize;
    loop {
        print!("\x1B[2J");
        print_banner(modules::utils::Banners::Logo);
        println!("This CLI tool is just a collection of games, challenges and things that I thought would challenge me while learning Rust. This is intended to be an all-in-one CLI tool.");
        println!("Please, select an option from the following:");
        for (i, option) in modules.iter().enumerate() {
            let j:usize = i + 1;
            println!("   {}: {}", j, option);
        }
        selected = modules::utils::select_from_options(&modules);
        match selected {
            0 => modules::games::main(), // Games
            1 => modules::challenges::main(), // Challenges
            2 => modules::other::main(), // Other
            3 => break,
            _ => panic!()
        }
    }
}