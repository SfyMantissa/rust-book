struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    multitype_generics();
    mixup();
}

fn _largest_i32(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn _largest_char(list: &[char]) -> &char {
    let mut largest: &char = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn multitype_generics() {
    let both_integer: Point<i32, i32> = Point { x: 5, y: 10 };
    let both_float: Point<f32, f32> = Point { x: 1.0, y: 4.0 };
    let integer_and_float: Point<i32, f32> = Point { x: 5, y: 4.0 };

    println!(
        "Point with both integer values: x={}, y={}",
        both_integer.x, both_integer.y
    );
    println!(
        "Point with both float values: x={}, y={}",
        both_float.x, both_float.y
    );
    println!(
        "Point with integer and float values: x={}, y={}",
        integer_and_float.x, integer_and_float.y
    );
}

fn mixup() {
    let p1: Point<i32, f32> = Point { x: 5, y: 10.4 };
    let p2: Point<&str, char> = Point { x: "Hello", y: 'c' };
    let p3: Point<i32, char> = p1.mixup(p2);

    println!("p3.x = {} p3.y = {}", p3.x, p3.y);
}
