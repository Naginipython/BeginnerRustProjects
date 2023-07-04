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

    let tup: (String, i32) = ("Test", 1234);
}
