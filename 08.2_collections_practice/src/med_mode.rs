use std::io;
use rand::Rng;

use crate::continue_check::continue_check;

pub fn med_mode() {
    //1. Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; 
    //  a hash map will be helpful here) of the list.

    //idea: allow user to generate with rand, given amount and a range. research vector sorting, median should be simple, idk mode yet. HashMap then find largest?
    let mut continue_med = true;
    while continue_med {
        println!("Enter a size for the integer list: ");
        let size = user_int_input(false);
        if size == -1 {
            println!("Error: size invalid.");
            //TODO: handle
        }

        //Getting data
        println!("Enter a low range for the integer list: ");
        let low_range = user_int_input(true);

        println!("Enter a high range for the integer list: ");
        let high_range = user_int_input(true);
        
        //TODO: handle high < low

        //Generating list
        let mut list: Vec<i32> = Vec::new();

        for _i in 0..size {
            list.push(rand::thread_rng().gen_range(low_range..=high_range))
        }

        //Sorting list
        list.sort();

        //getting median

        //getting mode

        println!("{:?}", list);

        continue_med = continue_check();
    }
}

fn user_int_input(can_be_negative: bool) -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input a valid sentance.");

    let input = input.trim().parse().expect("Please choose a valid number.");

    if !can_be_negative && input <= 0 {
        -1
    } else {
        input
    }
}