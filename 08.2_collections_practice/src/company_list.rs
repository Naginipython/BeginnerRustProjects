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
    println!("Welcome to your employee organizer!\n\
        To use this application, please type in one of the following commands:\n\
        \tview (commands/employees/departments/[deparment])\n\
        \tadd [name] to [department]\n\
        \tadd department [department name]\n\
        \tremove [name] from [department]\n\
        \tremove department [department]\n\
        \texit app");

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
                                if !split_command.get(2).is_none() {
                                    let dep = split_command.get(2).unwrap();
                                    if departments.contains(&dep.to_string()) {
                                        let index = departments.iter().position(|x| *x == dep.to_string()).unwrap();
                                        departments.remove(index);
                                        println!("Department \"{dep}\" was removed");
                                    } else {
                                        println!("Error: department doesn't exists");
                                    }
                                } else {
                                    println!("Error: None or unknown department");
                                }
                            },
                            s => {
                                if employees.contains_key(&s.to_string()) {
                                    if !split_command.get(2).is_none() {
                                        if split_command.get(2).unwrap() == &"from" {
                                            //employee removing
                                            if !split_command.get(3).is_none() {
                                                let dep = split_command.get(3).unwrap();
                                                //check if dep exists
                                                if departments.contains(&dep.to_string()) {
                                                    employees.remove(&s.to_string());
                                                    println!("Employee {s} was removed");
                                                } else {
                                                    println!("Error: department \"{dep}\" does not exist");
                                                }
                                            } else {
                                                println!("Error: None or unknown employee");
                                            }
                                        } else {
                                            println!("Error: Command not typed correctly");
                                        }
                                    } else {
                                        println!("Error: Command not typed correctly");
                                    }
                                } else {
                                    println!("Error: employee doesn't exists");
                                }
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
                                        } else {
                                            println!("Error: Command not typed correctly");
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
                                //Sort
                                let mut emp = Vec::from_iter(employees.keys());
                                emp.sort();

                                //Set up display
                                let mut output = String::new();
                                let mut first = 0;
                                output = format!("{}", output + "[");
                                for s in emp {
                                    if first == 0 {
                                        output = format!("{}", output + s + ": " + employees.get(s).unwrap());
                                        first += 1;
                                    } else {
                                        output = format!("{}", output + ", " + s + ": " + employees.get(s).unwrap());
                                    }
                                }
                                output = format!("{}", output + "]");

                                //display
                                println!("Employees: {:?}", output);
                            },
                            "commands" => {
                                println!("\tview (commands/employees/[deparment])\n\
                                    \tadd [name] to [department]\n\
                                    \tadd department [department name]\n\
                                    \tremove [name] from [department]\n\
                                    \tremove department [department]\n\
                                    \texit app");
                            },
                            "departments" => {
                                departments.sort();
                                println!("Departments: {:?}", departments);
                            },
                            s => {
                                //check for if department exists
                                if departments.contains(&s.to_string()) {
                                    //get employees
                                    let mut e = Vec::new();
                                    for (key, val) in &employees {
                                        if val == s {
                                            e.push(key);
                                        }
                                    }

                                    //display
                                    println!("Employees in {s}: {:?}", e);
                                }
                            },
                        }
                    },
                }
            },
            _ => println!("Error: Couldn't find string"),
        }
    }
}