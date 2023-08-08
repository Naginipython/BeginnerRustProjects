use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    //This returns a Result, to be matched with Ok or Err
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            //                      also returns a Result
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };

    //Using "match" is considered primitive. In lesson 13, I will learn about closures. Here is the same code but in a closure
    let _greeting_file = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            //it looks like it just makes the Ok go through, and the chunk of code is error handling
            File::create("hello2.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {:?}", e);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //unwrap() will either finalize an Ok result, or call a panic!()
    let _greeting_file = File::open("hello3.txt").unwrap();

    //expect() is like unwrap, but will allow customization of panic!()
    let _greeting_file = File::open("hello4.txt").expect("hello4.txt should not be included in this project");
}

//A take on creating our own result. Result<T, E>
fn _read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    //The "?" allows for us to not need any error matching. After all, the code returns a Result, which can be used to find errors there
    File::open("hello5.txt")?.read_to_string(&mut username)?;

    Ok(username)
}