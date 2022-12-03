//! Day 3's Advent of Code puzzle
//! =============================
//! Puzzle input consists of a list of alphabetic characters. Each line of input
//! represents the items that a particular elf has packed for an expedition.
//!
//! Part 1
//! ------
//! Split each line of input in two and find the one character that is in both
//! halves. This item has been incorrectly packed.
//!
//! Part 2
//! ------
//! For every three elves, find the item that all three of them have packed.
//! This is the identification badge for that triplet of elves.

use std::collections::HashSet;
use std::io::stdin;

fn main() {
    let lines = stdin().lines();
    let mut priority = 0;
    let mut badge_priority = 0;
    let mut triplet = vec![];
    'search: for line in lines {
        if let Ok(sack) = line {
            // Prep for part 2.
            triplet.push(sack.clone());
            // Check to see if we have a triplet.
            if triplet.len() == 3 {
                // Even though this code is part 2, it has to be run first
                // since success on the first part goes back to the start
                // of the loop.
                let first = &triplet[0];
                let mut badge_set: HashSet<char> = HashSet::new();
                for elf in triplet.iter() {
                    let chars = elf.chars();
                    if elf == first {
                        // If this is the first run through, just dump everything
                        // in the list of candidates.
                        badge_set = chars.collect();
                    } else {
                        // Otherwise filter everything in the list of candidates
                        // that doesn't exist in this elf's sack.
                        let items: HashSet<char> = chars.collect();
                        for item in badge_set.clone() {
                            if !items.contains(&item) {
                                badge_set.remove(&item);
                            }
                        }
                    }
                }
                // If everything went okay, there should only ever be one item
                // in the set at this point.
                assert!(
                    badge_set.len() == 1,
                    "We found counterfeit badges: {:?}",
                    badge_set
                );
                let badges: Vec<_> = badge_set.into_iter().collect();
                badge_priority += priority_value(badges[0]);
            }
            let (upper, lower) = sack.split_at(sack.len() / 2);
            for item in upper.chars() {
                // If this were Python, I could just do `item in lower`,
                // but Rust doesn't think that's safe or necessary. Welp.
                if let Some(_) = lower.find(item) {
                    priority += priority_value(item);
                    // This shortcuts the search. After all, there should only ever
                    // be one item shared between the two halves.
                    continue 'search;
                }
            }
        }
    }
    // Print the solutions to the puzzles.
    println!("Final sum of priorities is {}", priority);
    println!("Final sum of badge priority is {}", badge_priority);
}

/// Find the priority value of an item. This is mainly for inputting
/// the solution as required by the Advent of Code site.
///
/// Priority value is determined as follows:
/// - Lowercase letters are their position in the alphabet, such that 'a' is 1
///     and 'z' is 26.
/// - Uppercase values are their position in the alphabet _plus 26_
///     to differentiate them from lowercase letters, such that 'A' is 27
///     and 'Z' is 52.
fn priority_value(item: char) -> u32 {
    let prior = item as u32;
    match item {
        'a'..='z' => prior - 96,
        'A'..='Z' => prior - 38,
        _ => unreachable!("That shouldn't be here..."),
    }
}
