use std::fs;
use std::collections::HashMap;

fn bubble_sort(arr: &mut Vec<(&str, i64)>) {
    let len = arr.len();
    #[allow(unused_assignments)]
    let mut need_to_sort: bool = false;

    loop {
        need_to_sort = false;
        for i in 0..len-1 {
            if compare_values(arr[i].0, arr[i+1].0) == true {
                arr.swap(i, i+1);
                need_to_sort = true;
            }
        }
        if need_to_sort == false {
            break;
        }
    }
}

fn letter_match(letter: char) -> u32 {
    match letter {
        'J' => 1, '2' => 2, '3' => 3, '4' => 4, '5' => 5,
        '6' => 6, '7' => 7, '8' => 8, '9' => 9,
        'T' => 10, 'Q' => 11, 
        'K' => 12, 'A' => 13, _ => 0
    }
}

fn get_rank(hand: &str) -> i64 {
    let mut doublons: HashMap<char, usize> = HashMap::new();
    let mut rank: i64 = 0;
    let mut number_j: i64 = 0;
    for car in hand.chars() {
        *doublons.entry(car).or_insert(0) += 1;
    }

    if let Some(number) = doublons.get(&'J') {
        number_j = *number as i64;
    }

    if let Some((letter, count)) = doublons
        .iter()
        .filter(|&(l, _)| l != &'J')
        .max_by_key(|&(_, number)| number) {
        match count {
            1 => {
                match number_j {
                    0 => rank = 1,
                    1 => rank = 2,
                    2 => rank = 4,
                    3 => rank = 6,
                    4 => rank = 7,
                    _ => rank = 0,
                }
            },
            2 => {
                if let Some((_letter_2, count_2)) = doublons
                    .iter()
                    .filter(|&(l, _)| l != letter && l != &'J')
                    .max_by_key(|&(_, number)| number)
                {
                    match count_2 {
                        2 => {
                            match number_j {
                                0 => rank = 3,
                                1 => rank = 5,
                                _ => rank = 0,
                            }
                        }
                        _ => {
                            match number_j {
                                0 => rank = 2,
                                1 => rank = 4,
                                2 => rank = 6,
                                3 => rank = 7,
                                _ => rank = 0,
                            }
                        }
                    }
                } else {
                    rank = 7;
                }
            },
            3 => {
                if let Some((_letter_2, count_2)) = doublons
                    .iter()
                    .filter(|&(l, _)| l != letter && l != &'J')
                    .max_by_key(|&(_, number)| number)
                {
                    match count_2 {
                        2 => rank = 5,
                        _ => {
                            match number_j {
                                0 => rank = 4,
                                1 => rank = 6,
                                2 => rank = 7,
                                _ => rank = 0,
                            }
                        }
                    }
                } else {
                    rank = 7;
                }
            },
            4 => {
                match number_j {
                    0 => rank = 6,
                    1 => rank = 7,
                    _ => rank = 0,
                }
            }
            5 => rank = 7,
            _ => rank = 0
        }
    } else {
        rank = 7;
    }
    rank
}

fn compare_values(hand1: &str, hand2: &str) -> bool {
    let rank1: i64 = get_rank(hand1);
    let rank2: i64 = get_rank(hand2);
    let mut need_to_switch = false;
    if rank1 > rank2 {
        need_to_switch = true
    } else if rank1 == rank2 {
        let strength = compare_char_by_char(hand1, hand2);
        if strength == true {
            need_to_switch = true;
        } 
    }
    need_to_switch
}

fn compare_char_by_char(hand1: &str, hand2: &str) -> bool {
    let mut iterator_hand1 = hand1.chars();
    let mut iterator_hand2 = hand2.chars();
    loop {
        match (iterator_hand1.next(), iterator_hand2.next()) {
            (Some(car1), Some(car2)) => {
                if letter_match(car1) > letter_match(car2) {
                    return true;
                } else if letter_match(car1) < letter_match(car2) {
                    return false;
                }
            }
            (None, None) => {
                return false;
            }
            _ => {
                return false;
            }
        }
    }
}

fn main() {
    let mut data_array: Vec<(&str, i64)> = vec![];
    let mut result: i64 = 0;
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");
    for line in file_content.lines() {
        let mut_parts: Vec<_> = line.split(" ").collect();
        data_array.push((mut_parts[0], mut_parts[1].parse::<i64>().unwrap()));
        
    }
    bubble_sort(&mut data_array);
    for (index, element) in data_array.iter().enumerate() {
        result += element.1 * (index as i64 + 1);
    }
    println!("{}", result);
}
