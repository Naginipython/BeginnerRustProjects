//Shortens the name we need for Asparagus
use crate::garden::vegetable::Asparagus;

//Looks for a src->garden->mod.rs OR src->garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
