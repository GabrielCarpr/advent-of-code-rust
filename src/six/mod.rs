use std::{fs::read_to_string, collections::HashSet};

use substring::Substring;

pub fn six() {
    let stream = read_to_string("src/six/input.txt").unwrap();

    const REQUIRED: usize = 14;
    let mut position = 0;

    loop {
        if position < REQUIRED {
            position += 1;
            continue;
        }

        let token = stream.substring(position-REQUIRED, position);
        let prev: HashSet<char> = token.chars().collect();
        if prev.len() < REQUIRED {
            position += 1;
            continue;
        }

        // Found it
        println!("Token: {}", token);
        println!("Position: {}", position);
        return;
    }
}