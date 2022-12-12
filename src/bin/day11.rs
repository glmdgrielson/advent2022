/// Day 11's Advent of Code puzzle
/// ==============================
/// Oh no, monkey business. Puzzle input consists of the behavior
/// of various monkeys. For once, all of it fits on a single screen
/// in my browser.
///
/// Part 1
/// ------
/// After 20 rounds of Monkey Business, what level of chaos have we reached?
use advent::{input_to_str, Advent};

/// An operation that a monkey can apply.
///
/// Both values take an [`Option`] because there's a chance the second
/// operand could be the pre-existing value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
	Add(Option<u32>),
	Multiply(Option<u32>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Monkey {
	/// The list of items this monkey currently has.
	items: Vec<u32>,
	/// The operation the monkey uses on your item.
	operation: Operation,
	/// The dividing factor your monkey uses to decide where to throw.
	factor: u32,
	/// The throw your monkey will make. The first value is on success,
	/// and the second value is on failure.
	decision: (usize, usize),
}

impl Monkey {
	/// Run the logic for all of the throws on this monkey's turn.
	///
	/// Note that as a side effect, this clears the monkey's list of items,
	/// since, y'know, the monkey just threw them all away.
	fn make_throws(&mut self) -> Vec<Throw> {
		let throws = self
			.items
			.iter()
			.map(|item| {
				let worry = match self.operation {
					Operation::Add(val) => match val {
						Some(val) => item + val,
						None => item + item,
					},
					Operation::Multiply(val) => match val {
						Some(val) => item * val,
						None => item * item,
					},
				};

				// Divide worry level by three, rounding down.
				let worry = worry / 3;

				let target = match worry % self.factor {
					0 => self.decision.0,
					_ => self.decision.1,
				};
				Throw {
					item: worry,
					dest: target,
				}
			})
			.collect();

		self.items = Vec::new();

		throws
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A struct to add names to data.
struct Throw {
	/// The item being thrown.
	item: u32,
	/// The monkey being thrown to.
	dest: usize,
}

#[derive(Clone, Debug)]
struct Day11(Vec<Monkey>);

// String constants used for parsing.
const OPERATION_PREFIX: &str = "  Operation: new = old";
const FACTOR_PREFIX: &str = "  Test: divisible by ";
const TRUE_PREFIX: &str = "    If true: throw to monkey ";
const FALSE_PREFIX: &str = "    If false: throw to monkey ";

impl Advent for Day11 {
	type Answer1 = usize;

	type Answer2 = usize;

	fn parse_input(input: &str) -> Self {
		let mut lines = input.lines();
		let mut monkeys = Vec::new();
		// I'm pretty sure Clippy is wrong about this one.
		// Unfortunately, this one is not a line by line iterator.
		#[allow(clippy::while_let_loop)]
		loop {
			// Match first line ("Monkey 0:")
			match lines.next() {
				Some(monkey) => {
					assert!(
						monkey.starts_with("Monkey "),
						"Well, things have gotten worse..."
					)
				}
				None => break,
			}
			// Parse "Starting items:" into our list.
			let items = lines
				.next()
				.expect("Wait, that monkey isn't doing anything.");
			let items = match items.strip_prefix("  Starting items: ") {
				Some(list) => list
					.split(", ")
					.map(|c| c.parse().expect("Wait, that's a banana..."))
					.collect(),
				None => panic!("Wait, those are coconuts... ({})", items),
			};

			// Parse "  Operation: new = old" and get the logic.
			let oper = lines.next().expect("We need more on the monkey!");
			let operation = match oper.strip_prefix(OPERATION_PREFIX) {
				Some(op) => {
					let mut parts = op.split(' ');
					parts.next(); // Skip an empty value.
					let operator = match parts.next() {
						Some(op) => op,
						None => panic!("Some math is needed here!"),
					};
					let operand =
						parts.next().expect("This verb needs an object!");
					let operand = match operand {
						"old" => None,
						_ => Some(
							operand
								.parse()
								.expect("Keep the variables out of this!"),
						),
					};
					match operator {
						"*" => Operation::Multiply(operand),
						"+" => Operation::Add(operand),
						_ => panic!("My calculator can't do that!"),
					}
				}
				None => panic!("What is that monkey doing?"),
			};

			let factor = lines.next().expect("We need more on the monkey!");
			let factor = factor
				.strip_prefix(FACTOR_PREFIX)
				.expect("So the monkey isn't doing anything? Ya sure?")
				.parse()
				.expect("That won't divide anything!");

			// Parse the behavior of this monkey.
			let yay = lines
				.next()
				.expect("We need more info on that monkey!")
				.strip_prefix(TRUE_PREFIX)
				.expect("Well it has to go somewhere!")
				.parse()
				.expect("Which monkey was that again?");
			let nay = lines
				.next()
				.expect("We need more info on that monkey!")
				.strip_prefix(FALSE_PREFIX)
				.expect("Well, it has to go somewhere!")
				.parse()
				.expect("Which monkey was that again?");

			// Skip an empty line.
			lines.next();

			let monkey = Monkey {
				items,
				operation,
				factor,
				decision: (yay, nay),
			};
			monkeys.push(monkey);
		}

		Day11(monkeys)
	}

	fn part_one(&self) -> usize {
		let mut monkeys = self.0.clone();
		// Hardcoding the number of monkeys. Hopefully this doesn't burn me.
		let mut checks = [0; 8];

		for _ in 0..20 {
			for idx in 0..monkeys.len() {
				checks[idx] += monkeys[idx].items.len();
				let throws = monkeys[idx].make_throws();
				for throw in throws {
					monkeys[throw.dest].items.push(throw.item);
				}
			}
		}

		// Sort the list of checks.
		checks.sort_by(|a, b| b.cmp(a));
		// Return the product of the top two results.
		checks[0] * checks[1]
	}

	fn part_two(&self) -> Self::Answer2 {
		let mut monkeys = self.0.clone();
		let factor: u32 = self.0.iter().map(|m| m.factor).product();
		let factor = factor as u64;
		// Hardcoding the number of monkeys. Hopefully this doesn't burn me.
		let mut checks = [0; 8];

		for _ in 0..10_000 {
			for idx in 0..monkeys.len() {
				checks[idx] += monkeys[idx].items.len();
				let throws: Vec<_> = {
					let throws = monkeys[idx]
						.items
						.iter()
						.map(|&item| {
							// Make this number less huge.
							let item = item as u64;

							let worry = match monkeys[idx].operation {
								Operation::Add(val) => match val {
									Some(val) => item + (val as u64),
									None => item + item,
								},
								Operation::Multiply(val) => match val {
									Some(val) => item * (val as u64),
									None => item * item,
								},
							};

							let worry: u32 = (worry % factor) as u32;

							let target = match worry % monkeys[idx].factor {
								0 => monkeys[idx].decision.0,
								_ => monkeys[idx].decision.1,
							};
							Throw {
								item: worry,
								dest: target,
							}
						})
						.collect();

					monkeys[idx].items = Vec::new();

					throws
				};
				for throw in throws {
					monkeys[throw.dest].items.push(throw.item);
				}
			}
		}

		// Sort the list of checks.
		checks.sort_by(|a, b| b.cmp(a));
		// Return the product of the top two results.
		checks[0] * checks[1]
	}
}

fn main() {
	let data = Day11::parse_input(&input_to_str());
	println!("The peak amount of monkey business is {}", data.part_one());
	println!(
		"The total amount of monkey business with anxiety is {}",
		data.part_two()
	);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_input() {
		let monkey = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n";

		let expected = vec![Monkey {
			items: vec![79, 98],
			operation: Operation::Multiply(Some(19)),
			factor: 23,
			decision: (2, 3),
		}];
		let runner = Day11::parse_input(monkey);
		let actual = runner.0;

		assert_eq!(expected, actual);
	}

	#[test]
	fn test_make_throws() {
		let mut monkey = Monkey {
			items: vec![79, 98],
			operation: Operation::Multiply(Some(19)),
			factor: 23,
			decision: (2, 3),
		};

		let expected =
			vec![Throw { item: 500, dest: 3 }, Throw { item: 620, dest: 3 }];
		let actual = monkey.make_throws();

		assert!(monkey.items.is_empty());
		assert_eq!(expected, actual);
	}

	#[test]
	fn test_part_two() {
		use std::fs::File;
		use std::io::Read;

		let mut file = File::open("src/input/day11-example.txt")
			.expect("File reading failed.");
		let mut example = String::new();
		file.read_to_string(&mut example)
			.expect("Reading has failed.");

		assert_eq!(Day11::parse_input(&example).part_two(), 2713310158);
	}
}
