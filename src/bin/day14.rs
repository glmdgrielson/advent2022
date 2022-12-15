//! Day 14's Advent of Code puzzle
//! ==============================
//! Puzzle input consists of a series of lines, representing a cave.
//!
//! Part 1
//! ------
//! How many sand units can fall before sand reaches below the floor
//! of the cave?

use advent::{input_to_str, Advent, Point};
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Day14 {
	/// The complete set of points at which this puzzle considers solid.
	/// Every point not included is therefore assumed empty.
	///
	/// Note that for the purposes of this puzzle, `y` represents
	/// distance _below_ the x-axis.
	maze: HashSet<Point<u32>>,
	/// The point at which sand falls "forever".
	floor: u32,
}

impl Advent for Day14 {
	type Answer1 = u32;

	type Answer2 = ();

	fn parse_input(input: &str) -> Self {
		let mut maze = HashSet::new();
		// We can reasonably expect that nothing ever goes _higher_
		// than 500 units, right?
		let mut floor = 0;
		for line in input.lines() {
			let points =
				line.split(" -> ").map(parse_point).collect::<Vec<_>>();
			let mut head = points[0];
			maze.insert(head);
			for tail in &points[1..] {
				// Making some basic assertions.
				assert_ne!(&head, tail, "Path goes nowhere");
				assert!(
					head.x == tail.x || head.y == tail.y,
					"Path ill-formed"
				);

				// Extracting values from the Points.
				let Point { x: h_x, y: h_y } = head;
				let Point { x: t_x, y: t_y } = *tail;
				if h_x == t_x {
					// The x-values are the same. Iterate over y-values.

					let upper = h_y.max(t_y);
					let lower = h_y.min(t_y);

					// Iterating over every y-value between the two.
					for y_idx in lower..=upper {
						// Picking an x-value arbitrarily since
						// it's the same for both.
						maze.insert(Point { x: h_x, y: y_idx });

						// Check the floor.
						if y_idx > floor {
							floor = y_idx;
						}
					}
				} else {
					// The y-values are the same. Iterate over x-values.\

					let upper = h_x.max(t_x);
					let lower = h_x.min(t_x);

					// Iterating over every x-value between the points.
					for x_idx in lower..=upper {
						maze.insert(Point { x: x_idx, y: h_y });
					}
				}
				head = *tail;
			}
		}
		Day14 { maze, floor }
	}

	fn part_one(&self) -> Self::Answer1 {
		let floor = self.floor;
		let mut maze = self.maze.clone();
		let mut sand = Point { x: 500, y: 0 };
		let mut sand_counter = 0;
		'fall: loop {
			// Check to see if we've gone past the floor.
			if sand.y > floor {
				break 'fall;
			}

			let next_points = [
				Point {
					x: sand.x,
					y: sand.y + 1,
				},
				Point {
					x: sand.x - 1,
					y: sand.y + 1,
				},
				Point {
					x: sand.x + 1,
					y: sand.y + 1,
				},
			];

			for point in next_points {
				if point.y > floor {
					break 'fall;
				}
				if !maze.contains(&point) {
					sand = point;
					continue 'fall;
				}
			}

			maze.insert(sand);
			sand_counter += 1;
			sand = Point { x: 500, y: 0 };
		}
		sand_counter
	}
}

fn parse_point(text: &str) -> Point<u32> {
	let mut point = text.split(',');
	let x = point.next().expect("Missing coordinate in point");
	let y = point.next().expect("Missing coordinate in point");

	let x: u32 = x.parse().expect("Malformed point in input");
	let y = y.parse().expect("Malformed point in input");

	Point { x, y }
}

fn main() {
	let runner = Day14::parse_input(&input_to_str());
	println!(
		"The total number of sand units before stabilizing is {}",
		runner.part_one()
	);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse() {
		let example = "498,4 -> 498,6 -> 496,6";
		let runner = Day14::parse_input(example);

		let mut expected = HashSet::new();
		expected.insert(Point { x: 498, y: 4 });
		expected.insert(Point { x: 498, y: 5 });
		expected.insert(Point { x: 498, y: 6 });
		expected.insert(Point { x: 497, y: 6 });
		expected.insert(Point { x: 496, y: 6 });
		let actual = runner.maze;

		assert_eq!(runner.floor, 6);
		assert_eq!(expected, actual);
	}

	#[test]
	fn test_part_one() {
		use std::fs::File;
		use std::io::Read;

		let mut file = File::open("src/input/day14-example.txt")
			.expect("File reading failed.");
		let mut example = String::new();
		file.read_to_string(&mut example)
			.expect("Reading has failed.");

		assert_eq!(Day14::parse_input(&example).part_one(), 24);
	}
}
