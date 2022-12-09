pub trait Advent {
	/// The parsed results of the input file.
	type Data;
	/// The output desired for Part one of the puzzle.
	type Answer1;
	/// The output desired for Part 2 of the puzzle.
	type Answer2;

	fn parse_input(input: &str) -> Self::Data;
	fn part_one(data: Self::Data) -> Self::Answer1;
	fn part_two(data: Self::Data) -> Self::Answer2 {
		Self::part_one(data);
		todo!("Part 2's puzzle is not yet known, give us a minute.");
	}
}
