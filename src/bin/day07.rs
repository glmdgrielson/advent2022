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
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdin;
use std::rc::Rc;

/// The size of directory we care about.
const SMALL_DIRECTORY: usize = 100_000;

#[derive(Clone, Debug, PartialEq)]
/// Represents a file on disk. This could probably be attached to its
/// directory, but I don't want to attach a lifetime to this right now.
struct File {
	path: String,
	size: usize,
	name: String,
}

type NodeRef = Rc<RefCell<Node>>;

#[derive(Clone, Debug, PartialEq)]
/// Represents an item in the filesystem. In true Unix fashion,
/// this could represent a fire or directory.
struct Node {
	size: Option<usize>,
	children: HashMap<String, NodeRef>,
	parent: Option<NodeRef>,
	is_file: bool,
}

impl Node {
	fn root() -> Self {
		Node {
			size: None,
			children: HashMap::new(),
			parent: None,
			is_file: false,
		}
	}
}

// Today's code involves a good deal of theft, unfortunately.
fn main() {
	let lines = stdin().lines().filter_map(|l| match l {
		Ok(line) => Some(line),
		Err(err) => panic!("Welp, your input failed: {}", err),
	});
	let root = Rc::new(RefCell::new(Node::root()));
	let mut cur_node = Rc::clone(&root);
	for line in lines {
		let tokens: Vec<_> = line.split(' ').collect();
		if tokens[0] == "$" {
			match tokens[1] {
				"cd" => {
					let folder = tokens[2];
					cur_node = match folder {
						".." => Rc::clone(
							cur_node.borrow().parent.as_ref().unwrap_or_else(
								|| panic!("Has anybody seen {}'s mom?", folder),
							),
						),
						"/" => root.clone(),
						_ => cur_node
							.borrow()
							.children
							.get(folder)
							.unwrap_or_else(|| {
								panic!(
									"Looking for a {}, has anyone seen it?",
									folder
								)
							})
							.clone(),
					};
				}
				"ls" => {
					// This does absolutely nothing.
				}
				command => panic!("{} is not a valid command.", command),
			}
		} else {
			let size_or_dir = tokens[0];
			let name = tokens[1];
			if !cur_node.borrow().children.contains_key(name) {
				let child = Rc::new(RefCell::new(Node::root()));
				let mut mut_child = child.borrow_mut();
				if size_or_dir != "dir" {
					mut_child.is_file = true;
					mut_child.size = Some(
						size_or_dir
							.parse()
							.expect("Sizes are in decimal, not base64!"),
					);
				}
				mut_child.parent = Some(Rc::clone(&cur_node));
				cur_node
					.borrow_mut()
					.children
					.insert(name.to_owned(), Rc::clone(&child));
			}
		}
	}
	let mut sizes: Vec<usize> = vec![];
	let borrowed = root.borrow();
	let (_, sizes) = calc_sum(&borrowed, &mut sizes);
	let res: usize = sizes.iter().filter(|&s| *s < SMALL_DIRECTORY).sum();
	println!("Total sum of all of the small directories is {}", res);
}

/// And today we get to play with _recursive_ functions!
/// This calculates the sum we need for the Advent of Code answer.
fn calc_sum<'a>(
	node: &'a Node,
	sizes: &'a mut Vec<usize>,
) -> (usize, &'a mut Vec<usize>) {
	if node.is_file {
		return (node.size.expect("Is this /dev/null?"), sizes);
	}
	let sum_c = node
		.children // Get the children of this node
		.values() // Get just the sizes.
		.map(|c| calc_sum(&c.borrow(), sizes).0) // Map this function onto the children.
		.sum(); // Add them all together.
	sizes.push(sum_c); // Add this result to the size list.
	(sum_c, sizes)
}
