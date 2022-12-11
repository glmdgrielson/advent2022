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
/// Both values take an option because there's a chance the second
/// operand could be a pre-existing value.
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
	decision: (u32, u32),
}

#[derive(Clone, Debug)]
struct Day11(Vec<Monkey>);

// String constants used for parsing.
const OPERATION_PREFIX: &str = "Operation: new = old";
const FACTOR_PREFIX: &str = "Test: divisible by ";
const TRUE_PREFIX: &str = "If true: throw to monkey ";
const FALSE_PREFIX: &str = "If false: throw to monkey ";

impl Advent for Day11 {
	type Answer1 = ();

	type Answer2 = ();

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
			let items = match items.strip_prefix("Starting items: ") {
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

	fn part_one(&self) -> Self::Answer1 {
		todo!()
	}
}

fn main() {
	let data = Day11::parse_input(&input_to_str());
	data.part_one();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_input() {
		let monkey = "Monkey 0:\n\
		Starting items: 79, 98\n\
		Operation: new = old * 19\n\
		Test: divisible by 23\n\
		  If true: throw to monkey 2\n\
		  If false: throw to monkey 3\n";

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
}
