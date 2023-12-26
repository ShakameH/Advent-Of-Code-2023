use std::fs;

fn cosmic_expansion(map: &mut Vec<Vec<char>>) {
    let mut column_len = map.len();
    let mut row_len = map[0].len();
    let mut i = 0;
    let mut has_extended: bool = false;
    // Row
    while i < column_len {
        let mut is_empty: bool = true;
        for j in 0..row_len {
            if map[i][j] != '.' {
                is_empty = false;
            }
        }
        if has_extended == true {
            has_extended = false;
            i += 1;
            continue;
        }
        if is_empty == true {
            let new_line: Vec<char> = vec!['.'; column_len];
            map.insert(i, new_line);
            column_len += 1;
            has_extended = true;
        }   
        i += 1;
    }

    // Column
    i = 0;
    while i < row_len {
        let mut is_empty: bool = true;
        if has_extended == true {
            has_extended = false;
            i += 1;
            continue;
        }
        for j in 0..column_len {
            if map[j][i] == '#' {
                is_empty = false;
                break;
            }
        }
        if is_empty == true {
            for k in 0..column_len {
                map[k].insert(i, '.');
            }
            row_len += 1;
            has_extended = true;
        }
        i += 1;
    }
}

fn convert_to_num(map: &mut Vec<Vec<char>>) -> Vec<(char, (usize, usize))> {
    let mut galaxies: Vec<(char, (usize, usize))> = vec![];
    let mut galaxy_name = 1;
    let column_len = map.len();
    let row_len = map[0].len();

    for i in 0..column_len {
        for j in 0..row_len {
            if map[i][j] != '.' {
                map[i][j] = std::char::from_u32(galaxy_name as u32).unwrap();
                galaxies.push((map[i][j], (j, i)));
                galaxy_name += 1;
            }
        }
    }
    galaxies
}

fn main() {
    let mut result = 0;
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");
    let mut map: Vec<Vec<char>> = vec![];
    for line in file_content.lines() {
        let line_vec: Vec<char> = line.chars().collect();
        map.push(line_vec);
    }
    cosmic_expansion(&mut map);
    let galaxies = convert_to_num(&mut map);
    let galaxies_len = galaxies.len();
    let mut shift = 0;
    for i in 0..galaxies_len {
        for j in 0+shift..galaxies_len {
            if galaxies[i] != galaxies[j] {
                result += (galaxies[i].1.0 as i32 - galaxies[j].1.0 as i32).abs() + (galaxies[i].1.1 as i32 - galaxies[j].1.1 as i32).abs();
            }
        }
        shift += 1;
    }
    println!("{}", result);
}
