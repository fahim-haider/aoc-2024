extern crate regex;

use std::fs;
use regex::Regex;

// Finds valid mul commands and pushes them to be multiplied
fn find_valid_mul (memory_content: &String, re: &Regex) -> Vec<String>{
    re.find_iter(memory_content)
                        .map(|s| s.as_str())
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
}

// Calculates product of the command if do() is enabled (flag is set to true)
fn calculate_product (command: String, flag: &mut bool) -> i32{
    if command.contains("don't") {
        *flag = false;
    } else if command.contains("do") {
        *flag = true;
    }
    else {
        if flag == &mut true {
            let cleaned: Vec<String> = command
                        .replace("mul(", "")
                        .replace(")", "")
                        .split(',')
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();

            if cleaned.len() == 2 {
                let number1 = cleaned[0].trim().parse::<i32>().unwrap_or(0);
                let number2 = cleaned[1].trim().parse::<i32>().unwrap_or(0);
                return number1*number2;
            }
            else {
                println!("Something went wrong!");
            }
        }
    }
    0
}

fn main() {
    let content = fs::read_to_string("src/input.in").unwrap();
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
    let mut flag: bool = true;

    // Part 1: Find valid mul operations and calculate the sum of the products
    let valid_cmds = find_valid_mul(&content, &re);

    let sum: i32 = valid_cmds.iter()
                        .map(|c| calculate_product(c.clone(), &mut true))
                        .sum();
    println!("Part 1 result = {sum}");

    // Part 2: include functionality for do and don't using a flag! If true, that means
    // the last control statement was do(), if false, then the last control statement was don't()
    let sum2: i32 = valid_cmds.iter()
                        .map(|c| calculate_product(c.clone(), &mut flag))
                        .sum();
    println!("Part 2 result = {sum2}");
}

/*
    - Could have done this way more efficiently using RegEx... SO I DID
    - Planned out my functions well, i think 
    - Some map and filter functions were tricky, but this was good practice
    - I definitely need to learn about pointers and passing as reference in Rust
*/