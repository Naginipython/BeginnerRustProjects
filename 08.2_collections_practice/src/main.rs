use std::io;
use crate::med_mode::med_mode;
use crate::piglatin::piglatin;
use crate::company_list::company_list;

pub mod med_mode;
pub mod piglatin;
pub mod company_list;
pub mod continue_check;

fn main() {
    'program_loop: loop {
        println!("In this example, this program will do one of 3 things:\n
        1: Generate a list of integers, and return the median and mode\n
        2: Convert a sentance to pig latin\n
        3: Add employees to a list, sorted\n
        4: Press 4 to exit program");
        loop {
            println!("To proceed, please choose a number corresponding to the above: ");
            let mut choice = String::new();
            
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice: u32 = choice.trim().parse().expect("Please choose a number.");

            if choice < 5 && choice > 0 {
                match choice {
                    1 => med_mode(),
                    2 => piglatin(),
                    3 => company_list(),
                    4 => break 'program_loop,
                    _ => println!("Error: Somehow proceeded with choice below 0, and above 4"),
                }
                break;
            }
        }
    }
}
