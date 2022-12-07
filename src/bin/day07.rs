/// Day 7's Advent of Code puzzle
/// =============================
/// Puzzle input consists of a terminal session. Two commands are being used:
/// `cd`, which changes the current directory, and `ls` which lists all of the
/// files in the specified directory.
///
/// Part 1
/// ------
/// Find the sum of all of the "small" directories, where small is defined
/// as taking no more than `100_000` bytes.
use std::io::stdin;
use std::collections::HashMap;

/// The size of directory we care about.
const SMALL_DIRECTORY: usize = 100_000;

#[derive(Clone, Debug)]
enum Command<'a> {
	LS,
	CD(&'a str),
}


#[derive(Clone, Debug, PartialEq)]
/// Represents a file on disk. This could probably be attached to its
/// directory, but I don't want to attach a lifetime to this right now.
struct File {
	path: String,
	size: usize,
	name: String,
}

#[derive(Clone, Debug, PartialEq)]
/// Represents a directory in the filesystem. This takes a lifetime (let the
/// water hold you down) because this is a recursive structure.
struct Directory<'a> {
	path: String,
	subdirs: Vec<Directory<'a>>,
	files: Vec<File>,
	parent: Option<&'a Directory<'a>>,
}

fn main() {
	let lines = stdin().lines();
	for line in lines {
		if let Ok(line) = line {
			todo!()
		} else {
			// This unwrap is safe because we just failed a pattern match
			// on the other option.
			let err = line.unwrap_err();
			panic!("Welp, your input failed: {}.", err);
		}
	}
	todo!()
}
