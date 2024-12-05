use std::fs;
use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Cannot read input");
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;
    let mut should_multiply = true;

    for cap in re.captures_iter(&contents) {
        let command = cap[0].chars().take(3).collect::<String>();
        match command.as_str() {
            "mul" => {
                if should_multiply {
                    sum += &cap[1].parse::<i32>().unwrap() * &cap[2].parse::<i32>().unwrap()
                }
            }
            "do(" => {
                should_multiply = true
            }
            "don" => {
                should_multiply = false
            }
            _ => {
                panic!("unknown command found, {}", command)
            }
        }
    }

    println!("Sum: {}", sum)

}
