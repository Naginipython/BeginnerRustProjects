fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let t = s; //s has moved to t, and can't be used.

    let u = t.clone(); //u and t have the same value, different pointers

    //The value from t is moved to the parameter, and t cannot be used.
    takes_ownership(t);

    let u = returns_ownership(u);

    let (s, len) = calc_len(u);

    println!("String is: \"{s}\", length of it is: {len}");
}

fn takes_ownership(string: String) {
    println!("{}", string);
}

fn returns_ownership(string: String) -> String {
    println!("{}", string);
    string
}

fn calc_len(string: String) -> (String, usize) {
    let len = string.len();
    (string, len)
}