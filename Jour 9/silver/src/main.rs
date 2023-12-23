use std::fs;

fn fill_array(tree: &mut Vec<Vec<i64>>) {
    let mut index: usize = 0;
    let mut can_break = false;
    while can_break == false {
        can_break = true;
        let mut differences: Vec<i64> = vec![];
        for i in 0.. tree[index].len() - 1 {
            let difference = tree[index][i+1] - tree[index][i];
            differences.push(difference);
            if difference != 0 {
                can_break = false;
            }
        }
        tree.push(differences);
        index += 1;
    }
}

fn main() {
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");
    let mut result: i64 = 0;
    for line in file_content.lines() {
        let values: Vec<i64> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
        let mut tree: Vec<Vec<i64>> = vec![];
        tree.push(values);
        fill_array(&mut tree);
        let index_tree: usize = tree.len() - 1;
        for i in (0..index_tree).rev() {
            if i == index_tree {
                tree[i].push(0);
            } else {
                let next_value = tree[i + 1].last().unwrap().clone();
                let previous_value = tree[i].last().unwrap().clone();
                tree[i].push(next_value + previous_value);
            }
        }
        result += tree[0].last().unwrap();
    }
    println!("{}", result);
}
