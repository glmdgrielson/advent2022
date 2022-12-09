/// Day 10's Advent of Code puzzle
/// ==============================

use advent::{Advent, input_to_str};

#[derive(Debug)]
struct Day10;

impl Advent for Day10 {
    type Answer1 = ();

    type Answer2 = ();

    fn parse_input(input: &str) -> Self {
        todo!()
    }

    fn part_one(&self) -> Self::Answer1 {
        todo!()
    }
}

fn main() {
	let input = input_to_str();
	let code = Day10::parse_input(&input);
	code.part_one();
}
