# print("\t\tPython Plaground Result")
from typing import List


def coinChange(coins: List[int], amount: int):
    # Memoization dictionary to store the results of subproblems
    memo = {}

    def helper(remaining):
        # Base cases
        if remaining == 0:
            return 0
        if remaining < 0:
            return float('inf')

        # Check if the result is already in the memo dictionary
        if remaining in memo:
            return memo[remaining]

        # Initialize the minimum number of coins to infinity
        min_coins = float('inf')

        # Try every coin in the coins array
        for coin in coins:
            # Recursively solve the subproblem for the remaining amount
            num_coins = helper(remaining - coin)
            # Update the minimum number of coins
            if num_coins != float('inf'):
                min_coins = min(min_coins, num_coins + 1)

        # Store the result in the memo dictionary
        memo[remaining] = min_coins

        return memo[remaining]

    # Get the result for the given amount
    result = helper(amount)
    # If the result is infinity, it means it's not possible to make the amount
    return result if result != float('inf') else -1

# Example usage
coins = [1, 2, 5]
amount = 11
print(coinChange(coins, amount))  # Output: 3

