use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Cannot read input");
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;

    for cap in re.captures_iter(&contents) {
        sum += &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap();
    }

    println!("Sum: {}", sum)

}
