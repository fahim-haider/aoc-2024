use std::fs;

fn parse_input_file(input_file: &str) -> (Vec<i32>, Vec<i32>){
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = vec![];

    let contents = fs::read_to_string(input_file);

    for line in contents.unwrap().lines() {
        let split_text = line.split(' ');

        list1.push(split_text.clone().next().unwrap().parse::<i32>().unwrap());
        list2.push(split_text.last().unwrap().parse::<i32>().unwrap());
    }

    return (list1, list2);
}

fn main() {
    // The task is split into parts: 
    // 1. Read the input file and fill in arrays
    
    let (mut vec1,mut vec2) = parse_input_file("src/input.txt");

    println!("vec1[0]: {} \t vec2[0]: {}", vec1[0],vec2[0]);

    // 2. Calculate the difference between the arrays
}

/* Some notes!
    - str is primitive, immutable
    - String is mutable, and its size is not known at compile time
    - unwrap returns the actual value from a result or option type
    - split text returns an interator interestingly
*/