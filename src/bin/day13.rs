//! Day 13's Advent of Code puzzle
//! ==============================
//! Puzzle input consists of pairs of packets.
//!
//! Oh this is gonna be painful.
//!
//! Part 1
//! ------
//! Determine which packets are not in the right order.

use advent::{input_to_str, Advent};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Day13(Vec<(PacketData, PacketData)>);

#[derive(Clone, Debug)]
enum PacketData {
	List(Vec<PacketData>),
	Number(u32),
}

impl PartialOrd for PacketData {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for PacketData {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(PacketData::List(x), PacketData::List(y)) => {
				let len = x.len().max(y.len());
				for idx in 0..len {
					if idx >= x.len() {
						return Ordering::Less;
					}
					if idx >= y.len() {
						return Ordering::Greater;
					}
					let res = x[idx].compare(&y[idx]);
					if res.is_ne() {
						return res;
					}
				}
				Ordering::Equal
			}
			(PacketData::List(_), PacketData::Number(num)) => {
				self.cmp(&PacketData::List(vec![PacketData::Number(*num)]))
			}
			(PacketData::Number(num), PacketData::List(_)) => {
				PacketData::List(vec![PacketData::Number(*num)]).cmp(other)
			}
			(PacketData::Number(this), PacketData::Number(that)) => {
				this.cmp(that)
			}
		}
	}
}

impl PartialEq for PacketData {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::List(x), Self::List(y)) => {
				if x.len() == y.len() {
					for idx in 0..x.len() {
						if x[idx] != y[idx] {
							return false;
						}
					}
					true
				} else {
					false
				}
			}
			(Self::Number(l0), Self::Number(r0)) => l0 == r0,
			_ => false,
		}
	}
}

impl Eq for PacketData {}

impl PacketData {
	fn compare(&self, other: &PacketData) -> Ordering {
		match (self, other) {
			(PacketData::List(x), PacketData::List(y)) => {
				let len = x.len().max(y.len());
				for idx in 0..len {
					if idx >= x.len() {
						return Ordering::Less;
					}
					if idx >= y.len() {
						return Ordering::Greater;
					}
					let res = x[idx].compare(&y[idx]);
					if res.is_ne() {
						return res;
					}
				}
				Ordering::Equal
			}
			(PacketData::List(_), PacketData::Number(num)) => {
				self.compare(&PacketData::List(vec![PacketData::Number(*num)]))
			}
			(PacketData::Number(num), PacketData::List(_)) => {
				PacketData::List(vec![PacketData::Number(*num)]).compare(other)
			}
			(PacketData::Number(this), PacketData::Number(that)) => {
				this.cmp(that)
			}
		}
	}
}

impl Advent for Day13 {
	type Answer1 = usize;

	type Answer2 = usize;

	fn parse_input(input: &str) -> Self {
		let mut pairs = Vec::new();
		let mut lines = input.lines();
		loop {
			let one = lines.next().expect("Incomplete input detected!");
			let two = lines.next().expect("Incomplete input detected!");
			if lines.next().is_none() {
				// End of file reached, we're done here.
				break;
			}

			let one = parse_packet(one);
			let two = parse_packet(two);

			pairs.push((one, two));
		}
		Day13(pairs)
	}

	fn part_one(&self) -> Self::Answer1 {
		let mut idx_sum = 0;
		for (pair_idx, (this, that)) in self.0.iter().enumerate() {
			let this = match this {
				PacketData::List(list) => list,
				PacketData::Number(_) => panic!("Root value should be list."),
			};
			let that = match that {
				PacketData::List(list) => list,
				PacketData::Number(_) => panic!("Root value should be list."),
			};

			for inner_idx in 0..this.len().max(that.len()) {
				if inner_idx >= this.len() {
					// Packet 2 is longer than packet 1.
					// This is an error; add its index to the sum.
					idx_sum += pair_idx + 1;
					eprintln!("Found mismatch {}", pair_idx + 1);
					break;
				}

				if inner_idx >= that.len() {
					// Packet 1 is longer than packet 2.
					break;
				}

				// Check if this element is less than that element.
				let ordered = this[inner_idx].cmp(&that[inner_idx]);
				if ordered.is_ne() {
					if ordered.is_lt() {
						idx_sum += pair_idx + 1;
						eprintln!("Found mismatch {}", pair_idx + 1);
					}
					break;
				}
			}
		}
		idx_sum
	}

	fn part_two(&self) -> Self::Answer2 {
		let mark_one =
			PacketData::List(vec![PacketData::List(vec![PacketData::Number(
				2,
			)])]);
		let mark_two =
			PacketData::List(vec![PacketData::List(vec![PacketData::Number(
				6,
			)])]);

		let mut packets = vec![mark_one.clone(), mark_two.clone()];
		for (one, two) in self.0.iter() {
			packets.push(one.clone());
			packets.push(two.clone());
		}
		packets.sort();
		let idx_one = packets.iter().position(|packet| packet == &mark_one);
		let idx_two = packets.iter().position(|packet| packet == &mark_two);

		match (idx_one, idx_two) {
			(Some(one), Some(two)) => {
				// These are one-indexed in the solution.
				(one + 1) * (two + 1)
			}
			_ => unreachable!("Cannot find dividers!"),
		}
	}
}

