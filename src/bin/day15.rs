//! Day 15's Advent of Code puzzle
//! ==============================
//! Puzzle input consists of a list of sensors and corresponding beacons.
//!
//! Part 1
//! ------
//! In a given row, where can there not be beacons?
//!
//! Part 2
//! ------
//! What's the _one_ spot that can't have a beacon?

use advent::{input_to_str, Advent, Point};
use std::collections::HashSet;

/// The row we care about for the purposes of Part 1's puzzle.
const MAJOR_ROW: i64 = 2_000_000;

/// The maximum size of the map for Part 2's puzzle.
const MAX_COORDINATE: i64 = 4_000_000;

#[derive(Debug)]
struct Day15 {
	/// The list of sensors given by the input.
	sensors: Vec<Sensor>,
}

impl Advent for Day15 {
	type Answer1 = usize;

	type Answer2 = u64;

	fn parse_input(input: &str) -> Self {
		let mut sensors = Vec::new();
		// Format of line: "Sensor at x=A, y=B: closest beacon is at x=C, y=D"
		for line in input.lines() {
			let words = line.split(' ').collect::<Vec<_>>();

			// Format: "x=A,"
			let sen_x = words[2];
			let sen_x =
				sen_x.strip_prefix("x=").expect("Malformed location data");
			let sen_x =
				sen_x.strip_suffix(',').expect("Malformed location data");
			let sen_x = sen_x
				.parse::<i64>()
				.expect("That's not a position I can sense!");

			// Format "y=B:"
			let sen_y = words[3];
			let sen_y =
				sen_y.strip_prefix("y=").expect("Malformed location data");
			let sen_y =
				sen_y.strip_suffix(':').expect("Malformed location data");
			let sen_y =
				sen_y.parse().expect("That's not a position I can sense!");

			let sensor = Point { x: sen_x, y: sen_y };

			// Format: "x=A,"
			let bea_x = words[8];
			let bea_x =
				bea_x.strip_prefix("x=").expect("Malformed location data");
			let bea_x =
				bea_x.strip_suffix(',').expect("Malformed location data");
			let bea_x = bea_x
				.parse::<i64>()
				.expect("That's not a position I can sense!");

			// Format "y=B:"
			let bea_y = words[9];
			let bea_y =
				bea_y.strip_prefix("y=").expect("Malformed location data");
			let bea_y =
				bea_y.parse().expect("That's not a position I can sense!");

			let beacon = Point { x: bea_x, y: bea_y };

			sensors.push(Sensor {
				location: sensor,
				beacon,
			});
		}
		Day15 { sensors }
	}

	fn part_one(&self) -> Self::Answer1 {
		self.clear_beacons(MAJOR_ROW)
	}

	fn part_two(&self) -> u64 {
		let mut frequency = 0;
		'sensor: for sensor in self.sensors.iter() {
			let border_points = sensor.border_coordinates();
			let border_points = border_points.iter().filter(|&p| {
				(0 <= p.x && p.x <= MAX_COORDINATE)
					&& (0 <= p.y && p.y <= MAX_COORDINATE)
			});
			for point in border_points {
				if !self.sensors.iter().any(|s| s.can_sense_point(*point)) {
					frequency = point.x * MAX_COORDINATE + point.y;
					eprintln!("({}, {}) => {}", point.x, point.y, frequency);
					break 'sensor;
				}
			}
		}
		frequency as u64
	}
}

impl Day15 {
	fn clear_beacons(&self, row: i64) -> usize {
		let beacons = self.sensors.iter().map(|s| s.beacon).collect::<Vec<_>>();
		let mut empty_pos = HashSet::new();
		for sensor in self.sensors.iter() {
			let beacon_distance = sensor.beacon_distance();
			let y_distance = sensor.location.y.abs_diff(MAJOR_ROW);

			let remainder = beacon_distance.saturating_sub(y_distance);
			let remainder: i64 =
				remainder.try_into().expect("Point way out of bounds");
			for x in (sensor.location.x - remainder)
				..=(sensor.location.x + remainder)
			{
				if beacons.contains(&Point { x, y: row }) {
					continue;
				}
				empty_pos.insert(x);
			}
		}
		empty_pos.len()
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Sensor {
	/// The location this sensor is found in.
	location: Point<i64>,
	/// The beacon reported to be close to this sensor.
	beacon: Point<i64>,
}

impl Sensor {
	fn beacon_distance(&self) -> u64 {
		let sensor = self.location;
		let beacon = self.beacon;

		let x = sensor.x.abs_diff(beacon.x);

		let y = sensor.y.abs_diff(beacon.y);

		x + y
	}

	fn border_coordinates(&self) -> Vec<Point<i64>> {
		let distance: i64 = self
			.beacon_distance()
			.try_into()
			.expect("Out of bounds of map");
		let distance = distance + 1;
		let mut border_points = Vec::new();

		let north = Point {
			x: self.location.x,
			y: self.location.y + distance,
		};
		let south = Point {
			x: self.location.x,
			y: self.location.y - distance,
		};
		let east = Point {
			x: self.location.x - distance,
			y: self.location.y,
		};
		let west = Point {
			x: self.location.x + distance,
			y: self.location.y,
		};

		let mut current = north;
		// NW border
		while current != west {
			current.x += 1;
			current.y -= 1;
			border_points.push(current);
		}
		// SW border
		while current != south {
			current.x -= 1;
			current.y -= 1;
			border_points.push(current);
		}
		// SE border
		while current != east {
			current.x -= 1;
			current.y += 1;
			border_points.push(current);
		}
		// NE border
		while current != north {
			current.x += 1;
			current.y += 1;
			border_points.push(current);
		}

		border_points
	}

	fn can_sense_point(&self, point: Point<i64>) -> bool {
		let dist = self.location.x.abs_diff(point.x)
			+ self.location.y.abs_diff(point.y);
		dist <= self.beacon_distance()
	}
}

fn main() {
	let runner = Day15::parse_input(&input_to_str());
	println!(
		"The number of squares that can't be beacons is {}",
		runner.part_one()
	);
	println!("The tuning frequency is {}", runner.part_two());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[cfg(test)]
	const EXAMPLE_FILE: &str = "src/input/day15-example.txt";

	#[test]
	fn test_part_one() {
		use advent::get_example_input;

		let example = get_example_input(EXAMPLE_FILE);
		let runner = Day15::parse_input(&example);

		assert_eq!(runner.clear_beacons(10), 26);
	}
}
