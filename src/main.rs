//! Day 1's Advent of Code puzzle
//! =============================
//! Should this be in `main.rs`? No, but the file was already there and I didn't
//! feel like moving it. Maybe later.
//!
//! Puzzle input consists of a list of numbers, occasionally separated by
//! empty lines. These represent a series of elves with varying amounts
//! of snacks. An empty line separates one elf's collection from another.
//! Non-empty lines represent the calorie count of the current snack.
//! These elves are hungry.
//!
//! Part 1
//! ------
//! Return the elf with the best snacks. (Read: the elf with the highest
//! calorie count)
//!
//! Part 2
//! ------
//! Return the calorie count of the top THREE elves, using the same criteria
//! as part 1.

use std::io::stdin;
fn main() {
	let mut elves = vec![0];
	let mut index = 0;
	let lines = stdin().lines();
	for line in lines {
		if let Ok(line) = line {
			if line == "" {
				// If the line is empty, we move onto the next elf.
				elves.push(0);
				index += 1;
			} else {
				// Otherwise, we add the current value to the current elf.
				if let Ok(cal) = line.parse::<i32>() {
					elves[index] += cal;
				} else {
					// Realistically, this should never be reached.
					// After all, the input is pre-prepared and we should be
					// able to trust it. The infosec people are currently
					// laughing at my naivete.
					panic!("Invalid value")
				}
			}
		} else {
			// We shouldn't really get here, because this implies something
			// went wrong that's WAAAAAAY out of our league.
			panic!("AAAAAAAAAAH!")
		}
	}
	let elf = Iterator::max(elves.iter());
	if let Some(elf) = elf {
		println!("The correct elf is {}", elf);
	} else {
		// This shouldn't be reached, because otherwise it means
		// that the input is empty and I've just wasted this computer's time.
		panic!("AAAAAAAAAAAAH")
	}
	// This gets the three elves for part 2. Relatively simple.
	let mut sorts = elves.clone();
	sorts.sort_by(|a, b| b.cmp(a));
	let sum = sorts[0] + sorts[1] + sorts[2];
	println!("The sum of the top three elves is {}", sum);
}
