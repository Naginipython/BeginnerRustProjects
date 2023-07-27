#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let m = Message::Write(String::from("Hello"));

    m.print(); //method test
    display(m); //function test

    let m = Message::Move {x: 3, y: 5};

    m.print(); //method test
    display(m); //function test

    let m = Message::ChangeColor(30, 30, 30);

    m.print(); //method test
    display(m); //function test

    let m = Message::Quit;

    m.print(); //method test
    display(m); //function test

    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), //alternatively, if we didnt want a variable such as "other", we can do:
    //  _ => (), //This code catches all, and does nothing. Match must exhaust options.
    }

    let m = Message::Write(String::from("Test"));

    if let Message::Write(_string) = m { //Changed Write(String), for rust_analyzer reasons
        println!("The message is Write");
    }
    /* same as doing this:
    match m {
        Message::Write(String) => println!("The message is Write"),
        _ => (),
    }
    */
}

fn display(m: Message) {
    match m { //If used in a method, change "Message" to "Self"
        Message::Quit => println!("App has quit"),
        Message::Move { x, y } => {
            println!("App has moved to ({x}, {y})");
        },
        Message::Write(s) => println!("App has written {s}"),
        Message::ChangeColor(r, g, b) => println!("App's color is now [{r}, {g}, {b}]"),
    }
}

fn add_fancy_hat() { println!("added hat"); }
fn remove_fancy_hat() { println!("removed hat"); }
fn move_player(_pos: u8) { println!("moved player"); }