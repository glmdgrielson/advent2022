//! Day 20's Advent of Code puzzle
//! ==============================
//! Monkey math!
//!
//! Part 1
//! ------
//! What does the `root` monkey evaluate to?
//!
//! Part 2
//! ------
//! We are `humn`. What do we need to report so that both of the monkeys
//! `root` cares about result in the same value?

use advent::{input_to_str, Advent};
use std::collections::HashMap;

#[derive(Debug)]
struct Day21 {
	/// The whole squad of monkeys.
	team: HashMap<String, Monkey>,
	// /// Specifically the Monkey part one cares about.
	// root: Monkey,
}

impl Advent for Day21 {
	type Answer1 = i64;

	type Answer2 = i64;

	fn parse_input(input: &str) -> Self {
		let names = input
			.lines()
			.map(|l| {
				// The names all have the same length so this hack is fine?
				l.split_at(4)
			})
			.collect::<HashMap<_, _>>();
		let mut monkeys = names
			.keys()
			.map(|&k| (k.to_owned(), Monkey::Integer(0)))
			.collect::<HashMap<_, _>>();
		for (index, line) in input.lines().enumerate() {
			eprintln!("Monkey #{}: {}", index, line);
			let (name, op) = line.split_at(4);
			let name = name.to_string();
			let op = op.strip_prefix(": ").expect("Missing separator");
			match op.parse::<i64>() {
				Ok(num) => {
					monkeys.entry(name).and_modify(|m| {
						*m = Monkey::Integer(num);
					});
				}
				Err(_) => {
					let words = op.split(' ').collect::<Vec<_>>();
					// The monkey on the left.
					let left = words[0];
					// The actual operation occuring.
					let func = match words[1] {
						"+" => Operation::Add,
						"-" => Operation::Sub,
						"*" => Operation::Mul,
						"/" => Operation::Div,
						_ => unreachable!("Invalid monkey business"),
					};
					// The monkey on the right.
					let right = words[2];
					monkeys.entry(name).and_modify(|m| {
						*m = Monkey::Operation {
							op: func,
							left: left.to_owned(),
							right: right.to_owned(),
						};
					});
				}
			}
		}
		Day21 { team: monkeys }
	}

	fn part_one(&self) -> Self::Answer1 {
		self.team
			.get("root")
			.expect("Missing root!")
			.evaluate(&self.team)
	}

