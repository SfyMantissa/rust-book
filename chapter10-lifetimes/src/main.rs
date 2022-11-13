use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    equal_lifetimes();
    string1_longer_lifetime();
    important_excerpt();
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn equal_lifetimes() {
    let string1: String = String::from("abcd");
    let string2: &str = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// We’ve told Rust that the lifetime of the reference returned by the longest
// function is the same as the smaller of the lifetimes of the references
// passed in.
fn string1_longer_lifetime() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

// Attempting to use result after string2 has gone out of scope
// fn string2_out_of_scope() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }

fn important_excerpt() {
    let novel: String = String::from("Call me Ishmael. Some years ago...");
    let first_sentence: &str = novel.split('.').next().expect("Could not find a '.");
    let i: ImportantExcerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("The important excerpt is: {}", i.part);
}

fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
