//! Day 16's Advent of Code puzzle
//! ==============================
//! Puzzle input consists of a list of valves.
//! 
//! Part 1
//! ------
//! What's the most pressure we can generate in 30 minutes?

use advent::{input_to_str, Advent};
use std::collections::HashMap;

#[derive(Debug)]
struct Day16(HashMap<String, Valve>);

/// Represents the data given from one line of puzzle input.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Valve {
	/// The flow rate of this valve.
	flow_rate: u32,
	/// The other valves this one links to.
	tunnels: Vec<String>,
}

impl Valve {
	fn pressure_at_time(&self, time: u32) -> u32 {
		self.flow_rate * time
	}

	fn traverse_tunnels(&self, cave: HashMap<String, Valve>) {
		// 
	}
}

impl Advent for Day16 {
	type Answer1 = u32;

	type Answer2 = ();

	fn parse_input(input: &str) -> Self {
		let mut valves = HashMap::new();
		// Format: "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
		for line in input.lines() {
			let words = line.split(' ').collect::<Vec<_>>();
			let key = words[1].to_owned();
			let flow_rate = words[4]
				.strip_prefix("rate=")
				.expect("This valve don't got flow!");
			let flow_rate = flow_rate
				.strip_suffix(';')
				.expect("Valve data missing, whine on Steam");
			let flow_rate = flow_rate
				.parse::<u32>()
				.expect("Valve isn't working, whine on Steam");
			let tunnels = words[9..]
				.join(" ")
				.split(", ")
				.map(|s| s.to_owned())
				.collect();

			valves.insert(key, Valve { flow_rate, tunnels });
		}

		Day16(valves)
	}

	fn part_one(&self) -> Self::Answer1 {
		todo!()
	}
}

fn main() {
	let runner = Day16::parse_input(&input_to_str());
	println!("Max pressure in 30 minutes is {}", runner.part_one());
}
