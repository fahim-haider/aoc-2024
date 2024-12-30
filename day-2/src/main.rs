use std::fs;

fn parse_input_file(input_file: &str) -> Vec<Vec<i32>> {
    let mut list: Vec<Vec<i32>> = Vec::new();

    let contents = fs::read_to_string(input_file);

    for line in contents.unwrap().lines() {
        let split_text: Vec<i32> = line.split(' ').map(|a| a.parse::<i32>().unwrap()).collect();

        list.push(split_text);
    }

    list
}

fn count_safe_reports(list: Vec<Vec<i32>>) -> i32 {
    let mut safe_count: i32 = 0;

    for report in list.iter() {
        if report.is_sorted_by(|a,b| a <= b) 
        || report.is_sorted_by(|a,b| a >= b) {
            /*for level in report.is_sorted() {

            }
            */
            safe_count += 1;
        }
    }

    safe_count
}

fn main() {
    // Part 1:
    // Read the input file and fill in arrays
    let mut list = parse_input_file("src/input.txt");
    println!("After parsing:\tvec1[0]: {}", list[0][0]);

    // Find safe reports
    let safe_reports = count_safe_reports(list);
    println!("Safe reports: {safe_reports}");

}