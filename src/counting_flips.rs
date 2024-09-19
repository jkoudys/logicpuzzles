use rand::Rng;

// Define an enum for the possible outcomes
enum Outcome {
    Alice,
    Bob,
    Tie,
}

fn simulate_game(flips: usize) -> Outcome {
    let mut rng = rand::thread_rng();
    // Generate 100 random coins (1 for heads, 0 for tails)
    let coins: Vec<i32> = (0..flips).map(|_| rng.gen_range(0..=1)).collect();

    let found = (0..(flips / 2 - 1)).map(|v| v * 2).chain((0..(flips / 2 - 1)).map(|v| v * 2 + 1))
            .zip(0..flips)
            .map(|(a, b)| (coins[a], coins[b]))
            .scan((0, 0), |count, coin| {
                count.0 += coin.0;
                count.1 += coin.1;
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

pub fn run_simulations(num_simulations: usize) -> (f64, f64, f64) {
    let mut alice_wins = 0;
    let mut bob_wins = 0;
    let mut ties = 0;

    for _ in 0..num_simulations {
        match simulate_game(100) {
            Outcome::Alice => alice_wins += 1,
            Outcome::Bob => bob_wins += 1,
            Outcome::Tie => ties += 1,
        }
    }

    // Calculate percentages
    let alice_percentage = (alice_wins as f64 / num_simulations as f64) * 100.0;
    let bob_percentage = (bob_wins as f64 / num_simulations as f64) * 100.0;
    let tie_percentage = (ties as f64 / num_simulations as f64) * 100.0;

    (alice_percentage, bob_percentage, tie_percentage)
}
