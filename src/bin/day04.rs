//! Day 4's Advent of Code puzzle
//! =============================
//! Puzzle input consists of pairs of elves and the areas they have been
//! assigned, in the form `A-B,C-D`.
//!
//! Part 1
//! ------
//! Find out how many pairs have redundant ranges. These would be pairs where
//! one elf's range is completely contained by the other elf.
//!
//! Part 2
//! ------
//! Find out how many pairs contain any sort of overlap.
#![warn(clippy::all)]
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
	let lines = stdin().lines();
	// Aaaaaaaaand guess who forgot to make the counter variable mutable? Oops.
	let mut contains_count = 0;
	let mut overlap_count = 0;
	for line in lines {
		if let Ok(pair) = line {
			// `this` should be the first elf. `that` is the second elf.
			let (this, that) = if pair.find(',').is_some() {
				let line = pair.split(',').collect::<Vec<_>>();
				(line[0], line[1])
			} else {
				panic!("Cleanup crew to aisle 7!")
			};
			let this = if this.find('-').is_some() {
				let items = this.split('-').collect::<Vec<_>>();
				let (upper, lower) = (items[0], items[1]);
				let upper = upper.parse::<i32>();
				let lower = lower.parse::<i32>();
				match (upper, lower) {
					(Ok(upper), Ok(lower)) => (upper, lower),
					_ => panic!("What are you elves doing?"),
				}
			} else {
				panic!("Elf is slacking off!")
			};
			let that = if that.find('-').is_some() {
				let items = that.split('-').collect::<Vec<_>>();
				let (upper, lower) = (items[0], items[1]);
				let upper = upper.parse::<i32>();
				let lower = lower.parse::<i32>();
				match (upper, lower) {
					(Ok(upper), Ok(lower)) => (upper, lower),
					_ => panic!("What are you elves doing?"),
				}
			} else {
				panic!("Elf is slacking off!")
			};
			if contains(this, that) {
				contains_count += 1;
			}
			if overlaps(this, that) {
				overlap_count += 1;
			}
		} else {
			panic!("Input has failed! AAAAAAAAAH!");
		}
	}
	println!(
		"The number of poorly planned elf pairs is {}",
		contains_count
	);
	println!("The number of unneeded collaborations is {}", overlap_count);
}

fn contains(this: (i32, i32), that: (i32, i32)) -> bool {
	match this.0.cmp(&that.0) {
		Ordering::Less => {
			// The start of the first range is before the start
			// of the second range.
			// The end of the first range is _after_ the start
			// of the second range, so it counts.
			this.1 >= that.1
		}
		Ordering::Equal => {
			// The start of this range is exactly equal
			// the start of the second range. Therefore, one elf
			// containing another should be guaranteed?
			true
		}
		Ordering::Greater => {
			// The start of this range is _inside_ the start of the
			// other range. Ergo, we need to check if this range is
			// smaller.
			this.1 <= that.1
		}
	}
}

/// Determine whether one range overlaps another range.
///
/// This checks for the criteria for Part 2 of the puzzle.
///
/// I stole this solution from elsewhere, so I'm not _entirely_
/// sold on the logic of it?
fn overlaps(this: (i32, i32), that: (i32, i32)) -> bool {
	// Check whether the start of `this` overlaps with the end of `that`.
	if this.0 <= that.1 {
		// Now reverse the check to make sure yes, we're dealing with
		// the same numbers.
		that.0 <= this.1
	} else {
		false
	}
}
