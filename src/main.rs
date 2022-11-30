use std::env::args;
use std::collections::HashMap;

mod one;

type Callback = fn() -> ();

fn main() {
    let mut days: HashMap<u8, Callback> = HashMap::new();
    days.insert(1, one::one);

    let sel = &args().collect::<Vec<String>>()[1];
    let index = sel.parse::<u8>().unwrap();

    days[&index]();
}
