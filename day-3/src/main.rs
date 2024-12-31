extern crate regex;

use std::fs;
use regex::Regex;

fn parse_input_file (input_file: &str) -> Vec<String> {
    let contents = fs::read_to_string(input_file);

    contents.unwrap().lines()
            .map(|a| a.parse::<String>().unwrap())
            .collect::<Vec<String>>()
}

// Finds valid mul commands and pushes them to be multiplied
fn find_valid_mul (memory_content: &String) -> Vec<String>{
    let substr: Vec<String> = memory_content.clone()
                                    .split_inclusive(')')
                                    .map(|a| a.to_string())
                                    .filter(|b| b.contains("mul("))
                                    .map(|mut c| {
                                                    c.replace_range(..(c.find("mul").unwrap()), "");
                                                    c = c.replace("mul", "");
                                                    if c.contains("mul") {
                                                        c.replace_range(..(c.find("mul").unwrap()), "");
                                                    }
                                                    c
                                                })
                                    .filter(|d| {
                                                    d.clone()
                                                        .replace("(", "")
                                                        .replace(")", "")
                                                        .split(',')
                                                        .all(|e| e.chars().all(|f| f.is_digit(10)))
                                                })
                                    .collect();

    for (index, value) in substr.iter().enumerate() {
        println!("After parsing, substr[{index}]: {}", value);
    }
    substr
}

fn calculate_product (command: String) -> i32{
    let cleaned: Vec<String> = command
                            .replace("mul(", "")
                            .replace(")", "")
                            .split(',')
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>();

    let mut number1 = 0;
    let mut number2 = 0;
    
    if cleaned.len() == 2 {
        number1 = cleaned[0].trim().parse::<i32>().unwrap();
        number2 = cleaned[1].trim().parse::<i32>().unwrap();
    }
    else {
        println!("Something went wrong!");
    }
    
    number1*number2
}

fn main() {
    let content = fs::read_to_string("src/input.in").unwrap();

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    // Part 1: Find valid mul operations and calculate the sum of the products
    let valid_cmds = content.iter()
                                    .flat_map(|line| find_valid_mul(line))
                                    .collect::<Vec<_>>();
    println!("After cleaning up cmds: substr[0]: {}", valid_cmds.len());

    let sum:i32 = valid_cmds.iter()
                            .map(|cmd| calculate_product(cmd.clone()))
                            .sum();
    println!("Total sum of products: \t{sum}\ntest of [0]:{}", calculate_product(valid_cmds[0].to_string()));
}

/*
    - Could have done this way more efficiently using RegEx...


*/