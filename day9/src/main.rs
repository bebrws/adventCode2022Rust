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

use std::cmp;
use std::cmp::Ordering;
use std::num;
use std::{collections::BinaryHeap, collections::HashMap, fmt, fmt::Formatter, io::BufRead};

struct Bridge {
    hx: i64,
    hy: i64,
    tx: i64,
    ty: i64,
    all_positions: HashMap<(i64, i64), i64>,
}

impl Bridge {
    fn new() -> Bridge {
        let mut ap = HashMap::new();
        ap.insert((0, 0), 1);
        Bridge {
            hx: 0,
            hy: 0,
            tx: 0,
            ty: 0,
            all_positions: ap,
        }
    }

    fn move_dir_count(&mut self, dir: &str, count: i64) {
        for i in 0..count {
            match dir {
                "U" => {
                    self.hy += 1;
                }
                "D" => {
                    self.hy -= 1;
                }
                "L" => {
                    self.hx -= 1;
                }
                "R" => {
                    self.hx += 1;
                }
                _ => {
                    panic!("Unknown direction: {:?}", dir);
                }
            }

            let tail_dist = cmp::max((self.hx - self.tx).abs(), (self.hy - self.ty).abs());
            if tail_dist > 1 {
                let dy = self.hy - self.ty;
                let dx = self.hx - self.tx;
                if dy != 0 {
                    self.ty += dy / dy.abs();
                }
                if dx != 0 {
                    self.tx += dx / dx.abs();
                }
                self.all_positions.insert((self.tx, self.ty), 1);
            }
            println!("Bridge: {:?}", self);
        }
    }

    fn get_num_tail_positions(&self) -> usize {
        return self.all_positions.len();
    }
}

impl fmt::Debug for Bridge {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Head {{ hx: {}, hy: {} }} Tail {{ tx: {}, ty: {} }}",
            self.hx, self.hy, self.tx, self.ty
        )
    }
}

// √((x_2-x_1)²+(y_2-y_1)²)
// 2,2 , 1,1 dist = 2
// 2,2 , 1,2 dist = 1
fn is_not_space(c: char) -> bool {
    c != ' '
}

fn parse_direction(s: &str) -> IResult<&str, &str> {
    return take_while(is_not_space)(s);
}

fn parse_line(s: &str) -> Option<(&str, i64)> {
    if let Ok((rest, (dir, count))) = separated_pair(parse_direction, tag(" "), digit1)(s) {
        return Some((dir, count.parse::<i64>().unwrap()));
    }
    return None;
}

fn main() {
    let bf = advent_tools::get_buffered_file("input");
    let mut lines = Vec::new();
    for line_option in bf.lines() {
        let line = line_option.expect("Couldnt read line");
        lines.push(line.clone());
    }

    let mut bridge = Bridge::new();

    for line in lines {
        println!("line: {:?}", line);
        if let Some((dir, count)) = parse_line(&line) {
            println!("  dir: {:?}, count: {:?}", dir, count);
            bridge.move_dir_count(dir, count);
        }
    }

    println!("Num tail positions: {:?}", bridge.get_num_tail_positions());
}
