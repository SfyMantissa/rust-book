use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    read_username_from_file();
}

fn _panic_macro() {
    panic!("Crash and burn!");
}

fn _panic_library() {
    let v = vec![1, 2, 4];
    v[99];
}

fn _panic_read_file() {
    let greeting_file_result = File::open("hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn _panic_read_file_closures() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn _panic_read_file_unwrap() {
    let _greeting_file = File::open("hello.txt").unwrap();
}

fn _panic_read_expect() {
    let _gretting_file =
        File::open("Hello.txt").expect("Hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("Hola.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_ternary() -> Result<String, io::Error> {
    let mut username_file = File::open("Hola.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
