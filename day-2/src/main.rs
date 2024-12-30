use std::fs;

fn parse_input_file(input_file: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(input_file);

    contents.unwrap().lines()
            .map(|st| st.split(' '))
            .map(|a| a.map(|e| e.parse::<i32>().unwrap()))
            .map(|vec| vec.collect())
            .collect::<Vec<_>>()
}

fn is_report_safe(report: &[i32]) -> bool {
    // Find if this report matches the difference criteria
    let diff = report
                    .windows(2)
                    .all(|w| (w[1] - w[0]).abs() <= 3 && (w[1] - w[0]).abs() >= 1);

    // Return true if report is safe
    diff
    && (report
        .is_sorted_by(|a,b| a > b)
    || report
        .is_sorted_by(|a,b| a < b))
}

fn is_report_partially_safe(report: &[i32]) -> bool {
    // Create all possible subreports of report
    let mut subreports: Vec<Vec<i32>> = vec![];
    for index in 0..report.len() {
        let mut temp = report.to_vec();
        temp.remove(index);
        subreports.push(temp);
    }

    // Find if any of them are safe
    subreports.iter().any(|r| is_report_safe(&r))
}

fn main() {
    // Part 1: Read the input file and fill in arrays
    let list = parse_input_file("src/input.txt");
    println!("After parsing:\tvec1[0]: {}", list[0][0]);

    // Find safe reports
    let safe_reports = list
                        .iter()
                        .filter(|r| is_report_safe(r))
                        .count();
    println!("Completely safe reports: {safe_reports}");

    // Part 2: Find partially safe reports
    let partially_safe = list
                        .iter()
                        .filter(|r| is_report_partially_safe(r))
                        .count();
    println!("Partially safe reports: {partially_safe}");
}

/*
    - Very challenging day... learned alot about mapping and iterators today
    - This really made me realize how important it is to plan out your methods
    - Initially, my functions handled all the counting... then, i realized 
    that making the functions just do the logic made the problem easier to solve.
*/