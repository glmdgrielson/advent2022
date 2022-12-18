//! Day 17's Advent of Code puzzle
//! ==============================
//! TETRIS!
//!
//! Part 1
//! ------
//! After 2022 rounds of falling, how high is the tower?

use advent::{input_to_str, Advent};

#[derive(Debug)]
struct Day18(Vec<Cube>);

/// A cube of poorly scanned volcanic dust.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cube {
	x: u32,
	y: u32,
	z: u32,
}

impl Advent for Day18 {
	type Answer1 = u32;

	type Answer2 = ();

	fn parse_input(input: &str) -> Self {
		// Format: x,y,z
		let cubes = input
			.lines()
			.map(|line| {
				let mut coordinates = line.split(',');
				let x = coordinates.next().expect("Missing x");
				let x = x.parse().expect("Where the heck is that?");
				let y = coordinates.next().expect("Missing x");
				let y = y.parse().expect("Where the heck is that?");
				let z = coordinates.next().expect("Missing x");
				let z = z.parse().expect("Where the heck is that?");

				Cube { x, y, z }
			})
			.collect();
		Day18(cubes)
	}

	fn part_one(&self) -> Self::Answer1 {
		todo!()
	}
}

fn main() {
	let runner = Day18::parse_input(&input_to_str());
	println!("The top of the stack is {} units high", runner.part_one());
}
