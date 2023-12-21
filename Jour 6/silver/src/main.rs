use std::fs;

fn main() {
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");
    let mut time_array: Vec<i32> = vec![0, 0, 0, 0];
    let mut record_array: Vec<i32> = vec![0, 0, 0, 0];
    let mut number_win_array: Vec<i32> = vec![0, 0, 0, 0];
    
    for (index, line) in file_content.lines().enumerate() {
        if index == 0 {
            time_array = line.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
        } else {
            record_array = line.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
        }
    }
    let mut distance: i32 = 0;
    for race_number in 0..time_array.len() {
        for time_pressed in 0..time_array[race_number] + 1 {
            distance = time_pressed * (time_array[race_number] - time_pressed);
            if distance > record_array[race_number] {
                number_win_array[race_number] += 1;
            }
        }
    }
    let mut result: i32 = 1;
    for win_race in number_win_array {
        result *= win_race;
    }
    println!("{}", result);
}
