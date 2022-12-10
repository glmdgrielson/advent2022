/// Day 10's Advent of Code puzzle
/// ==============================
/// Puzzle input consists of a list of instructions, either `noop` which bumps
/// the cycle counter, or `addx n` which adds `n` to some register after two
/// cycles.
///
/// Part 1
/// ------
/// Find the value of the register at varying cycles.
/// 
/// Part 2
/// ------
/// Use the instructions to render to the screen. Figure out what is being
/// printed to the output.
use advent::{input_to_str, Advent};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
struct Day10(Vec<Instruction>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
	Add(i32),
	Noop,
}

impl fmt::Display for Instruction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Instruction::Add(n) => f.write_fmt(format_args!("addx {}", n)),
			Instruction::Noop => f.write_str("noop"),
		}
	}
}

impl Advent for Day10 {
	type Answer1 = i32;

	type Answer2 = String;

	fn parse_input(input: &str) -> Self {
		let mut code = Vec::new();
		for line in input.lines() {
			let words = line.split(' ').collect::<Vec<_>>();
			match words[0] {
				"noop" => {
					code.push(Instruction::Noop);
				}
				"addx" => {
					let val = words[1].parse();
					match val {
						Ok(val) => {
							code.push(Instruction::Add(val));
						}
						Err(err) => {
							panic!("Parsing has failed: {}", err)
						}
					}
				}
				command => panic!("Unsupported instruction {}", command),
			}
		}
		Day10(code)
	}

	fn part_one(&self) -> i32 {
		let critical = [20, 60, 100, 140, 180, 220];
		let critical_values = self.execute(&critical);
		// Check that we've hit all of the critical positions.
		assert_eq!(critical.len(), critical_values.len());
		critical
			// Convert into iterator.
			.iter()
			// Add the values into the mix.
			.zip(critical_values)
			// Multiply the value times the cycle count
			.map(|(&n, v)| n * v)
			// Sum everything up.
			.sum()
	}

	fn part_two(&self) -> String {
		self.draw()
	}
}

impl Day10 {
	fn execute(&self, critical: &[i32]) -> Vec<i32> {
		let mut critical_values = Vec::new();

		let mut cycle_count = 0;
		let mut register = 1;
		for task in &self.0 {
			cycle_count += 1;
			if critical.contains(&cycle_count) {
				critical_values.push(register);
			}
			match task {
				Instruction::Add(n) => {
					// Bump the cycle counter.
					cycle_count += 1;
					// Check to see if we hit an important cycle
					// mid-instruction.
					if critical.contains(&cycle_count) {
						critical_values.push(register);
					}
					register += n;
				}
				Instruction::Noop => {
					// This does NOTHING!
				}
			}
		}

		critical_values
	}

	fn draw(&self) -> String {
		let mut grid = String::new();

		let mut cycle_count = 0;
		let mut register = 1;
		for task in &self.0 {
			// Check to see if this pixel should be lit up.
			if (register - 1..=register + 1).contains(&cycle_count) {
				grid.push('#');
			} else {
				grid.push('.');
			}
			// Check to see if we're about to overflow.
			if cycle_count == 39 {
				grid.push('\n');
				cycle_count = 0;
			} else {
				cycle_count += 1;
			}
			match task {
				Instruction::Add(n) => {
					// Check to see if this pixel should be lit up.
					if (register - 1..=register + 1).contains(&cycle_count) {
						grid.push('#');
					} else {
						grid.push('.');
					}
					// Check to see if we're about to overflow.
					if cycle_count == 39 {
						grid.push('\n');
						cycle_count = 0;
					} else {
						cycle_count += 1;
					}
					register += n;
				}
				Instruction::Noop => {
					// This does NOTHING!
				}
			}
		}

		grid
	}
}

fn main() {
	let input = input_to_str();
	let code = Day10::parse_input(&input);
	let one = code.part_one();
	println!("The sum of the important cycles is {}", one);

	println!("The output of part 2 is:");
	println!("{}", code.part_two());
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_input() {
		let example = "noop\naddx 3\naddx -5";

		let expected = Day10(vec![
			Instruction::Noop,
			Instruction::Add(3),
			Instruction::Add(-5),
		]);
		let actual = Day10::parse_input(example);

		assert_eq!(expected, actual);
	}

	#[cfg(test)]
	fn get_example() -> String {
		use std::fs::File;
		use std::io::Read;

		let file = File::open("src/input/day10-example.txt");
		let mut file = match file {
			Ok(file) => file,
			Err(err) => panic!("Example input failed: {}", err),
		};
		let mut example = String::new();
		file.read_to_string(&mut example)
			.unwrap_or_else(|err| panic!("File reading failed: {}", err));

		example
	}

	#[test]
	fn test_execute_small() {
		let tasks =
			vec![Instruction::Noop, Instruction::Add(3), Instruction::Add(-5)];
		let runner = Day10(tasks);

		// Since I didn't hardcode the critical values, I can use this for
		// debugging!
		let critical = [1, 2, 3, 4, 5];
		let expected = vec![1, 1, 1, 4, 4];
		let actual = runner.execute(&critical);

		assert_eq!(expected, actual);
	}

	#[test]
	fn test_execute_large() {
		let example = get_example();
		let critical = [20, 60, 100, 140, 180, 220];

		let expected = vec![21, 19, 18, 21, 16, 18];
		let actual = Day10::parse_input(&example).execute(&critical);

		assert_eq!(expected, actual);
	}

	#[test]
	fn test_part_one() {
		let example = get_example();
		let data = Day10::parse_input(&example);
		assert_eq!(data.part_one(), 13140);
		// assert_eq!()
	}

	#[test]
	fn test_part_two() {
		let example = get_example();

		let expected = "##..##..##..##..##..##..##..##..##..##..\n\
		###...###...###...###...###...###...###.\n\
		####....####....####....####....####....\n\
		#####.....#####.....#####.....#####.....\n\
		######......######......######......####\n\
		#######.......#######.......#######.....\n";
		let actual = Day10::parse_input(&example).part_two();

		assert_eq!(expected, actual);
	}
}
