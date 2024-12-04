use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let lines = contents.lines();

    let mut safe_reports = 0;

    for line in lines {
        let reports: Vec<i32> = line.split(" ")
            .filter_map(|s| s.parse::<i32>().ok()).collect();

        if check_for_safeness(&reports) {
            safe_reports += 1;
        }
    }


    println!("Safe reports: {}", safe_reports);
}

fn check_for_safeness(reports: &Vec<i32>) -> bool {
    if base_safeness(&reports) {
       return true;
    }
    for i in 0..reports.len() {
        let new_report = reports.iter().enumerate()
            .filter(|(idx, _)| *idx != i).map(|(_, &val) | val).collect();
        if base_safeness(&new_report) {
            return true;
        }
    }
    false
}

fn base_safeness(reports: &Vec<i32>) -> bool {
    let increasing = reports[0] < reports[1];

    for i in 1..reports.len() {
        let diff  = reports[i] - reports[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 || (increasing && diff < 0) || (!increasing && diff > 0) {
            return false
        }
    }
    true
}
