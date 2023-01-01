//! Day 16's Advent of Code puzzle
//! ==============================
//! Puzzle input consists of a list of valves.
//!
//! Part 1
//! ------
//! What's the most pressure we can generate in 30 minutes?

use advent::{input_to_str, Advent};
use std::collections::{BinaryHeap, HashMap, HashSet};

/// The total amount of time we have to work with.
const TOTAL_TIME: u32 = 30;

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
	fn get_pressure_at_time(&self, time: u32) -> u32 {
		self.flow_rate * time
	}
}

/// A struct representing the state of traversal.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Progress {
	/// The current pressure being released at the moment.
	pub released_pressure: u32,
	/// The amount of time passed at this moment..
	pub time_elapsed: u32,
	/// The valve currently being visited.
	pub current_valve: String,
	/// The names of all of the currently open valves.
	pub open_valves: HashSet<String>,
	/// The list of pathways that need not be explored at the current valve.
	pub visited_valves: HashSet<String>,
}

impl Default for Progress {
	fn default() -> Self {
		Progress {
			released_pressure: 0,
			time_elapsed: 0,
			current_valve: String::from("AA"),
			open_valves: HashSet::new(),
			visited_valves: HashSet::new(),
		}
	}
}

impl PartialOrd for Progress {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Progress {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.released_pressure
			.cmp(&other.released_pressure)
			.then_with(|| self.time_elapsed.cmp(&other.time_elapsed))
			.then_with(|| self.open_valves.len().cmp(&other.open_valves.len()))
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
		let initial_progress = Progress::default();
		let mut progress_opts = BinaryHeap::new();
		progress_opts.push(initial_progress);
		let working_valves = (self.0)
			.iter()
			.filter_map(|(name, valve)| {
				if valve.flow_rate > 0 {
					Some(name.clone())
				} else {
					None
				}
			})
			.collect::<HashSet<_>>();

		let mut max_pressure = 0;
		while let Some(mut progress) = progress_opts.pop() {
			if progress.open_valves == working_valves {
				// Every valve that works has been opened.
				// There's nothing left for us to do.
				if progress.released_pressure > max_pressure {
					max_pressure = progress.released_pressure;
				}
				continue;
			}

			// Increment the time.
			let time = progress.time_elapsed + 1;
			if time >= TOTAL_TIME {
				// We've hit our maximum, stop doing work.
				if progress.released_pressure > max_pressure {
					max_pressure = progress.released_pressure;
				}
				continue;
			}

			// Adjust time.
			progress.time_elapsed = time;
			// Mark this valve as visited.
			progress
				.visited_valves
				.insert(progress.current_valve.clone());

			// Get a handle of the valve we care about.
			let valve = (self.0)
				.get(&progress.current_valve)
				.expect("Missing valve here!");

			// Open this valve.
			if !progress.open_valves.contains(&progress.current_valve)
				&& valve.flow_rate > 0
			{
				let mut next_progress = progress.clone();
				next_progress
					.open_valves
					.insert(progress.current_valve.clone());
				next_progress.released_pressure += valve
					.get_pressure_at_time(TOTAL_TIME - progress.time_elapsed);
				next_progress.visited_valves.clear();
				progress_opts.push(next_progress);
			}

			// Visit other valves.
			for next_valve in valve
				.tunnels
				.iter()
				.filter(|&v| !progress.visited_valves.contains(v))
			{
				let mut next_progress = progress.clone();
				next_progress.current_valve = next_valve.clone();
				progress_opts.push(next_progress);
			}
		}
		max_pressure
	}
}

fn main() {
	let runner = Day16::parse_input(&input_to_str());
	println!("Max pressure in 30 minutes is {}", runner.part_one());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_part_one() {
		use advent::get_example_input;

		let example = get_example_input("src/input/day16-example.txt");
		let runner = Day16::parse_input(&example);

		assert_eq!(runner.part_one(), 1651);
	}
}
