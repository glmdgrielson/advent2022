/// Day 6's Advent of Code puzzle
/// =============================
/// For the first time, puzzle input is a single stream of characters. Oh no.
///
/// Part 1
/// ------
/// Find the index of the first four characters that are unique.
use std::io::{stdin, Read};

fn main() {
	let bytes = stdin().bytes();
	let mut marker: Vec<_> = vec![];
	let mut idx = 0;
	for (i, byte) in bytes.enumerate() {
		if let Ok(ch) = byte {
			marker.push(ch);
			if marker.len() == 4 {
				let mut set = marker.clone();
				set.sort();
				set.dedup();
				if set.len() == 4 {
					idx = i + 1;
					break;
				}
				marker.remove(0);
			}
			// idx += 1;
		} else {
			panic!("Welp, your input has failed. Oops!");
		}
	}
	println!("The first marker {:?} can be found at index {}", marker, idx);
}
