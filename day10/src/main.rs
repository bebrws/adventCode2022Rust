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

enum Op {
    Noop,
    AddX(i32),
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Op::Noop => write!(f, "Noop"),
            Op::AddX(x) => write!(f, "AddX({})", x),
        }
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Op::Noop => write!(f, "Noop"),
            Op::AddX(x) => write!(f, "AddX({})", x),
        }
    }
}

struct VM {
    // x_qeueue: Vec<i32>,
    x_tmp: i32,
    x: i32,
    addx_cycles_left: usize,
    cycle: usize,
    pc: usize,
    program: Vec<Op>,
}

impl VM {
    fn new(program: Vec<Op>) -> VM {
        VM {
            // x_qeueue: Vec::new(),
            x_tmp: 0,
            x: 1,
            addx_cycles_left: 0,
            cycle: 0,
            pc: 0,
            program: program,
        }
    }

    fn step(&mut self) -> Option<usize> {
        if self.pc >= self.program.len() && self.addx_cycles_left == 0 {
            return None;
        }
        if self.addx_cycles_left == 0 {
            self.x += self.x_tmp;
            self.x_tmp = 0;
        }
        if self.addx_cycles_left > 0 {
            self.addx_cycles_left -= 1;
        } else {
            let op = &self.program[self.pc];
            println!("op: {}", op);
            match op {
                Op::Noop => {}
                Op::AddX(x) => {
                    // self.x_qeueue.push(*x);
                    self.x_tmp = *x;
                    self.addx_cycles_left = 1;
                }
            }
            self.pc += 1;
        }
        self.cycle += 1;
        println!(
            "cycle: {} pc: {} addx_cycles: {} x: {}",
            self.cycle, self.pc, self.addx_cycles_left, self.x
        );
        Some(self.cycle)
    }
}

impl Iterator for VM {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}

fn is_not_space(c: char) -> bool {
    c != ' '
}

fn parse_direction(s: &str) -> IResult<&str, &str> {
    return take_while(is_not_space)(s);
}

fn parse_noop(s: &str) -> IResult<&str, &str> {
    tag("noop")(s)
}

fn parse_addx(s: &str) -> IResult<&str, &str> {
    tag("addx")(s)
}

fn parse_addx_number(s: &str) -> IResult<&str, (&str, &str)> {
    // separated_pair(
    //     alt((tag(" -"), tag(" "))),
    //     digit1,
    //     space0,
    // )(s)
    pair(alt((tag(" -"), tag(" "))), digit1)(s)
}

fn parse_line(s: &str) -> Option<Op> {
    // alt((parse_noop, parse_addx))
    if let Ok((rest, noop_tag)) = parse_noop(s) {
        return Some(Op::Noop);
    } else if let Ok((rest_addx, addx_str)) = parse_addx(s) {
        if let Ok((rest, (is_neg_str, digit_str))) = parse_addx_number(rest_addx) {
            let is_neg = is_neg_str == " -";
            let count_str = format!("{}{}", if is_neg { "-" } else { "" }, digit_str);
            return Some(Op::AddX(count_str.parse::<i32>().unwrap()));
        }
    }
    return None;
}

fn main() {
    let bf = advent_tools::get_buffered_file("input");
    let mut lines: Vec<String> = Vec::new();
    let mut program = Vec::new();
    for line_option in bf.lines() {
        let line = line_option.expect("Couldnt read line");
        if let Some(op) = parse_line(&line) {
            program.push(op);
        }
    }

    let mut vm = VM::new(program);

    let mut sssum = 0;
    loop {
        if let Some(cycle) = vm.step() {
            if cycle >= 20 {
                let x = cycle - 20;
                if x == 0 || x % 40 == 0 {
                    println!("**at 20 or 40 - cycle: {} x: {}", cycle, vm.x);
                    let signal_strength = cycle * vm.x as usize;
                    println!("signal strength: {}", signal_strength);
                    sssum += signal_strength;
                }
            }
            // println!("cycle: {:?}", cycle);
            // println!("  x: {:?}", vm.x);
        } else {
            break;
        }
    }

    println!("signal strength sum: {}", sssum);

    // println!("program: {:?}", program);
}