	fn part_two(&self) -> Self::Answer2 {
		let mut team = self.team.clone();
		team.entry("humn".to_owned()).and_modify(|m| {
			*m = Monkey::Unknown;
		});
		let root = self.team.get("root").expect("Missing root!");
		match root {
			Monkey::Operation { op: _, left, right } => {
				let left = team.get(left).expect("Missing partner!");
				let right = team.get(right).expect("Missing partner!");
				let one = left.try_evaluate(&team);
				let two = right.try_evaluate(&team);
				match (one, two) {
					(Some(_), Some(_)) => panic!("Where's the human?"),
					(None, Some(target)) => left.equal_target(&team, target),
					(Some(target), None) => right.equal_target(&team, target),
					(None, None) => panic!("Too many humans!"),
				}
			}
			_ => panic!("Invalid root!"),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Monkey {
	Integer(i64),
	Operation {
		op: Operation,
		left: String,
		right: String,
	},
	Unknown,
}

impl Monkey {
	fn evaluate(&self, map: &HashMap<String, Monkey>) -> i64 {
		match self {
			Monkey::Integer(num) => *num,
			Monkey::Operation { op, left, right } => {
				let left = map.get(left).expect("Missing a monkey!");
				let right = map.get(right).expect("Missing a monkey!");
				match op {
					Operation::Add => left.evaluate(map) + right.evaluate(map),
					Operation::Sub => left.evaluate(map) - right.evaluate(map),
					Operation::Mul => left.evaluate(map) * right.evaluate(map),
					Operation::Div => left.evaluate(map) / right.evaluate(map),
				}
			}
			Monkey::Unknown => panic!("Human stupid"),
		}
	}

	/// Tries to evaluate this monkey. If the human is found somewhere,
	/// it returns [`None`].
	fn try_evaluate(&self, map: &HashMap<String, Monkey>) -> Option<i64> {
		if self.has_unknown_input(map) {
			match self {
				Monkey::Integer(_) => {
					unreachable!("Constant evaluation failed!")
				}
				Monkey::Operation { op, left, right } => {
					let left = map.get(left).expect("Missing partner!");
					let right = map.get(right).expect("Missing partner!");
					let one = left.try_evaluate(map);
					let two = right.try_evaluate(map);
					match (one, two) {
						(Some(one), Some(two)) => match op {
							Operation::Add => Some(one + two),
							Operation::Sub => Some(one - two),
							Operation::Mul => Some(one * two),
							Operation::Div => Some(one / two),
						},
						(None, Some(_)) => None,
						(Some(_), None) => None,
						(None, None) => {
							unreachable!("Extraneous humans found!")
						}
					}
				}
				Monkey::Unknown => None,
			}
		} else {
			Some(self.evaluate(map))
		}
	}

	fn has_unknown_input(&self, map: &HashMap<String, Monkey>) -> bool {
		match self {
			Monkey::Integer(_) => false,
			Monkey::Operation { op: _, left, right } => {
				let left = map.get(left).expect("Missing partner!");
				let right = map.get(right).expect("Missing partner!");
				left.has_unknown_input(map) || right.has_unknown_input(map)
			}
			Monkey::Unknown => true,
		}
	}

	/// Attempt to resolve this monkey into equaling the target value.
	fn equal_target(&self, map: &HashMap<String, Monkey>, target: i64) -> i64 {
		match self {
			Monkey::Integer(num) => *num,
			Monkey::Operation { op, left, right } => {
				let left = map.get(left).expect("Missing partner!");
				let right = map.get(right).expect("Missing partner!");
				let one = left.try_evaluate(map);
				let two = right.try_evaluate(map);
				match op {
					Operation::Add => {
						// target = constant + unknown
						if let Some(num) = one {
							assert!(two.is_none());
							right.equal_target(map, target - num)
						} else if let Some(num) = two {
							left.equal_target(map, target - num)
						} else {
							panic!("Monkey haystack failure!")
						}
					}
					Operation::Sub => {
						if let Some(num) = one {
							// target = constant - unknown
							// Ergo, unknown = constant - target
							assert!(two.is_none());
							right.equal_target(map, num - target)
						} else if let Some(num) = two {
							// target = unknown - constant
							// Ergo, unknown = target + constant
							left.equal_target(map, target + num)
						} else {
							panic!("Monkey haystack failure!")
						}
					}
					Operation::Mul => {
						// target = constant * unknown
						// Ergo, unknown = target / constant
						if let Some(num) = one {
							assert!(two.is_none());
							right.equal_target(map, target / num)
						} else if let Some(num) = two {
							left.equal_target(map, target / num)
						} else {
							panic!("Monkey haystack failure!")
						}
					},
					Operation::Div => {
						if let Some(num) = one {
							// target = constant / unknown
							// Ergo, unknown = constant / target
							assert!(two.is_none());
							right.equal_target(map, num / target)
						} else if let Some(num) = two {
							// target = unknown / constant
							// Ergo, unknown = target * constant
							left.equal_target(map, target * num)
						} else {
							panic!("Monkey haystack failure!")
						}
					},
				}
			}
			Monkey::Unknown => target,
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
	Add,
	Sub,
	Mul,
	Div,
}

fn main() {
	let runner = Day21::parse_input(&input_to_str());
	println!("Root monkey says {}", runner.part_one());
	println!("Human monkey needs to say {}", runner.part_two());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_two() {
		use advent::get_example_input;
		let example = get_example_input("src/input/day21-example.txt");
		let runner = Day21::parse_input(&example);

		assert_eq!(runner.part_two(), 301)
		// let runner = 
	}
}
