use std::fmt;
use std::fs::File;
use std::io::stdin;
use std::io::Read;
use std::ops::{Add, AddAssign, Sub};

mod advent;
pub use crate::advent::Advent;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A point struct since I keep switching up x and y coordinates.
pub struct Point<T = i32> {
	/// The horizontal value.
	pub x: T,
	/// The vertical value.
	pub y: T,
}

impl<T> Add for Point<T>
where
	T: Add<Output = T>,
{
	type Output = Self;

	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		Point {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T> AddAssign for Point<T>
where
	T: AddAssign,
{
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T> Add<(T, T)> for Point<T>
where
	T: Add<Output = T>,
{
	type Output = Point<T>;

	fn add(self, rhs: (T, T)) -> Self::Output {
		Point {
			x: self.x + rhs.0,
			y: self.y + rhs.1,
		}
	}
}

impl<T> AddAssign<(T, T)> for Point<T>
where
	T: AddAssign,
{
	fn add_assign(&mut self, rhs: (T, T)) {
		self.x += rhs.0;
		self.y += rhs.1;
	}
}

impl<T> Sub for Point<T>
where
	T: Sub<Output = T>,
{
	type Output = Point<T>;

	fn sub(self, rhs: Self) -> Self::Output {
		Point {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T> Sub<(T, T)> for Point<T>
where
	T: Sub<Output = T>,
{
	type Output = Point<T>;

	fn sub(self, rhs: (T, T)) -> Self::Output {
		Point {
			x: self.x - rhs.0,
			y: self.y - rhs.1,
		}
	}
}

/// This is formatted as if this was a tuple.
impl fmt::Display for Point {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_fmt(format_args!("({}, {})", self.x, self.y))
	}
}

pub fn point(x: i32, y: i32) -> Point {
	Point { x, y }
}

pub fn input_to_str() -> String {
	stdin()
		.lines()
		.filter_map(|l| match l {
			Ok(l) => Some(l),
			Err(err) => panic!("Welp, your input failed: {}", err),
		})
		.collect::<Vec<_>>()
		.join("\n")
}

/// A convenience file for running tests.
/// 
/// Takes the file name of an example input and returns the text contents
/// of said file. This is useful for example input.
/// ```
/// use advent::get_example_input;
///
/// let example = get_example_input("src/input/day14-example.txt");
/// ```
pub fn get_example_input(filename: &str) -> String {
	let mut file = File::open(filename).expect("File reading failed.");
	let mut example = String::new();
	file.read_to_string(&mut example)
		.expect("Reading has failed.");

	example
}
