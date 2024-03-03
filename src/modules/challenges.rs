use crate::modules::utils::pause;

use super::utils::print_banner;

#[allow(dead_code)]

pub fn main() {
    print_banner(super::utils::Banners::Challenges);
    println!("Hello to the \"Challenges\" tab, where I will add a lot of challenges like a visual 3D representation of a donut or visual sorting or idk, I've yet to decide.");
    println!("There's nothing yet, press any key to go back.");
    pause();
}