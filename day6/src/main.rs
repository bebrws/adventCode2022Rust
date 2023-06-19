use std::collections::HashMap;

use std::io::BufRead;

fn dec_and_remove(map: &mut HashMap<char, i32>, c: char) {
    let val = map.get_mut(&c);
    if let Some(opt_val) = val {
        *opt_val -= 1;
        if *opt_val == 0 {
            map.remove(&c);
        }
    }
}

fn main() {
    let mut last_four_map = HashMap::new();

    let bf = advent_tools::get_buffered_file("input");

    let mut overlapping: u32 = 0;
    for line in bf.lines() {
        let line = line.unwrap();
        println!("line: {:?}", line);
        let line_chars: Vec<char> = line.chars().collect();
        let mut marker = 0;
        for (i, c) in line.chars().enumerate() {
            println!("c : {:?}", c);
            last_four_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
            println!("last_four_map {:?}", last_four_map);

            if last_four_map.len() == 14 && last_four_map.values().all(|&count| count <= 1) {
                marker = i;
                break;
            }
            // let total_values = last_four_map.values().fold(0, |acc, v| acc + v);
            // println!("total_values : {:?}", total_values);
            if i >= 13 {
                println!("i : {:?}", i);
                println!("i-4 : {:?}", i - 13);
                println!("line_chars[i - 13] : {:?}", line_chars[i - 13]);
                dec_and_remove(&mut last_four_map, line_chars[i - 13]);
            }
        }
        println!("marker {:?}", marker + 1);
    }
}
