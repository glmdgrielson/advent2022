//! Day 20's Advent of Code puzzle
//! ==============================
//! Monkey math!
//!
//! Part 1
//! ------
//! What does the `root` monkey evaluate to?

#![allow(dead_code)]

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
	type Answer1 = u64;

	type Answer2 = ();

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
			match op.parse::<u64>() {
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
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Monkey {
	Integer(u64),
	Operation {
		op: Operation,
		left: String,
		right: String,
	},
}

impl Monkey {
	fn evaluate(&self, map: &HashMap<String, Monkey>) -> u64 {
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
}
