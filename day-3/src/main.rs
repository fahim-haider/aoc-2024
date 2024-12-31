use std::fs;

fn parse_input_file(input_file: &str) -> Vec<String> {
    let contents = fs::read_to_string(input_file);

    contents.unwrap().lines()
            .map(|a| a.parse::<String>().unwrap())
            .collect::<Vec<String>>()
}

// Finds valid mul commands and pushes them to be multiplied
fn find_valid_mul (memory_content: &String) -> Vec<String>{
    let mut substr: Vec<String> = memory_content.clone()
                                    .split_inclusive(')')
                                    .map(|a| a.to_string())
                                    .filter(|b| b.contains("mul("))
                                    .map(|mut c| {
                                                    c.replace_range(0..(c.find("mul").unwrap()), ""); 
                                                    c
                                                })
                                    .collect();

    for index in 0..substr.len() {
        println!("After parsing:\tsubstr[{index}]: {}", substr[index]);
    }

    
    substr
}

fn main() {
    let content = parse_input_file("src/input.in");

    // Part 1: Find valid mul operations and calculate the sum of the products
    /*
    let clean_string: Vec<String> = content.clone().iter()
        .map(|a| find_valid_mul(a))
        .map(|a| a.iter().map(|e| e.to_string()))
        .collect();
    */
    find_valid_mul(&content[0]);

}
