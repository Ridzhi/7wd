use std::error::Error;
use engine::{Age, Deck, Prepare};
// trait Deck {
//     fn pull(&mut self, id: u8) -> Result<(), Box<dyn Error>>;
// }
//
// struct Node {
//     open: bool,
//     token1: Option<String>,
//     token2: Option<String>,
//     token3: Option<String>,
// }

fn main() {
    // let text = "\
    //   foo\r\nbar\n\nbaz\r\
    // ";
    // let mut lines = text.lines();
    // println!("_{}", lines.next().unwrap());
    let buildings = Prepare::get_random_buildings();

    // let d = Deck::new(Deck::get_layout(Age::I), buildings[&Age::I].clone());
    println!("deck package");
}