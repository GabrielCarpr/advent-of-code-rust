use std::env::args;
use std::collections::HashMap;

mod one;
mod two;
mod three;
mod four;
mod five;

type Callback = fn() -> ();

fn main() {
    let mut days: HashMap<u8, Callback> = HashMap::new();
    days.insert(1, one::one);
    days.insert(2, two::two);
    days.insert(3, three::three);
    days.insert(4, four::four);
    days.insert(5, five::five);

    let sel = &args().collect::<Vec<String>>()[1];
    let index = sel.parse::<u8>().unwrap();

    days[&index]();
}
