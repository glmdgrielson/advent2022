//! Day 17's Advent of Code puzzle
//! ==============================
//! TETRIS!
//!
//! Part 1
//! ------
//! After 2022 rounds of falling, how high is the tower?

use advent::{input_to_str, Advent};

#[derive(Debug)]
struct Day17(Vec<WindDirection>);

/// The way the wind blows in a certain direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WindDirection {
	Left,
	Right,
}

impl Advent for Day17 {
	type Answer1 = u32;

	type Answer2 = ();

	fn parse_input(input: &str) -> Self {
		let directions = input
			.chars()
			.map(|c| match c {
				'>' => WindDirection::Right,
				'<' => WindDirection::Left,
				_ => unreachable!("Incorrect direction"),
			})
			.collect();
		Day17(directions)
	}

	fn part_one(&self) -> Self::Answer1 {
		let mut floor = vec![0u32; 5];
		todo!()
	}
}

fn main() {
	let runner = Day17::parse_input(&input_to_str());
	println!("The top of the stack is {} units high", runner.part_one());
}
