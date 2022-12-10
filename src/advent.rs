pub trait Advent {
	/// The output desired for Part 1 of the puzzle.
	type Answer1;
	/// The output desired for Part 2 of the puzzle.
	type Answer2;

	/// Convert input (from stdin, perhaps) into a usable data format.
	fn parse_input(input: &str) -> Self;
	/// Solve the first part of the puzzle.
	fn part_one(&self) -> Self::Answer1;
	/// Solve part two of the puzzle.
	///
	/// This is a separate function because the first part's
	/// result is useless for part 2. Have fun rewriting that code.
	fn part_two(&self) -> Self::Answer2 {
		self.part_one();
		todo!("Part 2's puzzle is not yet known, give us a minute.");
	}
}
