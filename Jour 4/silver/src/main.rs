use std::fs;

fn main() {
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");
    let mut total_result = 0;
    for line in file_content.lines() {
        let mut result = 0;
        let parts: Vec<_> = line.split("|").collect();
        let mut winners: Vec<_> = parts[0].split_whitespace().collect();
        let numbers: Vec<_> = parts[1].split_whitespace().collect();
        winners.remove(0);
        winners.remove(0);
        for winner in &winners {
            for number in &numbers {
                if number == winner {
                    match result {
                        0 => result = 1,
                        _ => result *= 2,
                    }
                }
            }
        }
    total_result += result;
    }
    println!("{}", total_result);
}
