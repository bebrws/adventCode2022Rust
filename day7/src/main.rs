use std::io::BufRead;

fn main() {
    let bf = advent_tools::get_buffered_file("input");

    for line in bf.lines() {
        let line = line.unwrap();
        println!("line: {:?}", line);
    }
}
