use std::fs;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Pos {
    x: i32,
    y: i32 
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    fn get_next(&self, direction: &Direction) -> Pos {
        match direction{
            Direction::Up => Pos { x: self.x, y: self.y - 1 },
            Direction::Down => Pos { x: self.x, y: self.y + 1 },
            Direction::Left => Pos { x: self.x - 1, y: self.y },
            Direction::Right => Pos { x: self.x + 1, y: self.y }
        }
    }

    fn get_cell(&self, map: &Vec<Vec<char>>) -> Option<char> {
        if  self.y < 0 || self.x < 0 || self.y as usize >= map.len() || self.x as usize >= map[0].len() {
            return None;
        }
        Some(map[self.y as usize][self.x as usize])
    }

    fn set_cell(&self, map: &mut Vec<Vec<char>>, cell: char) {
        map[self.y as usize][self.x as usize] = cell;
    }
}

struct Guard {
    pos: Pos,
    direction: Direction
}

impl Guard {
    fn from_map(map: &Vec<Vec<char>>) -> Guard {
        for (y, row) in map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 'v' {
                    return Guard {
                        pos: Pos::new(x as i32, y as i32),
                        direction: Direction::Down
                    };
                } else if *cell == '^' {
                    return Guard {
                        pos: Pos::new(x as i32, y as i32),
                        direction: Direction::Up
                    };
                } else if *cell == '<' {
                    return Guard {
                        pos: Pos::new(x as i32, y as i32),
                        direction: Direction::Left
                    };
                } else if *cell == '>' {
                    return Guard {
                        pos: Pos::new(x as i32, y as i32),
                        direction: Direction::Right
                    };
                }
            }
        }

        panic!("No guard found");
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("./day6/src/input/map.txt").expect("Error reading file");
    let mut map: Vec<Vec<char>> = input.split("\n").map(|line| line.chars().collect()).collect();

    /*-------------------------------
         Puzzle 1
    -------------------------------*/
    let mut total = 1; // Start at 1 because the first cell is already cleaned
    let mut guard = Guard::from_map(&map);

    'main: loop {
        let next_pos = guard.pos.get_next(&guard.direction);
        let cell = next_pos.get_cell(&map);

        match cell {
            Some('#') => {
                guard.turn_right();
            },
            Some('.') => {
                guard.pos.set_cell(&mut map, 'X');
                total += 1;
                guard.pos = next_pos;
            },
            Some('X') => {
                guard.pos.set_cell(&mut map, 'X');
                guard.pos = next_pos;
            },
            None => {
                break 'main;
            },
            _ => {
                panic!("Invalid cell");
            }
        }
    } 

    println!("Total: {}", total);
    // print_map(&map);

    Ok(())
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
} 