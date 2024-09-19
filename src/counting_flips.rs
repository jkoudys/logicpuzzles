use rand::random;
use std::collections::HashMap;
use std::fmt;

// Define an enum for the possible outcomes
#[derive(PartialEq, Eq, Hash)]
pub enum Outcome {
    Alice,
    Bob,
    Tie,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let outcome_str = match self {
            Outcome::Alice => "alice",
            Outcome::Bob => "bob",
            Outcome::Tie => "tie",
        };
        write!(f, "{}", outcome_str)
    }
}

fn simulate_game(flips: usize) -> Outcome {
    // Generate 100 random coins (1 for heads, 0 for tails)
    let coins = random::<u128>();

    let found = (0..(flips / 2 - 1))
        .map(|v| v * 2)
        .chain((0..(flips / 2 - 1)).map(|v| v * 2 + 1))
        .zip(0..flips)
        .map(|(a, b)| (coins & (1 << a) > 0, coins & (1 << b) > 0))
        .scan((0, 0), |count, coin| {
            count.0 += coin.0 as u8;
            count.1 += coin.1 as u8;
            Some(*count)
        })
        .find(|count| count.0 == 2 || count.1 == 2);

    match found {
        None | Some((2, 2)) => Outcome::Tie,
        Some((2, _)) => Outcome::Bob,
        Some((_, 2)) => Outcome::Alice,
        _ => unreachable!(),
    }
}

pub fn run_simulations(num_simulations: usize) -> HashMap<Outcome, usize> {
    (0..num_simulations).fold(HashMap::new(), |mut hm, _| {
        hm.entry(simulate_game(10))
            .and_modify(|count| *count += 1)
            .or_insert(1);
        hm
    })
}
