// use aggregator::{Summary, Tweet};

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn show(&self) -> String {
        format!("{}", self.x.to_string() + ", " + &self.y.to_string())
    }
}

fn main() {
    let num_list = vec![34, 50, 25, 100, 65];
    let result = largest(&num_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest number is {result}");

    let integer = Point {x: 5, y:10};
    let float = Point {x: 1.0, y:4.0};
    //Note: both x and y must be the same type, since they're both T. Otherwise, make it <T, U>
    println!("Integer points to: {}, {}", integer.x(), integer.y);
    println!("Float points to: {}", float.show());

    // let tweet = Tweet {
    //     username: String::from("horse_ebooks"),
    //     content: String::from(
    //         "of course, as you probably already know, people",
    //     ),
    //     reply: false,
    //     retweet: false,
    // };

    // println!("1 new tweet: {}", tweet.summarize());
}
