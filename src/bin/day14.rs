//! Day 14's Advent of Code puzzle
//! ==============================
//! Puzzle input consists of a series of lines, representing a cave.

use advent::{input_to_str, Advent, Point};
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Day14 {
	/// The complete set of points at which this puzzle considers solid.
	/// Every point not included is therefore assumed empty.
	maze: HashSet<Point>,
	/// The point at which sand falls "forever".
	floor: usize,
}

impl Advent for Day14 {
	type Answer1 = u32;

	type Answer2 = ();

	fn parse_input(input: &str) -> Self {
		let maze = HashSet::new();
		// We can reasonably expect that nothing ever goes _higher_
		// than 500 units, right?
		let floor = 500;
		for line in input.lines() {
			let points = line.split(" -> ").collect::<Vec<_>>();
			for pair in points.windows(2) {
				let one = pair[0];
				let two = pair[1];
			}
		}
		Day14 { maze, floor }
	}

	fn part_one(&self) -> Self::Answer1 {
		todo!()
	}
}

fn main() {
	let runner = Day14::parse_input(&input_to_str());
	println!(
		"The total number of sand units before stabilizing is {}",
		runner.part_one()
	);
}
