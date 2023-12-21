use std::fs;

fn main() {
    let file_content = fs::read_to_string("../input.txt").expect("Error reading file");

    for i in (0..20).step_by(2)
    {
        let state: Vec<i64> = Vec::new();
        let mut seeds: Vec<Vec<i64>> = vec![state.clone(); 8];
        let mut index_vec: usize = 0;
        let mut previous_seeds: Vec<i64> = vec![];
        
        for (index, line) in file_content.lines().enumerate() {
            if index == 0 {
                let seed_range: Vec<i64> = line.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
                for seed in seed_range[i]..(seed_range[i]+seed_range[index+1]) {
                    seeds[0].push(seed);
                }
            }
             else {
                match line {
                    "seed-to-soil map:" | "soil-to-fertilizer map:" |
                    "fertilizer-to-water map:" | "water-to-light map:" |
                    "light-to-temperature map:" | "temperature-to-humidity map:" |
                    "humidity-to-location map:" => index_vec += 1,
                    "" => {
                        if index != 1 {
                            previous_seeds = seeds[index_vec - 1].clone();
                            for seed in &previous_seeds {
                                seeds[index_vec].push(*seed);
                            }
                            seeds[index_vec-1].clear();
                        }                    
                    },
                    _ => {
                        let maps: Vec<i64> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                        previous_seeds.clear();
                        previous_seeds.extend_from_slice(&seeds[index_vec - 1]);
                        for seed in &previous_seeds {
                            if seed >= &maps[1] && seed <= &(maps[1] + maps[2] - 1) {
                                seeds[index_vec].push(maps[0] - maps[1] + seed);
                                if let Some(index) = seeds[index_vec - 1].iter().position(|value| value == seed) {
                                    seeds[index_vec - 1].swap_remove(index);
                                }
                            }
                        }         
                    }
                }
            }
        }
        previous_seeds = seeds[index_vec - 1].clone();
        for seed in &previous_seeds {
            seeds[index_vec].push(*seed);
        }
        let min_value = seeds[7].iter().min();
        match min_value {
            Some(min) => println!("{}", min),
            None => println!("Vector empty"),
        }
    }
    
}
