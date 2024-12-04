use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let input = fs::read_to_string("./day4/src/input/charList.txt").expect("Error reading file");
   let char_matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    /*-------------------------------
         Puzzle 1
    -------------------------------*/
    let mut total = 0;

    for row in 0..char_matrix.len() {
        for col in 0..char_matrix[row].len() {
            // TODO break early if its not possible to find xmas
            total += find_xmas(&char_matrix, row, col);
        }
    }

    println!("Total: {}", total);

    /*-------------------------------
         Puzzle 2
    -------------------------------*/
    let mut total2 = 0;

    for row in 1..char_matrix.len()-1 {
        for col in 1..char_matrix[row].len()-1 {
            if find_mas(&char_matrix, row, col) {
                total2 += 1;
            }
        }
    }

    println!("Total 2: {}", total2);

    Ok(())
}

fn find_xmas(char_matrix: &Vec<Vec<char>>, row: usize, col: usize) -> u32 {
    let mut vertical = String::new();
    let mut horizontal = String::new();
    let mut diagonal_right = String::new();
    let mut diagonal_left = String::new();

    if (char_matrix.len() - row) >= 4 {
        vertical = (0..4).map(|i| char_matrix[row + i][col]).collect::<String>();
    }

    if (char_matrix[row].len() - col) >= 4 {
        horizontal = (0..4).map(|i| char_matrix[row][col + i]).collect::<String>();
    }

    if !vertical.is_empty() && !horizontal.is_empty() {
        diagonal_right = (0..4).map(|i| char_matrix[row + i][col + i]).collect::<String>();
    }

    if !vertical.is_empty() && col >= 3 {
        diagonal_left = (0..4).map(|i| char_matrix[row + i][col - i]).collect::<String>();
    }

    count_true(&[
        vertical == "XMAS",
        vertical == "SAMX",
        horizontal == "XMAS",
        horizontal == "SAMX",
        diagonal_right == "XMAS",
        diagonal_right == "SAMX",
        diagonal_left == "XMAS",
        diagonal_left == "SAMX"
    ])
}

fn find_mas(char_matrix: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let right_down = (0..3).map(|i| char_matrix[row - 1 + i][col - 1 + i]).collect::<String>();
    let right_up = (0..3).map(|i| char_matrix[row + 1 - i][col - 1 + i]).collect::<String>();

    (right_down == "MAS" || right_down == "SAM") && (right_up == "MAS" || right_up == "SAM")
}

fn count_true(values: &[bool]) -> u32 {
    values.iter().filter(|&&b| b).count() as u32
}