mod counting_flips;

fn main() {
    let num_simulations = 100000;
    let (alice, bob, tie) = counting_flips::run_simulations(num_simulations);

    println!("After {} simulations:", num_simulations);
    println!("Alice wins: {:.2}%", alice);
    println!("Bob wins: {:.2}%", bob);
    println!("Ties: {:.2}%", tie);
}
