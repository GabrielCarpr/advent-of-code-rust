use std::{collections::HashMap, fs::read_to_string, ops::Deref};

use regex::Regex;

#[derive(Debug)]
struct Crate(char);

impl Crate {
    fn from(letter: char) -> Crate {
        return Crate(letter);
    }
}

#[derive(Debug)]
struct Stack(Vec<Crate>);

impl Stack {
    fn new() -> Stack {
        Stack(vec!())
    }

    fn stack(&mut self, c: Crate) {
        self.0.push(c);
    }

    fn from(crates: &[char]) -> Stack {
        let mut stack = Stack::new();
        for c in crates {
            stack.stack(Crate::from(*c));
        }
        return stack
    }

    fn unload(&mut self) -> Crate {
        return self.0.pop().unwrap();
    }
}

#[derive(Debug)]
struct LoadingZone {
    crates: HashMap<u8, Stack>,
}

impl LoadingZone {
    fn new() -> LoadingZone {
        LoadingZone { crates: HashMap::new() }
    }

    fn add_stack(&mut self, name: u8, stack: Stack) {
        self.crates.insert(name, stack);
    }

    fn load(&mut self, cmd: &Command) {
        for i in 0..cmd.moves {
            let c = self.crates.get_mut(&cmd.from).unwrap().unload();
            self.crates.get_mut(&cmd.to).unwrap().stack(c);
        }
    }
}

fn load_zone() -> LoadingZone {
    let lines = read_to_string("./src/five/input.txt")
        .unwrap();
    let loading_lines = lines
        .lines()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let mut loading_lines_rev = loading_lines.iter().rev();

    let stack_names = loading_lines_rev.next().unwrap();
    let mut zone = LoadingZone::new();

    let stacks = stack_names
        .deref()
        .split("   ")
        .map(|name| name.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    let start: u8 = stacks[0];

    for stack in stacks.iter() {
        zone.add_stack(stack.clone(), Stack::new());
    }

    for line in loading_lines_rev {
        let crates = line
            .split(']')
            .map(|l| l.replace('[', "").replace(']', "").trim().to_string());
    }

    return zone;
}

fn make_zone() -> LoadingZone {
    let mut zone = LoadingZone::new();
    zone.add_stack(
        1,
        Stack::from(&['J', 'H', 'P', 'M', 'S', 'F', 'N', 'V'])
    );
    zone.add_stack(
        2,
        Stack::from(&['S', 'R', 'L', 'M', 'J', 'D', 'Q'])
    );
    zone.add_stack(3, Stack::from(&['N', 'Q', 'D', 'H', 'C', 'S', 'W', 'B']));
    zone.add_stack(4, Stack::from(&['R', 'S', 'C', 'L']));
    zone.add_stack(5, Stack::from(&['M', 'V', 'T', 'P', 'F', 'B']));
    zone.add_stack(6, Stack::from(&['T', 'R', 'Q', 'N', 'C']));
    zone.add_stack(7, Stack::from(&['G', 'V', 'R']));
    zone.add_stack(8, Stack::from(&['C', 'Z', 'S', 'P', 'D', 'L', 'R']));
    zone.add_stack(9, Stack::from(&['D', 'S', 'J', 'V', 'G', 'P', 'B', 'F']));

    return zone;
}

struct Command {
    moves: u8,
    from: u8,
    to: u8,
}

impl Command {
    fn from(instruction: &str) -> Command {
        let reggie = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
        let caps = reggie.captures(instruction).unwrap();

        let amount: u8 = caps.get(1).unwrap().as_str().parse().unwrap();
        let from: u8 = caps.get(2).unwrap().as_str().parse().unwrap();
        let to: u8 = caps.get(3).unwrap().as_str().parse().unwrap();

        return Command {
            moves: amount,
            from,
            to,
        }
    }
}

fn load_commands() -> Vec<Command> {
    return read_to_string("src/five/commands.txt")
        .unwrap()
        .lines()
        .map(Command::from)
        .collect();
}

pub fn five() {
    println!("Five");

    let mut zone = make_zone();
    let commands = load_commands();

    for cmd in commands {
        zone.load(&cmd);
    }

    println!(
        "Tops: {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
        zone.crates.get_mut(&1).unwrap().unload(),
        zone.crates.get_mut(&2).unwrap().unload(),
        zone.crates.get_mut(&3).unwrap().unload(),
        zone.crates.get_mut(&4).unwrap().unload(),
        zone.crates.get_mut(&5).unwrap().unload(),
        zone.crates.get_mut(&6).unwrap().unload(),
        zone.crates.get_mut(&7).unwrap().unload(),
        zone.crates.get_mut(&8).unwrap().unload(),
        zone.crates.get_mut(&9).unwrap().unload(),
    )
}