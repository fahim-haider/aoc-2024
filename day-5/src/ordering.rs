use std::cmp::Ordering::*;

// This strategy is based on maneatingape's solution on GitHub...
// I wanted to try it to learn how to simplify this challenge

fn parse(rules: Vec<Vec<i32>>, reports: Vec<Vec<i32>>) -> (i32, i32){
    // Create an array with all elements set to Greater by default
    let mut order = [[Greater; 100]; 100];

    // Set relevant pairs in the array as Less than based on rules
    for pair in rules.iter() {
        let (from,to) = (pair[0] as usize,pair[1] as usize);
        order[from][to] = Less;
    }

    let mut ud_report: Vec<i32> = Vec::new();
    let mut part1_count = 0;
    let mut part2_count = 0;

    for report in reports {
        ud_report.clear();
        ud_report.extend(report);
        let middle = ud_report.len() / 2;

        if ud_report
            .is_sorted_by(|&from,&to| 
                                order[from as usize][to as usize] == Less) {
            part1_count += ud_report[middle];
        } else {
            ud_report
                .select_nth_unstable_by(middle, |&from,&to| 
                                        order[from as usize][to as usize]);
            part2_count += ud_report[middle];
        }
    }
    

    (part1_count,part2_count)
}

pub fn try_ordering(rules: Vec<Vec<i32>>, reports: Vec<Vec<i32>>) {
    let (part1,part2) = parse(rules.clone(), reports.clone());
    println!("Part1: {part1}\nPart2: {part2}");
}