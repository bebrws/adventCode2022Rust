use std::io::BufReader;
use std::fs::File;

pub fn get_buffered_file(filename: &str) -> BufReader<File> {
    let file = File::open(filename).expect("File not found");
    BufReader::new(file)
}