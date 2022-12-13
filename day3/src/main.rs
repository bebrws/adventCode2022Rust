use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let bf = advent_tools::get_buffered_file("input");

    let mut sum: u32 = 0;
    for line in bf.lines() {
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
        let (comp1, comp2) = bytes.split_at(bytes.len() / 2);

        let mut items_to_count = HashMap::new();

        comp1.iter().for_each(|b| {
            items_to_count.insert(*b, 1);
        });
        comp2.iter().for_each(|b| {
            if let Some(count) = items_to_count.get_mut(b) {
                *count += 1;
                if *count == 2 {
                    sum += *b as u32;
                    println!("item: {:?}", *b);
                }
            }
        });
        println!("");
    }
    println!("sum: {:?}", sum);
}
