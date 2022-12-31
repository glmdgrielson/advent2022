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
//! We are `humn`. What do we have to report such that both of `root`'s
//! partners equate to the same value?

use advent::{input_to_str, Advent};
use std::collections::HashMap;
use std::fmt::Display;

/// The squad of monkeys, matched with their names.
#[derive(Debug)]
struct Day21(HashMap<String, Monkey>);

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
		for line in input.lines() {
			// eprintln!("Monkey #{}: {}", index, line);
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
		Day21(monkeys)
	}

	fn part_one(&self) -> Self::Answer1 {
		self.0.get("root").expect("Missing root!").evaluate(&self.0)
	}

	fn part_two(&self) -> Self::Answer2 {
		let mut map = self.0.clone();
		map.entry("humn".to_owned())
			.and_modify(|m| *m = Monkey::Unknown);
		let root = self.0.get("root").expect("Missing root!");
		match root {
			// The root monkey is an operation, but we only care about
			// the names of the monkeys involved.
			Monkey::Operation { op: _, left, right } => {
				let left = map.get(left).expect("Missing monkey partner!");
				let right = map.get(right).expect("Missing monkey partner!");
				let one = left.try_evaluate(&map);
				let two = right.try_evaluate(&map);
				match (one, two) {
					(Ok(_), Ok(_)) => {
						unreachable!("And the human fits in where?")
					}
					(Ok(target), Err(part)) => {
						eprintln!("{} = {}", target, part);
						// Found a human on the right.
						part.equal_target(target)
					}
					(Err(part), Ok(target)) => {
						eprintln!("{} = {}", target, part);
						// Found a human on the left.
						part.equal_target(target)
					}
					(Err(_), Err(_)) => {
						unreachable!("Extraneous fleshbags detected!")
					}
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

type MonkeyTeam = HashMap<String, Monkey>;

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
			Monkey::Unknown => panic!("Human shouldn't be evaluated!"),
		}
	}

	/// Evaluate this monkey, but short circuit on `humn`.
	///
	/// Note that this function ignores the special casing for `root`.
	fn try_evaluate(&self, map: &MonkeyTeam) -> Result<i64, Partial> {
		match self {
			Monkey::Integer(num) => Ok(*num),
			Monkey::Operation { op, left, right } => {
				if left == "humn" {
					match op {
						Operation::Add => {
							let monkey =
								map.get(right).expect("Missing partner!");
							// Right side has to evaluate to something.
							let value = monkey.evaluate(map);
							Err(Partial::Add(
								Box::new(Partial::Value(None)),
								Box::new(Partial::Value(Some(value))),
							))
						}
						Operation::Sub => {
							let monkey =
								map.get(right).expect("Missing partner!");
							// Right side has to evaluate to something.
							let value = monkey.evaluate(map);
							Err(Partial::Sub(
								Box::new(Partial::Value(None)),
								Box::new(Partial::Value(Some(value))),
							))
						}
						Operation::Mul => {
							let monkey =
								map.get(right).expect("Missing partner!");
							// Right side has to evaluate to something.
							let value = monkey.evaluate(map);
							Err(Partial::Mul(
								Box::new(Partial::Value(None)),
								Box::new(Partial::Value(Some(value))),
							))
						}
						Operation::Div => {
							let monkey =
								map.get(right).expect("Missing partner!");
							// Right side has to evaluate to something.
							let value = monkey.evaluate(map);
							Err(Partial::Div(
								Box::new(Partial::Value(None)),
								Box::new(Partial::Value(Some(value))),
							))
						}
					}
				} else if right == "humn" {
					match op {
						Operation::Add => {
							let monkey =
								map.get(left).expect("Missing partner!");
							// Left side has to evaluate to something.
							let value = monkey.evaluate(map);
							Err(Partial::Add(
								Box::new(Partial::Value(Some(value))),
								Box::new(Partial::Value(None)),
							))
						}
						Operation::Sub => {
							let monkey =
								map.get(left).expect("Missing partner!");
							// Left side has to evaluate to something.
							let value = monkey.evaluate(map);
							Err(Partial::Sub(
								Box::new(Partial::Value(Some(value))),
								Box::new(Partial::Value(None)),
							))
						}
						Operation::Mul => {
							let monkey =
								map.get(left).expect("Missing partner!");
							// Left side has to evaluate to something.
							let value = monkey.evaluate(map);
							Err(Partial::Mul(
								Box::new(Partial::Value(Some(value))),
								Box::new(Partial::Value(None)),
							))
						}
						Operation::Div => {
							let monkey =
								map.get(left).expect("Missing partner!");
							// Left side has to evaluate to something.
							let value = monkey.evaluate(map);
							Err(Partial::Div(
								Box::new(Partial::Value(Some(value))),
								Box::new(Partial::Value(None)),
							))
						}
					}
				} else {
					// Both sides exist.
					let one = map.get(left).expect("Missing monkey Partner!");
					let two = map.get(right).expect("Missing monkey partner!");
					let one = one.try_evaluate(map);
					eprintln!("Left side: {} -> {:?}", left, one);
					let two = two.try_evaluate(map);
					eprintln!("Right side: {} -> {:?}", right, two);
					match (one, two) {
						// Both values are numbers; behave as in `evaluate`
						(Ok(one), Ok(two)) => match op {
							Operation::Add => Ok(one + two),
							Operation::Sub => Ok(one - two),
							Operation::Mul => Ok(one * two),
							Operation::Div => Ok(one / two),
						},
						// Left side is a number, right side has a human
						(Ok(one), Err(two)) => match op {
							Operation::Add => Err(Partial::Add(
								Box::new(Partial::Value(Some(one))),
								Box::new(two),
							)),
							Operation::Sub => Err(Partial::Sub(
								Box::new(Partial::Value(Some(one))),
								Box::new(two),
							)),
							Operation::Mul => Err(Partial::Mul(
								Box::new(Partial::Value(Some(one))),
								Box::new(two),
							)),
							Operation::Div => Err(Partial::Div(
								Box::new(Partial::Value(Some(one))),
								Box::new(two),
							)),
						},
						// Left side has a human, right side has a number
						(Err(one), Ok(two)) => match op {
							Operation::Add => Err(Partial::Add(
								Box::new(one),
								Box::new(Partial::Value(Some(two))),
							)),
							Operation::Sub => Err(Partial::Sub(
								Box::new(one),
								Box::new(Partial::Value(Some(two))),
							)),
							Operation::Mul => Err(Partial::Mul(
								Box::new(one),
								Box::new(Partial::Value(Some(two))),
							)),
							Operation::Div => Err(Partial::Div(
								Box::new(one),
								Box::new(Partial::Value(Some(two))),
							)),
						},
						// Both sides have humans?
						// I could probably just panic here, honestly...
						(Err(one), Err(two)) => match op {
							Operation::Add => {
								Err(Partial::Add(Box::new(one), Box::new(two)))
							}
							Operation::Sub => {
								Err(Partial::Sub(Box::new(one), Box::new(two)))
							}
							Operation::Mul => {
								Err(Partial::Mul(Box::new(one), Box::new(two)))
							}
							Operation::Div => {
								Err(Partial::Div(Box::new(one), Box::new(two)))
							}
						},
					}
					// match op {
					// 	Operation::Add => Ok(one + two),
					// 	Operation::Sub => Ok(one - two),
					// 	Operation::Mul => Ok(one * two),
					// 	Operation::Div => Ok(one / two),
					// }
				}
			}
			Monkey::Unknown => Err(Partial::Value(None)),
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

/// An enum representing a partial operation.
///
/// [`Add`][Partial::Add], [`Sub`][Partial::Sub], [`Mul`][Partial::Mul],
/// [`Div`][Partial::Div] all take two [boxed][Box] [`Partial`]s.
///
/// [`Value`][Partial::Value] is provided for completion. It takes an [Option]
/// where [`Some`] means that a value does exist and [`None`] means that
/// the value has been overwritten.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Partial {
	Add(Box<Partial>, Box<Partial>),
	Sub(Box<Partial>, Box<Partial>),
	Mul(Box<Partial>, Box<Partial>),
	Div(Box<Partial>, Box<Partial>),
	Value(Option<i64>),
}

impl Partial {
	/// Try to get a number out of this Partial.
	///
	/// Add, Sub, Mul, and Div all try to perform their respective operations,
	/// while Value just returns itself.
	fn try_evaluate(&self) -> Result<i64, Partial> {
		match self {
			Partial::Add(one, two) => {
				let one = &**one;
				let two = &**two;
				let one = one.try_evaluate();
				let two = two.try_evaluate();
				match (one, two) {
					(Ok(one), Ok(two)) => Ok(one + two),
					(Ok(one), Err(two)) => Err(Partial::Add(
						Box::new(Partial::Value(Some(one))),
						Box::new(two),
					)),
					(Err(two), Ok(one)) => Err(Partial::Add(
						Box::new(Partial::Value(Some(one))),
						Box::new(two),
					)),
					(Err(_), Err(_)) => {
						panic!("Should only have one empty value")
					}
				}
			}
			Partial::Sub(one, two) => {
				let one = &**one;
				let two = &**two;
				let one = one.try_evaluate();
				let two = two.try_evaluate();
				match (one, two) {
					(Ok(one), Ok(two)) => Ok(one - two),
					(Ok(one), Err(two)) => Err(Partial::Sub(
						Box::new(Partial::Value(Some(one))),
						Box::new(two),
					)),
					(Err(two), Ok(one)) => Err(Partial::Sub(
						Box::new(Partial::Value(Some(one))),
						Box::new(two),
					)),
					(Err(_), Err(_)) => {
						panic!("Should only have one empty value")
					}
				}
			}
			Partial::Mul(one, two) => {
				let one = &**one;
				let two = &**two;
				let one = one.try_evaluate();
				let two = two.try_evaluate();
				match (one, two) {
					(Ok(one), Ok(two)) => Ok(one * two),
					(Ok(one), Err(two)) => Err(Partial::Mul(
						Box::new(Partial::Value(Some(one))),
						Box::new(two),
					)),
					(Err(two), Ok(one)) => Err(Partial::Mul(
						Box::new(Partial::Value(Some(one))),
						Box::new(two),
					)),
					(Err(_), Err(_)) => {
						panic!("Should only have one empty value")
					}
				}
			}
			Partial::Div(one, two) => {
				let one = &**one;
				let two = &**two;
				let one = one.try_evaluate();
				let two = two.try_evaluate();
				match (one, two) {
					(Ok(one), Ok(two)) => Ok(one / two),
					(Ok(one), Err(two)) => Err(Partial::Div(
						Box::new(Partial::Value(Some(one))),
						Box::new(two),
					)),
					(Err(two), Ok(one)) => Err(Partial::Div(
						Box::new(Partial::Value(Some(one))),
						Box::new(two),
					)),
					(Err(_), Err(_)) => {
						panic!("Should only have one empty value")
					}
				}
			}
			Partial::Value(num) => match num {
				Some(num) => Ok(*num),
				None => Err(Partial::Value(None)),
			},
		}
	}

	/// If this Partial is incomplete (i.e. contains [`Value`][Partial::Value]
	/// ([`None`])), try to make this partial equal the given target.
	///
	/// If this Partial _is_ complete, return the result of evaluating it.
	fn equal_target(&self, target: i64) -> i64 {
		match self {
			Partial::Add(one, two) => {
				let one = one.try_evaluate();
				let two = two.try_evaluate();
				match (one, two) {
					(Ok(one), Ok(two)) => one * two,
					(Ok(yea), Err(nay)) => nay.equal_target(target - yea),
					(Err(nay), Ok(yea)) => nay.equal_target(target - yea),
					(Err(_), Err(_)) => {
						unreachable!("Only one monkey should be blank!")
					}
				}
			}
			Partial::Sub(one, two) => {
				let one = one.try_evaluate();
				let two = two.try_evaluate();
				match (one, two) {
					(Ok(one), Ok(two)) => one * two,
					(Ok(yea), Err(nay)) => {
						let target = target + yea;
						// let target = -target;
						nay.equal_target(target)
					}
					(Err(nay), Ok(yea)) => {
						let target = target - yea;
						let target = -target;
						nay.equal_target(target)
					},
					(Err(_), Err(_)) => {
						unreachable!("Only one monkey should be blank!")
					}
				}
			}
			Partial::Mul(one, two) => {
				let one = one.try_evaluate();
				let two = two.try_evaluate();
				match (one, two) {
					(Ok(one), Ok(two)) => one * two,
					(Ok(yea), Err(nay)) => nay.equal_target(target / yea),
					(Err(nay), Ok(yea)) => nay.equal_target(target / yea),
					(Err(_), Err(_)) => {
						unreachable!("Only one monkey should be blank!")
					}
				}
			}
			Partial::Div(one, two) => {
				let one = one.try_evaluate();
				let two = two.try_evaluate();
				match (one, two) {
					(Ok(one), Ok(two)) => one * two,
					(Ok(yea), Err(nay)) => nay.equal_target(yea / target),
					(Err(nay), Ok(yea)) => nay.equal_target(target * yea),
					(Err(_), Err(_)) => {
						unreachable!("Only one monkey should be blank!")
					}
				}
			}
			Partial::Value(num) => {
				if let Some(num) = num {
					*num
				} else {
					target
				}
			}
		}
		//
	}
}

impl Display for Partial {
	/// This implementation writes out a partial as if it were
	/// a mathematical equation.
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Partial::Add(one, two) => {
				f.write_fmt(format_args!("{} + {}", *one, *two))
			}
			Partial::Sub(one, two) => {
				f.write_fmt(format_args!("{} - {}", *one, *two))
			}
			Partial::Mul(one, two) => {
				f.write_fmt(format_args!("{} * {}", *one, *two))
			}
			Partial::Div(one, two) => {
				f.write_fmt(format_args!("{} / {}", *one, *two))
			}
			Partial::Value(num) => {
				if let Some(num) = num {
					f.write_fmt(format_args!("{}", num))
				} else {
					f.write_str("humn")
				}
			}
		}
	}
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
	fn test_try_evaluate() {
		use advent::get_example_input;
		let example = Day21::parse_input(&get_example_input(
			"src/input/day21-example.txt",
		));

		assert_eq!(example.part_two(), 301);
	}
}
