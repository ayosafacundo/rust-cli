use crate::modules::utils::{pause, print_banner};

pub fn main() {
    print_banner(crate::modules::utils::Banners::Other);
    println!("Welcome, Here's where the \"Other\" section will be, but there's nothing yet.");
    pause();
}