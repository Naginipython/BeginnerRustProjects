use std::thread;
use std::time::Duration;
use std::sync::mpsc; //multiple producer, single consumer

fn main() {
    //thread::spawn returns a "JoinHandle"
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //Program waits for the thread. Without this, the thread doesn't finish here.
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        //"move" allows the vector to be used here, otherwise, there is a possibility it can change during.
        //It takes ownership.
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    //16.2
    //creates a transmitter and receiver
    //tx can be cloned, everything goes to rx.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        //"send" returns a Result<T, E>
        tx.send(val).unwrap();
        // println!("val is {}", val); //Fails, because val's ownership changed.

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    //recv will block and wait for the value to be sent. Returns a Result<T, E>
    //alternatively, "try_recv" will not wait, but also return a Result immediately.
    let receieved = rx.recv().unwrap();
    println!("Got: {}", receieved);

    for receieved in rx {
        println!("Got: {}", receieved);
    }
}
