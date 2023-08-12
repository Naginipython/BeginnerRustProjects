use std::io;

use crate::continue_check::continue_check;

pub fn piglatin() {
    //2. Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words 
    //  that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    let mut continue_pig = true;
    while continue_pig {
        println!("Enter a sentance to be translated: ");
        let mut words = String::new();
        io::stdin()
            .read_line(&mut words)
            .expect("Input a valid sentance.");

        let mut output = String::new();
        
        for word in words.split_whitespace() {
            let first_char: &char = &word.chars().nth(0).unwrap();

            match first_char {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    output = format!("{}", output + word + "-hay ");
                },
                _ => {
                    let first_char: &str = &first_char.to_string();
                    let temp: &str = &word[1..word.len()];
                    output = format!("{}", output + temp + "-" + &first_char + "ay ");
                }
            }

        }
        println!("{}", output);

        continue_pig = continue_check();
    }
}