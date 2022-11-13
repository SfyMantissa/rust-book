use std::collections::HashMap;

fn main() {
    let mut integers1: Vec<usize> = vec![2, 1, 5, 4, 3];
    median(&mut integers1);
    modes(&mut integers1);

    let mut integers2: Vec<usize> = vec![4, 2, 6, 1, 5, 3];
    median(&mut integers2);
    modes(&mut integers2);

    let mut integers3: Vec<usize> = vec![1, 2, 6, 1, 5, 3];
    median(&mut integers3);
    modes(&mut integers3);
}

fn median(integers: &mut Vec<usize>) {
    println!(
        "\nThis is the passed vector mutable reference: {:?}",
        integers
    );

    integers.sort();

    println!(
        "This is the sorted vector mutable reference: {:?}",
        integers
    );

    let length: usize = integers.len();
    let median: f64;

    if length % 2 != 0 {
        median = integers[length / 2] as f64;
    } else {
        let upper_index: usize = length / 2;
        let lower_index: usize = upper_index - 1;
        let sum: f64 = (integers[lower_index] + integers[upper_index]) as f64;
        median = sum / 2.0;
    }
    println!("The median is: {}", median);
}

fn modes(integers: &mut Vec<usize>) {
    let mut modes: Vec<usize> = Vec::new();
    let mut occurences: HashMap<&mut usize, usize> = HashMap::new();
    let mut max_occurences: usize = 0;

    for int in integers {
        let count: &mut usize = occurences.entry(int).or_insert(0);
        *count += 1;

        if *count >= max_occurences {
            max_occurences = *count;
        }
    }

    for (key, val) in &occurences {
        if *val == max_occurences {
            modes.push(**key);
        }
    }

    println!("Modes for the given set are: {:?}", modes);
}
