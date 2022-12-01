use std::io::stdin;
fn main() {
    let mut elves = vec![0];
    let mut index = 0;
    let lines = stdin().lines();
    for line in lines {
        if let Ok(line) = line {
            if line == "" {
                elves.push(0);
                index += 1;
            } else {
                if let Ok(cal) = i32::from_str_radix(&line, 10) {
                    elves[index] += cal;
                } else {
                    panic!("Invalid value")
                }
            }
        } else {
            panic!("AAAAAAAAAAH!")
        }
    }
    let elf = Iterator::max(elves.iter());
    if let Some(elf) = elf {
        println!("The correct elf is {}", elf);
    } else {
        panic!("AAAAAAAAAAAAH")
    }
    let mut sorts = elves.clone();
    sorts.sort_by(|a, b| b.cmp(a));
    let sum = sorts[0] + sorts[1] + sorts[2];
    println!("The sum of the top three elves is {}", sum);
}
