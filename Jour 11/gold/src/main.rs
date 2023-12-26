use std::fs;

fn cosmic_expansion(map: &mut Vec<Vec<char>>) -> (Vec<i32>, Vec<i32>) {
    let column_len = map.len();
    let row_len = map[0].len();
    let mut expansion_indexes: (Vec<i32>, Vec<i32>) = (vec![], vec![]);

    // Row indexes
    for i in 0..column_len {
        let mut is_empty = true;
        for j in 0..row_len {
            if map[i][j] != '.' {
                is_empty = false;
            }
        }
        if is_empty == true {
            expansion_indexes.1.push(i as i32);
        }
    }

    // Column indexes
    for i in 0..row_len {
        let mut is_empty = true;
        for j in 0..column_len {
            if map[j][i] != '.' {
                is_empty = false;
            }
        }
        if is_empty == true {
            expansion_indexes.0.push(i as i32);
        }
    }
    expansion_indexes

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
    let mut result: i128 = 0;
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");
    let mut map: Vec<Vec<char>> = vec![];
    for line in file_content.lines() {
        let line_vec: Vec<char> = line.chars().collect();
        map.push(line_vec);
    }
    let index_expansion = cosmic_expansion(&mut map);
    let galaxies = convert_to_num(&mut map);
    let galaxies_len = galaxies.len();
    let mut row_size = 0;
    let mut col_size = 0;
    let mut shift = 0;
    for i in 0..galaxies_len {
        for j in 0+shift..galaxies_len {
            if galaxies[i] != galaxies[j] {
            
                for row_index in &index_expansion.0 {
                    if (galaxies[i].1.0 > *row_index as usize && galaxies[j].1.0 < *row_index as usize) || (galaxies[j].1.0 > *row_index as usize && galaxies[i].1.0 < *row_index as usize) {
                        row_size += 999999;
                    }
                }
                for col_index in &index_expansion.1 {
                    if (galaxies[i].1.1 > *col_index as usize && galaxies[j].1.1 < *col_index as usize) || (galaxies[j].1.1 > *col_index as usize && galaxies[i].1.1 < *col_index as usize) {
                        col_size += 999999;
                    }
                }

                result += (galaxies[i].1.0 as i128 - galaxies[j].1.0 as i128).abs() + (galaxies[i].1.1 as i128 - galaxies[j].1.1 as i128).abs() + row_size as i128 + col_size as i128;

                row_size = 0;
                col_size = 0;
            }
        }
        shift += 1;
    }
    println!("{:?}", index_expansion);
    println!("{}", result);
}
