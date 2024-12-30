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

fn calculate_total_difference(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>) -> (Vec<i32>, i32) {
    vec1.sort();
    vec2.sort();

    let mut result: Vec<i32> = vec![];
    let mut sum: i32 = 0;

    for i in 0..vec1.len() {
        let difference: i32 = (vec1[i]-vec2[i]).abs();
        sum += difference;
        result.push(difference);
    }

    return (result, sum);
}

fn main() {
    // Read the input file and fill in arrays
    
    let (mut vec1,mut vec2) = parse_input_file("src/input.txt");

    println!("After parsing:\nvec1[0]: {} \t vec2[0]: {}", vec1[0],vec2[0]);

    // Sort arrays and calculate the difference between them in a new array

    let (mut result, mut sum) = calculate_total_difference(&mut vec1, &mut vec2);

    println!("After calculations:\nvec1[0]: {}\t vec2[0]: {}\t result[0]: {}\t sum: {}", 
                vec1[0],vec2[0],result[0], sum);


}

/* Some notes!
    - str is primitive, immutable
    - String is mutable, and its size is not known at compile time
    - unwrap returns the actual value from a result or option type
    - split text returns an interator interestingly
*/