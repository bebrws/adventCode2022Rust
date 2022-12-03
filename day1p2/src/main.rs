use std::io::BufRead;

fn main() {
    let bf = advent_tools::get_buffered_file("input");

    let mut elfs: Vec<u32> = Vec::new();
    let mut cals: u32 = 0;

    for line in bf.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            elfs.push(cals);
            cals = 0;
        } else {
            let line_int = line.parse::<u32>().unwrap();
            cals += line_int;
        }
        println!("{}", line);
    }
    elfs.push(cals);

    elfs.sort();

    println!("elfs: {:?}", elfs);
    println!(
        "total: {:?}",
        elfs[elfs.len() - 1] + elfs[elfs.len() - 2] + elfs[elfs.len() - 3]
    );
}
