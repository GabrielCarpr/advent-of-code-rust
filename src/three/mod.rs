use std::{fs::read_to_string, collections::{HashSet, VecDeque}, iter::FromIterator};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Item(char);

impl Item {
    fn from(letter: char) -> Item {
        return Item(letter);
    }

    fn priority(&self) -> u32 {
        if self.0.is_uppercase() {
            return self.0 as u32 - 38;
        } else {
            return self.0 as u32 - 96;
        }
    }
}

#[derive(Debug)]
struct Rucksack {
    items: Vec<Item>,
}

impl Rucksack {
    fn from_line(line: String) -> Rucksack {
        let items = line.chars().map(|char| Item::from(char)).collect();
        return Rucksack{items};
    }

    fn compartment_size(&self) -> usize {
        let size = self.items.len();
        return size / 2;
    }

    fn first_compartment(&self) -> &[Item] {
        let size = self.compartment_size();
        return &self.items[0..size];
    }

    fn second_compartment(&self) -> &[Item] {
        let size = self.compartment_size();
        return &self.items[size..self.items.len()]
    }

    fn duplicate(&self) -> &Item {
        let first: HashSet<&Item> = HashSet::from_iter(self.first_compartment());
        let second = HashSet::from_iter(self.second_compartment());
        let mut intersection = first.intersection(&second);

        return intersection.nth(0).unwrap();
    }
}

#[derive(Debug)]
struct Group {
    first: Rucksack,
    second: Rucksack,
    third: Rucksack,
}

impl Group {
    fn next(lines: &mut VecDeque<String>) -> Group {
        Group{
            first: Rucksack::from_line(lines.pop_front().unwrap()),
            second: Rucksack::from_line(lines.pop_front().unwrap()),
            third: Rucksack::from_line(lines.pop_front().unwrap()),
        }
    }

    fn badge(&self) -> Item {
        let first: HashSet<Item> = HashSet::from_iter(self.first.items.clone());
        let second: HashSet<Item> = HashSet::from_iter(self.second.items.clone());
        let third: HashSet<Item> = HashSet::from_iter(self.third.items.clone());
        let mut intersection: HashSet<Item> = first.intersection(&second).copied().collect();
        intersection = intersection.intersection(&third).copied().collect();

        return *intersection.iter().nth(0).unwrap();
    }
}

fn load() -> VecDeque<String> {
    let data = read_to_string("./src/three/input.txt")
        .unwrap()
        .split("\n")
        .map(|val| val.to_string())
        .collect();
    return data;
}

pub fn three() {
    println!("Three");  
    
    let mut lines = load();

    let mut groups: Vec<Group> = vec!();
    while lines.len() > 0 {
        groups.push(Group::next(&mut lines));
    }

    let result: u32 = groups
        .iter()
        .map(|group| group.badge().priority())
        .sum();

    println!("{:?}", result);
}