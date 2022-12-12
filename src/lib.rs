use std::fmt;
use std::io::stdin;
use std::ops::{Add, AddAssign};

mod advent;
pub use advent::Advent;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A point struct since I keep switching up x and y coordinates.
pub struct Point {
	/// The horizontal value.
	pub x: i32,
	/// The vertical value.
	pub y: i32,
}

impl Add for Point {
	type Output = Self;

	#[inline]
	fn add(self, rhs: Self) -> Self::Output {
		Point {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl AddAssign for Point {
	#[inline]
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl Add<(i32, i32)> for Point {
	type Output = Point;

	fn add(self, rhs: (i32, i32)) -> Self::Output {
		Point {
			x: self.x + rhs.0,
			y: self.y + rhs.1,
		}
	}
}

impl AddAssign<(i32, i32)> for Point {
	fn add_assign(&mut self, rhs: (i32, i32)) {
		self.x += rhs.0;
		self.y += rhs.1;
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
