use std::collections::HashMap;
use std::fs;
use std::ops::AddAssign;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let lines = contents.lines();

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    let mut occurences: HashMap<u32, u32> = HashMap::new();

    for line in lines {
        let mut locations = line.split("   ");
        left.push(locations.next().unwrap().parse::<u32>().unwrap());

        let right_element = locations.next().unwrap().parse::<u32>().unwrap();
        right.push(right_element);

        if ! occurences.contains_key(&right_element) {
            occurences.insert(right_element, 1);
        } else {
            occurences.get_mut(&right_element).unwrap().add_assign(1);
        }
    }

    left.sort();
    right.sort();

    let mut distance = 0;
    let mut score = 0;

    for i in 0..left.len() {
        distance += left[i].abs_diff(right[i]);
        if occurences.contains_key(&left[i]) {
            score += left[i] * occurences[&left[i]];
        }
    }

    println!("Distance: {}", distance);
    println!("Score: {}", score);
}
