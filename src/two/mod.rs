use std::fs::read_to_string;

fn load() -> String {
    return read_to_string("./src/two/input.txt").unwrap();
}

fn parse

pub fn two() {
    println!("{:?}", load());
}