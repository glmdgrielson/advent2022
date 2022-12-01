use std::io::stdin;
fn main() {
    let mut elves = vec![0];
    let mut index = 0;
    let lines = stdin().lines();
    for line in lines {
        match line {
            Ok(line) => {
                if line == "" {
                    elves.push(0);
                    index += 1;
                } else {
                    match i32::from_str_radix(&line, 10) {
                        Ok(cal) => {
                            elves[index] += cal;
                        }
                        Err(_) => panic!("Invalid value")
                    }
                }
            },
            Err(_) => {
                panic!("AAAAAAAAAAH!")
            }
        }
    }
    let elf = Iterator::max(elves.iter());
    match elf {
        Some(elf) => {
            let num = elves.iter().find(|item| item == &elf);
            match num {
                Some(num) => println!("The correct elf is {}", num),
                None => panic!("The elves are starving!")
            }
        }
        None => {
            panic!("AAAAAAAAAAAAH")
        }
    }
    let mut sorts = elves.clone();
    sorts.sort_by(|a, b| b.cmp(a));
    let sum = sorts[0] + sorts[1] + sorts[2];
    println!("The sum of the top three elves is {}", sum);
}
