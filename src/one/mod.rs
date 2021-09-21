use std::fs::read_to_string;


fn load() -> Vec<u64> {
    let data: Vec<u64> = read_to_string("./src/one/input.txt")
        .unwrap()
        .split("\n")
        .map(|x| x.to_string().parse::<u64>().unwrap())
        .collect();

    return data;
}

pub fn one() {
    println!("One!");
    let data = load();

    for num in 0..data.len() {
        for othernum in 0..data.len() {
            if num == othernum {
                continue;
            }

            for third in 0..data.len() {
                if third == num || third == othernum {
                    continue;
                }

                if data[num] + data[othernum] + data[third] == 2020 {
                    println!("Answer: {}", data[num] * data[othernum] * data[third]);
                    return;
                }
            }
        }
    }
}