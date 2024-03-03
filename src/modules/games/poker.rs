use crate::modules::utils::pause;

#[allow(unused)]
pub fn start() {
    println!("The game is yet to be developed. Press any key to go back");
    pause();
    crate::modules::games::main();
}