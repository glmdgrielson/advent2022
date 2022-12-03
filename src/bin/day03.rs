use std::io::stdin;

fn main() {
    let lines = stdin().lines();
    let mut priority = 0;
    'search: for line in lines {
        if let Ok(sack) = line {
            let (upper, lower) = sack.split_at(sack.len() / 2);
            for item in upper.chars() {
                if let Some(_) = lower.find(item) {
                    // let prior = item as u32;
                    // println!("Item found {}, with priority {}", item, prior);
                    priority += priority_value(item);
                    continue 'search;
                }
            }
        } else {
            panic!("Oh shoot, we forgot the luggage!")
        }
    }
    println!("Final sum of priorities is {}", priority);
}

fn priority_value(item: char) -> u32 {
    let prior = item as u32;
    match item {
        'a' ..= 'z' => {
            prior - 96
        }
        'A' ..= 'Z' => {
            prior - 38
        }
        _ => unreachable!("That shouldn't be here...")
    }
}