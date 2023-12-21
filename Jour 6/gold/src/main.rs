use std::fs;

fn main() {
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");
    let mut time: i64 = 0;
    let mut distance_record: i64 = 0;
    let mut number_wins: i64 = 0;
    
    for (index, line) in file_content.lines().enumerate() {
        if index == 0 {
            let time_array: Vec<String> = line.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
            let mut time_string: String = String::from("");
            for tim in time_array {
                time_string.push_str(&tim);
            }

            time = time_string.parse::<i64>().unwrap();
        } else {
            let distance_array: Vec<String> = line.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
            let mut distance_record_string: String = String::from("");
            for dist in distance_array {
                distance_record_string.push_str(&dist);
            }
            distance_record = distance_record_string.parse::<i64>().unwrap();
        }
    }
    let mut distance: i64 = 0;
    for time_pressed in 0..time + 1 {
        distance = time_pressed * (time - time_pressed);
        if distance > distance_record {
            number_wins += 1;
        }
    }
    println!("{}", number_wins);
}
