use std::fs;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("./day3/src/input/mulCode.txt").expect("Error reading file");

    /*-------------------------------
        Puzzle 1	
    -------------------------------*/
    let total = calc_muls(&input);

    println!("Total: {}", total);

    /*-------------------------------
        Puzzle 2
    -------------------------------*/
    let dont_regex = Regex::new(r"don't\(\)[\s\S]*?do\(\)").expect("Error creating regex");
    let mut input2: String = dont_regex.replace_all(&input, "").into_owned();
    // If there are still don'ts, split the string and just take the first half
    input2 = input2.split("don't").collect::<Vec<&str>>()[0].to_string();

    let total2 = calc_muls(&input2);

    println!("Total 2: {}", total2);

    Ok(())
}

fn calc_muls(input: &str) -> u32 {
    let mask = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("Error creating regex");
    let mut total: u32 = 0;

    for (_, [a, b]) in mask.captures_iter(&input).map(|cap| cap.extract()) {
        total += a.parse::<u32>().expect("Error parsing a") * b.parse::<u32>().expect("Error parsing b");
    }

    total
}