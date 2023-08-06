use std::io;

pub fn company_list() {
    //3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to 
    //  Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted 
    //  alphabetically.
    println!("Welcome to your employee organizer!
To use this application, please type in one of the following commands:
    view (employees/[deparment])
    add [name] to [department]
    add department [department name]
    remove [name] from [department]
    remove department [department]
    show commands
    exit app");
    
    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Error: Couldn't read line");

        let mut split_command = Vec::new();

        for word in command.split_whitespace() {
            split_command.push(word);
        }

        match split_command.get(0) {
            Some(s) => println!("{s}"),
            _ => println!("failed"),
        }
    }
}