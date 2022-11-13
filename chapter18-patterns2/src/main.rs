struct Point1 {
    x: i32,
    y: i32,
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor1(i32, i32, i32),
    ChangeColor2(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Post {
    Hello { id: i32 },
}

fn main() {
    match_named_variables();
    match_named_variables_fix();
    match_multiple_patterns();
    match_range();
    destructing_struct();
    struct_matching();
    destructing_enum();
    nested_enums();
    complex_destructing();
    ignoring_values();
    ignoring_remaining_values();
    match_guard();
    bindings();
}

fn match_named_variables() {
    // Note the scope
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn match_named_variables_fix() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn match_multiple_patterns() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_range() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

fn destructing_struct() {
    let p = Point1 { x: 0, y: 7 };
    let Point1 { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    // Shorter version
    let Point1 { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn struct_matching() {
    let p = Point1 { x: 0, y: 7 };

    match p {
        Point1 { x, y: 0 } => println!("On the x axis at {}", x),
        Point1 { x: 0, y } => println!("On the y axis at {}", y),
        Point1 { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

fn destructing_enum() {
    let msg = Message::ChangeColor1(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor1(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
        _ => (),
    }
}

fn nested_enums() {
    let msg = Message::ChangeColor2(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor2(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor2(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }
}

fn complex_destructing() {
    let ((feet, inches), Point1 { x, y }) = ((3, 10), Point1 { x: 3, y: -10 });
    println!("Distance: {} feet, {} inches", feet, inches);
    println!("The point has coordinates: x = {}, y = {}", x, y);
}

fn ignoring_values() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("Setting is {:?}", setting_value);
}

fn ignoring_remaining_values() {
    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }
}

fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

fn bindings() {
    let msg = Post::Hello { id: 5 };

    match msg {
        Post::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Post::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Post::Hello { id } => println!("Found some other id: {}", id),
    }
}
