use std::fs::read_to_string;


fn load() -> Vec<String> {
    let data: Vec<String> = read_to_string("./src/one/input.txt")
        .unwrap()
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    return data;
}

pub fn one() {
    println!("One!");
    let data = load();

    let mut elves: Vec<i32> = [].to_vec();
    let mut current_total = 0;
    for num in 0..data.len() {
        let val = &data[num];
        if val == "" {
            elves.push(current_total);
            current_total = 0;
            continue;
        }

        current_total += val.parse::<i32>().unwrap();
    }

    let mut take_highest = || -> i32 {
        let mut highest = 0;
        for num in 0..elves.len() {
            let elf = elves[num];
            if elf > highest {
                highest = elf;
            }
        }
        
        for num in 0..elves.len() {
            if elves[num] == highest {
                return elves.remove(num);
            }
        }
        return 0;
    };

    println!("Highest elves total: {}", take_highest() + take_highest() + take_highest());
}
