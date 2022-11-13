fn main() {
    deep_copy();
    stack_only_copy();
    ownership_test1();
    ownership_test2();
    mutable_referece();
    hello_slices();
    array_slices();
}

// WON'T COMPILE
// fn double_free_error() {
//     // s1 and s1 reference the same pointer, length, and capacity.
//     let s1 = String::from("hello");
//     let s2 = s1;

//     // Will produce an error, because s1 is no longer valid.
//     // Rust does this to avoid "double free error".
//     // "Move" == "Shallow copy" + "Invalidation".
//     println!("{}, world!", s1);
// }

fn deep_copy() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_only_copy() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn ownership_test1() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length1(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length1(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn ownership_test2() {
    // Passing a reference == "borrowing".
    let s1 = String::from("hello");
    let len = calculate_length2(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length2(s: &String) -> usize {
    // This function "borrows".
    // References are immutable by default.
    s.len()
}

fn mutable_referece() {
    // There can be only one mutable reference at a time in one scope.
    // Having a mutable and immutable reference in the same scope is also not possible.
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// WON'T COMIPLE
// fn dangling_reference() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s is dropped, but the reference is returned!

fn hello_slices() {
    let s = String::from("Hello slices!");

    let full0 = &s[..];
    let full1 = &s[0..s.len()];
    let full2 = &s[0..];

    let hello = &s[..5];
    let slices = &s[6..];

    print!(
        "These are full strings: \n{}\n{}\n{}\n",
        full0, full1, full2
    );

    println!("This is just 'Hello': {}", hello);
    println!("This is just 'slices!': {}", slices);
}

fn array_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