/// This takes packet input and returns with parsed [`PacketData`].
fn parse_packet(data: &str) -> PacketData {
	let packet_str = data.strip_prefix('[').expect("Invalid packet format");
	let packet_str =
		packet_str.strip_suffix(']').expect("Invalid packet format");
	let mut packet_list = vec![Vec::new()];
	for entry in packet_str.split(',') {
		let mut datum = entry;
		// Remove open brackets
		while let Some(stripped) = datum.strip_prefix('[') {
			packet_list.push(Vec::new());
			datum = stripped;
		}

		// Remove close brackets
		let mut nest_levels = 0;
		while let Some(stripped) = datum.strip_suffix(']') {
			nest_levels += 1;
			datum = stripped;
		}

		// Check for empty lists
		if !datum.is_empty() {
			// Parse a number
			packet_list
				.last_mut()
				.expect("Parse state error occured!")
				.push(PacketData::Number(
					datum.parse().expect("Malformed number detected"),
				));
		}

		for _ in 0..nest_levels {
			let resolve_list =
				packet_list.pop().expect("Improper nesting detected");
			packet_list
				.last_mut()
				.expect("Parse state error occured")
				.push(PacketData::List(resolve_list))
		}
	}
	assert_eq!(packet_list.len(), 1, "Packet failed to parse");
	// packet_list[0]
	PacketData::List(packet_list[0].clone())
}

fn main() {
	let runner = Day13::parse_input(&input_to_str());
	println!("The sum of mismatched pairs is {}", runner.part_one());
}

fn _copied_parse(
	input_string: &str,
) -> Vec<(Vec<PacketData>, Vec<PacketData>)> {
	let mut packets = Vec::new();

	let mut first_packet_list_parse: Vec<Vec<PacketData>> = vec![Vec::new()];
	let mut second_packet_list_parse: Vec<Vec<PacketData>> = vec![Vec::new()];
	let mut parsing_second = false;
	for packet_str in input_string.lines().filter(|s| !s.is_empty()) {
		let packet_str =
			packet_str.strip_prefix('[').expect("Invalid packet format");
		let packet_str =
			packet_str.strip_suffix(']').expect("Invalid packet format");

		for entry in packet_str.split(',') {
			let mut datum = entry;
			while let Some(datum_stripped) = datum.strip_prefix('[') {
				if parsing_second {
					second_packet_list_parse.push(Vec::new());
				} else {
					first_packet_list_parse.push(Vec::new());
				}
				datum = datum_stripped;
			}

			let mut resolve_levels: u32 = 0;
			while let Some(datum_stripped) = datum.strip_suffix(']') {
				resolve_levels += 1;
				datum = datum_stripped;
			}

			if !datum.is_empty() {
				if parsing_second {
					second_packet_list_parse.last_mut().unwrap().push(
						PacketData::Number(
							datum.parse().expect("Malformed number detected"),
						),
					);
				} else {
					first_packet_list_parse.last_mut().unwrap().push(
						PacketData::Number(
							datum.parse().expect("Malformed number detected"),
						),
					);
				}
			}

			for _ in 0..resolve_levels {
				if parsing_second {
					let resolve_list = second_packet_list_parse.pop().unwrap();
					second_packet_list_parse
						.last_mut()
						.unwrap()
						.push(PacketData::List(resolve_list));
				} else {
					let resolve_list = first_packet_list_parse.pop().unwrap();
					first_packet_list_parse
						.last_mut()
						.unwrap()
						.push(PacketData::List(resolve_list));
				}
			}
		}

		if parsing_second {
			parsing_second = false;
			assert!(
				first_packet_list_parse.len() == 1
					&& second_packet_list_parse.len() == 1,
				"All lists resolved correctly"
			);
			packets.push((
				first_packet_list_parse[0].clone(),
				second_packet_list_parse[0].clone(),
			));
			first_packet_list_parse = vec![Vec::new()];
			second_packet_list_parse = vec![Vec::new()];
		} else {
			parsing_second = true;
		}
	}
	packets.push((
		first_packet_list_parse[0].clone(),
		second_packet_list_parse[0].clone(),
	));

	packets
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_input() {
		let example = "[[1],4]";

		let expected = PacketData::List(vec![
			PacketData::List(vec![PacketData::Number(1)]),
			PacketData::Number(4),
		]);
		let actual = parse_packet(example);

		assert_eq!(expected, actual);
	}

	#[test]
	fn test_other_parse() {
		let example = "[[1],4]\n[[1],[2,3,4]]";

		let expected = &_copied_parse(example)[0];
		let expected = (
			PacketData::List(expected.0.clone()),
			PacketData::List(expected.1.clone()),
		);

		eprintln!("{:?}", expected);

		let actual = (parse_packet("[[1],4]"), parse_packet("[[1],[2,3,4]]"));

		assert_eq!(expected, actual);
	}

	#[test]
	fn test_part_one() {
		use std::fs::File;
		use std::io::Read;

		let mut file = File::open("src/input/day13-example.txt")
			.expect("File reading failed.");
		let mut example = String::new();
		file.read_to_string(&mut example)
			.expect("Reading has failed.");

		assert_eq!(Day13::parse_input(&example).part_one(), 13);
	}
}
