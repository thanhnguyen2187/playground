import pprint
from collections import defaultdict
from dataclasses import dataclass
from typing import Literal

import numpy as np
import pandas as pd

# Question 26: American Put Option with Delta Hedging
# Parameters: S0=180, r=2%, sigma=25%, T=6 months, K=182, N=25 steps


@dataclass
class CashFlow:
    type: Literal[
        "sell_call",
        "sell_put",
        "long_shares",
        "short_shares",
        "exercise_call",
        "exercise_put",
    ]
    value_usd: float
    value_share: float


def binomial_american_option_with_delta(S0, K, r, sigma, T, N, option_type='call'):
    """
    Price an American option using the binomial tree model with delta calculation

    Parameters:
    S0 (float): Initial stock price
    K (float): Strike price
    r (float): Risk-free interest rate (annual)
    sigma (float): Volatility (annual)
    T (float): Time to maturity (in years)
    N (int): Number of steps in the tree
    option_type (str): 'call' or 'put'

    Returns:
    tuple: (option_price, stock_tree, option_tree, delta_tree)
    """
    dt = T / N
    u = np.exp(sigma * np.sqrt(dt))
    d = 1 / u
    p = (np.exp(r * dt) - d) / (u - d)

    S = np.zeros((N+1, N+1))
    V = np.zeros((N+1, N+1))
    Delta = np.zeros((N+1, N+1))

    # Compute stock prices at each node
    for i in range(N+1):
        for j in range(i+1):
            S[i, j] = S0 * (u ** (i-j)) * (d ** j)

    # Initialize option values at maturity
    if option_type.lower() == 'call':
        for j in range(N+1):
            V[N, j] = max(0, S[N, j] - K)
    else:  # put option
        for j in range(N+1):
            V[N, j] = max(0, K - S[N, j])

    # Backward induction with early exercise
    for i in reversed(range(N)):
        for j in range(i+1):
            # Expected value if held (continuation value)
            V[i, j] = np.exp(-r * dt) * (p * V[i+1, j] + (1 - p) * V[i+1, j+1])

            # Check for early exercise opportunity
            if option_type.lower() == 'call':
                exercise_value = max(0, S[i, j] - K)
            else:  # put option
                exercise_value = max(0, K - S[i, j])

            # American option value is max of continuation and exercise
            V[i, j] = max(V[i, j], exercise_value)

            Delta[i, j] = (V[i+1, j] - V[i+1, j+1]) / (S[i+1, j] - S[i+1, j+1])

    return V[0, 0], S, V, Delta


def simulate_delta_hedging_path_put_seller(
    stock_tree,
    option_tree,
    delta_tree,
    path_indices,
    K,
    r,
    T,
    N,
):
    """
    Simulate delta hedging of a put seller for a specific path.

    path_indices: list of 0s and 1s indicating up (0) or down (1) moves
    """
    dt = T / N

    # Initialize tracking variables
    stock_prices = []
    option_prices = []
    deltas = [0]

    # Grouped by time for later cash account calculation
    cash_flows_grouped = defaultdict(list)
    cash_flows_grouped[0].append(
        CashFlow(
            type="sell_put",
            value_usd=option_tree[0, 0],
            value_share=0,
        ),
    )

    i, j = 0, 0
    tree_indices = [(i, j)]
    for index in path_indices:
        i += 1
        j += index
        tree_indices.append((i, j))

    # We skip the last index as it doesn't have delta for calculation
    for time, tree_index in enumerate(tree_indices[:-1]):
        stock_price = stock_tree[tree_index]
        stock_prices.append(stock_price)
        option_prices.append(option_tree[tree_index])

        delta = delta_tree[tree_index]
        deltas.append(delta)
        diff = deltas[-1] - deltas[-2]

        if diff != 0:
            action_type: Literal["short_shares", "long_shares"] = (
                "short_shares"
                if diff < 0
                else "long_shares"
            )
            cash_flow = CashFlow(
                type=action_type,
                value_usd=abs(diff) * stock_price,
                value_share=abs(diff),
            )
            cash_flows_grouped[time].append(cash_flow)

    times = list(range(N + 1))
    stock_prices.append(stock_tree[tree_indices[-1]])
    option_prices.append(option_tree[tree_indices[-1]])
    deltas.append(None)
    if stock_prices[-1] < K:
        cash_flow = CashFlow(
            type="exercise_put",
            value_usd=K,
            value_share=0,
        )
        cash_flows_grouped[N].append(cash_flow)

    # TODO: implement
    cash_account = [0]
    for time in range(N + 1):
        # Take into account interest
        cash_account.append(cash_account[-1] * np.exp(r * dt))
        cash_flows = cash_flows_grouped[time]
        for cash_flow in cash_flows:
            match cash_flow.type:
                case "sell_put":
                    cash_account[-1] += cash_flow.value_usd
                case "short_shares":
                    cash_account[-1] += cash_flow.value_usd
                case "long_shares":
                    cash_account[-1] -= cash_flow.value_usd
                case "exercise_put":
                    cash_account[-1] -= cash_flow.value_usd

    df = pd.DataFrame()
    df['Time'] = times
    df['Stock Price'] = stock_prices
    df['Option Price'] = option_prices
    df['Delta'] = deltas[1:]
    df['Cash Account'] = cash_account[1:]

    return df, cash_flows_grouped


# Main execution
def main():
    # Parameters from Q26
    S0 = 180
    K = 182
    r = 0.02
    sigma = 0.25
    T = 0.5  # 6 months
    N = 25

    print("Question 26: American Put Option Pricing and Delta Hedging")
    print("=" * 60)
    print(f"Parameters: S0={S0}, K={K}, r={r*100}%, σ={sigma*100}%, T={T} years, N={N} steps")
    print()

    # Part (a): Price the American Put option
    option_price, stock_tree, option_tree, delta_tree = binomial_american_option_with_delta(S0, K, r, sigma, T, N, 'put')

    print(f"American Put Option Price: ${option_price:.2f}")
    print()

    # Random path where the stock price mostly goes down
    path_indices = [0, 1, 1, 1, 1] * 5
    # path_indices = [0] * 25
    df, cash_flows_grouped = simulate_delta_hedging_path_put_seller(
        stock_tree=stock_tree,
        option_tree=option_tree,
        delta_tree=delta_tree,
        path_indices=path_indices,
        K=K,
        r=r,
        T=T,
        N=N,
    )

    print(df)
    pprint.pp(cash_flows_grouped)


if __name__ == "__main__":
    main()
