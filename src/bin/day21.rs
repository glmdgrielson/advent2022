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
		todo!()
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
			},
			Monkey::Unknown => panic!("Human stupid"),
		}
	}

	fn try_evaluate(&self, map: &HashMap<String, Monkey>) -> Option<i64> {
		// 
		todo!()
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
