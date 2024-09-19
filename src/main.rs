mod counting_flips;

fn main() {
    let num_simulations = 100000;
    let scores = counting_flips::run_simulations(num_simulations);

    println!("After {} simulations:", num_simulations);
    for (outcome, &wins) in scores.iter() {
        println!(
            "{} wins: {:.2}%",
            outcome,
            wins as f64 / num_simulations as f64 * 100f64
        );
    }
    println!(
        "Bob:Alice ratio: {:.2}",
        *scores.get(&counting_flips::Outcome::Bob).unwrap() as f64
            / *scores.get(&counting_flips::Outcome::Alice).unwrap() as f64
    );
}
