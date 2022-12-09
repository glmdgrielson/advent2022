/// Day 9's Advent of Code puzzle
/// =============================
/// Puzzle input consists of a list of directions.
///
/// Part 1
/// ------
/// How many different positions does the tail of this rope meet?
use std::cmp::Ordering;
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
	/// Move the tail position so that it's near the head position.
	fn adjust_tail(&mut self) {
		// Adjust y value.
		match self.head.0.cmp(&self.tail.0) {
			Ordering::Less => {
				if self.head.0 != self.tail.0 - 1 {
					self.tail.0 -= 1;
				}
			}
			Ordering::Equal => {
				// nothing needs to be done here.
			}
			Ordering::Greater => {
				if self.head.0 != self.tail.0 + 1 {
					self.tail.0 += 1;
				}
			}
		}
		// Adjust x value.
		match self.head.1.cmp(&self.tail.1) {
			Ordering::Less => {
				if self.head.1 != self.tail.1 - 1 {
					self.tail.1 -= 1;
				}
			}
			Ordering::Equal => {
				// Already equal, we can leave this be.
			}
			Ordering::Greater => {
				if self.head.1 != self.tail.1 + 1 {
					self.tail.1 += 1;
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
		positions - 1
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
			}
		}
	}
	tail_positions.len()
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
		use std::collections::BTreeSet;
		let mut rope = Rope {
			head: (0, 0),
			tail: (0, 0),
		};
		let mut actual = BTreeSet::new();
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

		let mut expected = BTreeSet::new();
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

		assert_eq!(expected, actual);
		//
	}
}
