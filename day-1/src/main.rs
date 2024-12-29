
// str is primitive, immutable
// String is mutable, and its size is not known at compile time

fn parse_input_file(input_file: &str) -> (Vec<i32>, Vec<i32>){
    let mut list1: Vec<i32> = Vec<i32>;
    for line in input_file.lines() {
        let (mut number1, mut number2) = line.split_at(5);
    }

    return (number1, number2);
}

fn main() {
    // The task is split into parts: 
    // 1. Read the input file and fill in arrays
    
    let (mut vec1,mut vec2) = parse_input_file("src/input.txt");

    println!("With text:\nvec1: {} \t vec2: {}", vec1[0],vec2[0]);

    // 2. Calculate the difference between the arrays
}