use std::io;
use std::io::Write;

fn main() {
    println!("What would you like to do?");
    println!("1. Convert temperature from Fahrenheit to Celsius.");
    println!("2. Get the nth Fibonacci number.");
    println!("3. Print lyrics to the song \"Twelve Days of Christmas\".");

    print!("Please, input your choice (1-3): ");

    let choice: u8 = readline_helper()
        .trim()
        .parse()
        .expect("Input is not an integer.");

    match choice {
        1 => fahrenheit_to_celcius(),
        2 => nth_fibonacci_number(),
        3 => twelve_days_of_christmas(),
        _ => println!("\nMust choose between 1-3. Aborting."),
    }
}

fn fahrenheit_to_celcius() {
    print!("\nPlease, input temperature in °F: ");

    let fahrenheit: f64 = readline_helper()
        .trim()
        .parse()
        .expect("Input is not a float.");

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("Temperature in °C is: {}", celsius);
}

fn nth_fibonacci_number() {
    print!("\nPlease, input the index of Fibonacci number: ");

    let fibonacci_index: u16 = readline_helper()
        .trim()
        .parse()
        .expect("Input is not a valie u16.");

    let fibonacci_number: u32 = fibonacci_helper(fibonacci_index);

    println!(
        "Fibonacci number with index {} is {}.",
        fibonacci_index, fibonacci_number
    );
}

fn twelve_days_of_christmas() {
    let days_array = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let verse_array = [
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut counter = 0;

    while counter < 12 {
        println!("\n[Verse {}]", counter + 1);
        print!(
            "On the {} day of Christmans,\nmy true love sent to me\n",
            days_array[counter]
        );
        for index in (0..counter).rev() {
            println!("{}", verse_array[index]);
        }
        println!("A patridge in a pear tree");

        counter += 1;
    }
}

fn fibonacci_helper(index: u16) -> u32 {
    if index == 1 || index == 2 {
        1
    } else {
        fibonacci_helper(index - 1) + fibonacci_helper(index - 2)
    }
}

fn readline_helper() -> String {
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    return input;
}
