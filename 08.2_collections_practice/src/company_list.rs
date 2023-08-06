use std::io;
use std::collections::HashMap;

enum Commands {
    Exit,
    Remove,
    Add,
    View,
}

pub fn company_list() {
    //3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to 
    //  Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted 
    //  alphabetically.
    println!("Welcome to your employee organizer!
To use this application, please type in one of the following commands:
    view (commands/employees/departments/[deparment])
    add [name] to [department]
    add department [department name]
    remove [name] from [department]
    remove department [department]
    exit app");

    let mut departments: Vec<String> = Vec::new();
    let mut employees: HashMap<String, String> = HashMap::new();
    
    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Error: Couldn't read line");

        let mut split_command = Vec::new();

        for word in command.split_whitespace() {
            split_command.push(word);
        }


        //Finds inital main command, places inside an enum
        let mut c = Commands::Exit;
        match split_command.get(0) {
            Some(com) => {
                match *com {
                    "exit" => c = Commands::Exit,
                    "remove" => c = Commands::Remove,
                    "add" => c = Commands::Add,
                    "view" => c = Commands::View,
                    _ => println!("Error"),

                }
            },
            _ => println!("Error: Couldn't find string"),
        }

        //Checks second part of command, doing different actions depending on inital command
        match split_command.get(1) {
            Some(com) => {
                match c {
                    Commands::Exit => {
                        if *com == "app" {
                            println!("Exiting app");
                            break;
                        } else {
                            println!("Error: Couldn't understand command");
                        }
                    },
                    Commands::Remove => {
                        match *com {
                            "department" => {
                                println!("remove department not implemented");
                            },
                            s => {
                                println!("remove names not implemented. name: {s}");
                            }
                        }
                    },
                    Commands::Add => {
                        match *com {
                            "department" => {
                                if !split_command.get(2).is_none() {
                                    let dep = split_command.get(2).unwrap();
                                    if !departments.contains(&dep.to_string()) {
                                        departments.push(dep.to_string());
                                        println!("Department \"{dep}\" was added");
                                    } else {
                                        println!("Error: department already exists");
                                    }
                                    
                                } else {
                                    println!("Error: None or unknown department");
                                }
                            },
                            s => {
                                if !employees.contains_key(&s.to_string()) {
                                    if !split_command.get(2).is_none() {
                                        if split_command.get(2).unwrap() == &"to" {
                                            //Employee adding
                                            if !split_command.get(3).is_none() {
                                                let dep = split_command.get(3).unwrap();
                                                //check if dep exists
                                                if departments.contains(&dep.to_string()) {
                                                    employees.insert(s.to_string(), dep.to_string());
                                                    println!("Employee {s} was added to \"{dep}\"");
                                                } else {
                                                    println!("Error: department \"{dep}\" does not exist");
                                                }
                                            } else {
                                                println!("Error: None or unknown employee");
                                            }
                                        }
                                    } else {
                                        println!("Error: Command not typed correctly");
                                    }
                                } else {
                                    println!("Error: {s} already has department");
                                }
                            }
                        }
                    },
                    Commands::View => {
                        match *com {
                            "employees" => {
                                //TODO: SORT
                                println!("Employees: {:?}", employees);
                            },
                            "commands" => {
                                println!("    view (commands/employees/[deparment])
    add [name] to [department]
    add department [department name]
    remove [name] from [department]
    remove department [department]
    exit app");
                            },
                            "departments" => {
                                println!("Departments: {:?}", departments);
                            },
                            s => {
                                //check for if department exists, then show
                                println!("view departments not implemented. department: {s}");
                            },
                        }
                    },
                }
            },
            _ => println!("Error: Couldn't find string"),
        }
    }
}