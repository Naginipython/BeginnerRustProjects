use std::io;

pub fn continue_check() -> bool {
    println!("would you like to go again? (T or F)");
    loop {
        let mut cont = String::new();

        io::stdin()
            .read_line(&mut cont)
            .expect("Failed to read line");

        let mut cont: char = match cont.trim().parse() {
            Ok(c) => c,
            Err(_) => '\0',
        };
        
        if cont.is_lowercase() {
            //possibly a bug, so one online user claims
            cont = cont.to_uppercase().next().unwrap();
        }

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