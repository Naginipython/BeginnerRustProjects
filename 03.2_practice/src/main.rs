use std::io;
use std::{thread, time};

fn main() {
    'program_loop: loop {
        println!("In this example, this program will do 1 of three things:\n
        1: Convert temperatures between Fahrenheit and Celsius.\n
        2: Generate the nth Fibonacci number.\n
        3: Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.\n
        4: Exit program.");

        'chooing_example_loop: loop {
            println!("To proceed, please choose a number corresponding to the above: ");

            let mut choice = String::new();

            io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read line");

            let choice: u32 = choice.trim().parse().expect("Please choose a number.");

            if choice < 5 && choice > 0 {
                match choice {
                    1 => temp(),
                    2 => fib(),
                    3 => carol(),
                    4 => break 'program_loop,
                    _ => println!("Error: Somehow proceeded with choice below 0, and above 4"),
                }
                break 'chooing_example_loop;
            }
        }
    }
}

fn temp() {
    let mut continue_temp = true;
    while continue_temp {
        println!("Would you like to convert to Celsius or Fahrenheit? (C or F)");

        loop {
            //Getting the temp type
            let mut temp_type = String::new();

            io::stdin()
            .read_line(&mut temp_type)
            .expect("Failed to read line");

            let temp_type: char = match temp_type.trim().parse() {
                Ok(num) => num,
                Err(_) => '\0',
            };

            if temp_type != 'F' && temp_type != 'C' {
                println!("Please choose C or F");
            } else {
                //Getting the temp number
                println!("Please choose a temperature: ");
                loop {

                    let mut temp = String::new();

                    io::stdin()
                    .read_line(&mut temp)
                    .expect("Failed to read line");

                    let temp: f64 = match temp.trim().parse() {
                        Ok(num) => num,
                        Err(_) => f64::INFINITY,
                    };

                    if temp == f64::INFINITY {
                        println!("Please choose a number.");
                    } else {
                        //Doing the math
                        let answer: f64;

                        match temp_type {
                            'F' => {
                                answer = (temp * (9.0/5.0)) + 32.0;
                                println!("{temp} from Celsius to Fahrenheit is {answer}");
                            },
                            'C' => {
                                answer = ((temp-32.0)*5.0)/9.0;
                                println!("{temp} from Fahrenheit to Celsius is {answer}");
                            }
                            _ => println!("Error: Somehow got here without proper F or C check.")
                        }
                        break;
                    }
                }
                break;
            }
        }
        continue_temp = continue_check();
    }
}

fn fib() {
    let mut continue_fib = true;
    while continue_fib {
        loop {
            println!("Give a number 'n' for the Fibonacci number: ");
            let mut n = String::new();

            io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

            let n: u32 = match n.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let fib = fib_helper(n);
            println!("Fibonacci number for {n} is: {fib}");
            break;
        }
        continue_fib = continue_check();
    }
}

fn fib_helper(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib_helper(n-1) + fib_helper(n-2)
}

fn carol() {
    let delay = time::Duration::from_millis(1000);
    let days = ["First", "Second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = [
        "partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    //Ideally, would like to add a stop button.

    for day in days {
        println!("\nOn the {} day of Christmas", day);
        thread::sleep(delay);
        println!("My true love gave to me");
        thread::sleep(delay);

        let mut index = days.iter().position(|&x| x == day).unwrap() + 1;
        while index > 0 {
            //Notable first case difference
            let mut start_case = "And a";
            if day.eq("First") {
                start_case = "A"
            }

            if index == 1 {
                //Also first case difference
                println!("{} {}", start_case, gifts[index-1]);
                thread::sleep(delay);
            } else {
                println!("{}", gifts[index-1]);
                thread::sleep(delay);
            }
            index = index - 1;
        }
    }
}

fn continue_check() -> bool {
    //Checking if we will continue
    println!("Would you like to go again? (T or F)");
    loop {
        //Getting the temp type
        let mut cont = String::new();

        io::stdin()
        .read_line(&mut cont)
        .expect("Failed to read line");

        let cont: char = match cont.trim().parse() {
            Ok(num) => num,
            Err(_) => '\0',
        };
        if cont != 'T' && cont != 'F' {
            println!("Please choose T or F");
        } else {
            if cont == 'F' {
                return false;
            }
            break;
        }
    }
    return true;
}