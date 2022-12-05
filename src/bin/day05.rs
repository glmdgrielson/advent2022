//! Day 5's Advent of Code puzzle
//! -----------------------------
//! I hate this one already.
//!
//! Puzzle input consists of a diagram of stacks as well as a list of
//! instructions. The actual logic itself is a simple Tower of Hanoi thing.
//! This is just gonna be a parsing headache. At the top is a "sketch", showing
//! rows of crates. Then there's an empty line.
//!
//! Then there's just a series of instructions. That should be easy. Ish.
//!
//! Part 1
//! ------
//! Read the top crate of each stack.

use std::fmt::{self, Display, Formatter};
use std::io::stdin;

/// This enum is copied from elsewhere so that I can
/// use my standard `stdin().lines()` method.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParserState {
	Stacks,
	Noise,
	Instructions,
}

#[derive(Clone, Copy, PartialEq, Eq)]
/// An instruction in the input. Could this be a tuple? Yeah, but I want to
/// be able to read this on Christmas Day, so struct it is!
struct Instruction {
	count: usize,
	source: usize,
	dest: usize,
}

impl Display for Instruction {
	/// This is technically unnecessary, but I thought it'd be handy.
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!(
			"move {} from {} to {}",
			self.count, self.source, self.dest
		))
	}
}

fn main() {
	let lines = stdin().lines();
	let mut stacks = Vec::new();
	let mut state = ParserState::Stacks;
	let mut instructions = Vec::new();
	for line in lines {
		if let Ok(line) = line {
			match state {
				ParserState::Stacks => {
					if line.starts_with(" 1") {
						// This is where the useful information about the stacks
						// STOPS, and as such, we can skip it.
						state = ParserState::Noise;
						continue;
					}
					let chars = line.chars().collect::<Vec<_>>();
					for (idx, val) in chars.chunks(4).enumerate() {
						if stacks.len() <= idx {
							stacks.push(vec![]);
						}
						if val[1] != ' ' {
							// And as such, matches `[?] `...
							stacks[idx].push(val[1]);
						}
					}
				}
				ParserState::Noise => {
					// This should just be an empty line, so we can skip it.
					state = ParserState::Instructions;
					// What we DO need to do is reverse all of the stacks.
					for stack in stacks.iter_mut() {
						stack.reverse();
					}
					continue;
				}
				ParserState::Instructions => {
					// Right, time for the fun part.
					// Line format is `move X from Y to Z`
					let words: Vec<_> = line.split(" ").collect();
					// This is after `move` in the input.
					let count = words[1].parse().expect(
						"There is no north of here, it's the North Pole!",
					);
					// This is after `from` in the input.
					let start: usize = words[3]
						.parse()
						.expect("This crane can't lift i crates, y'know.");
					// This is after `to` in the input.
					let stop: usize = words[5]
						.parse()
						.expect("Sure, just drop it off a cliff...");
					instructions.push(Instruction {
						count,
						// Since this isn't Lua, we need to decrement here.
						source: start - 1,
						// And here.
						dest: stop - 1,
					});
				}
			}
		}
	}
	instructions.into_iter().for_each(|task| {
		for _ in 0..task.count {
			let item =
				stacks[task.source].pop().expect("Can't stack dirt, Claus!");
			stacks[task.dest].push(item);
		}
	});
	// Iterators are fun.
	let result = stacks
		.iter()
		.map(|s| s.last())
		.filter(|s| s.is_some())
		.map(|s| s.expect("Was this supposed to be empty?"))
		.collect::<String>();
	println!("End result from above looks like {}", result);
	// todo!("Oh holy night, I hate this already.")
}
