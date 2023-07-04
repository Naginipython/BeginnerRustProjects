fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = 2 * x;
        println!("The value of x on the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    const AMOUNT_OF_SECONDS_IN_THREE_HOURS: u32 = 60 * 60 * 3;

    let spaces = "   ";
    let spaces = spaces.len();

    println!("Amount of seconds in 3 hours constant: {AMOUNT_OF_SECONDS_IN_THREE_HOURS}");
    println!("How long is the spaces variable: {spaces}");

    let tup: (&str, i32) = ("Test", 1234);

    let float: f64 = 6.24;

    println!("Tuple in action: {}, {}. Float in action: {float}", tup.0, tup.1);

    let _arr = [3; 5]; //array of five 3's
    let _barr: [i32; 3] = [1, 2, 3];
    
    let _y = {
        let x = 3;
        x + 1 //NO SEMICOLON: makes it a statement, and is not returned
    };

    let x = plus_one(5);
    println!("The inline function number: {x}");
}

//Rust prefers snake case, aka lower case with indents
fn plus_one(x: i32) -> i32 {
    x + 1 //I CAN return, but if I don't use a semicolon, its not necessary
}
