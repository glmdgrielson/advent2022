use std::fmt;

mod advent;
pub use advent::Advent;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
/// A point struct since I keep switching up x and y coordinates.
pub struct Point {
	/// The horizontal value.
	x: i32,
	/// The vertical value.
	y: i32,
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
