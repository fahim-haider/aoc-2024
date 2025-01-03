mod hashing;
mod ordering;

use std::fs;
use std::time::Instant;

// Split the input file into the rules and reports
fn parse_input_file(file: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let contents = fs::read_to_string(file).unwrap();

    let lines: Vec<&str> = contents.lines()
                                .collect();

    // lines already removes \n, so we check for an empty string to find an empty line
    let mut iterator = lines.split(|line| *line == "");

    // convert to String because ownership needs to be transferred
    // convert to a vector of i32 after splitting
    let rules: Vec<Vec<i32>> = iterator.next()
                    .unwrap_or(&[])
                    .iter()
                    .map(ToString::to_string)
                    .map(|rule| rule.split("|")
                        .map(|a| a.parse::<i32>()
                                .unwrap()).collect::<Vec<_>>())
                    .collect();
    let reports: Vec<Vec<i32>> = iterator.next()
                    .unwrap_or(&[])
                    .iter()
                    .map(ToString::to_string)
                    .map(|str| str.split(',')
                                        .map(|num| num.parse::<i32>().unwrap())
                                        .collect())
                    .collect();
    
    (rules,reports)
}

fn main() {
    // Split the input files into two vectors, rules and pages
    let (rules, reports) = parse_input_file("src/input.in");

    // Solve challenge using HashMaps
    let start = Instant::now();
    hashing::try_hashing(rules.clone(), reports.clone());
    let duration = start.elapsed();
    println!("Time taken using HashMaps: {:?}", duration);

    // Solve challenge using Ordering
    let start = Instant::now();
    ordering::try_ordering(rules.clone(), reports.clone());
    let duration = start.elapsed();
    println!("Time taken using Ordering and Arrays: {:?}", duration)
}
/*
    - Got more practice with HashMaps and HashSets!
    - worked alot with vector manipulation to sort according to the 
    challenges criteria
    - Found a better way after completing this challenge, so i learned 
    to modularize code in Rust to try both methods!
    - Learned Ordering to simplify this challenge with the help of online tutorials
    - Learned more about Arrays in Rust
    RESULTS: (time taken for both parts)
    HashMaps took 74 ms, but Arrays + Ordering took 1 ms!!
*/