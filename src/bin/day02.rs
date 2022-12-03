//! Day 2's Advent of Code puzzle
//! =============================
//! Puzzle input is a strategy guide containing two letters separated
//! by a space. The first letter is "A", "B", or "C" and has a consistent
//! meaning:
//! - "A" means rock.
//! - "B" means paper.
//! - "C" means scissors.
//!
//! The second character is "X", "Y", or "Z" and each half of the challenge
//! interprets it differently. Due to this, the logic has been seperated into
//! two different functions, named [`guess`] and [`cheat`] for my amusement.
//!
//! The final result is your total score, which is calculated as follows:
//! 1. Assign points based on what you threw. As TV Tropes would put it, poor
//!   predictable Rock.
//!     - Rock is one point.
//!     - Paper is two points.
//!     - Scissors is three points.
//! 2. Assign points based on your end state.
//!     - If you won, you get 6 points.
//!     - If you tied, you get 3 points.
//!     - If you lost, you get nothing.
//!
//! Part 1
//! ------
//! See the function [`guess`].
//!
//! Part 2
//! ------
//! See the function [`cheat`].

use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let lines = stdin().lines();
    let mut score = 0;
    let mut score2 = 0;
    for line in lines {
        if let Ok(round) = line {
            score += guess(round.clone());
            score2 += cheat(round);
        } else {
            panic!("AAAAAAAAAH!")
        }
    }
    println!("Final score is {}.", score);
    println!("The final score with cheating is {}.", score2);
}

/// This function solves the first half of the puzzle, where the second
/// character in the guide assumes that the second character is the throw
/// you should make to win.
/// - "X" is Rock.
/// - "Y" is Paper.
/// - "Z" is Scissors.
///
/// Therefore, this function concerns itself with calculating score based on
/// how the game turns out.
///
/// The returned value is the answer to part 1.
fn guess(round: String) -> i32 {
    let mut score = 0;
    let res: Vec<&str> = round.split(" ").collect();
    let (this, that) = (res[0], res[1]);
    // Figure out what the other elf is throwing.
    let this = match this {
        "A" => Throw::Rock,
        "B" => Throw::Paper,
        "C" => Throw::Scissors,
        _ => unreachable!("That's not a valid play!"),
    };
    // Figure out what you need to throw according to the guide.
    // This also increments the score, since we already know what we should
    // be adding to it.
    let that = match that {
        "X" => {
            score += 1;
            Throw::Rock
        }
        "Y" => {
            score += 2;
            Throw::Paper
        }
        "Z" => {
            score += 3;
            Throw::Scissors
        }
        _ => unreachable!("What are you doing?"),
    };
    // Increment the score by the result of the match.
    match that.result(&this) {
        Ordering::Less => {
            // No point in incrementing an empty score.
            // score += 0;
        }
        Ordering::Equal => {
            score += 3;
        }
        Ordering::Greater => {
            score += 6;
        }
    };
    score
}

/// Now the fun part. This part assumes the second character is how you need to
/// _throw the match_ to get the optimal score.
/// - "X" means you win.
/// - "Y" means you tie.
/// - "Z" means you lose.
///
/// This function therefore concerns itself with figuring out what to respond
/// with to achieve the ideal victory conditions.
///
/// The returned value is the answer to part 2.
///
/// Side note
/// ---------
/// I wonder if it's guaranteed that this secondary half always results in
/// a greater number than the first half. After all, if it didn't, there's no
/// point in being this sneaky.
fn cheat(round: String) -> i32 {
    let mut score = 0;
    // I wonder if in Python this would be one line instead of two.
    let res: Vec<&str> = round.split(" ").collect();
    let (this, that) = (res[0], res[1]);
    // Figure out what the other guy is doing. This is the same as in
    // `guess`, except with a snarkier response to invalid input.
    let this = match this {
        "A" => Throw::Rock,
        "B" => Throw::Paper,
        "C" => Throw::Scissors,
        _ => unreachable!("Hey, they're not supposed to cheat!"),
    };
    // Figure out how the strategy guide says we should throw the match.
    // Given that we already know the result, we can bump up the score here.
    let result = match that {
        "X" => {
            // No point in incrementing an empty score.
            // Other than maybe keeping up appearances of fairness...
            // score += 0;
            Ordering::Less
        }
        "Y" => {
            score += 3;
            Ordering::Equal
        }
        "Z" => {
            score += 6;
            Ordering::Greater
        }
        _ => unreachable!("Uh, give up I guess?"),
    };
    // Figure out what throw will throw the match. Pun intended.
    // Adjust the score accordingly.
    match this.cheat(result) {
        Throw::Rock => {
            score += 1;
        }
        Throw::Paper => {
            score += 2;
        }
        Throw::Scissors => {
            score += 3;
        }
    }
    score
}

#[derive(PartialEq, Eq, Debug)]
/// Represents a Rock Paper Scissors throw.
enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    /// I was going to use [`PartialOrd`] for this, but that seemed a bit too
    /// much like I was abusing the mechanism. So I did this. Same enum, but
    /// not using the operators in bad ways.
    ///
    /// * [`Ordering::Greater`] represents a victory.
    /// * [`Ordering::Equal`] represents a tie.
    /// * [`Ordering::Less`] represents a loss.
    fn result(&self, other: &Throw) -> Ordering {
        match self {
            Throw::Rock => match other {
                Throw::Rock => Ordering::Equal,
                Throw::Paper => Ordering::Less,
                Throw::Scissors => Ordering::Greater,
            },
            Throw::Paper => match other {
                Throw::Rock => Ordering::Greater,
                Throw::Paper => Ordering::Equal,
                Throw::Scissors => Ordering::Less,
            },
            Throw::Scissors => match other {
                Throw::Rock => Ordering::Less,
                Throw::Paper => Ordering::Greater,
                Throw::Scissors => Ordering::Equal,
            },
        }
    }
    /// Fulfill the requirements added by part 2. Again, the ordering
    /// represents the result of the game, and the returned value is what you
    /// need to throw in response.
    ///
    /// * [`Ordering::Less`] results in the value you need to LOSE.
    /// * [`Ordering::Equal`] results in the value you need to TIE.
    /// * [`Ordering::Greater`] results in the value you need to WIN.
    fn cheat(&self, result: Ordering) -> Self {
        match self {
            Throw::Rock => match result {
                Ordering::Less => Throw::Scissors,
                Ordering::Equal => Throw::Rock,
                Ordering::Greater => Throw::Paper,
            },
            Throw::Paper => match result {
                Ordering::Less => Throw::Rock,
                Ordering::Equal => Throw::Paper,
                Ordering::Greater => Throw::Scissors,
            },
            Throw::Scissors => match result {
                Ordering::Less => Throw::Paper,
                Ordering::Equal => Throw::Scissors,
                Ordering::Greater => Throw::Rock,
            },
        }
    }
}
