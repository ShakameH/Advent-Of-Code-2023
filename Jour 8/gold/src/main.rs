use std::fs;
use std::collections::HashMap;

fn pgcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        pgcd(b, a % b)
    }
}

fn ppcm(a: i128, b: i128) -> i128 {
    (a * b) / pgcd(a, b)
}

fn find_multiple(array: &[i128]) -> i128 {
    let mut result = array[0];
    for &number in array.iter().skip(1) {
        result = ppcm(result, number);
    }
    result
}

fn main() {
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut path: &str = "";
    for (index, line) in file_content.lines().enumerate() {
        if index == 0 {
            path = line;
        } else if index != 0 && index != 1 {
            let modified_string: String = line.replace(&[',','(',')'], "");
            let mut parsed_line: Vec<String> = modified_string.split(" ").map(String::from).collect();
            parsed_line.remove(1);
            map.insert(parsed_line[0].clone(), (parsed_line[1].clone(), parsed_line[2].clone()));
        }
    }
    let mut position_array: Vec<&str> = vec![];
    let mut iterations_array: Vec<i128> = vec![];
    let mut result: i128 = 1;
    let mut need_to_break: bool = false;
    let mut number_end_in_array: usize = 0;

    for (position, _) in map.iter().filter(|&(l,_)| l.ends_with("A")) {
            position_array.push(position);
            iterations_array.push(0);
    }
    
    for i in 0..position_array.len() {
        need_to_break = false;
        loop {
            for c in path.chars() {
                match c {
                    'L' => {
                        if let Some((path_left, _)) = map.get(position_array[i]) {
                            position_array[i] = path_left;
                        }
                    },
                    'R' => {
                        if let Some((_, path_right)) = map.get(position_array[i]) {
                            position_array[i] = path_right;
                        }
                    },
                    _ => {},
                }
                iterations_array[i] += 1;
                if position_array[i].ends_with("Z") {
                    println!("{:?}", position_array);
                    need_to_break = true;
                    break;
                }
            }
            if need_to_break == true {
                break;
            }
        }
    }
    result = find_multiple(&iterations_array);
    println!("{:?}", result);
}
