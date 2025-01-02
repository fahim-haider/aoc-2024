use std::fs;
use std::collections::HashMap;

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

    println!("{:?}", rules[0]);
    println!("{:?}", reports[0]);
    
    (rules,reports)
}

fn internalize_rules(rules: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    // The hashmap will contain all the numbers that it should be before!
    let mut hash: HashMap<i32, Vec<i32>> = HashMap::new();

    // Iterate through the rules and split the strings at |
    // Add the rules into the hash map
    // Collect the results rules, but it is not used... lol
    let _rules: Vec<Vec<i32>> = rules.iter()
                .map(|a| {match hash.get_mut(&a[0]) {
                    Some(v) => {v.push(a[1]); a.clone()} 
                    None => {hash.insert(a[0], vec![a[1]]); a.clone()}
                }})
                .collect();
    hash
}

fn is_valid_report(page: Vec<i32>) -> bool {


    false
}

fn count_valid_pages(rules: Vec<Vec<i32>>, reports: Vec<Vec<i32>>) -> i32 {
    let hash = internalize_rules(rules);
    println!("{:?}", hash[&61]);

    reports.iter().filter(|&report| is_valid_report(report)).count() as i32
}

fn main() {
    // Split the input files into two vectors, rules and pages
    let (rules, reports) = parse_input_file("src/input.in");

    // Part 1: Check which pages are valid
    let count = count_valid_pages(rules, reports);
    println!("{count}");
}