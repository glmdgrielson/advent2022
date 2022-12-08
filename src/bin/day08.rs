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
		.map(|l| l.as_bytes().to_owned())
		.collect::<Vec<_>>();
	println!("Number of seen trees is {}", tree_finder(&input));
	println!("Most scenic spot is {}", scenic_tester(&input));
}

fn tree_finder(grid: &[Vec<u8>]) -> usize {
	let mut seen = HashSet::new();

	loop_interior(grid.iter(), |(r, row)| {
		let mut tallest = row[0];
		loop_interior(row.iter(), |(c, &height)| {
			if height > tallest {
				tallest = height;
				seen.insert((r, c));
			}
		});

		tallest = row[row.len() - 1];
		loop_interior(row.iter().rev(), |(c, &height)| {
			if height > tallest {
				tallest = height;
				seen.insert((r, c));
			}
		});
	});

	for c in 1..grid[0].len() - 1 {
		let mut tallest = grid[0][c];
		loop_interior(grid.iter(), |(r, row)| {
			let height = row[c];
			if height > tallest {
				tallest = height;
				seen.insert((r, c));
			}
		});

		tallest = grid[grid.len() - 1][c];
		loop_interior(grid.iter().rev(), |(r, row)| {
			let height = row[c];
			if height > tallest {
				tallest = height;
				seen.insert((r, c));
			}
		});
	}

	seen.len() + grid.len() * 2 + grid[0].len() * 2 - 4
}

fn scenic_tester(grid: &[Vec<u8>]) -> usize {
	let mut map = HashSet::new();
	grid.iter().enumerate().for_each(|(row, line)| {
		line.iter().enumerate().for_each(|(col, _)| {
			map.insert((row, col));
		});
	});
	let mut max_score = 0;
	for (r, c) in map.iter() {
		let (r, c) = (*r, *c);

		eprintln!("Item ({}, {}) is {}", r, c, grid[r][c]);

		let score = get_score(grid, r, c);

		if score > max_score {
			max_score = score;
		}
	}
	// for (r, c) in map.keys() {
	// 	//
	// }
	// let directi
	// scenic_score(map);
	max_score
}

fn get_score(grid: &[Vec<u8>], r: usize, c: usize) -> usize {
	let row = &grid[r];
	let left = &row[0..c];
	let mut l_score = 0;
	left.iter().rfold(0, |i, &c| {
		if i < c {
			l_score += 1;
			c
		} else {
			i
		}
	});
	let score = l_score;
	let right = &row[c + 1..];
	let mut r_score = 0;
	right.iter().fold(0, |i, &height| {
		if i < height {
			r_score += 1;
			height
		} else {
			i
		}
	});
	let score = score * r_score;
	let col = grid.iter().map(|row| row[c]).collect::<Vec<_>>();
	let up = &col[0..r];
	let mut u_score = 0;
	up.iter().rfold(0, |i, &height| {
		if i < height {
			u_score += 1;
			height
		} else {
			i
		}
	});
	let score = score * u_score;
	let down = &col[r + 1..];
	let mut d_score = 0;
	down.iter().fold(0, |acc, &height| {
		if acc < height {
			d_score += 1;
			height
		} else {
			acc
		}
	});
	let score = score * d_score;
	eprintln!(
		"Item ({}, {}) has score {} ({} * {} * {} * {})",
		r, c, score, l_score, r_score, u_score, d_score
	);
	score
}

#[cfg(test)]
mod test {
	#[cfg(test)]
	use crate::get_score;

	#[cfg(test)]
	const EXAMPLE: &str = "30373\n25512\n65332\n33549\n35390";
	#[test]
	fn example_part1() {
		use crate::tree_finder;
		let example: Vec<_> =
			EXAMPLE.lines().map(|s| s.as_bytes().to_owned()).collect();
		assert_eq!(tree_finder(&example), 21);
	}

	#[test]
	fn example_part2() {
		use crate::scenic_tester;
		let data = [
			vec![3, 0, 3, 7, 3],
			vec![2, 5, 5, 1, 2],
			vec![6, 5, 3, 3, 2],
			vec![3, 3, 5, 4, 9],
			vec![3, 5, 3, 9, 0],
		];
		assert_eq!(scenic_tester(&data), 8);
	}

	#[test]
	fn check_score() {
		let data = [
			vec![3, 0, 3, 7, 3],
			vec![2, 5, 5, 1, 2],
			vec![6, 5, 3, 3, 2],
			vec![3, 3, 5, 4, 9],
			vec![3, 5, 3, 9, 0],
		];
		assert_eq!(get_score(&data, 2, 3), 8)
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
