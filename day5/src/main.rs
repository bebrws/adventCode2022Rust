use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let bf = advent_tools::get_buffered_file("input");

    let mut overlapping: u32 = 0;
    for line in bf.lines() {
        let line = line.unwrap();
        println!("");
    }
    // println!("overlapping: {:?}", overlapping);
}
