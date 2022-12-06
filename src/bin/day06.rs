/// Day 6's Advent of Code puzzle
/// =============================
/// For the first time, puzzle input is a single stream of characters. Oh no.
///
/// Part 1
/// ------
/// Find the index of the first four characters that are unique.
/// 
/// Part 2
/// ------
/// Find the index of the first sequence of _fourteen_ unique characters.
use std::io::{stdin, Read};

const PACKET_SIZE: usize = 4;
const MESSAGE_SIZE: usize = 14;

fn main() {
	let bytes = stdin().bytes();
	let mut marker: Vec<_> = vec![];
	let mut packet = 0;
	let mut message = 0;
	let mut part1 = false;
	let mut part2 = false;
	for (i, byte) in bytes.enumerate() {
		if let Ok(ch) = byte {
			marker.push(ch);
			if marker.len() >= PACKET_SIZE && !part1 {
				part1 = is_window_unique(&marker, PACKET_SIZE);
				if part1 {
					packet = i + 1;
				}
			}
			if marker.len() >= MESSAGE_SIZE && !part2 {
				part2 = is_window_unique(&marker, MESSAGE_SIZE);
				if part2 {
					message = i + 1;
				}
			}
			if part1 && part2 {
				break;
			}
		} else {
			panic!("Welp, your input has failed. Oops!");
		}
	}
	println!("The packet marker can be found at index {}", packet);
	println!("The message marker can be found at index {}", message);
}

fn is_window_unique(marker: &[u8], size: usize) -> bool {
	let mut set = marker.iter().rev().take(size).collect::<Vec<_>>();
	set.sort();
	set.dedup();
	set.len() == size
}
