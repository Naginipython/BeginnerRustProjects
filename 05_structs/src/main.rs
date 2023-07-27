struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user = build_user(String::from("naginipython"), String::from("naginipython@email_address.com"));

    user.email = String::from("not_naginipython@email_address.com");

    println!("user: {} \nemail: {} \nactive: {} \nsign in count: {}", user.username, user.email, user.active, user.sign_in_count);

    let user2 = User {
        email: String::from("different@email_address.com"),
        ..user //this copies the remaining stuff from user, into user2
    };

    println!("user: {} \nemail: {} \nactive: {} \nsign in count: {}", user2.username, user2.email, user2.active, user2.sign_in_count);

    let _black = Color(0, 0, 0);

    let rec1 = Rectangle {
        width: 32,
        height: 60,
    };

    println!("The area of the rectangle is: {}", area(&rec1));

    println!("rect1 is {:#?}", rec1); //The '#' is optional, for styling

    println!("The area of the rectangle is: {}", rec1.area());

    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rec1.can_hold(&rec2));

    let sqr = Rectangle::square(30);

    println!("The area of the square is: {}", sqr.area());
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, //convienient feature, since they're named the same
        email,
        sign_in_count: 1,
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}