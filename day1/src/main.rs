use std::{collections::HashMap, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare lists
    let input = fs::read_to_string("./day1/src/input/locations.txt").expect("Error reading file");
    let items = input.split("\n").filter(|&element| !element.is_empty()).collect::<Vec<&str>>();
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = items.iter().map(|&element| {
        let mut split = element.split("   ");
        (
            split.next().expect("Error trying to extract elements.").parse::<u32>().expect("An Element wasn't a number."), 
            split.next().expect("Error trying to extract elements.").parse::<u32>().expect("An Element wasn't a number.")
        )
    }).unzip();

    if left.len() != right.len() {
        panic!("Lists are not equal in length");
    }

    /*-------------------------------
        Puzzle 1
    -------------------------------*/
    // Sort the lists
    left.sort();
    right.sort();

    let mut total_distance: u32 = 0;

    for i in 0..left.len() {
        let distance: u32 = (left[i] as i32 - right[i] as i32).abs() as u32;
        total_distance += distance;
    }

    println!("Total distance: {}", total_distance);

    /*-------------------------------
        Puzzle 2
        - Sort from puzzle 1 not required
    -------------------------------*/
    let mut similarity_score: u32 = 0;
    let mut right_lookup: HashMap::<u32, u32> = HashMap::new();

    // Create a lookup table for the right list, which contains the number of times each element appears
    for i in 0..right.len() {
        let count = right_lookup.entry(right[i]).or_insert(0);
        *count += 1;
    }

    for i in 0..left.len() {
        similarity_score += left[i] * right_lookup.get(&left[i]).unwrap_or(&0);
    }

    println!("Similarity score: {}", similarity_score);

    Ok(())
}
