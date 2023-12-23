use std::fs;
use std::collections::HashMap;

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
    let mut position = "AAA";
    let mut result: i32 = 0;
    let mut need_to_break: bool = false;
    loop {
        for c in path.chars() {
            match c {
                'L' => {
                    if let Some((path_left, _)) = map.get(position) {
                        position = path_left;
                    }
                },
                'R' => {
                    if let Some((_, path_right)) = map.get(position) {
                        position = path_right;
                    }
                },
                _ => {},
            }
            result += 1;
            if position == "ZZZ" {
                need_to_break = true;
                break;
            }
        }
        if need_to_break == true {
            break;
        }
    }
    
    println!("{}", result);
}
