use std::thread;

fn main() {
    let closure = |x| x + 1;
    let n = closure(1);

    println!("{}", n);
    println!("{}", closure(5));

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    println!("Mutability:");

    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    println!("Before defining closure: {:?}", list);

    //makes it so list can be used, dont fully understand yet
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    //Iterators
    let v1 = vec![1, 2, 3];
    let vec_iter = v1.iter();

    for val in vec_iter {
        println!("Got: {}", val);
    }

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    //into_iter takes ownership of v1
    let v3: Vec<_> = v1.into_iter().filter(|x| x % 2 == 1).collect();
    assert_eq!(v3, vec![1, 3]);

    //because ownership is taken, this fails
    // let v4: Vec<_> = v1.iter().map(|x| x + 2).collect();
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}