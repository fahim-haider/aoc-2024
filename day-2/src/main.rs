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
        if report.windows(2).all(|w| (w[1] - w[0]).abs() <= 3 && (w[1] - w[0]).abs() >= 1) {
            if report.is_sorted_by(|a,b| a < b) 
            || report.is_sorted_by(|a,b| a > b) {
                safe_count += 1;
            } else {
                for element in 0..report.len() {
                    let mut clone = report.clone();
                    clone.remove(element);
                    if clone.is_sorted_by(|a,b| a < b) 
                    || clone.is_sorted_by(|a,b| a > b) {
                        safe_count += 1;
                        break;
                    }
                }
            }
        }
    }
    safe_count
}

fn main() {
    // Part 1:
    // Read the input file and fill in arrays
    let list = parse_input_file("src/input.txt");
    println!("After parsing:\tvec1[0]: {}", list[0][0]);

    // Find safe reports
    let safe_reports = count_safe_reports(list);
    println!("Safe reports: {safe_reports}");

}