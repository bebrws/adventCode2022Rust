use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, space0, space1},
    combinator::map,
    combinator::map_res,
    error::ParseError,
    sequence::preceded,
    sequence::{self, delimited, pair},
    sequence::{separated_pair, tuple},
    IResult,
};

use nom::bytes::streaming::take_while;
use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, fmt, io::BufRead, rc::Rc};

#[derive(PartialEq, Eq, Clone)]
struct NodeType {
    name: String,
    dirs: HashMap<String, Rc<RefCell<NodeType>>>,
    files: HashMap<String, u32>,
    parent: Option<Rc<RefCell<NodeType>>>,
}

impl NodeType {
    fn new(name: String, parent: Option<Rc<RefCell<NodeType>>>) -> Rc<RefCell<Self>> {
        // println!(
        //     "Creating dir {} with parent {:?}",
        //     name,
        //     parent.as_ref().map(|p| p.borrow().name.clone())
        // );
        Rc::new(RefCell::new(NodeType {
            name: name,
            dirs: HashMap::new(),
            files: HashMap::new(),
            parent: parent,
        }))
    }

    fn add_file(&mut self, name: String, size: u32) {
        self.files.insert(name, size);
    }

    fn add_dir(&mut self, name: String, parent: Option<Rc<RefCell<NodeType>>>) {
        if !self.dirs.contains_key(&name) {
            let new_dir = NodeType::new(name.clone(), parent);
            self.dirs.insert(name, Rc::clone(&new_dir));
        }
    }

    fn cd_dir(
        &mut self,
        name: String,
        parent: Option<Rc<RefCell<NodeType>>>,
    ) -> Rc<RefCell<NodeType>> {
        if name == ".." {
            return self.parent.as_ref().unwrap().clone();
        } else {
            println!("Looking for dir {} in {:?}", name, self.dirs.keys());
            Rc::clone(self.dirs.get(&name).unwrap())
        }
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeType")
            .field("name", &self.name)
            .field("dirs", &self.dirs.keys().collect::<Vec<_>>()) // Print only the names of the directories
            .field("files", &self.files)
            .field(
                "parent",
                &self.parent.as_ref().map(|p| p.borrow().name.clone()),
            ) // Indicate presence of a parent, but don't print it
            .finish()
    }
}

fn parse_cd_command(input: &str) -> IResult<&str, &str> {
    let (input, other) = tag("$ cd ")(input)?;
    println!("input other: {},{}", input, other);
    take_while(|c: char| c != '\n' && c != '\r')(input)
}

fn parse_instruction<'a>(
    s: &'a str,
    cur_dir: Rc<RefCell<NodeType>>,
    root_dir: Rc<RefCell<NodeType>>,
) -> Rc<RefCell<NodeType>> {
    // println!("s: {}", s);
    let cd_parse: IResult<&str, &str> = tag("$ cd ")(s);
    if let Ok((dir_str, _)) = cd_parse {
        println!(
            "cd dir_name: {}",
            dir_str // cur_dir.as_ref().borrow().name
        );
        if dir_str == "/" {
            // return Rc::clone(&root_dir);
            return root_dir;
        }
        let cur_dir_clone = Rc::clone(&cur_dir);
        println!(
            "cur_dir_clone dir_name: {}",
            cur_dir_clone.as_ref().borrow().name
        );
        return cur_dir
            .as_ref()
            .borrow_mut()
            .cd_dir(dir_str.to_string(), Some(Rc::clone(&cur_dir_clone)));
    }

    let dir_name_parse: IResult<&str, &str> = tag("dir ")(s);
    if let Ok((dir_name_str, _)) = dir_name_parse {
        println!(
            "dir_name: {}",
            dir_name_str,
            // cur_dir.as_ref().borrow().name
        );
        let cur_dir_clone = Rc::clone(&cur_dir);
        cur_dir
            .as_ref()
            .borrow_mut()
            .add_dir(dir_name_str.to_string(), Some(Rc::clone(&cur_dir_clone)));
        return cur_dir;
    }

    let binding = s.to_string() + "\n";
    let file_size_name_parse: IResult<&str, (u32, &str)> = tuple((
        map_res(digit1, |num_str: &str| num_str.parse::<u32>()),
        space1, // Parse and discard the space
        take_while(|c: char| c != ' ' && c != '\n'),
    ))(binding.as_str())
    .map(|(next_input, (size, _, name))| (next_input, (size, name)));
    if let Ok((_, (file_size, file_name))) = file_size_name_parse {
        println!("file_size: {}, file_name: {}", file_size, file_name);
        cur_dir
            .as_ref()
            .borrow_mut()
            .add_file(file_name.to_string(), file_size);
    }
    cur_dir
}

fn main() {
    let bf = advent_tools::get_buffered_file("input");

    let mut root_dir = NodeType::new("/".to_string(), None);

    let mut cur_dir = Rc::clone(&root_dir);

    for line in bf.lines() {
        let after = Rc::clone(&root_dir);
        let line = line.expect("Failed to read line");
        // let x = parseContainer(line.as_str());
        // println!("{:?}", parseContainer(line.as_str()));
        // println!("line: {:?}", line);
        println!("line: {}", line);
        // println!("before name: {}", cur_dir.as_ref().borrow().name);
        cur_dir = parse_instruction(line.as_str(), Rc::clone(&cur_dir), after);
        // println!("after name: {}", cur_dir.as_ref().borrow().name);
        *cur_dir.borrow_mut() = Rc::clone(&cur_dir);
    }

    println!("{:?}", cur_dir);
}
