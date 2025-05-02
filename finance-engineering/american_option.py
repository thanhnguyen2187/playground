import sympy as sp
from dataclasses import dataclass
from typing import Set, Tuple
from pprint import pp


@dataclass(eq=True, frozen=True)
class Node:
    up_count: int
    down_count: int
    value: float


def sorted_tuple(t: Tuple):
    return tuple(sorted(t))


def calculate_risk_neutral_probability(r, dt, u, d):
    """
    Calculate risk-neutral probability (often denoted as ``p``).

    :param r: risk-free rate (e.g. ``0.1`` for 10%)
    :param dt: time step size in year (e.g. ``1`` for one year; ``0.5`` for 6 months)
    :param u: down factor (e.g. ``0.8`` for -20%)
    :param d: up factor (e.g. ``1.2`` for 20%)
    :return:
    """
    return (sp.exp(r * dt) - d) / (u - d)


def calculate_discount_factor(r, dt):
    """
    Calculate the discount factor.

    :param dt: time step size in year (e.g. ``1`` for one year; ``0.5`` for 6 months)
    :param r: risk-free rate (e.g. ``0.1`` for 10%)
    :return:
    """

    return sp.exp(-r * dt)


def calculate_next_nodes(prev_nodes: Set[Node], u, d):
    """
    Calculate the next nodes in a binomial tree.

    :param prev_nodes: a set of previous nodes
    :param u: up factor (e.g. ``1.2`` for 20%)
    :param d: down factor (e.g. ``0.8`` for -20%)
    :return:
    """
    next_nodes = set()

    for node_current in prev_nodes:
        node_up = Node(
            value=node_current.value * u,
            up_count=node_current.up_count + 1,
            down_count=node_current.down_count,
        )
        node_down = Node(
            value=node_current.value * d,
            up_count=node_current.up_count,
            down_count=node_current.down_count + 1,
        )
        next_nodes.add(node_up)
        next_nodes.add(node_down)

    return next_nodes


def generate_binomial_tree(S0, u, d, N):
    """
    Generate a binomial tree for a stock.

    :param S0: initial stock price
    :param u: up factor (e.g. ``1.2`` for 20%)
    :param d: down factor (e.g. ``0.8`` for -20%)
    :param N: number of steps
    :return:
    """

    initial_node = Node(value=S0, up_count=0, down_count=0)
    tree = [{initial_node}]

    for _ in range(N):
        next_nodes = calculate_next_nodes(prev_nodes=tree[-1], u=u, d=d)
        tree.append(next_nodes)

    return tree


def main():
    tree = generate_binomial_tree(
        S0=145,
        u=1.2,
        d=0.8,
        T=2,
        N=2,
        r=0.1,
    )
    pp(tree)


if __name__ == "__main__":
    main()