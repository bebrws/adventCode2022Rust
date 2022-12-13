use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let bf = advent_tools::get_buffered_file("input");

    let mut total: u32 = 0;

    let mut group_line = 0;

    let mut items_in_each: Vec<HashSet<u8>> = Vec::new();
    // vec![HashSet::new(), HashSet::new(), HashSet::new()];
    let mut items_count: HashMap<u8, u32> = HashMap::new();

    for line in bf.lines() {
        if group_line == 0 {
            for i in 0..3 {
                items_in_each.push(HashSet::new());
            }
        }

        let line = line.unwrap();
        let bytes = line
            .as_bytes()
            .iter()
            .map(|b| {
                if *b >= b'a' {
                    *b - ('a' as u8) + 1
                } else {
                    *b - ('A' as u8) + 27
                }
            })
            .collect::<Vec<u8>>();

        bytes.iter().for_each(|b| {
            items_in_each[group_line].insert(*b);
        });

        group_line += 1;

        if group_line == 3 {
            group_line = 0;
            for i in 0..3 {
                for k in items_in_each[i].iter() {
                    if let Some(count) = items_count.get_mut(k) {
                        *count += 1;
                        if *count == 3 {
                            total += *k as u32;
                        }
                    } else {
                        items_count.insert(*k, 1);
                    }
                }
            }
            println!("items_count: {:?}", items_count);
            items_in_each = Vec::new();
            items_count = HashMap::new();
        }

        println!("");
    }
    println!("total: {:?}", total);
}
