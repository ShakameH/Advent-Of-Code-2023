use std::fs;

fn main() {
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");
    let mut total_result = 0;
    let mut instance_count: Vec<i32> = vec![];
    for _i in 0..11 {
        instance_count.push(0);
    }
    
    for line in file_content.lines() {
        let mut result = 0;
        let parts: Vec<_> = line.split("|").collect();
        let mut winners: Vec<_> = parts[0].split_whitespace().collect();
        let numbers: Vec<_> = parts[1].split_whitespace().collect();

        winners.remove(0);
        winners.remove(0);

        for _i in 0..instance_count[0] + 1 {
            total_result += 1;
            for winner in &winners {
                for number in &numbers {
                    if number == winner {
                        result += 1;
                    }
                }
            }
            while result != 0 {
                instance_count[result] += 1;
                result -= 1;
            }
        }
        instance_count.remove(0);
        instance_count.push(0);
    }
    println!("{}", total_result);
}
