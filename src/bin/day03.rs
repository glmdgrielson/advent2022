use std::collections::HashSet;
use std::io::stdin;

fn main() {
    // This is a horrifying hack that only works because every puzzle
    // takes input from a file. I'd love to know a better way of doing this.
    let lines = stdin().lines();
    let lines: Vec<_> = lines.filter(|l| l.is_ok()).map(|l| l.unwrap()).collect();
    let mut priority = 0;
    'search: for sack in lines.clone() {
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
    let mut badge_priority = 0;
    for set in lines.chunks(3) {
        let first = set[0].clone();
        let mut badge_set: HashSet<char> = HashSet::new();
        for elf in set {
            let chars = elf.chars();
            if *elf == first {
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
        assert!(badge_set.len() == 1, "We found counterfeit badges: {:?}", badge_set);
        let badges: Vec<_> = badge_set.into_iter().collect();
        badge_priority += priority_value(badges[0]);
    }
    // Print the solutions to the puzzles.
    println!("Final sum of priorities is {}", priority);
    println!("Final sum of badge priority is {}", badge_priority);
}

fn priority_value(item: char) -> u32 {
    let prior = item as u32;
    match item {
        'a'..='z' => prior - 96,
        'A'..='Z' => prior - 38,
        _ => unreachable!("That shouldn't be here..."),
    }
}
