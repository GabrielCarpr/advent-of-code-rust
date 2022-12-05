use std::fs::read_to_string;

#[derive(Debug)]
struct Assignment(u32, u32);

impl Assignment {
    fn from(text: String) -> Assignment {
        let (start, end) = text.split_once('-').unwrap();
        let start_from = start.parse().unwrap();
        let end_at = end.parse().unwrap();

        return Assignment(start_from, end_at);
    }

    fn contains(&self, other: &Assignment) -> bool {
        return self.0 <= other.0 && self.1 >= other.1;
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        return self.0 <= other.0 && self.1 >= other.0 ||
            self.1 >= other.1 && self.1 <= other.0;

    }
}

#[derive(Debug)]
struct Pair(Assignment, Assignment);

impl Pair {
    fn from(text: String) -> Pair {
        let (first, second) = text.split_once(',').unwrap();

        return Pair(
            Assignment::from(first.to_string()),
            Assignment::from(second.to_string()),
        );
    }

    fn overlaps(&self) -> bool {
        return self.0.overlaps(&self.1) ||
            self.1.overlaps(&self.0) ||
            self.0.contains(&self.1) ||
            self.1.contains(&self.0);
    }
}

pub fn four() {
    println!("Four");

    let data = load();

    let result = data
        .iter()
        .map(|line| Pair::from(line.clone()))
        .filter(|pair| pair.overlaps());

    println!("{:?}", result.collect::<Vec<Pair>>().len());
}

fn load() -> Vec<String> {
    let data = read_to_string("./src/four/input.txt")
        .unwrap()
        .split("\n")
        .map(|val| val.to_string())
        .collect();
    return data;
}