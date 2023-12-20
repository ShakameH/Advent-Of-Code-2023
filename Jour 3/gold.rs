use std::fs;

#[derive(Clone)]
struct GearNumber {
    name: String,
    index: usize,
    length: usize
}

#[derive(Clone)]
struct Gear {
    index: usize,
    number_gear: usize,
    gear_number_array: Vec<usize>
}

fn modify_gear_number(vec: &mut Vec<GearNumber>, new_element: String) {
    if let Some(gear) = vec.last_mut() {
        gear.name.push_str(&new_element);
        gear.length += 1;
    }
}

fn add_gear_number(gear_array: &mut Vec<Gear>, number_name: String) -> usize {
    let mut result: usize = 0;
    if let Some(gear) = gear_array.last_mut() {
        gear.number_gear += 1;
        gear.gear_number_array.push(number_name.parse().unwrap());
        if gear.number_gear == 2 {
            result += gear.gear_number_array[0] * gear.gear_number_array[1];
        }
    }
    result
}

fn add_previous_number(gear_array: &mut Vec<Gear>, gear_previous_array: &mut Vec<GearNumber>) -> usize {
    let mut result: usize = 0;
    if let Some(last_gear) = gear_array.last().cloned() {
        for gear_number in gear_previous_array {
            if last_gear.index - 1 <= gear_number.index + gear_number.length - 1 && gear_number.index <= last_gear.index + 1 {
                result += add_gear_number(gear_array, gear_number.name.clone());
            }
        }
    }
    result
}
fn add_bottom_gear(previous_gear_array: &mut Vec<Gear>, current_gear_number_array: &mut Vec<GearNumber>) -> usize {
    let mut result: usize = 0;
    for gear in previous_gear_array.iter_mut() {
        for gear_number in current_gear_number_array.iter_mut() {
            if gear.index >= gear_number.index.saturating_sub(1) && gear.index <= gear_number.index + gear_number.length {
                gear.number_gear += 1;
                gear.gear_number_array.push(gear_number.name.parse().unwrap());
                if gear.number_gear == 2 {
                    result += gear.gear_number_array[0] * gear.gear_number_array[1];
                }
            }
        }
    }
    result
}


fn main() {

    let mut result: usize = 0;
    let file_content = fs::read_to_string("./input.txt").expect("Error reading file");
    
    let mut gear_number_array_previous: Vec<GearNumber> = vec![];
    let mut gear_number_array_current: Vec<GearNumber> = vec![];
    let mut gear_array_previous: Vec<Gear> = vec![];
    let mut gear_array_current: Vec<Gear> = vec![];
    let mut is_previous_num: bool = false;
    let mut is_previous_gear: bool = false;
    let mut need_to_store: bool = false;

    for line in file_content.lines() {
        for (i_car, car) in line.chars().enumerate() {
            if car == '*' {
                is_previous_gear = true;
                let new_gear = Gear {
                    index: i_car,
                    number_gear: 0,
                    gear_number_array: vec![]
                };
                gear_array_current.push(new_gear);
                result += add_previous_number(&mut gear_array_current, &mut gear_number_array_previous); 
                if is_previous_num {
                    is_previous_num = false;
                    if let Some(last) = gear_number_array_current.last() {
                        result += add_gear_number(&mut gear_array_current, last.name.clone())
                    }
                }
            }
            else if car.is_digit(10) {
                if is_previous_num == false {
                    let current_number = GearNumber {
                        name: String::from(car),
                        index: i_car,
                        length: 1
                    };
                    gear_number_array_current.push(current_number);
                    is_previous_num = true;
                    if is_previous_gear {
                        need_to_store = true;
                    }
                }
                else {
                    modify_gear_number(&mut gear_number_array_current, String::from(car));
                }
                continue;
            }
            else {
                if is_previous_num {
                    is_previous_num = false;
                } else if is_previous_gear {
                    is_previous_gear = false;
                }
                if need_to_store {
                    if let Some(last) = gear_number_array_current.last() {
                        result += add_gear_number(&mut gear_array_current, last.name.clone())
                    }
                    need_to_store = false;
                }
            }
        }
        result += add_bottom_gear(&mut gear_array_previous, &mut gear_number_array_current);

        gear_number_array_previous.clear();
        gear_number_array_previous = gear_number_array_current.clone();
        gear_number_array_current.clear();

        gear_array_previous.clear();
        gear_array_previous = gear_array_current.clone();
        gear_array_current.clear();
    }
    println!("{}", result);
}