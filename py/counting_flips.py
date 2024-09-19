import random

def simulate_game():
    # Generate 100 coins with random heads (1) or tails (0)
    coins = [random.choice([0, 1]) for _ in range(100)]
    
    # Alice and Bob both start the same
    alice_index = 0
    bob_index = 0
    alice_heads = 0
    bob_heads = 0

    # Alice's and Bob's turns
    while alice_index < 100 or bob_index < 100:
        # Alice's turn
        if alice_index < 100:
            if coins[alice_index] == 1:
                alice_heads += 1
            alice_index += 1
        
        # Bob's turn
        if bob_index < 100:
            if coins[bob_index] == 1:
                bob_heads += 1
            bob_index += 2  # Bob goes to the next odd index
        
        # Bob goes back to evens if he finishes odds
        if bob_index >= 100:
            bob_index = 1

        if alice_heads == 2 and bob_heads == 2:
            return "Tie"
        if bob_heads == 2:
            return "Bob"
        if alice_heads == 2:
            return "Alice"

    return "Tie"

def run_simulations(num_simulations):
    results = {"Alice": 0, "Bob": 0, "Tie": 0}
    
    for _ in range(num_simulations):
        result = simulate_game()
        results[result] += 1
    
    # Calculate percentages
    percentages = {key: (value / num_simulations) * 100 for key, value in results.items()}
    
    return percentages

# Run the simulation 100,000 times
num_simulations = 1000000
percentages = run_simulations(num_simulations)

print(f"After {num_simulations} simulations:")
print(f"Alice: {percentages['Alice']:.2f}%")
print(f"Bob: {percentages['Bob']:.2f}%")
print(f"Tie: {percentages['Tie']:.2f}%")

