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
    crt: CRT,
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
            crt: CRT::new(),
            x_tmp: 0,
            x: 1,
            addx_cycles_left: 0,
            cycle: 0,
            pc: 0,
            program: program,
        }
    }

    fn step(&mut self) -> Option<CRT> {
        if self.pc >= self.program.len() && self.addx_cycles_left == 0 {
            return None;
        }
        if self.addx_cycles_left == 0 {
            self.x += self.x_tmp;
            self.x_tmp = 0;
            if self.x >= 0 {
                self.crt.set_sprite_x(self.x as usize);
            } else {
                panic!("\n\n\n\n X LESS THAN 0 \n\n\n\n");
            }
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
        Some(self.crt)
    }
}

impl Iterator for VM {
    type Item = CRT;

    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}

struct Sprite {
    x: usize,
}

impl Sprite {
    fn is_pixel_on(&self, crt_x: usize) -> bool {
        if crt_x - 1 == self.x || crt_x == self.x || crt_x + 1 == self.x {
            return true;
        }
        false
    }

    fn set_x(&mut self, x: usize) {
        self.x = x;
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Pixel {
    On,
    Off,
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Pixel::On => write!(f, "#"),
            Pixel::Off => write!(f, "."),
        }
    }
}

struct CRT {
    lines: Vec<Vec<Pixel>>,
    sprite: Sprite,
    pos: usize,
}

impl CRT {
    fn new() -> CRT {
        CRT {
            lines: vec![vec![Pixel::Off; 40]; 6],
            sprite: Sprite { x: 1 },
            pos: 0,
        }
    }

    fn draw(&mut self) {
        let line = self.pos / 40;
        let line_offset = self.pos % 40;
        if self.sprite.is_pixel_on(self.pos) {
            self.lines[line][line_offset] = Pixel::On;
        } else {
            self.lines[line][line_offset] = Pixel::Off;
        }
    }

    fn set_sprite_x(&mut self, sprite_x: usize) {
        self.sprite.set_x(sprite_x);
    }
}

impl fmt::Display for CRT {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        println!("CRT:");
        for line in &self.lines {
            for pixel in line {
                write!(f, "{}", pixel)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
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
        } else {
            break;
        }
    }

    println!("{}", crt);
}
