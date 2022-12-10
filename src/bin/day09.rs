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

	/// Move the tail position so that it's near the head position.
	fn adjust_tail(&mut self) {
		self.tail = adjust_tail(self.head, self.tail);
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

	let positions = part_one(tasks.clone());
	println!(
		"The number of positions the tail has reached is {}",
		positions
	);

	let extra_positions = part_two(tasks);
	println!(
		"The number of positions the guy way in the back has reached is {}",
		extra_positions
	);
	//
}

fn parse_input(task: String) -> Vec<Instruction> {
	let mut tasks = vec![];
	for line in task.lines() {
		let parts = line.split(' ').collect::<Vec<_>>();
		let direction = parts[0];
		let count = parts[1].parse().expect("I can't count that high.");
		let task = match direction {
			"U" => Instruction {
				direction: Direction::North,
				count,
			},
			"D" => Instruction {
				direction: Direction::South,
				count,
			},
			"L" => Instruction {
				direction: Direction::East,
				count,
			},
			"R" => Instruction {
				direction: Direction::West,
				count,
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

fn part_two(tasks: Vec<Instruction>) -> usize {
	let mut rope = [(0, 0); 10];

	let mut tail_positions = HashSet::new();
	tail_positions.insert((0, 0));
	for task in tasks {
		for _ in 0..task.count {
			match task.direction {
				Direction::North => {
					rope[0] = (rope[0].0 + 1, rope[0].1);
				}
				Direction::South => {
					rope[0] = (rope[0].0 - 1, rope[0].1);
				}
				Direction::East => {
					rope[0] = (rope[0].0, rope[0].1 - 1);
				}
				Direction::West => {
					rope[0] = (rope[0].0, rope[0].1 + 1);
				}
			}
			for idx in 1..rope.len() {
				rope[idx] = adjust_tail(rope[idx - 1], rope[idx]);
			}
			if !tail_positions.contains(&rope[9]) {
				tail_positions.insert(rope[9]);
			}
		}
	}
	tail_positions.len()
}

fn adjust_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
	let mut tail = tail;
	loop {
		let x_diff = head.1 - tail.1;
		let y_diff = head.0 - tail.0;
		match (x_diff, y_diff) {
			(0, 0) => break,
			(0, y) => {
				if y.abs() == 1 {
					break;
				} else if y > 0 {
					tail.0 += 1;
				} else {
					tail.0 -= 1;
				}
			}
			(x, 0) => {
				if x.abs() == 1 {
					break;
				} else if x > 0 {
					tail.1 += 1;
				} else {
					tail.1 -= 1;
				}
			}
			(x, y) => {
				if x.abs() == 1 && y.abs() == 1 {
					break;
				} else {
					if x.is_positive() {
						tail.1 += 1;
					} else if x.is_negative() {
						tail.1 -= 1;
					}
					if y.is_positive() {
						tail.0 += 1;
					} else if y.is_negative() {
						tail.0 -= 1;
					}
				}
			}
		}
	}
	tail
}
