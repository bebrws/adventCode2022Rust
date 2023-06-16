use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, space0},
    combinator::map_res,
    error::ParseError,
    sequence::preceded,
    sequence::separated_pair,
    sequence::{self, delimited, pair},
    IResult,
};

use nom::bytes::streaming::take_while;

use std::collections::HashMap;
use std::io::BufRead;

fn is_space(c: char) -> bool {
    c == ' '
}

fn parseContainer(s: &str) -> IResult<&str, &str> {
    preceded(take_while(is_space), delimited(tag("["), alpha1, tag("]")))(s)
}

fn parseRow(line: &str) {
    let mut to_parse = line;
    let mut row: Vec<&str> = Vec::new();
    while let Ok((rest, c)) = parseContainer(to_parse) {
        row.push(c);
        to_parse = rest;
    }
    println!("row: {:?}", row);
}

fn main() {
    let ship: Vec<Vec<char>> = Vec::new();

    let bf = advent_tools::get_buffered_file("input");

    let mut overlapping: u32 = 0;
    for line in bf.lines() {
        let line = line.unwrap();
        // let x = parseContainer(line.as_str());
        // println!("{:?}", parseContainer(line.as_str()));
        parseRow(line.as_str());
    }
    // println!("overlapping: {:?}", overlapping);
}
