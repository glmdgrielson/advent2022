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

fn guess(round: String) -> i32 {
    let mut score = 0;
    let res: Vec<&str> = round.split(" ").collect();
    let (this, that) = (res[0], res[1]);
    let this = match this {
        "A" => Throw::Rock,
        "B" => Throw::Paper,
        "C" => Throw::Scissors,
        _ => unreachable!("That's not a valid play!")
    };
    // let mut debug = 0;
    let that = match that {
        "X" => {
            score += 1;
            // debug += 1;
            Throw::Rock
        },
        "Y" => {
            score += 2;
            // debug += 2;
            Throw::Paper
        },
        "Z" => {
            score += 3;
            // debug += 3;
            Throw::Scissors
        },
        _ => unreachable!("What are you doing?")
    };
    match that.result(&this) {
        Ordering::Less => {
            // No point in incrementing an empty score.
            // score += 0;
        },
        Ordering::Equal => {
            score += 3;
            // debug += 3;
        },
        Ordering::Greater => {
            score += 6;
            // debug += 6;
        },
    };
    score
    // print!("This round is {}", debug);
}

fn cheat(round: String) -> i32 {
    let mut score = 0;
    let res: Vec<&str> = round.split(" ").collect();
    let (this, that) = (res[0], res[1]);
    let this = match this {
        "A" => Throw::Rock,
        "B" => Throw::Paper,
        "C" => Throw::Scissors,
        _ => unreachable!("Hey, they're not supposed to cheat!")
    };
    // let mut debug = 0;
    let result = match that {
        "X" => {
            // No point in incrementing an empty score.
            // Other than maybe keeping up appearances of fairness...
            // score += 0;
            // debug += 1;
            Ordering::Less
        },
        "Y" => {
            score += 3;
            // debug += 2;
            Ordering::Equal
        },
        "Z" => {
            score += 6;
            // debug += 3;
            Ordering::Greater
        },
        _ => unreachable!("Uh, give up I guess?")
    };
    match this.cheat(result) {
        Throw::Rock => {score += 1;},
        Throw::Paper => {score += 2;},
        Throw::Scissors => {score += 3;},
    }
    // print!("This round is {}", debug);
    score
}

#[derive(PartialEq, Eq, Debug)]
/// Represents a Rock Paper Scissors throw.
enum Throw {
    Rock,
    Paper,
    Scissors
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
            Throw::Rock => {
                match other {
                    Throw::Rock => Ordering::Equal,
                    Throw::Paper => Ordering::Less,
                    Throw::Scissors => Ordering::Greater,
                }
            }
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
    /// need to return.
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