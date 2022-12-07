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
///
/// Part 2
/// ------
/// Now it's time to clean up space. Find the smallest directory that will
/// give us enough space, so we don't `rm -rf /` like a dolt.
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdin;
use std::rc::Rc;

/// The size of directory we care about.
const SMALL_DIRECTORY: usize = 100_000;
const TOTAL_DISK_SPACE: usize = 70_000_000;
const SPACE_NEEDED: usize = 30_000_000;

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
	let (cur_size, sizes) = calc_sum(&borrowed, &mut sizes);
	let res: usize = sizes.iter().filter(|&s| *s < SMALL_DIRECTORY).sum();
	println!("Total sum of all of the small directories is {}", res);
	let needed = SPACE_NEEDED - (TOTAL_DISK_SPACE - cur_size);
	let dead_dir = sizes.iter().filter(|&x| *x > needed).min();
	match dead_dir {
		Some(dead_size) => {
			println!("Good news, you can clear up {}!", dead_size)
		}
		None => panic!("Welp, you need a new computer. Sorry!"),
	}
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
		// Get the children of this node
		.children
		// Get just the sizes.
		.values()
		// Map this function onto the children.
		.map(|c| calc_sum(&c.borrow(), sizes).0)
		// Add them all together.
		.sum();
	sizes.push(sum_c); // Add this result to the size list.
	(sum_c, sizes)
}
