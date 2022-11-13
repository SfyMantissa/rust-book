fn main() {
    largest_number();
    largest_number_with_helper();
}

fn largest_number() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let mut largest: &i32 = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list: Vec<i32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let mut largest: &i32 = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

fn largest_number_with_helper() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let result: &i32 = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list: Vec<i32> = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result: &i32 = largest(&number_list);
    println!("The largest number is {}", result);
}

fn largest(list: &[i32]) -> &i32 {
    let mut largest: &i32 = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
