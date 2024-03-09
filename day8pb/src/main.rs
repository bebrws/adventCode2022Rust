use std::cmp::Ordering;
use std::os::macos::raw;
use std::{collections::BinaryHeap, collections::HashMap, fmt, fmt::Formatter, io::BufRead};

// #[derive(PartialEq, Eq, Clone)]
#[derive(Eq, Clone)]
struct Tree {
    row: usize,
    col: usize,
    score: usize,
}

impl Tree {
    fn new(row: usize, col: usize, score: usize) -> Tree {
        Tree {
            row: row,
            col: col,
            score: score,
        }
    }

    fn get_location_string(&self) -> String {
        format!("{}:{}", self.col + 1, self.row + 1)
    }

    fn get_row(&self) -> usize {
        self.row
    }

    fn get_col(&self) -> usize {
        self.col
    }

    fn get_score(&self) -> usize {
        self.score
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tree {{ col: {}, row: {}, score: {} }}",
            self.col + 1,
            self.row + 1,
            self.score
        )
    }
}

fn main() {
    let bf = advent_tools::get_buffered_file("input");
    let mut lines = Vec::new();
    for line_option in bf.lines() {
        let line = line_option.expect("Couldnt read line");
        lines.push(line.clone());
        // println!("line: {:?}", line);
    }

    let mut tree_map = HashMap::new();
    let lines_row_count = lines.len();
    let lines_col_count = lines[0].len();
    for r in 0..lines_row_count {
        for c in 0..lines_col_count {
            tree_map.insert(format!("{}:{}", c + 1, r + 1), Vec::new());
        }
    }

    println!("\n\nLooking left to right from top to bottom");
    let mut row_index = 0;
    for line in lines.iter() {
        println!("\n\nNew row: {row_index}");
        for col_start in 0..lines_col_count {
            let cur_tree_height = line
                .chars()
                .skip(col_start)
                .take(1)
                .next()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();
            let mut tree_heights = Vec::new();
            let mut tree_count = 0;
            println!(
                "\ncol_start: {} cur_tree_height: {cur_tree_height}",
                col_start
            );
            line[0..col_start]
                .chars()
                .enumerate()
                .for_each(|(col_index, c)| {
                    let height = c.to_string().parse::<usize>().unwrap();
                    tree_heights.push(height);
                });

            let mut max_height = 0;
            for (col_index, height) in tree_heights.iter().enumerate().rev() {
                println!("row_index: {row_index} col_index: {col_index}  col_start - 1: {}  height: {height}", col_start - 1);
                tree_count += 1;
                if *height >= cur_tree_height {
                    println!("  adding 1");
                    break;
                }
            }
            let tree = Tree::new(row_index, col_start, tree_count);
            println!("Adding tree: {:?}", tree);
            tree_map
                .get_mut(&tree.get_location_string())
                .unwrap()
                .push(tree.get_score());
        }
        row_index += 1;
    }

    println!("\n\nLooking right to left from top to bottom");
    let mut row_index = 0;
    for line in lines.iter() {
        println!("\n\nNew row: {row_index}");
        for col_start in 0..lines_col_count {
            let cur_tree_height = line
                .chars()
                .skip(lines_col_count - 1 - col_start)
                .take(1)
                .next()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();
            let mut tree_heights = Vec::new();
            let mut tree_count = 0;
            println!(
                "\ncol_start: {} cur_tree_height: {cur_tree_height}",
                col_start
            );
            line.chars().rev().collect::<String>()[0..col_start]
                .chars()
                .enumerate()
                .for_each(|(col_index, c)| {
                    let height = c.to_string().parse::<usize>().unwrap();
                    tree_heights.push(height);
                });

            let mut max_height = 0;
            for (col_index, height) in tree_heights.iter().enumerate().rev() {
                let col = lines_col_count - 1 - col_index;
                println!(
                    "row_index: {row_index} col: {col}  col_start - 1: {}  height: {height}",
                    col_start - 1
                );
                tree_count += 1;
                if *height >= cur_tree_height {
                    println!("  adding 1");
                    break;
                }
            }
            let tree = Tree::new(row_index, lines_col_count - 1 - col_start, tree_count);
            println!("Adding tree: {:?}", tree);
            tree_map
                .get_mut(&tree.get_location_string())
                .unwrap()
                .push(tree.get_score());
        }
        row_index += 1;
    }

    println!("\n\nLooking top to bottom from left to right");
    for col_index in 0..lines_col_count {
        println!("\n\nNew col: {col_index}");
        let mut row_index = 0;
        for line in lines.iter() {
            let cur_tree_height = line
                .chars()
                .nth(col_index)
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();
            let mut tree_heights = Vec::new();
            let mut tree_count = 0;

            println!("\nrow: {} cur_tree_height: {cur_tree_height}", row_index);
            for row_count in 0..row_index {
                let height = lines[row_count]
                    .chars()
                    .nth(col_index)
                    .unwrap()
                    .to_string()
                    .parse::<usize>()
                    .unwrap();
                tree_heights.push(height);
            }

            let mut max_height = 0;
            for (col_index, height) in tree_heights.iter().enumerate().rev() {
                let col = lines_col_count - 1 - col_index;
                println!("row_index: {row_index}  col_index: {col_index}  height: {height}");
                tree_count += 1;
                if *height >= cur_tree_height {
                    println!("  adding 1");
                    break;
                }
            }
            let tree = Tree::new(row_index, col_index, tree_count);
            println!("Adding tree: {:?}", tree);
            tree_map
                .get_mut(&tree.get_location_string())
                .unwrap()
                .push(tree.get_score());

            row_index += 1;
        }
    }

    println!("\n\nLooking bottom to top from left to right");
    for col_index in 0..lines_col_count {
        println!("\n\nNew col: {col_index}");
        let mut row_index = 0;
        for line in lines.iter().rev() {
            let cur_tree_height = line
                .chars()
                .nth(col_index)
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();
            let mut tree_heights = Vec::new();
            let mut tree_count = 0;

            println!("\nrow: {} cur_tree_height: {cur_tree_height}", row_index);
            for row_count in 0..row_index {
                let height = lines[lines_row_count - 1 - row_count]
                    .chars()
                    .nth(col_index)
                    .unwrap()
                    .to_string()
                    .parse::<usize>()
                    .unwrap();
                tree_heights.push(height);
            }

            let mut max_height = 0;
            for (col_index, height) in tree_heights.iter().enumerate().rev() {
                let col = lines_col_count - 1 - col_index;
                println!(
                    "row_index: {}  col_index: {col_index}  height: {height}",
                    lines_row_count - 1 - row_index
                );
                tree_count += 1;
                if *height >= cur_tree_height {
                    println!("  adding 1");
                    break;
                }
            }
            let tree = Tree::new(lines_row_count - 1 - row_index, col_index, tree_count);
            println!("Adding tree: {:?}", tree);
            tree_map
                .get_mut(&tree.get_location_string())
                .unwrap()
                .push(tree.get_score());

            row_index += 1;
        }
    }

    let mut highest_vis = 0;
    for r in 0..lines_row_count {
        for c in 0..lines_col_count {
            let trees_vec = tree_map.get(&format!("{}:{}", c + 1, r + 1)).unwrap();
            println!("trees_vec  {:?}", trees_vec);
            let score = trees_vec.iter().fold(0, |acc, &s| match acc {
                0 => s,
                _ => acc * s,
            });
            println!("{}:{} has score {}", c + 1, r + 1, score);
            if score > highest_vis {
                highest_vis = score;
            }
        }
    }
    println!("highest_vis: {}", highest_vis);
}
