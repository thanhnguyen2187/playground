import random
import sympy as sp
from typing import Literal


def asian_option_mc(
    option_type: Literal["call", "put"],
    S0,
    K,
    T,
    r,
    u,
    d,
    N,
    M,
):
    """
    Calculate Asian option price using Monte Carlo simulation.

    :param option_type: either "call" or "put"
    :param S0: initial stock price
    :param K: strike price
    :param T: time to maturity in years
    :param r: risk-free interest rate
    :param u: up factor
    :param d: down factor
    :param N: number of time steps
    :param M: number of simulations
    :return: tuple of ``(stock_price_path, option_price)``
    """
    if option_type not in ["call", "put"]:
        raise ValueError("option_type must be either 'call' or 'put'")

    S0 = sp.Float(S0)
    K = sp.Float(K)
    T = sp.Float(T)
    r = sp.Float(r)


    # Calculate time step and factors
    dt = T / N
    # sigma = sp.sqrt(sp.ln(u) * sp.ln(1/d)) / sp.sqrt(dt)
    p = (sp.exp(r * dt) - d) / (u - d)

    # Convert to float for probability comparison
    discount = sp.exp(-r * T)

    # Initialize arrays for paths and option values
    S = [[S0 for _ in range(N + 1)] for _ in range(M)]
    Asian = [sp.Float(0) for _ in range(M)]

    for j in range(M):
        Total = S0

        for i in range(1, N+1):
            # Generate random movement (up or down)
            if random.random() < p:
                S[j][i] = S[j][i-1] * u
            else:
                S[j][i] = S[j][i-1] * d

            Total += S[j][i]

        # Calculate Asian option payoff
        avg_price = Total / (N + 1)
        payoff = (
            sp.Max(avg_price - K, sp.Integer(0))
            if option_type == "call"
            else sp.Max(K - avg_price, sp.Integer(0))
        )
        Asian[j] = discount * payoff

    # Convert to numerical form for the final result
    Asian_numerical = [float(val.evalf()) for val in Asian]

    return S, Asian_numerical


def get_asian_option_price(option_type, S0, K, T, r, u, d, N, M):
    """
    Calculate the average Asian option price from Monte Carlo simulation.

    :return: Average option price and standard error
    """
    _, Asian_prices = asian_option_mc(
        option_type=option_type,
        S0=S0,
        K=K,
        T=T,
        r=r,
        u=u,
        d=d,
        N=N,
        M=M,
    )

    mean_price = sum(Asian_prices) / len(Asian_prices)

    # Calculate standard error
    if M > 1:
        variance = sum((x - mean_price) ** 2 for x in Asian_prices) / (M - 1)
        std_error = sp.sqrt(variance / M)
    else:
        std_error = sp.Rational(0)

    return float(mean_price), float(std_error)


def main():
    option_type = "call"
    u = 1.2
    d = 0.8
    T = 2
    N = 2
    r = 0
    K = 90
    S0 = 100
    M = 100_000
    price = get_asian_option_price(
        option_type=option_type,
        S0=S0,
        K=K,
        T=T,
        r=r,
        u=u,
        d=d,
        N=N,
        M=M,
    )
    sp.pprint(price)


if __name__ == "__main__":
    main()