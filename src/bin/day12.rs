/// Day 12's Advent of Code puzzle
/// ==============================
/// Puzzle input consists of _MAZE_.
///
/// Part 1
/// ------
/// Find the shortest path from `S` to `E`.
///
/// Part 2
/// ------
/// Find the shortest path from the end to any point of lowest elevation.
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};

use advent::{input_to_str, Advent};

#[derive(Debug, Clone)]
struct Day12 {
	maze: Vec<Vec<usize>>,
	start: Coordinate,
	end: Coordinate,
}

impl Advent for Day12 {
	type Answer1 = u32;

	type Answer2 = u32;

	fn parse_input(input: &str) -> Self {
		let mut start_coord: Option<Coordinate> = None;
		let mut end_coord: Option<Coordinate> = None;
		let height_map: Vec<Vec<usize>> = input
			.lines()
			.enumerate()
			.map(|(line_index, line)| {
				line.chars()
					.enumerate()
					.map(|(c_index, c)| match c {
						'S' => {
							start_coord = Some(Coordinate {
								x: c_index,
								y: line_index,
							});
							0
						}
						'E' => {
							end_coord = Some(Coordinate {
								x: c_index,
								y: line_index,
							});
							25
						}
						_ => (c as usize) - 97,
					})
					.collect()
			})
			.collect();
		let start = start_coord.expect("Starting point not found.");
		let end = end_coord.expect("Ending coordinate not found.");

		Day12 {
			maze: height_map,
			start,
			end,
		}
	}

	fn part_one(&self) -> Self::Answer1 {
		let start_path = PathProgress {
			coord: self.start,
			steps_taken: 0,
		};
		let mut paths = BinaryHeap::new();
		paths.push(Reverse(start_path));
		let mut visited_coords: HashSet<Coordinate> = HashSet::new();

		let mut steps_taken = 0;
		while let Some(Reverse(path)) = paths.pop() {
			if path.coord == self.end {
				steps_taken = path.steps_taken;
				break;
			}
			if visited_coords.contains(&path.coord) {
				continue;
			}

			let steps_taken = path.steps_taken + 1;
			visited_coords.insert(path.coord);
			let current_height = self.maze[path.coord.y][path.coord.x];

			if let Some(left_coord) = path.coord.left() {
				if self.maze[left_coord.y][left_coord.x] <= current_height + 1
					&& !visited_coords.contains(&left_coord)
				{
					paths.push(Reverse(PathProgress {
						coord: left_coord,
						steps_taken,
					}));
				}
			}

			if let Some(up_coord) = path.coord.up() {
				if self.maze[up_coord.y][up_coord.x] <= current_height + 1
					&& !visited_coords.contains(&up_coord)
				{
					paths.push(Reverse(PathProgress {
						coord: up_coord,
						steps_taken,
					}));
				}
			}

			let right_coord = path.coord.right();
			if right_coord.x < self.maze[right_coord.y].len()
				&& self.maze[right_coord.y][right_coord.x] <= current_height + 1
				&& !visited_coords.contains(&right_coord)
			{
				paths.push(Reverse(PathProgress {
					coord: right_coord,
					steps_taken,
				}));
			}

			let down_coord = path.coord.down();
			if down_coord.y < self.maze.len()
				&& self.maze[down_coord.y][down_coord.x] <= current_height + 1
				&& !visited_coords.contains(&down_coord)
			{
				paths.push(Reverse(PathProgress {
					coord: down_coord,
					steps_taken,
				}));
			}
		}

		steps_taken
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl Coordinate {
	fn left(&self) -> Option<Self> {
		if self.x > 0 {
			Some(Self {
				x: self.x - 1,
				y: self.y,
			})
		} else {
			None
		}
	}

	fn up(&self) -> Option<Self> {
		if self.y > 0 {
			Some(Self {
				x: self.x,
				y: self.y - 1,
			})
		} else {
			None
		}
	}

	fn right(&self) -> Self {
		Self {
			x: self.x + 1,
			y: self.y,
		}
	}

	fn down(&self) -> Self {
		Self {
			x: self.x,
			y: self.y + 1,
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PathProgress {
	coord: Coordinate,
	steps_taken: u32,
}

impl Ord for PathProgress {
	fn cmp(&self, other: &Self) -> Ordering {
		self.steps_taken
			.cmp(&other.steps_taken)
			.then_with(|| self.coord.cmp(&other.coord))
	}
}

impl PartialOrd for PathProgress {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

fn main() {
	let runner = Day12::parse_input(&input_to_str());
	println!("Path to the top takes {} steps", runner.part_one());
}
