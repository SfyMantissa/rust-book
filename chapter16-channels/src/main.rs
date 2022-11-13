use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    send_single_value();
    println!();
    send_multiple_values();
    println!();
    send_multiple_values_from_multiple_threads();
}

fn send_single_value() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi!");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn send_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals: Vec<String> = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread!"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn send_multiple_values_from_multiple_threads() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals: Vec<String> = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread!"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals: Vec<String> = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
