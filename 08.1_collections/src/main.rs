fn main() {
    let mut v: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];

    v.push(1);
    v.push(2);

    //getting vectors
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third vector is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third vector is {third}"),
        None => println!("There is no third vector"),
    }

    let sixth = v.get(5);
    match sixth {
        Some(x) => println!("The sixth vector is {x}"),
        None => println!("There is no sixth vector"),
    }

    //I cannot "get" then add to a mutable vector, and use the "get" due to pointers. I can, however, use mutable vectors like this:
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 1;
    }
    println!("{:?}", v);

    //Vectors can only use the same type within them. However, we can use enum to have more
    #[derive(Debug)]
    enum Spreadsheet {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Spreadsheet::Int(3),
        Spreadsheet::Text(String::from("blue")),
        Spreadsheet::Float(10.12),
    ];

    println!("{:?}", row);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    //notes: have to re-initalize s1 since its used, and the following code is a better way to use "+"
    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}");
    //note: fomat doesn't take ownership
    println!("{s}\n{s1}"); 

    //indexing strings
    let s = String::from("Hello world!");
    for c in s.chars() {
        print!("{c} ");
    }
    print!("\n");

    // HASHMAPS
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //"get" returns Option<&V>, copied returns Option<i32>, and "unwrap_or" returns the i32 or 0 (if there's no entry)

    println!("Blue team's score is {score}");

    //looping through entries
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    //overriding value
    scores.insert(use_team_name(&team_name), 25);
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Blue team's score is now {score}");

    //unknown if override or insert
    scores.entry(String::from("Yellow")).or_insert(55); //Doesn't override
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn use_team_name(s: &String) -> String {
    //I created this function because I cannot use &team_name in a HashMap insert
    s.clone()
}