/// Day 8's Advent of Code puzzle
/// =============================
/// Puzzle input consists of a rectangle of numbers.
///
/// Part 1
/// ------
/// How many trees are visible from the outside? (I'm reminded of one of the
/// Simon Tatham puzzles.)
use std::collections::HashSet;
use std::io::stdin;
use std::iter::ExactSizeIterator;

fn main() {
	let input = stdin()
		.lines()
		.filter_map(|l| match l {
			Ok(line) => Some(line),
			Err(err) => panic!("Welp, your input failed: {}", err),
		})
		.collect::<Vec<_>>();
	println!("Number of seen trees is {}", tree_finder(&input));
}

fn tree_finder(input: &[String]) -> usize {
	let grid = input
		.iter()
		.map(|l| {
			l.as_bytes()
				.iter()
				.map(|&b| (b - b'0'))
				.collect::<Vec<u8>>()
		})
		.collect::<Vec<_>>();
	let mut seen = HashSet::new();
	let seen_count = (grid.len() * 2) // length of a row
		    + (grid[0].len() * 2) // length of a column
		    - 4;
	// number of corners counted twice.
	// Thank you best practices! Iterators are fun.
	loop_interior(grid.iter(), |(r, row)| {
		let mut tallest = row[0];
		loop_interior(row.iter(), |(c, &height)| {
			if height > tallest {
				tallest = height;
				seen.insert((r, c));
			}
			//
		});
		let mut tallest = row[row.len() - 1];
		loop_interior(row.iter().rev(), |(c, &height)| {
			if height > tallest {
				tallest = height;
				seen.insert((r, c));
			}
		});
	});
	loop_interior(grid[0].iter(), |(c, _)| {
		let mut tallest = grid[0][c];
		loop_interior(grid.iter(), |(r, _)| {
			let height = grid[r][c];
			if height > tallest {
				tallest = height;
				seen.insert((r, c));
			}
		});
		let mut tallest = grid[grid.len() - 1][c];
		loop_interior(grid.iter().rev(), |(r, _)| {
			let height = grid[r][c];
			if height > tallest {
				tallest = height;
				seen.insert((r, c));
			}
		});
	});
	seen_count + seen.len()
}
mod test {
	#[allow(dead_code)]
	const EXAMPLE: &str = "30373\n25512\n65332\n33549\n35390";
	#[test]
	fn example_part1() {
		use crate::tree_finder;
		let example: Vec<_> = EXAMPLE.lines().map(|s| s.to_owned()).collect();
		assert_eq!(tree_finder(&example), 21);
	}
}

/// Run a function over every element except the first and last.
///
/// Because Clippy, like Python, thinks using ranges to iterate
/// is kind of ugly. I can kind of see why, but for the puzzle at hand...
fn loop_interior<I, F>(iter: I, func: F)
where
	I: ExactSizeIterator + Clone,
	F: FnMut((usize, I::Item)),
{
	let length = iter.len();
	iter.enumerate() // Add index to iterator.
		.take(length - 1) // Ignore the last element.
		.skip(1) // Ignore the first element.
		.for_each(func);
	//
}
