use std::fs;
use std::collections::{HashMap, HashSet};

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

fn is_valid_report(report: &Vec<i32>, hash: &mut HashMap<i32, Vec<i32>>) -> bool {
    for page in 1..report.len() {
        let set: HashSet<i32> = report[..page].iter().cloned().collect();
        if let Some(items) = hash.get_mut(&report[page]) {
            if items.iter().any(|item| set.contains(item)) {
                return false;
            }
        }
    }
    true
}

fn sum_valid_pages(rules: Vec<Vec<i32>>, reports: Vec<Vec<i32>>) -> i32 {
    let mut hash: HashMap<i32, Vec<i32>> = internalize_rules(rules);
    let mut sum_mid_page: i32 = 0;

    let valid_reports:Vec<&Vec<i32>> = reports.iter()
                        .filter(|&report| is_valid_report(report, &mut hash))
                        .collect();

    let _temp: Vec<&Vec<i32>> = valid_reports.iter()
                        .map(|&report| {sum_mid_page += report[report.len()/2]; 
                                    report})
                        .collect();
    sum_mid_page
}

fn sort_report(report: &Vec<i32>, hash: &mut HashMap<i32, Vec<i32>>) -> Vec<i32>{
    let mut sorted: Vec<i32> = report.clone();
    let mut is_sorted: bool = false;

    while !is_sorted {
        is_sorted = true;
        for page in 1..sorted.len() {
            let prev_elements: Vec<i32> = sorted[..page].iter().cloned().collect();
            let current: i32 = sorted[page];
            if let Some(rules) = hash.get(&current) {
                for (j, prev_el) in prev_elements.iter().enumerate() {
                    if rules.contains(&prev_el) {
                        sorted.swap(page, j);
                        is_sorted = false;
                        break;
                    }
                }
            }
        }
    }
    sorted
}

fn sum_invalid_pages(rules: Vec<Vec<i32>>, reports: Vec<Vec<i32>>) -> i32 {
    let mut hash: HashMap<i32, Vec<i32>> = internalize_rules(rules);
    let mut sum_sorted: i32 = 0;

    // Filter out the valid reports, so we only sort the invalid reports
    let invalid_reports:Vec<&Vec<i32>> = reports.iter()
                        .filter(|&report| !is_valid_report(report, &mut hash))
                        .collect();

    let _temp: Vec<Vec<i32>> = invalid_reports.iter()
                        .map(|&report| sort_report(report, &mut hash))
                        .map(|report| {sum_sorted += report[report.len()/2]; 
                                    report})
                        .collect();

    sum_sorted
}

fn main() {
    // Split the input files into two vectors, rules and pages
    let (rules, reports) = parse_input_file("src/input.in");

    // Part 1: Sum the middle value of valid pages
    let sum_mid_page = sum_valid_pages(rules.clone(), reports.clone());
    println!("Part1: {sum_mid_page}");

    // Part 1: Sum the middle value of incorrect pages after sorting
    let sum_sorted = sum_invalid_pages(rules.clone(), reports.clone());
    println!("Part2: {sum_sorted}");
}