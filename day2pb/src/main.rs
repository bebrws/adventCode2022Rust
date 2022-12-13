use std::io::BufRead;

struct Game {
    my_score: u32,
    other_score: u32,
}

impl Game {
    fn new() -> Game {
        Game {
            my_score: 0,
            other_score: 0,
        }
    }
    fn get_scores(&self) -> (u32, u32) {
        (self.my_score, self.other_score)
    }
    fn process_line(&mut self, line: String) {
        if line.len() != 3 {
            panic!("Invalid line length");
        }

        match line.chars().nth(0).unwrap() {
            'A' => match line.chars().nth(2).unwrap() {
                'X' => {
                    self.my_score += 3 + 0;
                    self.other_score += 1 + 6;
                }
                'Y' => {
                    self.my_score += 1 + 3;
                    self.other_score += 1 + 3;
                }
                'Z' => {
                    self.my_score += 2 + 6;
                    self.other_score += 1 + 0;
                }
                _ => panic!("Invalid character"),
            },
            'B' => match line.chars().nth(2).unwrap() {
                'X' => {
                    self.my_score += 1 + 0;
                    self.other_score += 2 + 6;
                }
                'Y' => {
                    self.my_score += 2 + 3;
                    self.other_score += 2 + 3;
                }
                'Z' => {
                    self.my_score += 3 + 6;
                    self.other_score += 2 + 0;
                }
                _ => panic!("Invalid character"),
            },
            'C' => match line.chars().nth(2).unwrap() {
                'X' => {
                    self.my_score += 2 + 0;
                    self.other_score += 3 + 6;
                }
                'Y' => {
                    self.my_score += 3 + 3;
                    self.other_score += 3 + 3;
                }
                'Z' => {
                    self.my_score += 1 + 6;
                    self.other_score += 3 + 0;
                }
                _ => panic!("Invalid character"),
            },
            _ => panic!("Invalid character"),
        }
    }
}
fn main() {
    let mut bf = advent_tools::get_buffered_file("input");
    let mut game = Game::new();
    for line in bf.lines() {
        let line = line.unwrap();
        game.process_line(line);
    }
    println!("scores (mine, other): {:?}", game.get_scores());
}
