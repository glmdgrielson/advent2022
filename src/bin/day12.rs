/// Day 12's Advent of Code puzzle
/// ==============================
/// Puzzle input consists of _MAZE_.
///
/// Part 1
/// ------
/// Find the shortest path from `S` to `E`.
use advent::{input_to_str, Advent, Point};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

const NULL_POINT: Point<usize> = Point {
	x: usize::MAX,
	y: usize::MAX,
};

#[derive(Clone, Debug)]
struct Day12 {
	maze: Vec<Vec<usize>>,
	start: Point<usize>,
	end: Point<usize>,
}

impl Advent for Day12 {
	type Answer1 = u32;

	type Answer2 = ();

	fn parse_input(input: &str) -> Self {
		// Filling this in with bogus fields so that I have a marker
		// for bad input.
		let mut start: Point<usize> = Point {
			x: usize::MAX,
			y: usize::MAX,
		};
		let mut end = Point {
			x: usize::MAX,
			y: usize::MAX,
		};

		let maze = input
			.lines()
			.enumerate()
			.map(|(c_idx, line)| {
				line.chars()
					.enumerate()
					.map(|(r_idx, r)| match r {
						'S' => {
							start = Point { x: r_idx, y: c_idx };
							0
						}
						'E' => {
							end = Point { x: r_idx, y: c_idx };
							25
						}
						_ => (r as usize) - 97,
					})
					.collect()
			})
			.collect();

		// Check that we've assigned to `start` and `end`.
		assert_ne!(start, NULL_POINT, "Starting position not found.");
		assert_ne!(end, NULL_POINT, "Ending position not found.");
		Day12 { maze, start, end }
	}

	fn part_one(&self) -> u32 {
		let west_border = self.maze[0].len();
		let south_border = self.maze.len();

		let start_path = PathProgress {
			point: self.start,
			steps: 0,
		};
		let mut paths = BinaryHeap::new();
		paths.push(Reverse(start_path));
		let mut visited_points = HashSet::new();

		let mut steps_taken = 0;
		while let Some(Reverse(path)) = paths.pop() {
			if path.point == self.end {
				steps_taken = path.steps;
				for Reverse(path) in paths {
					eprintln!("Potential path: {:?}", path);
				}
				break;
			}
			if visited_points.contains(&path.point) {
				continue;
			}

			let steps_taken = path.steps + 1;
			visited_points.insert(path.point);
			let row = path.point.x;
			let col = path.point.y;
			let current_height = self.maze[col][row];

			// Check east of us.
			if path.point.x >= 1 {
				let point: Point<usize> = path.point - (1, 0);
				if self.maze[point.y][point.x] <= current_height + 1
					&& !visited_points.contains(&point)
				{
					paths.push(Reverse(PathProgress {
						point,
						steps: steps_taken,
					}));
				}
			}

			// Check north of us.
			if path.point.y >= 1 {
				let point = path.point - (0, 1);
				if self.maze[point.y][point.x] <= current_height + 1
					&& !visited_points.contains(&point)
				{
					paths.push(Reverse(PathProgress {
						point,
						steps: steps_taken,
					}));
				}
			}

			// Check west of us.
			let point = path.point + (1, 0);
			if point.x < west_border
				&& self.maze[point.y][point.x] <= current_height + 1
				&& !visited_points.contains(&point)
			{
				paths.push(Reverse(PathProgress {
					point,
					steps: steps_taken,
				}));
			}

			// Check south of us.
			let point = path.point + (0, 1);
			if point.y < south_border
				&& self.maze[point.y][point.x] <= current_height + 1
				&& !visited_points.contains(&point)
			{
				paths.push(Reverse(PathProgress {
					point,
					steps: steps_taken,
				}));
			}
		}
		steps_taken
	}
}

#[derive(Clone, PartialEq, Eq,  Debug)]
struct PathProgress {
	/// The current point of this path.
	point: Point<usize>,
	/// The number of steps taken so far.
	steps: u32,
}

impl Ord for PathProgress {
	fn cmp(&self, other: &Self) -> Ordering {
		self.steps
			.cmp(&other.steps)
			.then_with(|| self.point.cmp(&other.point))
	}
}

impl PartialOrd for PathProgress {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.point.partial_cmp(&other.point) {
			Some(core::cmp::Ordering::Equal) => {}
			ord => return ord,
		}
		self.steps.partial_cmp(&other.steps)
	}
}

fn main() {
	let maze = Day12::parse_input(&input_to_str());
	println!("The shortest path in the maze is {}", maze.part_one());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		use std::fs::File;
		use std::io::Read;

		let file = File::open("src/input/day12-example.txt");
		let mut file = file.expect("Example input file is needed.");

		let mut example = String::new();
		file.read_to_string(&mut example)
			.expect("Input reading has failed.");
		
		assert_eq!(Day12::parse_input(&example).part_one(), 31);
	}
}
