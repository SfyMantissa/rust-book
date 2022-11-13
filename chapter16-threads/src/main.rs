use std::thread;
// use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    spawned_thread_finishes_before_main();
    println!();
    spawned_thread_finishes_after_main();
    println!();
    moving_value_in_thread();
}

fn spawned_thread_finishes_before_main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi! Number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi! Number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn spawned_thread_finishes_after_main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi! Number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi! Number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn moving_value_in_thread() {
    let v: Vec<i32> = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
