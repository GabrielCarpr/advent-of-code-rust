use std::env::args;
use std::collections::HashMap;

mod one;
mod two;
mod three;

type Callback = fn() -> ();

fn main() {
    let mut days: HashMap<u8, Callback> = HashMap::new();
    days.insert(1, one::one);
    days.insert(2, two::two);
    days.insert(3, three::three);

    let sel = &args().collect::<Vec<String>>()[1];
    let index = sel.parse::<u8>().unwrap();

    days[&index]();
}
