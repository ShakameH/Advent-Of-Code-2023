use std::fs;
use std::collections::VecDeque;

fn find_starting_position(array: &mut Vec<Vec<(char, i32)>>) -> (usize, usize) {
    let row_len: usize = array[0].len();
    let column_len: usize = array.len();
    for i in 0..column_len {
        for j in 0..row_len {
            if array[i][j].0 == 'S' {
                return (j, i);
            }
        }
    }
    (0, 0)
}

fn path_finder(array: &mut Vec<Vec<(char, i32)>>, position: (usize, usize), length: i32) {
    let mut stack = VecDeque::new();
    let max_row = array[0].len();
    let max_col = array.len();
    
    stack.push_back((position, length));

    while let Some(((x, y), len)) = stack.pop_back() {
        if array[y][x].1 == -1 || array[y][x].1 > len + 1 {
            array[y][x].1 = len + 1;
            match array[y][x].0 {
                'S' => {
                    if x + 1 < max_row {
                        match array[y][x+1].0 {
                            //West 
                            '-' | 'J' | '7' => stack.push_back(((x+1, y), len + 1)),
                            _ => {} 
                        }
                    }
                    if y + 1 < max_col {
                        match array[y+1][x].0 {
                            //North
                            '|' | 'L' | 'J' => stack.push_back(((x, y+1), len + 1)),
                            _ => {}
                        }
                    }
                    if x != 0 {
                        match array[y][x -1].0 {
                            //East
                            '-' | 'F' | 'L' => stack.push_back(((x-1, y), len + 1)),
                            _ => {}
                        }
                    }
                    if y != 0 {
                        match array[y -1][x].0 {
                            //South
                            '|' | '7' | 'F' => stack.push_back(((x, y-1), len + 1)),
                            _ => {}
                        }
                    }
                }
                '|' => {
                    if y != 0 {
                        match array[y -1][x].0 {
                            //South
                            '|' | '7' | 'F' => stack.push_back(((x, y-1), len + 1)),
                            _ => {}
                        }
                    }
                    if y + 1 < max_col {
                        match array[y+1][x].0 {
                            //North
                            '|' | 'L' | 'J' => stack.push_back(((x, y+1), len + 1)),
                            _ => {}
                        }
                    }
                }
                '-' => {
                    if x + 1 < max_row {
                        match array[y][x+1].0 {
                            //West 
                            '-' | 'J' | '7' => stack.push_back(((x+1, y), len + 1)),
                            _ => {} 
                        }
                    }
                    if x != 0 {
                        match array[y][x -1].0 {
                            //East
                            '-' | 'F' | 'L' => stack.push_back(((x-1, y), len + 1)),
                            _ => {}
                        }
                    }
                }
                'L' => {
                    if y != 0 {
                        match array[y -1][x].0 {
                            //South
                            '|' | '7' | 'F' => stack.push_back(((x, y-1), len + 1)),
                            _ => {}
                        }
                    }
                    if x + 1 < max_row {
                        match array[y][x+1].0 {
                            //West 
                            '-' | 'J' | '7' => stack.push_back(((x+1, y), len + 1)),
                            _ => {} 
                        }
                    }
                }
                'J' => {
                    if x != 0 {
                        match array[y][x -1].0 {
                            //East
                            '-' | 'F' | 'L' => stack.push_back(((x-1, y), len + 1)),
                            _ => {}
                        }
                    }
                    if y != 0 {
                        match array[y -1][x].0 {
                            //South
                            '|' | '7' | 'F' => stack.push_back(((x, y-1), len + 1)),
                            _ => {}
                        }
                    }
                }
                '7' => {
                    if y + 1 < max_col {
                        match array[y+1][x].0 {
                            //North
                            '|' | 'L' | 'J' => stack.push_back(((x, y+1), len + 1)),
                            _ => {}
                        }
                    }
                    if x != 0 {
                        match array[y][x -1].0 {
                            //East
                            '-' | 'F' | 'L' => stack.push_back(((x-1, y), len + 1)),
                            _ => {}
                        }
                    }
                }
                'F' => {
                    if x + 1 < max_row {
                        match array[y][x+1].0 {
                            //West 
                            '-' | 'J' | '7' => stack.push_back(((x+1, y), len + 1)),
                            _ => {} 
                        }
                    }
                    if y + 1 < max_col {
                        match array[y+1][x].0 {
                            //North
                            '|' | 'L' | 'J' => stack.push_back(((x, y+1), len + 1)),
                            _ => {}
                        }
                    }
                }
                _ => {},
            }
        }
    }
}

fn main() {
    let mut array: Vec<Vec<(char, i32)>> = vec![];
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");

    for line in file_content.lines() {
        let line_vec: Vec<(char,i32)> = line.chars().map(|c| (c, -1)).collect();
        array.push(line_vec);
    }
    let start_position: (usize, usize) = find_starting_position(&mut array);
    path_finder(&mut array, start_position, -1);
    let column_len = array.len();
    let row_len = array[0].len();

    let mut result = 0;
    for i in 0..column_len {
        for j in 0..row_len {
            if array[i][j].1 > result {
                result = array[i][j].1;
            }
        }
    }
    
    println!("{}", result);
}
