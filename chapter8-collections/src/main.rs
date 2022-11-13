use std::collections::HashMap;

fn main() {
    vector();
    vector_read();
    vector_iterate();
    vector_iterate_change();
    multitype_vector_enum();
    string();
    string_change();
    format_macro();
    string_slicing();
    iterating_strings();
    hashmap();
    hashmap_update_old_value();
}

fn vector() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    println!("Vectors are: {:?}, {:?}, {:?}", v1, v2, v3);
}

fn vector_read() {
    let v = vec![1, 2, 3, 4, 5];

    // If we want to crash when accessing element outside of range.
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // If we want to handle the case when the element outside of range is accessed.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
// The code in Listing 8-6 might look like it should work: why should
// a reference to the first element care about changes at the end of the
// vector? This error is due to the way vectors work: because vectors put the
// values next to each other in memory, adding a new element onto the end of
// the vector might require allocating new memory and copying the old elements
// to the new space, if there isn’t enough room to put all the elements next
// to each other where the vector is currently stored. In that case, the
// reference to the first element would be pointing to deallocated memory.
// The borrowing rules prevent programs from ending up in that situation.
// fn vector_crash() {
//     let mut v = vec![1, 2, 3, 4, 5];
//     let first = &v[0];
//     v.push(6);

//     println!("The first element is: {}", first);
// }

fn vector_iterate() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

fn vector_iterate_change() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("The rsulting vector is: {:?}", v);
}

fn multitype_vector_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Sample multitype vector is: {:#?}", row);
}

fn string() {
    // All work fine.
    let data = "initial contents";
    let _s = data.to_string();
    let _s = "initial contents".to_string();
    let s = String::from("initial contents");

    println!("This is a string: {}", s);
}

fn string_change() {
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("This is a concatenated string: {}", s);

    // "bar" is of type &str (string slice) because we don't want to take ownership.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    println!("s2 is {}", s2);

    // "push" takes just a single char
    let mut s3 = String::from("lo");
    s3.push('l');

    println!("This is 'lol': {}", s3);

    // Concatenation with the "+" operator.
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5;

    println!("This is a stirng concatenated with '+': {}", s6);

    // Can concatenate multiple Strings, &Strings, an &strs with "+"
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    let s10 = s7 + "-" + &s8 + "-" + &s9;

    println!("These are multiple strings concatenated with '+': {}", s10);
}

// format! macro uses references so that this call doesn’t take ownership of
// any of its parameters.
fn format_macro() {
    // ...Or I can use the "format!" macro for multiple concatenations.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{}-{}-{}", s1, s2, s3);

    println!("These are multiple strings concatenated with '+': {}", s4);
}

// Indexing into strings is not supported, because it's not clear whether you want bytes or chars.
// Besides, UTF-8 encoding has multiple bytes for a single char.
// fn string_indexing() {
//     let s1 = String::from("hello");
//     let h = s1[0];

//     // "З" is encoded in 2 bytes: 208 and 151, it's not clear what we want here.
//     let s2 = "Здравствуйте";
//     let answer = &s2[0];
// }

fn string_slicing() {
    let s = "Здравствуйте";
    let z = &s[0..2];

    println!("This is letter 'З': {}", z);
}

fn iterating_strings() {
    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}

fn hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Red")).or_insert(100);

    println!("This is a hashmap: {:#?}", scores);

    // Getting a value from a hashmap.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(v) => println!("This is score for team 'Blue': {}", v),
        None => println!("No value for this team."),
    };

    // Iterating over a hashmap.
    for (key, val) in &scores {
        println!("{}: {}", key, val);
    }

    // Insert a key-value pair if it's not present.
    let team_name = "Red".to_string();
    let score = scores.get(&team_name);

    match score {
        Some(v) => println!("This is score for team 'Red': {}", v),
        None => println!("No value for this team."),
    };
}

fn hashmap_update_old_value() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
