/// Day 9's Advent of Code puzzle
/// =============================
/// Puzzle input consists of a list of directions.
///
/// Part 1
/// ------
/// How many different positions does the tail of this rope meet?
use std::collections::HashSet;
use std::io::stdin;

/// Represents the rope. We're using signed numbers here since we're
/// going around in terms of the origin. As such, the starting point is (0, 0).
#[derive(Clone, Copy, Debug)]
struct Rope {
	head: (i32, i32),
	tail: (i32, i32),
}

impl Rope {
	/// Move the head of the rope, adjusting the tail as a side effect.
	fn move_head(&mut self, direction: Direction) {
		match direction {
			Direction::North => {
				self.head = (self.head.0 + 1, self.head.1);
			}
			Direction::South => {
				self.head = (self.head.0 - 1, self.head.1);
			}
			Direction::East => {
				self.head = (self.head.0, self.head.1 - 1);
			}
			Direction::West => {
				self.head = (self.head.0, self.head.1 + 1);
			}
		}
		self.adjust_tail();
	}

	fn move_head_to(&mut self, pos: (i32, i32)) {
		self.head = (self.head.0 + pos.0, self.tail.1 + pos.1);
		self.adjust_tail();
	}

	/// Move the tail position so that it's near the head position.
	fn adjust_tail(&mut self) {
		loop {
			let x_diff = self.head.1 - self.tail.1;
			let y_diff = self.head.0 - self.tail.0;
			match (x_diff, y_diff) {
				(0, 0) => break,
				(0, y) => {
					if y.abs() == 1 {
						break;
					} else if y > 0 {
						self.tail.0 += 1;
					} else {
						self.tail.0 -= 1;
					}
				}
				(x, 0) => {
					if x.abs() == 1 {
						break;
					} else if x > 0 {
						self.tail.1 += 1;
					} else {
						self.tail.1 -= 1;
					}
				}
				(x, y) => {
					if x.abs() == 1 && y.abs() == 1 {
						break;
					} else {
						if x.is_positive() {
							self.tail.1 += 1;
						} else if x.is_negative() {
							self.tail.1 -= 1;
						}
						if y.is_positive() {
							self.tail.0 += 1;
						} else if y.is_negative() {
							self.tail.0 -= 1;
						}
					}
				}
			}
		}
	}
}

/// A direction to move.
/// - North is considered `(+1, 0)`.
/// - South is considered `(-1, 0)`.
/// - East is considered `(0, -1)`.
/// - West is considered `(0, +1)`.
///
/// ...don't ask why it's `(y, x)` in this implementation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
	North,
	South,
	East,
	West,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction {
	direction: Direction,
	count: u32,
}

fn main() {
	let lines = stdin()
		.lines()
		.filter_map(|l| match l {
			Ok(l) => Some(l),
			Err(err) => panic!("Welp, your input failed: {}", err),
		})
		.collect::<Vec<_>>();
	let data = lines.join("\n");

	let tasks: Vec<Instruction> = parse_input(data);

	let positions = part_one(tasks);
	println!(
		"The number of positions the tail has reached is {}",
		positions
	);
	//
}

fn parse_input(task: String) -> Vec<Instruction> {
	let mut tasks = vec![];
	for line in task.lines() {
		let parts = line.split(' ').collect::<Vec<_>>();
		let direction = parts[0];
		let count = parts[1];
		let task = match direction {
			"U" => Instruction {
				direction: Direction::North,
				count: count.parse().expect("I can't count that high."),
			},
			"D" => Instruction {
				direction: Direction::South,
				count: count.parse().expect("I can't count that high."),
			},
			"L" => Instruction {
				direction: Direction::East,
				count: count.parse().expect("I can't count that high."),
			},
			"R" => Instruction {
				direction: Direction::West,
				count: count.parse().expect("I can't count that high."),
			},
			dir => panic!("Where do you want me to go? '{}'?", dir),
		};
		tasks.push(task);
	}
	tasks
}

fn part_one(tasks: Vec<Instruction>) -> usize {
	let mut rope = Rope {
		head: (0, 0),
		tail: (0, 0),
	};
	let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
	tail_positions.insert((0, 0));
	// Insert the origin, since we start there.
	for task in tasks {
		// For `count` number of times...
		for _ in 0..task.count {
			// Move the head of this rope.
			rope.move_head(task.direction);
			// If the tail has not already visited this position...
			if !tail_positions.contains(&rope.tail) {
				// ...add this position to the list.
				tail_positions.insert(rope.tail);
				eprintln!("Adding position {:?}...", rope.tail);
			}
		}
	}
	tail_positions.len()
}

fn part_two(tasks: Vec<Instruction>) -> usize {
	let rope = [Rope {head: (0,0), tail: (0, 0)}; 10];
	// s
	todo!();
}

#[cfg(test)]
mod tests {
	use super::*;

	const EXAMPLE: &str = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
	const TASKS: &[Instruction] = &[
		Instruction {
			direction: Direction::West,
			count: 4,
		},
		Instruction {
			direction: Direction::North,
			count: 4,
		},
		Instruction {
			direction: Direction::East,
			count: 3,
		},
		Instruction {
			direction: Direction::South,
			count: 1,
		},
		Instruction {
			direction: Direction::West,
			count: 4,
		},
		Instruction {
			direction: Direction::South,
			count: 1,
		},
		Instruction {
			direction: Direction::East,
			count: 5,
		},
		Instruction {
			direction: Direction::West,
			count: 2,
		},
	];

	#[test]
	fn test_parse_input() {
		assert_eq!(TASKS.to_owned(), parse_input(EXAMPLE.to_owned()));
	}

	#[test]
	fn test_part_one() {
		let tasks = TASKS.to_owned();
		assert_eq!(part_one(tasks), 13);
	}

	#[test]
	fn test_move_head() {
		use std::collections::HashSet as Set;

		// Brute force all of the known positions.
		let mut expected = Set::new();
		expected.insert((0, 0));
		expected.insert((0, 1));
		expected.insert((0, 2));
		expected.insert((0, 3));
		expected.insert((1, 4));
		expected.insert((2, 1));
		expected.insert((2, 2));
		expected.insert((2, 3));
		expected.insert((2, 4));
		expected.insert((3, 3));
		expected.insert((3, 4));
		expected.insert((4, 2));
		expected.insert((4, 3));

		let mut rope = Rope {
			head: (0, 0),
			tail: (0, 0),
		};
		let mut actual = Set::new();
		actual.insert((0, 0)); // Insert the default position.
		for task in TASKS {
			// For `count` number of times...
			for _ in 0..task.count {
				// Move the head of this rope.
				rope.move_head(task.direction);
				// If the tail has not already visited this position...
				if !actual.contains(&rope.tail) {
					// ...add this position to the list.
					actual.insert(rope.tail);
				}
			}
		}

		assert_eq!(expected, actual);
		//
	}

	#[test]
	fn test_diagonal_move_up() {
		let mut rope = Rope {
			head: (1, 4),
			tail: (0, 3),
		};
		rope.move_head(Direction::North);
		assert_eq!(rope.tail, (1, 4));
	}
}
