use std::fs::read_to_string;

fn load() -> Vec<String> {
    let data = read_to_string("./src/two/input.txt")
        .unwrap()
        .split("\n")
        .map(|val| val.to_string())
        .collect();
    return data;
}

#[derive(Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq)]
enum DesiredResult {
    Lose,
    Draw,
    Win,
}

impl Action {
    fn from_action(action: &str) -> Action {
        return match action {
            "A" => Action::Rock,
            "B" => Action::Paper,
            "C" => Action::Scissors,
            "X" => Action::Rock,
            "Y" => Action::Paper,
            "Z" => Action::Scissors,
            _ => panic!("Unknown action: {}", action)
        };
    }

    fn beats(&self, opponent: &Action) -> i32 {
        return match self {
            Self::Rock => {
                match opponent {
                    Self::Rock => 3,
                    Self::Paper => 0,
                    Self::Scissors => 6,
                }
            },
            Self::Paper => {
                match opponent {
                    Self::Rock => 6,
                    Self::Paper => 3,
                    Self::Scissors => 0,
                }
            },
            Self::Scissors => {
                match opponent {
                    Self::Rock => 0,
                    Self::Paper => 6,
                    Self::Scissors => 3,
                }
            }
        }
    }

    fn points(&self) -> i32 {
        return match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl DesiredResult {
    fn from_string(string: &str) -> DesiredResult {
        return match string {
            "X" => DesiredResult::Lose,
            "Y" => DesiredResult::Draw,
            "Z" => DesiredResult::Win,
            _ => panic!("Unknown symbol for desired result"),
        }
    }

    fn counter_with(&self, opponent: &Action) -> Action {
        if self == &Self::Draw {
            return opponent.clone();
        }

        if self == &Self::Win {
            return match opponent {
                Action::Paper => Action::Scissors,
                Action::Rock => Action::Paper,
                Action::Scissors => Action::Rock,
            }
        }

        if self == &Self::Lose {
            return match opponent {
                Action::Paper => Action::Rock,
                Action::Rock => Action::Scissors,
                Action::Scissors => Action::Paper,
            }
        }

        panic!("Unknown type");
    }
}

pub fn two() {
    println!("Two!");

    let result: i32 = load()
        .iter()
        .map(|line| {
            println!("Round: {}", line);
            let mut chars = line.chars();
            let action = Action::from_action(chars.nth(0).expect("Could not get action from round").to_string().as_str());
            let result = DesiredResult::from_string(chars.nth(1).expect("Could not get response from round").to_string().as_str());

            let response = result.counter_with(&action);
            return response.points() + response.beats(&action);
        })
        .sum();

    println!("{:?}", result);
}