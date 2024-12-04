use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let lines = contents.lines();

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in lines {
        let mut locations = line.split("   ");
        left.push(locations.next().unwrap().parse::<u32>().unwrap());
        right.push(locations.next().unwrap().parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut distance = 0;

    for i in 0..left.len() {
        distance += left[i].abs_diff(right[i]);
    }

    println!("Distance: {}", distance);
}
