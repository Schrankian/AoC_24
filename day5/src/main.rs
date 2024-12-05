use std::{collections::{HashMap, HashSet}, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("./day5/src/input/printRules.txt").expect("Error reading file").replace("\r\n", "\n");    
    let (rule_input, puzzle_input) = input.split_once("\n\n").expect("Error splitting input");
    
    let rules = rule_input.lines().map(|line| {
            let (first, second) = line.split_once("|").expect("Error splitting line");
            let first = first.parse::<usize>().expect("Error parsing first");
            let second = second.parse::<usize>().expect("Error parsing second");
            (second, first)
        }).fold(HashMap::new(), |mut acc, (second, first)| {
            acc.entry(second).or_insert_with(Vec::new).push(first);
            acc
        });

    let puzzle_input = puzzle_input.lines().map(|line| {
        line.split(",").map(|num| num.parse::<usize>().expect("Error parsing number")).collect::<Vec<usize>>()
    }).collect::<Vec<Vec<usize>>>();

    /*-------------------------------
         Puzzle 1
    -------------------------------*/
    let mut total = 0;
    let mut incorrect: Vec<Vec<usize>> = Vec::new(); // For puzzle 2

    for input in &puzzle_input {
        if violates_rule(&input, &rules) {
            incorrect.push(input.clone());
            continue;
        }

        total += get_middle(&input);
    }

    println!("Total: {}", total);

    /*-------------------------------
         Puzzle 2
    -------------------------------*/
    let mut total2 = 0;

    for input in &incorrect {
        let mut input = input.clone();
        while let Some((pos1, pos2)) = get_rule_violation(&input, &rules) {
            input.swap(pos1, pos2);
        }

        total2 += get_middle(&input);
    }

    println!("Total 2: {}", total2);

    Ok(())
}

fn violates_rule(input: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> bool {
    let mut should_not_be_there: HashSet<usize> = HashSet::new();
    for number in input {
        if should_not_be_there.contains(&number) {
            return true;
        }
        if let Some(constrains) = rules.get(&number) {
            constrains.iter().for_each(|num| {
                should_not_be_there.insert(*num);
            });
        }
    }

    false
}

// If rule violation, return (position1, position2)
fn get_rule_violation(input: &Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> Option<(usize, usize)> {
    let mut should_not_be_there: Vec<(usize, usize)> = Vec::new(); // (position, number)
    for i in 0..input.len() {
        if let Some(rule_violation) = should_not_be_there.iter().find(|(_, rule_number)| *rule_number == input[i]) {
            return Some((rule_violation.0, i));
        }
        if let Some(constrains) = rules.get(&input[i]) {
            constrains.iter().for_each(|num| {
                should_not_be_there.push((i, *num));
            });
        }
    }

    None
}

fn get_middle(input: &Vec<usize>) -> usize {
        let middle = input.len() / 2;
        input[middle]
}
