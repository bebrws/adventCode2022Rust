use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let bf = advent_tools::get_buffered_file("input");

    let mut overlapping: u32 = 0;
    for line in bf.lines() {
        let line = line.unwrap();
        let lsplit = line.split(",").collect::<Vec<&str>>();
        let sec1 = lsplit[0].split("-").collect::<Vec<&str>>();
        let sec2 = lsplit[1].split("-").collect::<Vec<&str>>();

        let sec1Left = sec1[0].parse::<u32>().unwrap();
        let sec1Right = sec1[1].parse::<u32>().unwrap();

        let sec2Left = sec2[0].parse::<u32>().unwrap();
        let sec2Right = sec2[1].parse::<u32>().unwrap();

        if (sec1Left <= sec2Left && sec1Right >= sec2Right)
            || (sec2Left <= sec1Left && sec2Right >= sec1Right)
        {
            println!(
                "{}-{} and {}-{} are overlapping",
                sec1Left, sec1Right, sec2Left, sec2Right
            );
            overlapping += 1;
        }
        println!("");
    }
    println!("overlapping: {:?}", overlapping);
}
