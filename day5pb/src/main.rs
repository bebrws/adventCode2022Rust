use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, space0},
    combinator::map,
    combinator::map_res,
    error::ParseError,
    sequence::preceded,
    sequence::separated_pair,
    sequence::{self, delimited, pair},
    IResult,
};

use nom::bytes::streaming::take_while;
use std::io::BufRead;

#[derive(PartialEq, Eq, Debug)]
enum Container {
    Empty,
    Container(String),
}

fn is_space(c: char) -> bool {
    c == ' '
}

fn parse_empty_container(s: &str) -> IResult<&str, Container> {
    return map(tag("    "), |empty_str| Container::Empty)(s);
}

fn parse_full_container(s: &str) -> IResult<&str, Container> {
    return map(
        preceded(take_while(is_space), delimited(tag("["), alpha1, tag("]"))),
        |full_str: &str| Container::Container(full_str.to_string()),
    )(s);
}

fn parse_container(s: &str) -> IResult<&str, Container> {
    alt((parse_empty_container, parse_full_container))(s)
}

fn parse_instruction(s: &str) -> Option<(u32, u32, u32)> {
    let mut move_cmd = 0;
    let move_parse: IResult<&str, &str> = preceded(tag("move "), digit1)(s);
    if let Ok((rest, c)) = move_parse {
        move_cmd = c.parse::<u32>().unwrap();
        let from_parse: IResult<&str, &str> = preceded(tag(" from "), digit1)(rest);
        if let Ok((rest, c)) = from_parse {
            let from_cmd = c.parse::<u32>().unwrap();
            let to_parse: IResult<&str, &str> = preceded(tag(" to "), digit1)(rest);
            if let Ok((rest, c)) = to_parse {
                let to_cmd = c.parse::<u32>().unwrap();
                return Some((move_cmd, from_cmd, to_cmd));
            } else {
                return None;
            }
        } else {
            return None;
        }
    } else {
        return None;
    }
}

fn parse_container_row(line: String) -> Option<Vec<Container>> {
    let mut to_parse = line.as_str();
    let mut row: Vec<Container> = Vec::new();
    while let Ok((rest, c)) = parse_container(to_parse) {
        row.push(c);
        to_parse = rest;
    }
    if row.len() == 0 {
        return None;
    }
    return Some(row);
}

fn main() {
    let mut ship_rows: Vec<Vec<Container>> = Vec::new();
    let mut instructions: Vec<(u32, u32, u32)> = Vec::new();

    let bf = advent_tools::get_buffered_file("input");

    let mut overlapping: u32 = 0;
    for line in bf.lines() {
        let line = line.unwrap();
        // let x = parseContainer(line.as_str());
        // println!("{:?}", parseContainer(line.as_str()));
        println!("line: {:?}", line);
        if let Some(row) = parse_container_row(line.clone()) {
            ship_rows.push(row);
        }
        if let Some(instruction) = parse_instruction(line.clone().as_str()) {
            instructions.push(instruction);
        }
    }
    println!("ship_rows: {:?}", ship_rows);
    println!("instructions: {:?}", instructions);

    let num_columns = ship_rows.iter().map(|row| row.len()).max().unwrap();

    let mut ship: Vec<Vec<String>> = Vec::new();
    (0..num_columns).for_each(|_| ship.push(Vec::new()));
    for row in ship_rows.iter().rev() {
        for (col_num, container) in row.iter().enumerate() {
            if let Container::Container(container) = container {
                ship[col_num].push(container.clone());
            }
        }
    }
    println!("ship: {:?}", ship);

    for (move_cmd, from_cmd, to_cmd) in instructions.iter() {
        let from_cmd = *from_cmd as usize;
        let to_cmd = *to_cmd as usize;
        let move_cmd = *move_cmd as usize;
        let cur_col = &mut ship[from_cmd - 1];
        let new_col = &cur_col[0..cur_col.len() - move_cmd].to_vec();
        println!("new_col: {:?}", new_col);
        let move_col = &cur_col[cur_col.len() - move_cmd..cur_col.len()].to_vec();
        println!("move_col: {:?}", move_col);
        ship[from_cmd - 1] = new_col.clone();
        ship[to_cmd - 1].extend(move_col.clone());
    }

    println!("ship: {:?}", ship);

    println!("Answer:");
    for row in ship.iter() {
        print!("{}", row[row.len() - 1]);
    }
}
