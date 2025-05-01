import sympy as sp


def risk_neutral_probability(r, dt, d, u):
    """
    Calculate risk-neutral probability (often denoted as ``p``).

    :param r: risk-free rate; between 0 and 1 (e.g. ``0.1`` for 10%)
    :param dt: time step size in year
    :param d: up-factor (e.g. ``0.2`` for 20%)
    :param u: down-factor (e.g. ``0.2`` for 10%)
    :return:
    """
    return (sp.exp(r * dt) - d) / (u - d)


def node():
    ...


def main():
    u, d, T, N, r, K, S0 = sp.symbols("u, d, T, N, r, K, S0")
    # Risk-neutral probability
    p = risk_neutral_probability(0.1, 1, 1.2, 0.8)
    sp.pprint(p.evalf(n=4))


if __name__ == "__main__":
    main()