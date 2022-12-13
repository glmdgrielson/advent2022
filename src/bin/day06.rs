//! Day 6's Advent of Code puzzle
//! =============================
//! For the first time, puzzle input is a single stream of characters. Oh no.
//!
//! Part 1
//! ------
//! Find the index of the first four characters that are unique.
//!
//! Part 2
//! ------
//! Find the index of the first sequence of _fourteen_ unique characters.

use std::io::{stdin, Read};

// Constants for the size of the area needed for each step.
// The names are chosen as per the narrative around the puzzle.
const PACKET_SIZE: usize = 4;
const MESSAGE_SIZE: usize = 14;

fn main() {
	let bytes = stdin().bytes();
	let mut marker: Vec<_> = vec![];
	let mut packet = 0;
	let mut message = 0;
	let mut part1 = false;
	let mut part2 = false;
	// Since the puzzle for today wants to know where in the input
	// the marker is, we're using `enumerate` today. Note that
	// this method, like all indexing in Rust, starts at zero.
	// This means that I'm going to have to remember to adjust for the
	// inevitable off by one error. Insert the relevant Serge emote here.
	for (i, byte) in bytes.enumerate() {
		if let Ok(ch) = byte {
			// Add the current character to the data stream.
			marker.push(ch);
			// Check to see if we haven't already hit the packet.
			if marker.len() >= PACKET_SIZE && !part1 {
				// Run the uniqueness logic here. If this returns true,
				// we no longer need to run the logic here again.
				part1 = is_window_unique(&marker, PACKET_SIZE);
				// If the logic returns true, then we've found the answer
				// to Part 1.
				if part1 {
					packet = i + 1;
				}
			}
			// Part 2 is much the same, just with a different number.
			if marker.len() >= MESSAGE_SIZE && !part2 {
				part2 = is_window_unique(&marker, MESSAGE_SIZE);
				if part2 {
					message = i + 1;
				}
			}
			// If both parts have returned an answer, then we don't need to
			// keep looping and should bail.
			if part1 && part2 {
				break;
			}
		} else {
			// "Handle" error because I dislike `flatten` as a method.
			// The unwrap is safe because there's no way to get an `Ok` here
			// because we just failed a pattern match on that state.
			panic!("Welp, your input has failed: {}. Oops!", byte.unwrap_err());
		}
	}
	println!("The packet marker can be found at index {}", packet);
	println!("The message marker can be found at index {}", message);
}

/// Check whether the last `size` elements of this array are unique.
fn is_window_unique<T>(marker: &[T], size: usize) -> bool
where
	T: Ord,
{
	let mut set = marker
		.iter() // Create an iterator.
		.rev() // Reverse the iterator so that the back is more accessible.
		.take(size) // Grab the number of elements we actually care about.
		.collect::<Vec<_>>(); // Convert the iterator into a usable type.

	// Sort the vector. I'm kind of annoyed that this is in place.
	// I get WHY, but it's annoying that attaching `.sort` to something
	// results in a value that I can't use.
	set.sort();
	// Turns out this is more type safe than just using `HashSet`.
	set.dedup();
	// Check that our window is the size we need. If this is the same
	// size as before deduplication, that means that nothing has been removed
	// and thus, every element in this window is unique!
	set.len() == size
}
