use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("./day2/src/input/reports.txt").expect("Error reading file");
    let items: Vec<Vec<u32>> = input.split("\n").filter(|&element| !element.is_empty()).map(|element| {
        element.split(" ").map(|el| el.parse::<u32>().expect("An Element wasn't a number.")).collect::<Vec<u32>>()
    }).collect();


    /*-------------------------------
        Puzzle 1
    -------------------------------*/
    let mut total_safe = 0;

    for item in items.iter() {
        if is_save(&item) {
            total_safe += 1;
        }
    }

    println!("Total Safe: {}", total_safe);

    /*-------------------------------
        Puzzle 2
    -------------------------------*/
    let mut total_safe_with_chance = 0;

    'main: for item in items.iter() {
        for i in 0..item.len() {
            let mut modified_item = item.clone();
            modified_item.remove(i);

            if is_save(&modified_item) {
                total_safe_with_chance += 1;
                continue 'main;
            }
        }
    }

    println!("Total Safe with Chance: {}", total_safe_with_chance);

    Ok(())
}

fn is_save(item: &Vec<u32>) -> bool {
    let increasing = item.windows(2).any(|pair| pair[0] > pair[1]);
    let decreasing = item.windows(2).any(|pair| pair[0] < pair[1]);

    if (increasing && decreasing) || (!increasing && !decreasing) {
        return false;
    }

    for i in 0..item.len() - 1 {
        let diff = (item[i] as i32 - item[i + 1] as i32).abs() as u32;
        if diff > 3 || diff < 1 {
            return false;
        }
    }

    true
}