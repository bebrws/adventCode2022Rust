use std::cmp::Ordering;
use std::{collections::BinaryHeap, collections::HashMap, fmt, fmt::Formatter, io::BufRead};

// #[derive(PartialEq, Eq, Clone)]
#[derive(Eq, Clone)]
struct Tree {
    row: usize,
    col: usize,
    height: usize,
}

impl Tree {
    fn new(row: usize, col: usize, height: usize) -> Tree {
        Tree {
            row: row,
            col: col,
            height: height,
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

    fn get_height(&self) -> usize {
        self.height
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tree {{ row: {}, col: {}, height: {} }}",
            self.row, self.col, self.height
        )
    }
}

fn main() {
    let mut tree_map = HashMap::new();

    let bf = advent_tools::get_buffered_file("input");
    let mut lines = Vec::new();
    for line_option in bf.lines() {
        let line = line_option.expect("Couldnt read line");
        lines.push(line.clone());
        // println!("line: {:?}", line);
    }

    let lines_row_count = lines.len();
    let lines_col_count = lines[0].len();

    println!("\n\nLooking left to right from top to bottom");
    let mut row_index = 0;
    for line in lines.iter() {
        let edge_height = line
            .chars()
            .nth(0)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        println!("\nNew row: {row_index} edge_height: {edge_height}");
        let mut max_height = edge_height;
        line.chars().enumerate().for_each(|(col_index, c)| {
            let height = c.to_string().parse::<usize>().unwrap();
            println!("height: {height} edge_height: {edge_height}");
            if height > max_height {
                let tree = Tree::new(row_index, col_index, height);
                println!(
                    "adding tree row: {} height: {height} to map",
                    tree.get_location_string()
                );
                tree_map.insert(tree.get_location_string(), tree.get_height());
                max_height = height;
            }
        });
        row_index += 1;
    }

    println!("\n\nLooking right to left from top to bottom");
    let mut row_index = 0;
    for line in lines.iter() {
        let edge_height = line
            .chars()
            .nth(lines_col_count - 1)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        println!("\nNew row: {row_index} edge_height: {edge_height}");
        let mut max_height = edge_height;
        line.chars().rev().enumerate().for_each(|(col_index, c)| {
            let height = c.to_string().parse::<usize>().unwrap();
            println!("height: {height} edge_height: {edge_height}");
            if height > max_height {
                let tree = Tree::new(row_index, lines_col_count - 1 - col_index, height);
                println!(
                    "adding tree row: {} height: {height} to map",
                    tree.get_location_string()
                );
                tree_map.insert(tree.get_location_string(), tree.get_height());
                max_height = height;
            }
        });
        row_index += 1;
    }

    println!("\n\nLooking top to bottom from left to right");
    for col_index in 0..lines_col_count {
        let mut col: BinaryHeap<Tree> = BinaryHeap::new();
        let mut row_index = 0;
        let edge_height = lines[0]
            .chars()
            .nth(col_index)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        let mut max_height = edge_height;
        println!("\nNew col: {col_index} edge_height: {edge_height}");
        for line in lines.iter() {
            let height = line
                .chars()
                .nth(col_index)
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();
            println!("height: {height} edge_height: {edge_height}");
            if height > max_height {
                let tree = Tree::new(row_index, col_index, height);
                println!(
                    "adding tree row: {} height: {height} to map",
                    tree.get_location_string()
                );
                tree_map.insert(tree.get_location_string(), tree.get_height());
                max_height = height;
            }
            row_index += 1;
        }
    }

    println!("\n\nLooking bottom to top from left to right");
    for col_index in 0..lines_col_count {
        let mut col: BinaryHeap<Tree> = BinaryHeap::new();
        let mut row_index = 0;
        let edge_height = lines[lines_col_count - 1]
            .chars()
            .nth(col_index)
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap();
        let mut max_height = edge_height;
        println!("\nNew col: {col_index} edge_height: {edge_height}");
        for line in lines.iter().rev() {
            let height = line
                .chars()
                .nth(col_index)
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap();
            println!("height: {height} edge_height: {edge_height}");
            if height > max_height {
                let tree = Tree::new(lines_row_count - 1 - row_index, col_index, height);
                println!(
                    "adding tree row: {} height: {height} to map",
                    tree.get_location_string()
                );
                tree_map.insert(tree.get_location_string(), tree.get_height());
                max_height = height;
            }

            row_index += 1;
        }
    }

    // from top to bottom
    for row in 0..lines_row_count {
        let tree_left = Tree::new(row, 0, 1); // height doesnt matter
        tree_map.insert(tree_left.get_location_string(), tree_left.get_height());
        let tree_right = Tree::new(row, lines_col_count - 1, 1); // height doesnt matter
        tree_map.insert(tree_right.get_location_string(), tree_right.get_height());
    }
    for col in 0..lines_col_count {
        let tree_top = Tree::new(0, col, 1); // height doesnt matter
        tree_map.insert(tree_top.get_location_string(), tree_top.get_height());
        let tree_bottom = Tree::new(lines_row_count - 1, col, 1); // height doesnt matter
        tree_map.insert(tree_bottom.get_location_string(), tree_bottom.get_height());
    }
    let total_vis = tree_map.len();
    // println!("tree_map: {:?}", tree_map);
    println!("total_vis: {}", total_vis);
}
