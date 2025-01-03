use std::collections::{HashMap, HashSet};

// Return a HashMap with the key set to an i32 and its value is a vector 
// with all the i32s that the key should be less than
fn internalize_rules(rules: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    // The hashmap will contain all the numbers that it should be before!
    let mut hash: HashMap<i32, Vec<i32>> = HashMap::new();

    // Iterate through the rules and split the strings at |
    // Add the rules into the hash map
    // Collect the results rules, but it is not used... lol might be too 'lazy' now
    let _rules: Vec<Vec<i32>> = rules.iter()
                .map(|a| {match hash.get_mut(&a[0]) {
                    Some(v) => {v.push(a[1]); a.clone()}
                    None => {hash.insert(a[0], vec![a[1]]); a.clone()}
                }})
                .collect();
    hash
}

// Return true if the report is valid based on the rules in the HashMap, false otherwise
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

// Calculate and return the sum of the middle page of the valid reports
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

// Sort the invalid report based on the rules in the HashMap
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

// Find the sum of the middle element of each of the reports that were invalid before sorting
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

pub fn try_hashing(rules: Vec<Vec<i32>>, reports: Vec<Vec<i32>>) {
    // Part 1: Sum the middle value of valid pages
    let sum_mid_page = sum_valid_pages(rules.clone(), reports.clone());
    println!("Part1: {sum_mid_page}");

    // Part 2: Sum the middle value of invalid pages after sorting
    let sum_sorted = sum_invalid_pages(rules.clone(), reports.clone());
    println!("Part2: {sum_sorted}");
}