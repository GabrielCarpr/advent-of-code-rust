use std::fs::read_to_string;

#[derive(PartialEq)]
struct Node<'a> {
    name: String,
    size: Option<u32>,
    parent: Option<&'a Node>,
    children: Option<Vec<Node>>,
}

impl Node {
    fn is_dir(&self) -> bool {
        return self.children != None;
    }

    fn is_file(&self) -> bool {
        return self.size != None;
    }
}

struct FileSystem<'a> {
    nodes: Vec<Node>,
    current: Option<&'a Node>,
}

impl FileSystem<'_> {
    fn up(&mut self) {
        let parent = self.current.unwrap().parent.unwrap();
        self.current = Some(parent);
    }
}

pub fn seven() {
    println!("Seven");

    let mut fs = FileSystem{nodes: vec!(), current: None};

    let data = read_to_string("src/seven/input.txt").unwrap();
    data.lines().for_each(|line| {
        let split: Vec<&str> = line.split(" ").collect();
        match split[0] {
            "$" => match split[1] {
                "cd" => {
                    match split[2] {
                        ".." => {

                        }
                    }
                }
            }
        }
    })
}