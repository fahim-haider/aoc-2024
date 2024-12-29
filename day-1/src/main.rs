
// str is primitive, immutable
// String is mutable, and its size is not known at compile time

fn parse_input_file(input_file: &str) -> (Vec<i32>, Vec<i32>){
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = vec![];

    for line in input_file.lines() {
        let split_text = line.split(' ');

        let number1 = split_text.clone().next().unwrap();
        let number2 = split_text.last().unwrap();

        println!("Number1 is {number1}");

        //list1.push(number1.parse::<i32>().unwrap());
        //list2.push(number2.parse::<i32>().unwrap());

        //list1.push(split_text.clone().next().unwrap().parse::<i32>().unwrap());
        //list2.push(split_text.last().unwrap().parse::<i32>().unwrap());
    }

    return (list1, list2);
}

fn main() {
    // The task is split into parts: 
    // 1. Read the input file and fill in arrays
    
    let (mut vec1,mut vec2) = parse_input_file("src/input.txt");

    //println!("With text:\nvec1: {} \t vec2: {}", vec1[0],vec2[0]);

    // 2. Calculate the difference between the arrays
}