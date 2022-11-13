use std::ops::Deref;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use crate::List::{Cons, Nil};

fn main() {
    box_type();
    cons_list();
    basic_reference();
    box_reference();
    custom_smart_pointer();
}

fn box_type() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn cons_list() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("The cons list is: {:?}", list);
}

fn basic_reference() {
    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn box_reference() {
    // Equivalent to basic_reference
    let x: i32 = 5;
    let y: MyBox<i32> = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn custom_smart_pointer() {
    let _c: CustomSmartPointer = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d: CustomSmartPointer = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
}
