//! Day 17's Advent of Code puzzle
//! ==============================
//! Puzzle input is a series of ~~tubes~~ cubes.
//!
//! Part 1
//! ------
//! How many cube faces are not met by another cube?

use advent::{input_to_str, Advent};

#[derive(Debug)]
struct Day18(Vec<Cube>);

/// A cube of poorly scanned volcanic dust.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cube {
	x: u32,
	y: u32,
	z: u32,
}

// type Face = (Axis, Axis, Axis);

// impl Cube {
// 	/// Check if this cube is adjacent to another cube.
// 	fn adjacent(&self, other: &Cube) -> bool {
// 		if self.x.abs_diff(other.x) == 1 {
// 			// X axis is next to the other cube.
// 			self.y == other.y && self.z == other.z
// 		} else if self.y.abs_diff(other.y) == 1 {
// 			self.x == other.x && self.z == other.z
// 		} else if self.z.abs_diff(other.z) == 1 {
// 			self.x == other.x && self.y == other.y
// 		} else {
// 			false
// 		}
// 	}
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
// enum Axis {
// 	/// The horizontal side of a cube.
// 	X(u32),
// 	/// The vertical side of a cube.
// 	Y(u32),
// 	/// The ...stacked side of a cube.
// 	///
// 	/// We really need a better word than "stacked" for this.
// 	/// Thank you Stack Overflow!
// 	Z(u32),
// }

impl Advent for Day18 {
	type Answer1 = u32;

	type Answer2 = ();

	/// Aaaaaaaaah, a simple puzzle for parsing...
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
		let mut edges = 0;
		for cube in self.0.iter() {
			let cubes = [
				Cube {
					x: cube.x.saturating_sub(1),
					y: cube.y,
					z: cube.z,
				},
				Cube {
					x: cube.x.saturating_add(1),
					y: cube.y,
					z: cube.z,
				},
				Cube {
					x: cube.x,
					y: cube.y.saturating_sub(1),
					z: cube.z,
				},
				Cube {
					x: cube.x,
					y: cube.y.saturating_add(1),
					z: cube.z,
				},
				Cube {
					x: cube.x,
					y: cube.y,
					z: cube.z.saturating_sub(1),
				},
				Cube {
					x: cube.x,
					y: cube.y,
					z: cube.z.saturating_add(1),
				},
			];
			for other_cube in cubes {
				if !self.0.contains(&other_cube) {
					eprintln!("{:?} is not adjacent to {:?}", other_cube, cube);
					edges += 1;
				}
			}
			// if cube {
			// 	//
			// }
		}
		edges
	}
}

fn main() {
	let runner = Day18::parse_input(&input_to_str());
	println!("There are {} unmet faces.", runner.part_one());
}
